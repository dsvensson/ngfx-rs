use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Strips the enum type name (case-insensitive, ignoring underscores) from
/// each variant. Lets bindgen emit `NGFX_Result::Success` instead of
/// `NGFX_Result::NGFX_Result_Success`. Handles the SDK's mixed CamelCase /
/// SCREAMING_SNAKE_CASE variant styles uniformly.
#[derive(Debug)]
struct StripEnumPrefix;

impl bindgen::callbacks::ParseCallbacks for StripEnumPrefix {
    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        // Bindgen prepends the C keyword (`enum `, `struct `, `union `) to the
        // type name when passing it to callbacks — strip it.
        let enum_name = enum_name?
            .trim_start_matches("enum ")
            .trim_start_matches("struct ")
            .trim_start_matches("union ");
        // Walk both names letter-by-letter, ignoring underscores and case.
        // The cut lands at the longest common prefix where the variant
        // already sits at a word boundary (next char is `_`). This handles:
        //   NGFX_Result_Success                  matched against NGFX_Result
        //   NGFX_GraphicsCapture_HUD_POSITION_*  matched against NGFX_GraphicsCapture_HudPosition
        //   NGFX_GraphicsCapture_HVVM_*          matched against NGFX_GraphicsCapture_HvvmMode
        //   (the latter accepts a partial match because the variant uses an
        //    abbreviated tail.)
        let en: Vec<char> = enum_name
            .chars()
            .filter(|c| *c != '_')
            .flat_map(char::to_lowercase)
            .collect();
        let var: Vec<char> = original_variant_name
            .chars()
            .filter(|c| *c != '_')
            .flat_map(char::to_lowercase)
            .collect();

        // Best cut: walk forward, track the longest run where the next char in
        // the *original* variant is an underscore (i.e. word boundary).
        let mut alpha_idx = 0;
        let mut best_cut: Option<usize> = None;
        for (i, c) in original_variant_name.char_indices() {
            if c == '_' {
                // We're at a word boundary in the variant. Is the alpha prefix
                // so far a prefix of the enum name?
                if alpha_idx > 0 && alpha_idx <= en.len() && var[..alpha_idx] == en[..alpha_idx] {
                    best_cut = Some(i);
                }
                continue;
            }
            // Past matching point — stop.
            if alpha_idx >= en.len() || alpha_idx >= var.len() {
                break;
            }
            if var[alpha_idx] != en[alpha_idx] {
                break;
            }
            alpha_idx += 1;
        }

        let cut = best_cut?;
        let stripped = original_variant_name[cut..].trim_start_matches('_');
        if stripped.is_empty() {
            return None;
        }
        let needs_prefix = stripped.chars().next().is_some_and(|c| c.is_ascii_digit());
        Some(if needs_prefix {
            format!("_{stripped}")
        } else {
            stripped.to_string()
        })
    }
}

const FEATURES: &[(&str, &str)] = &[
    ("d3d12", "NGFX_SYS_D3D12"),
    ("vulkan", "NGFX_SYS_VULKAN"),
    ("opengl", "NGFX_SYS_OPENGL"),
    ("cuda", "NGFX_SYS_CUDA"),
    ("cudart", "NGFX_SYS_CUDART"),
    ("graphics-capture", "NGFX_SYS_GRAPHICS_CAPTURE"),
    ("gpu-trace", "NGFX_SYS_GPU_TRACE"),
    ("system-profiling", "NGFX_SYS_SYSTEM_PROFILING"),
];

fn feature_enabled(feat: &str) -> bool {
    let env_name = format!("CARGO_FEATURE_{}", feat.to_uppercase().replace('-', "_"));
    env::var_os(env_name).is_some()
}

fn rerun_for_headers(dir: &Path) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rerun_for_headers(&path);
        } else if path.extension().is_some_and(|e| e == "h") {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    let sdk_dir = env::var_os("NGFX_SDK_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join("..").join("ngfx-sdk").join("0.9.0"));
    let sdk_include = sdk_dir.join("include");

    println!("cargo:rerun-if-env-changed=NGFX_SDK_DIR");
    println!("cargo:rerun-if-changed=src/shim.c");
    println!("cargo:rerun-if-changed=src/shim.h");
    rerun_for_headers(&sdk_include);

    if feature_enabled("d3d12") && target_os != "windows" {
        panic!("ngfx-sys: feature `d3d12` requires target_os = \"windows\"");
    }

    // 1) Compile shim.c with cc.
    let mut cc_build = cc::Build::new();
    cc_build
        .file("src/shim.c")
        .include(&sdk_include)
        .warnings(false)
        .flag_if_supported("-Wno-unused-function")
        .flag_if_supported("-Wno-implicit-function-declaration")
        .flag_if_supported("/wd4505");

    for (feat, define) in FEATURES {
        if feature_enabled(feat) {
            cc_build.define(define, None);
        }
    }
    if feature_enabled("verbose-logging") {
        cc_build.define("NGFX_VERBOSE_LOGGING", None);
    }
    cc_build.compile("ngfx_sys_shim");

    // Platform-specific runtime deps used by the inline SDK helpers.
    if target_os == "windows" {
        // NGFX_Do_EnumerateInstallationsFromRegistry uses RegOpenKeyExW etc.
        println!("cargo:rustc-link-lib=dylib=advapi32");
        // SHGetFolderPathW / Known Folders via shlobj_core.h.
        println!("cargo:rustc-link-lib=dylib=shell32");
    } else if target_os == "linux" {
        // dlopen / dlsym.
        println!("cargo:rustc-link-lib=dylib=dl");
    }

    // 2) Generate Rust bindings from shim.h.
    let mut bindings = bindgen::Builder::default()
        .header("src/shim.h")
        .clang_arg(format!("-I{}", sdk_include.display()))
        .use_core()
        .ctypes_prefix("::core::ffi")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .parse_callbacks(Box::new(StripEnumPrefix))
        .derive_default(true)
        .derive_debug(true)
        .derive_copy(true)
        .allowlist_function("ngfx_sys_.*")
        .allowlist_function("NGFX_.*")
        .allowlist_type("NGFX_.*")
        .allowlist_type("Vk[A-Z].*")
        .allowlist_var("NGFX_.*")
        .blocklist_type("_GUID|HWND__|HINSTANCE__|tag.*|_RTL_.*|_IMAGE_.*")
        .layout_tests(true);

    for (feat, define) in FEATURES {
        if feature_enabled(feat) {
            bindings = bindings.clang_arg(format!("-D{}", define));
        }
    }
    if feature_enabled("verbose-logging") {
        bindings = bindings.clang_arg("-DNGFX_VERBOSE_LOGGING");
    }

    let bindings = bindings.generate().expect("bindgen failed");
    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(&out_path)
        .expect("failed to write bindings.rs");
}
