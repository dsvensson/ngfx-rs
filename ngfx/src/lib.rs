//! Safe Rust bindings for the NVIDIA Nsight Graphics SDK.
//!
//! Wraps [`ngfx_sys`] with `Result`-returning entry points, RAII session types,
//! and `ash` interop for Vulkan handles. The crate ships only stubs; a running
//! Nsight Graphics installation is required at runtime to populate the function
//! table.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub use ngfx_sys as sys;

#[cfg(any(feature = "gpu-trace", feature = "graphics-capture"))]
use std::path::Path;

#[cfg(feature = "vulkan")]
pub mod vulkan;

/// Re-export of the raw `NGFX_Result` enum from `ngfx-sys`.
pub use sys::NGFX_Result as RawResult;

/// SDK encodes struct versions as `sizeof(T) | (version << 16)` via the
/// `NGFX_MAKE_STRUCT_VERSION` macro, which bindgen can't translate.
#[cfg(feature = "vulkan")]
#[inline]
pub(crate) const fn struct_version<T>(version: u32) -> u32 {
    (core::mem::size_of::<T>() as u32) | (version << 16)
}

#[derive(Debug, thiserror::Error)]
#[error("NGFX call failed: {0:?}")]
pub struct Error(pub RawResult);

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn check(r: RawResult) -> Result<()> {
    if r == RawResult::Success {
        Ok(())
    } else {
        Err(Error(r))
    }
}

/// Installs the default library loader.
///
/// On Windows the default tries the absolute path the SDK supplies, and falls
/// back to `LoadLibraryW(basename)` — which returns the existing module handle
/// when the DLL is already loaded by an Nsight launcher (`ngfx-capture.exe`,
/// `ngfx-gputrace-profiler.exe`, etc.). On Unix it forwards to
/// `NGFX_LoadLib_NoVerification`.
///
/// Must be called once before any `inject` operation. The loader trusts the
/// path you (or the SDK) supply — use it only when you control or trust the
/// Nsight Graphics installation directory. For production, install a
/// signing-verified loader yourself with [`set_library_load_fn`].
pub fn set_library_load_default() {
    unsafe {
        #[cfg(windows)]
        sys::ngfx_sys_SetLibraryLoadFn(Some(win::smart_loader));
        #[cfg(not(windows))]
        sys::ngfx_sys_SetLibraryLoadFn(Some(sys::ngfx_sys_LoadLib_NoVerification));
    }
}

/// Probe the current process for already-loaded Nsight Graphics injection
/// DLLs and derive the install root from whichever one is loaded.
///
/// Useful when the app is launched by `ngfx-capture.exe`,
/// `ngfx-gputrace-profiler.exe`, or another Nsight launcher that pre-loads
/// the activity DLLs into the process. Returns `None` on non-Windows or when
/// no known DLL is loaded.
pub fn detect_install_path() -> Option<std::path::PathBuf> {
    #[cfg(windows)]
    {
        win::detect_install_path()
    }
    #[cfg(not(windows))]
    {
        None
    }
}

/// Installs a custom library loader. The callback is invoked with the absolute
/// path to each NGFX-managed DLL/SO the SDK wishes to load.
///
/// # Safety
///
/// The callback must follow the SDK contract: return `null` on failure,
/// otherwise a process-stable module handle (HMODULE / dlopen result). The
/// pointer it receives is a NUL-terminated `wchar_t*` on Windows / `char*`
/// elsewhere and must not be retained past the call.
pub unsafe fn set_library_load_fn(fn_ptr: sys::NGFX_LoadLibraryFnPtr) {
    unsafe { sys::ngfx_sys_SetLibraryLoadFn(fn_ptr) }
}

/// Encode `path` as a NUL-terminated buffer in whatever encoding the SDK
/// expects on the current platform (UTF-16 on Windows, UTF-8 on Unix).
#[cfg(any(feature = "gpu-trace", feature = "graphics-capture"))]
pub(crate) fn encode_path(path: &Path) -> PathBuffer {
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        PathBuffer(path.as_os_str().encode_wide().chain(Some(0)).collect())
    }
    #[cfg(not(windows))]
    {
        let mut v = path.to_string_lossy().into_owned().into_bytes();
        v.push(0);
        PathBuffer(v)
    }
}

#[cfg(all(windows, any(feature = "gpu-trace", feature = "graphics-capture")))]
pub(crate) struct PathBuffer(Vec<u16>);
#[cfg(all(not(windows), any(feature = "gpu-trace", feature = "graphics-capture")))]
pub(crate) struct PathBuffer(Vec<u8>);

#[cfg(any(feature = "gpu-trace", feature = "graphics-capture"))]
impl PathBuffer {
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const sys::NGFX_PathChar {
        self.0.as_ptr() as *const _
    }
}

/// Activity types the SDK exposes.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ActivityType {
    GraphicsCapture,
    GpuTrace,
    SystemProfiling,
}

impl ActivityType {
    pub(crate) fn to_sys(self) -> sys::NGFX_ActivityType {
        match self {
            Self::GraphicsCapture => sys::NGFX_ActivityType::GraphicsCapture,
            Self::GpuTrace => sys::NGFX_ActivityType::GPUTrace,
            Self::SystemProfiling => sys::NGFX_ActivityType::SystemProfiling,
        }
    }
}

/// `true` if the given activity has been injected into the current process.
pub fn is_activity_injected(activity: ActivityType) -> Result<bool> {
    let mut out = false;
    check(unsafe { sys::ngfx_sys_IsActivityInjected(activity.to_sys(), &mut out) })?;
    Ok(out)
}

/// `true` if the given activity has been initialized (after `inject` +
/// `initialize` have both been called).
pub fn is_activity_initialized(activity: ActivityType) -> Result<bool> {
    let mut out = false;
    check(unsafe { sys::ngfx_sys_IsActivityInitialized(activity.to_sys(), &mut out) })?;
    Ok(out)
}

#[cfg(windows)]
mod win {
    use std::ffi::{OsStr, OsString, c_void};
    use std::os::windows::ffi::{OsStrExt, OsStringExt};
    use std::path::{Path, PathBuf};
    use std::ptr;

    type HMODULE = *mut c_void;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn LoadLibraryW(name: *const u16) -> HMODULE;
        fn GetModuleHandleW(name: *const u16) -> HMODULE;
        fn GetModuleFileNameW(module: HMODULE, filename: *mut u16, size: u32) -> u32;
    }

    fn to_wide(s: &OsStr) -> Vec<u16> {
        s.encode_wide().chain(Some(0)).collect()
    }

    /// Loader used by [`crate::set_library_load_default`]. Tries the absolute
    /// path the SDK builds from `installationPath`; on failure, retries with
    /// just the file name, which lets Windows return an existing handle when
    /// the DLL is already loaded by an Nsight launcher.
    pub(crate) unsafe extern "C" fn smart_loader(name: *const u16) -> *mut c_void {
        if name.is_null() {
            return ptr::null_mut();
        }
        // First: try as the SDK gave it to us.
        let h = unsafe { LoadLibraryW(name) };
        if !h.is_null() {
            return h;
        }
        // Compute string length and extract basename.
        let mut len = 0;
        unsafe {
            while *name.add(len) != 0 {
                len += 1;
            }
        }
        let slice = unsafe { std::slice::from_raw_parts(name, len) };
        let os = OsString::from_wide(slice);
        let path = Path::new(&os);
        let Some(file) = path.file_name() else {
            return ptr::null_mut();
        };
        let wide = to_wide(file);
        unsafe { LoadLibraryW(wide.as_ptr()) }
    }

    /// Walk known NGFX injection DLLs; the first one that is already loaded
    /// in the process gives us the install root via its file path.
    pub(crate) fn detect_install_path() -> Option<PathBuf> {
        const DLLS: &[&str] = &[
            "WarpVizTarget.dll",
            "WarpViz.Injection.dll",
            "ngfx-capture-interception.dll",
            "ngfx-capture-injection.dll",
            "ngfx-api-bootstrap.dll",
            "ToolsInjection64.dll",
        ];
        for dll in DLLS {
            let wide = to_wide(OsStr::new(dll));
            let handle = unsafe { GetModuleHandleW(wide.as_ptr()) };
            if handle.is_null() {
                continue;
            }
            let mut buf = vec![0u16; 4096];
            let n = unsafe { GetModuleFileNameW(handle, buf.as_mut_ptr(), buf.len() as u32) };
            if n == 0 || n as usize >= buf.len() {
                continue;
            }
            buf.truncate(n as usize);
            let full = PathBuf::from(OsString::from_wide(&buf));
            // .../<install>/target/<variant>/<dll>  →  go up three.
            if let Some(install) = full.parent().and_then(Path::parent).and_then(Path::parent) {
                return Some(install.to_path_buf());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// With no Nsight install loaded, the function table holds NotImplemented
    /// stubs — exercises the full ngfx-sys -> NGFX_Globals -> stub path.
    #[cfg(feature = "vulkan")]
    #[test]
    fn vulkan_frame_boundary_routes_to_not_implemented() {
        let queue = ash::vk::Queue::null();
        let err = vulkan::frame_boundary(queue).unwrap_err();
        assert_eq!(err.0, RawResult::NotImplemented);
    }

    #[test]
    fn activity_state_queries_round_trip() {
        // Before any inject: should be false (and the call itself succeeds).
        let injected = is_activity_injected(ActivityType::GpuTrace).unwrap();
        assert!(!injected);
    }
}
