//! Graphics Capture activity: serialize a frame or sequence of frames to a
//! `.nsight-gfxcap` bundle for offline replay/analysis in Nsight Graphics.
//!
//! Lifecycle (Vulkan):
//!   1. [`Session::inject_vulkan`] — before `VkInstance` creation.
//!   2. [`Session::initialize_vulkan`] — after `VkDevice` is ready.
//!   3. [`Session::request_vulkan`] — schedule a capture for a frame/range.
//!   4. The capture is triggered automatically at the next matching delimiter,
//!      or manually via [`Session::start_vulkan`] / [`Session::stop_vulkan`].

use std::path::Path;

use crate::{Result, check, encode_path, struct_version, sys};

pub use sys::{
    NGFX_GraphicsCapture_Compression as Compression, NGFX_GraphicsCapture_Delimiter as Delimiter,
    NGFX_GraphicsCapture_HudPosition as HudPosition, NGFX_GraphicsCapture_HvvmMode as HvvmMode,
};

/// `NGFX_GraphicsCapture_InjectionSettings`. Constructed via the SDK's
/// `SetDefaults`; mutate fields on `raw` for advanced configuration.
pub struct Settings {
    pub raw: sys::NGFX_GraphicsCapture_InjectionSettings,
}

impl Settings {
    pub fn new() -> Self {
        let mut raw: sys::NGFX_GraphicsCapture_InjectionSettings = unsafe { core::mem::zeroed() };
        let r = unsafe { sys::ngfx_sys_GraphicsCapture_InjectionSettings_SetDefaults(&mut raw) };
        debug_assert_eq!(r, sys::NGFX_Result::Success);
        Self { raw }
    }

    pub fn frame_count(mut self, n: u32) -> Self {
        self.raw.frameCount = n;
        self
    }

    pub fn bundle_replayer(mut self, yes: bool) -> Self {
        self.raw.bundleReplayer = yes;
        self
    }

    pub fn compression(mut self, c: Compression) -> Self {
        self.raw.compression = c;
        self
    }

    pub fn no_hud(mut self, yes: bool) -> Self {
        self.raw.noHUD = yes;
        self
    }

    pub fn hud_position(mut self, p: HudPosition) -> Self {
        self.raw.hudPosition = p;
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

/// In-process Graphics Capture session.
pub struct Session {
    _private: (),
}

impl Session {
    /// Inject Graphics Capture into the current process. `installation_path`
    /// is the Nsight Graphics root install dir.
    #[cfg(feature = "vulkan")]
    pub fn inject_vulkan(installation_path: &Path, settings: &mut Settings) -> Result<Self> {
        crate::set_library_load_default();
        let path = encode_path(installation_path);
        let mut params = sys::NGFX_GraphicsCapture_Inject_Vulkan_Params_V1 {
            version: struct_version::<sys::NGFX_GraphicsCapture_Inject_Vulkan_Params_V1>(1),
            installationPath: path.as_ptr(),
            settings: &mut settings.raw,
        };
        check(unsafe { sys::ngfx_sys_GraphicsCapture_Inject_Vulkan(&mut params) })?;
        Ok(Self { _private: () })
    }

    #[cfg(feature = "vulkan")]
    pub fn initialize_vulkan(&self) -> Result<()> {
        let mut params = sys::NGFX_GraphicsCapture_InitializeActivity_Vulkan_Params_V1 {
            version: struct_version::<sys::NGFX_GraphicsCapture_InitializeActivity_Vulkan_Params_V1>(
                1,
            ),
        };
        check(unsafe { sys::ngfx_sys_GraphicsCapture_InitializeActivity_Vulkan(&mut params) })
    }

    /// Schedule a capture of `frames_to_capture` frames (1..=60), starting
    /// `frames_before_start` frames from now, delimited by `delimiter`.
    #[cfg(feature = "vulkan")]
    pub fn request_vulkan(
        &self,
        delimiter: Delimiter,
        frames_before_start: u32,
        frames_to_capture: u32,
    ) -> Result<()> {
        let mut params = sys::NGFX_GraphicsCapture_RequestCapture_Vulkan_Params_V1 {
            version: struct_version::<sys::NGFX_GraphicsCapture_RequestCapture_Vulkan_Params_V1>(1),
            delimiter,
            framesBeforeStart: frames_before_start,
            framesToCapture: frames_to_capture,
        };
        check(unsafe { sys::ngfx_sys_GraphicsCapture_RequestCapture_Vulkan(&mut params) })
    }

    /// Manually start a capture (use when not relying on a scheduled
    /// `request_vulkan` trigger).
    #[cfg(feature = "vulkan")]
    pub fn start_vulkan(&self) -> Result<()> {
        let mut params = sys::NGFX_GraphicsCapture_StartCapture_Vulkan_Params_V1 {
            version: struct_version::<sys::NGFX_GraphicsCapture_StartCapture_Vulkan_Params_V1>(1),
        };
        check(unsafe { sys::ngfx_sys_GraphicsCapture_StartCapture_Vulkan(&mut params) })
    }

    /// Manually stop a capture.
    #[cfg(feature = "vulkan")]
    pub fn stop_vulkan(&self) -> Result<()> {
        let mut params = sys::NGFX_GraphicsCapture_StopCapture_Vulkan_Params_V1 {
            version: struct_version::<sys::NGFX_GraphicsCapture_StopCapture_Vulkan_Params_V1>(1),
        };
        check(unsafe { sys::ngfx_sys_GraphicsCapture_StopCapture_Vulkan(&mut params) })
    }
}
