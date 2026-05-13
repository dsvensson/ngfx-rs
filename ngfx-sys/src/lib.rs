#![no_std]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, improper_ctypes)]
#![allow(clippy::missing_safety_doc, clippy::useless_transmute)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Pointer to the process-wide NGFX function table. The table is initially
/// populated with `_NotImplemented` stubs; entries are mutated by
/// `ngfx_sys_LoadInjectionLib` / `ngfx_sys_InjectActivity` once a Nsight
/// Graphics installation is loaded.
#[inline]
pub fn globals() -> *mut NGFX_Globals {
    unsafe { ngfx_sys_get_globals() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "vulkan")]
    #[test]
    fn vulkan_frame_boundary_returns_not_implemented_without_init() {
        // NGFX_MAKE_STRUCT_VERSION(T, v) == sizeof(T) | (v << 16). Bindgen does
        // not expand the sizeof-bearing macro, so compute it inline.
        let version =
            (core::mem::size_of::<NGFX_FrameBoundary_Vulkan_Params_V1>() as u32) | (1u32 << 16);
        let mut params = NGFX_FrameBoundary_Vulkan_Params_V1 {
            version,
            queue: core::ptr::null_mut(),
            outputResources: core::ptr::null_mut(),
            numOutputResources: 0,
        };
        let r = unsafe { ngfx_sys_FrameBoundary_Vulkan(&mut params) };
        assert_eq!(r, NGFX_Result::NotImplemented);
    }

    #[test]
    fn globals_pointer_non_null() {
        assert!(!globals().is_null());
    }
}
