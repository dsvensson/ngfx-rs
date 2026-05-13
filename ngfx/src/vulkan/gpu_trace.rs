//! GPU Trace activity: in-process trace capture lifecycle.
//!
//! Lifecycle:
//!   1. [`Session::inject`] — before `VkInstance` creation.
//!   2. [`Session::initialize`] — after `VkDevice` is ready.
//!   3. [`Session::activate`] with the queue that owns trace resources.
//!   4. [`Session::start`] → run workload → [`Session::stop`].
//!   5. Optionally [`Session::wait_for_status`] to confirm draining/idle.

use std::path::Path;

use ash::vk;
use ash::vk::Handle;

use crate::{Result, check, encode_path, struct_version, sys};

pub use sys::{NGFX_GPUTrace_Status as Status, NGFX_GPUTrace_StopTraceFlag as StopFlag};

/// `NGFX_GPUTrace_InjectionSettings`. Built via the SDK's `SetDefaults`
/// (which uses sentinels for unset fields — zero-init would be misread).
pub struct Settings {
    pub raw: sys::NGFX_GPUTrace_InjectionSettings,
}

impl Settings {
    /// Defaults as filled by the SDK's `NGFX_GPUTrace_InjectionSettings_SetDefaults`.
    pub fn new() -> Self {
        let mut raw: sys::NGFX_GPUTrace_InjectionSettings = unsafe { core::mem::zeroed() };
        let r = unsafe { sys::ngfx_sys_GPUTrace_InjectionSettings_SetDefaults(&mut raw) };
        debug_assert_eq!(r, sys::NGFX_Result::Success);
        Self { raw }
    }

    /// SDK-controlled start/stop, HUD hidden, GPU clock locked to base —
    /// suitable for headless / in-app capture. Caller bumps `max_duration_ms`
    /// for long workloads.
    pub fn in_app() -> Self {
        let mut s = Self::new();
        s.raw.startEvent = sys::NGFX_GPUTrace_StartEvent::NGFXSDK;
        s.raw.stopEvent = sys::NGFX_GPUTrace_StopEvent::NGFXSDK;
        // Defensive: SetDefaults may leave frameCount=1 even though
        // stopEvent=NGFXSDK should render stopParams unused.
        s.raw.stopParams.frameCount = u32::MAX;
        s.raw.hudPosition = sys::NGFX_GPUTrace_HUDPosition::Hidden;
        s.raw.vsyncMode = sys::NGFX_GPUTrace_VSyncMode::Off;
        s.raw.gpuClockMode = sys::NGFX_GPUTrace_GPUClockMode::LockToBase;
        s
    }

    pub fn max_duration_ms(mut self, ms: u32) -> Self {
        self.raw.maxDurationMs = ms;
        self
    }

    pub fn event_buffer_size_kb(mut self, kb: u32) -> Self {
        self.raw.eventBufferSizeKB = kb;
        self
    }

    pub fn timestamp_count(mut self, n: u32) -> Self {
        self.raw.timestampCount = n;
        self
    }

    pub fn collect_screenshot(mut self, yes: bool) -> Self {
        self.raw.collectScreenshot = yes;
        self
    }

    pub fn time_every_action(mut self, yes: bool) -> Self {
        self.raw.timeEveryAction = yes;
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

/// In-process GPU Trace session for a Vulkan application.
///
/// The session struct itself holds no resources; it's a phantom marker
/// representing "GPU Trace has been injected." Dropping it does not
/// uninject — the SDK has no uninject path.
pub struct Session {
    _private: (),
}

impl Session {
    /// `installation_path` is the Nsight Graphics root install dir (the
    /// directory containing `target/windows-desktop-nomad-x64/`).
    pub fn inject(installation_path: &Path, settings: &mut Settings) -> Result<Self> {
        crate::set_library_load_default();
        let path = encode_path(installation_path);
        let mut params = sys::NGFX_GPUTrace_Inject_Vulkan_Params_V1 {
            version: struct_version::<sys::NGFX_GPUTrace_Inject_Vulkan_Params_V1>(1),
            installationPath: path.as_ptr(),
            settings: &mut settings.raw,
        };
        check(unsafe { sys::ngfx_sys_GPUTrace_Inject_Vulkan(&mut params) })?;
        Ok(Self { _private: () })
    }

    pub fn initialize(&self) -> Result<()> {
        let mut params = sys::NGFX_GPUTrace_InitializeActivity_Vulkan_Params_V1 {
            version: struct_version::<sys::NGFX_GPUTrace_InitializeActivity_Vulkan_Params_V1>(1),
        };
        check(unsafe { sys::ngfx_sys_GPUTrace_InitializeActivity_Vulkan(&mut params) })
    }

    /// Allocate trace resources on `queue`. Blocking; needs the host.
    pub fn activate(&self, queue: vk::Queue) -> Result<()> {
        let mut params = sys::NGFX_GPUTrace_ActivateTrace_Vulkan_Params_V1 {
            version: struct_version::<sys::NGFX_GPUTrace_ActivateTrace_Vulkan_Params_V1>(1),
            queue: queue.as_raw() as *mut _,
        };
        check(unsafe { sys::ngfx_sys_GPUTrace_ActivateTrace_Vulkan(&mut params) })
    }

    pub fn start(&self) -> Result<()> {
        let mut params = sys::NGFX_GPUTrace_StartTrace_Vulkan_Params_V1 {
            version: struct_version::<sys::NGFX_GPUTrace_StartTrace_Vulkan_Params_V1>(1),
        };
        check(unsafe { sys::ngfx_sys_GPUTrace_StartTrace_Vulkan(&mut params) })
    }

    /// Stop the trace. `ImmediateCollection` drains synchronously — caller
    /// must guarantee no further work is scheduled on `queue`.
    pub fn stop(&self, queue: vk::Queue, flags: StopFlag) -> Result<()> {
        let mut params = sys::NGFX_GPUTrace_StopTrace_Vulkan_Params_V1 {
            version: struct_version::<sys::NGFX_GPUTrace_StopTrace_Vulkan_Params_V1>(1),
            flags,
            queue: queue.as_raw() as *mut _,
            outputResources: core::ptr::null_mut(),
            numOutputResources: 0,
        };
        check(unsafe { sys::ngfx_sys_GPUTrace_StopTrace_Vulkan(&mut params) })
    }

    /// Current trace status (Inactive / Active / Tracing / Draining / Errored).
    pub fn status(&self) -> Result<Status> {
        let mut params = sys::NGFX_GPUTrace_GetStatus_Params_V1 {
            version: struct_version::<sys::NGFX_GPUTrace_GetStatus_Params_V1>(1),
            status: sys::NGFX_GPUTrace_Status::Inactive,
        };
        check(unsafe { sys::ngfx_sys_GPUTrace_GetStatus(&mut params) })?;
        Ok(params.status)
    }

    /// Block until the trace reaches `status` or `timeout_ms` elapses (use a
    /// negative timeout to wait indefinitely).
    pub fn wait_for_status(&self, status: Status, timeout_ms: i32) -> Result<()> {
        let mut params = sys::NGFX_GPUTrace_WaitForStatus_Params_V1 {
            version: struct_version::<sys::NGFX_GPUTrace_WaitForStatus_Params_V1>(1),
            status,
            timeoutMs: timeout_ms,
        };
        check(unsafe { sys::ngfx_sys_GPUTrace_WaitForStatus(&mut params) })
    }
}

/// True if the GPU Trace activity has been injected into the current process.
pub fn is_injected() -> Result<bool> {
    let mut out = false;
    check(unsafe { sys::ngfx_sys_GPUTrace_IsInjected(&mut out) })?;
    Ok(out)
}

/// True if the GPU Trace activity has been initialized.
pub fn is_initialized() -> Result<bool> {
    let mut out = false;
    check(unsafe { sys::ngfx_sys_GPUTrace_IsInitialized(&mut out) })?;
    Ok(out)
}
