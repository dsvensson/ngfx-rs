//! Vulkan-side entry points (frame-boundary delimiters).

use ash::vk;
use ash::vk::Handle;

use crate::{Result, check, struct_version, sys};

/// Signal an NGFX frame boundary on `queue`.
///
/// Returns `NotImplemented` if no activity has injected its function table —
/// safe to call unconditionally each frame.
pub fn frame_boundary(queue: vk::Queue) -> Result<()> {
    let mut params = sys::NGFX_FrameBoundary_Vulkan_Params_V1 {
        version: struct_version::<sys::NGFX_FrameBoundary_Vulkan_Params_V1>(1),
        queue: queue.as_raw() as *mut _,
        outputResources: core::ptr::null_mut(),
        numOutputResources: 0,
    };
    unsafe { check(sys::ngfx_sys_FrameBoundary_Vulkan(&mut params)) }
}

/// Delimit a Streamline DLSS-FG present boundary.
///
/// `requested_frame_index` counts application-requested presents; for a
/// generated frame boundary, `generated_frame_index` is the zero-based index
/// within the current real frame.
pub fn dlss_fg_present_boundary(
    queue: vk::Queue,
    boundary_type: sys::NGFX_For_Dlss_DLSS_FG_PresentBoundaryType,
    generated_frame_index: u32,
    requested_frame_index: u32,
    present_info: *const sys::VkPresentInfoKHR,
) -> Result<()> {
    let mut params = sys::NGFX_DLSS_FG_PresentBoundary_Vulkan_Params_V1 {
        version: struct_version::<sys::NGFX_DLSS_FG_PresentBoundary_Vulkan_Params_V1>(1),
        type_: boundary_type,
        generatedFrameIndex: generated_frame_index,
        queue: queue.as_raw() as *mut _,
        pPresentInfo: present_info,
        requestedFrameIndex: requested_frame_index,
    };
    unsafe { check(sys::ngfx_sys_DLSS_FG_PresentBoundary_Vulkan(&mut params)) }
}
