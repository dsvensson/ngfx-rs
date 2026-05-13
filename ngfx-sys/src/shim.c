/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * Sole translation unit that includes Impl/NGFX_Core.h, which materializes
 * g_NGFX_Globals (via __declspec(selectany) / __attribute__((weak))) and the
 * NGFX_FUNCTION inline entry points. Every ngfx_sys_* symbol is a one-line
 * forwarder so bindgen has a stable extern surface.
 */

#include "shim.h"
#include "Impl/NGFX_Core.h"

/* Per-API entry-point headers. These define the NGFX_FUNCTION inlines that
 * our trampolines forward to (e.g. NGFX_FrameBoundary_Vulkan in NGFX_Vulkan.h).
 * Must be included in this TU so the inline definitions are visible. */
#ifdef NGFX_SYS_VULKAN
#  include "NGFX_Vulkan.h"
#endif
#ifdef NGFX_SYS_D3D12
#  include "NGFX_D3D12.h"
#endif
#ifdef NGFX_SYS_OPENGL
#  include "NGFX_OpenGL.h"
#endif
#ifdef NGFX_SYS_CUDA
#  include "NGFX_CUDA.h"
#endif
#ifdef NGFX_SYS_CUDART
#  include "NGFX_CUDART.h"
#endif

#ifdef NGFX_SYS_GRAPHICS_CAPTURE
#  include "NGFX_GraphicsCapture_Common.h"
#  if defined(NGFX_SYS_D3D12)
#    include "NGFX_GraphicsCapture_D3D12.h"
#  endif
#  if defined(NGFX_SYS_VULKAN)
#    include "NGFX_GraphicsCapture_Vulkan.h"
#  endif
#endif

#ifdef NGFX_SYS_GPU_TRACE
#  include "NGFX_GPUTrace_Common.h"
#  if defined(NGFX_SYS_D3D12)
#    include "NGFX_GPUTrace_D3D12.h"
#  endif
#  if defined(NGFX_SYS_VULKAN)
#    include "NGFX_GPUTrace_Vulkan.h"
#  endif
#  if defined(NGFX_SYS_OPENGL)
#    include "NGFX_GPUTrace_OpenGL.h"
#  endif
#  if defined(NGFX_SYS_CUDA)
#    include "NGFX_GPUTrace_CUDA.h"
#  endif
#  if defined(NGFX_SYS_CUDART)
#    include "NGFX_GPUTrace_CUDART.h"
#  endif
#endif

#ifdef NGFX_SYS_SYSTEM_PROFILING
#  include "NGFX_SystemProfiling_Common.h"
#  if defined(NGFX_SYS_D3D12)
#    include "NGFX_SystemProfiling_D3D12.h"
#  endif
#  if defined(NGFX_SYS_VULKAN)
#    include "NGFX_SystemProfiling_Vulkan.h"
#  endif
#endif

struct NGFX_Globals* ngfx_sys_get_globals(void) { return &g_NGFX_Globals; }

/* ---- Core lifecycle ---------------------------------------------------- */

void  ngfx_sys_SetLibraryLoadFn(NGFX_LoadLibraryFnPtr fn) { NGFX_SetLibraryLoadFn(fn); }
void* ngfx_sys_LoadLib_NoVerification(const NGFX_PathChar* libName) { return NGFX_LoadLib_NoVerification(libName); }
void  ngfx_sys_FreeLib(void* lib) { NGFX_FreeLib(lib); }
void* ngfx_sys_GetLibHandle(const NGFX_PathChar* libName) { return NGFX_GetLibHandle(libName); }
void* ngfx_sys_GetProc(void* lib, const char* name, void* defaultProc) { return NGFX_GetProc(lib, name, defaultProc); }

NGFX_Result ngfx_sys_EnumerateInstallations(NGFX_InstallationInfo* installations, uint32_t maxInstallations, uint32_t* numInstallations) {
    return NGFX_EnumerateInstallations(installations, maxInstallations, numInstallations);
}
void ngfx_sys_FreeInstallations(NGFX_InstallationInfo* installations, uint32_t numInstallations) {
    NGFX_FreeInstallations(installations, numInstallations);
}

NGFX_Result ngfx_sys_LoadInjectionLib(NGFX_ActivityType activityType, const NGFX_PathChar* installationPath, void* settings, void** loadedLib) {
    return NGFX_LoadInjectionLib(activityType, installationPath, settings, loadedLib);
}
NGFX_Result ngfx_sys_GetActivityLibHandle(NGFX_ActivityType* activityType, bool injectionLib, void** libHandle) {
    return NGFX_GetActivityLibHandle(activityType, injectionLib, libHandle);
}
NGFX_Result ngfx_sys_InjectActivity(NGFX_ActivityType activityType, const NGFX_PathChar* installationPath, void* settings) {
    return NGFX_InjectActivity(activityType, installationPath, settings);
}
NGFX_Result ngfx_sys_IsActivityInjected(NGFX_ActivityType activityType, bool* injected) {
    return NGFX_IsActivityInjected(activityType, injected);
}
NGFX_Result ngfx_sys_IsActivityInitialized(NGFX_ActivityType activityType, bool* initialized) {
    return NGFX_IsActivityInitialized(activityType, initialized);
}

/* ---- Frame boundary --------------------------------------------------- */

#ifdef NGFX_SYS_VULKAN
NGFX_Result ngfx_sys_FrameBoundary_Vulkan(NGFX_FrameBoundary_Vulkan_Params* params) { return NGFX_FrameBoundary_Vulkan(params); }
NGFX_Result ngfx_sys_DLSS_FG_PresentBoundary_Vulkan(NGFX_DLSS_FG_PresentBoundary_Vulkan_Params* params) { return NGFX_DLSS_FG_PresentBoundary_Vulkan(params); }
#endif
#ifdef NGFX_SYS_D3D12
NGFX_Result ngfx_sys_FrameBoundary_D3D12(NGFX_FrameBoundary_D3D12_Params* params) { return NGFX_FrameBoundary_D3D12(params); }
NGFX_Result ngfx_sys_DLSS_FG_PresentBoundary_D3D12(NGFX_DLSS_FG_PresentBoundary_D3D12_Params* params) { return NGFX_DLSS_FG_PresentBoundary_D3D12(params); }
#endif
#ifdef NGFX_SYS_OPENGL
NGFX_Result ngfx_sys_FrameBoundary_OpenGL(NGFX_FrameBoundary_OpenGL_Params* params) { return NGFX_FrameBoundary_OpenGL(params); }
#endif
#ifdef NGFX_SYS_CUDA
NGFX_Result ngfx_sys_FrameBoundary_CUDA(NGFX_FrameBoundary_CUDA_Params* params) { return NGFX_FrameBoundary_CUDA(params); }
#endif
#ifdef NGFX_SYS_CUDART
NGFX_Result ngfx_sys_FrameBoundary_CUDART(NGFX_FrameBoundary_CUDART_Params* params) { return NGFX_FrameBoundary_CUDART(params); }
#endif

/* ---- Graphics Capture -------------------------------------------------- */

#ifdef NGFX_SYS_GRAPHICS_CAPTURE
NGFX_Result ngfx_sys_GraphicsCapture_InjectionSettings_SetDefaults(NGFX_GraphicsCapture_InjectionSettings* settings) {
    return NGFX_GraphicsCapture_InjectionSettings_SetDefaults(settings);
}
#  if defined(NGFX_SYS_D3D12)
NGFX_Result ngfx_sys_GraphicsCapture_Inject_D3D12(NGFX_GraphicsCapture_Inject_D3D12_Params* params) { return NGFX_GraphicsCapture_Inject_D3D12(params); }
NGFX_Result ngfx_sys_GraphicsCapture_InitializeActivity_D3D12(NGFX_GraphicsCapture_InitializeActivity_D3D12_Params* params) { return NGFX_GraphicsCapture_InitializeActivity_D3D12(params); }
NGFX_Result ngfx_sys_GraphicsCapture_RequestCapture_D3D12(NGFX_GraphicsCapture_RequestCapture_D3D12_Params* params) { return NGFX_GraphicsCapture_RequestCapture_D3D12(params); }
NGFX_Result ngfx_sys_GraphicsCapture_StartCapture_D3D12(NGFX_GraphicsCapture_StartCapture_D3D12_Params* params) { return NGFX_GraphicsCapture_StartCapture_D3D12(params); }
NGFX_Result ngfx_sys_GraphicsCapture_StopCapture_D3D12(NGFX_GraphicsCapture_StopCapture_D3D12_Params* params) { return NGFX_GraphicsCapture_StopCapture_D3D12(params); }
#  endif
#  if defined(NGFX_SYS_VULKAN)
NGFX_Result ngfx_sys_GraphicsCapture_Inject_Vulkan(NGFX_GraphicsCapture_Inject_Vulkan_Params* params) { return NGFX_GraphicsCapture_Inject_Vulkan(params); }
NGFX_Result ngfx_sys_GraphicsCapture_InitializeActivity_Vulkan(NGFX_GraphicsCapture_InitializeActivity_Vulkan_Params* params) { return NGFX_GraphicsCapture_InitializeActivity_Vulkan(params); }
NGFX_Result ngfx_sys_GraphicsCapture_RequestCapture_Vulkan(NGFX_GraphicsCapture_RequestCapture_Vulkan_Params* params) { return NGFX_GraphicsCapture_RequestCapture_Vulkan(params); }
NGFX_Result ngfx_sys_GraphicsCapture_StartCapture_Vulkan(NGFX_GraphicsCapture_StartCapture_Vulkan_Params* params) { return NGFX_GraphicsCapture_StartCapture_Vulkan(params); }
NGFX_Result ngfx_sys_GraphicsCapture_StopCapture_Vulkan(NGFX_GraphicsCapture_StopCapture_Vulkan_Params* params) { return NGFX_GraphicsCapture_StopCapture_Vulkan(params); }
#  endif
#endif

/* ---- GPU Trace --------------------------------------------------------- */

#ifdef NGFX_SYS_GPU_TRACE
NGFX_Result ngfx_sys_GPUTrace_IsInjected(bool* injected) { return NGFX_GPUTrace_IsInjected(injected); }
NGFX_Result ngfx_sys_GPUTrace_IsInitialized(bool* initialized) { return NGFX_GPUTrace_IsInitialized(initialized); }
NGFX_Result ngfx_sys_GPUTrace_GetStatus(NGFX_GPUTrace_GetStatus_Params* params) { return NGFX_GPUTrace_GetStatus(params); }
NGFX_Result ngfx_sys_GPUTrace_WaitForStatus(NGFX_GPUTrace_WaitForStatus_Params* params) { return NGFX_GPUTrace_WaitForStatus(params); }
NGFX_Result ngfx_sys_GPUTrace_InjectionSettings_SetDefaults(NGFX_GPUTrace_InjectionSettings* settings) { return NGFX_GPUTrace_InjectionSettings_SetDefaults(settings); }
#  if defined(NGFX_SYS_D3D12)
NGFX_Result ngfx_sys_GPUTrace_Inject_D3D12(NGFX_GPUTrace_Inject_D3D12_Params* params) { return NGFX_GPUTrace_Inject_D3D12(params); }
NGFX_Result ngfx_sys_GPUTrace_InitializeActivity_D3D12(NGFX_GPUTrace_InitializeActivity_D3D12_Params* params) { return NGFX_GPUTrace_InitializeActivity_D3D12(params); }
NGFX_Result ngfx_sys_GPUTrace_ActivateTrace_D3D12(NGFX_GPUTrace_ActivateTrace_D3D12_Params* params) { return NGFX_GPUTrace_ActivateTrace_D3D12(params); }
NGFX_Result ngfx_sys_GPUTrace_StartTrace_D3D12(NGFX_GPUTrace_StartTrace_D3D12_Params* params) { return NGFX_GPUTrace_StartTrace_D3D12(params); }
NGFX_Result ngfx_sys_GPUTrace_StopTrace_D3D12(NGFX_GPUTrace_StopTrace_D3D12_Params* params) { return NGFX_GPUTrace_StopTrace_D3D12(params); }
#  endif
#  if defined(NGFX_SYS_VULKAN)
NGFX_Result ngfx_sys_GPUTrace_Inject_Vulkan(NGFX_GPUTrace_Inject_Vulkan_Params* params) { return NGFX_GPUTrace_Inject_Vulkan(params); }
NGFX_Result ngfx_sys_GPUTrace_InitializeActivity_Vulkan(NGFX_GPUTrace_InitializeActivity_Vulkan_Params* params) { return NGFX_GPUTrace_InitializeActivity_Vulkan(params); }
NGFX_Result ngfx_sys_GPUTrace_ActivateTrace_Vulkan(NGFX_GPUTrace_ActivateTrace_Vulkan_Params* params) { return NGFX_GPUTrace_ActivateTrace_Vulkan(params); }
NGFX_Result ngfx_sys_GPUTrace_StartTrace_Vulkan(NGFX_GPUTrace_StartTrace_Vulkan_Params* params) { return NGFX_GPUTrace_StartTrace_Vulkan(params); }
NGFX_Result ngfx_sys_GPUTrace_StopTrace_Vulkan(NGFX_GPUTrace_StopTrace_Vulkan_Params* params) { return NGFX_GPUTrace_StopTrace_Vulkan(params); }
#  endif
#  if defined(NGFX_SYS_OPENGL)
NGFX_Result ngfx_sys_GPUTrace_Inject_OpenGL(NGFX_GPUTrace_Inject_OpenGL_Params* params) { return NGFX_GPUTrace_Inject_OpenGL(params); }
NGFX_Result ngfx_sys_GPUTrace_InitializeActivity_OpenGL(NGFX_GPUTrace_InitializeActivity_OpenGL_Params* params) { return NGFX_GPUTrace_InitializeActivity_OpenGL(params); }
NGFX_Result ngfx_sys_GPUTrace_ActivateTrace_OpenGL(NGFX_GPUTrace_ActivateTrace_OpenGL_Params* params) { return NGFX_GPUTrace_ActivateTrace_OpenGL(params); }
NGFX_Result ngfx_sys_GPUTrace_StartTrace_OpenGL(NGFX_GPUTrace_StartTrace_OpenGL_Params* params) { return NGFX_GPUTrace_StartTrace_OpenGL(params); }
NGFX_Result ngfx_sys_GPUTrace_StopTrace_OpenGL(NGFX_GPUTrace_StopTrace_OpenGL_Params* params) { return NGFX_GPUTrace_StopTrace_OpenGL(params); }
#  endif
#  if defined(NGFX_SYS_CUDA)
NGFX_Result ngfx_sys_GPUTrace_Inject_CUDA(NGFX_GPUTrace_Inject_CUDA_Params* params) { return NGFX_GPUTrace_Inject_CUDA(params); }
NGFX_Result ngfx_sys_GPUTrace_InitializeActivity_CUDA(NGFX_GPUTrace_InitializeActivity_CUDA_Params* params) { return NGFX_GPUTrace_InitializeActivity_CUDA(params); }
NGFX_Result ngfx_sys_GPUTrace_ActivateTrace_CUDA(NGFX_GPUTrace_ActivateTrace_CUDA_Params* params) { return NGFX_GPUTrace_ActivateTrace_CUDA(params); }
NGFX_Result ngfx_sys_GPUTrace_StartTrace_CUDA(NGFX_GPUTrace_StartTrace_CUDA_Params* params) { return NGFX_GPUTrace_StartTrace_CUDA(params); }
NGFX_Result ngfx_sys_GPUTrace_StopTrace_CUDA(NGFX_GPUTrace_StopTrace_CUDA_Params* params) { return NGFX_GPUTrace_StopTrace_CUDA(params); }
#  endif
#  if defined(NGFX_SYS_CUDART)
NGFX_Result ngfx_sys_GPUTrace_Inject_CUDART(NGFX_GPUTrace_Inject_CUDART_Params* params) { return NGFX_GPUTrace_Inject_CUDART(params); }
NGFX_Result ngfx_sys_GPUTrace_InitializeActivity_CUDART(NGFX_GPUTrace_InitializeActivity_CUDART_Params* params) { return NGFX_GPUTrace_InitializeActivity_CUDART(params); }
NGFX_Result ngfx_sys_GPUTrace_ActivateTrace_CUDART(NGFX_GPUTrace_ActivateTrace_CUDART_Params* params) { return NGFX_GPUTrace_ActivateTrace_CUDART(params); }
NGFX_Result ngfx_sys_GPUTrace_StartTrace_CUDART(NGFX_GPUTrace_StartTrace_CUDART_Params* params) { return NGFX_GPUTrace_StartTrace_CUDART(params); }
NGFX_Result ngfx_sys_GPUTrace_StopTrace_CUDART(NGFX_GPUTrace_StopTrace_CUDART_Params* params) { return NGFX_GPUTrace_StopTrace_CUDART(params); }
#  endif
#endif

/* ---- System Profiling -------------------------------------------------- */

#ifdef NGFX_SYS_SYSTEM_PROFILING
NGFX_Result ngfx_sys_SystemProfiling_InjectionSettings_SetDefaults(NGFX_SystemProfiling_InjectionSettings* settings) { return NGFX_SystemProfiling_InjectionSettings_SetDefaults(settings); }
#  if defined(NGFX_SYS_D3D12)
NGFX_Result ngfx_sys_SystemProfiling_Inject_D3D12(NGFX_SystemProfiling_Inject_D3D12_Params* params) { return NGFX_SystemProfiling_Inject_D3D12(params); }
NGFX_Result ngfx_sys_SystemProfiling_InitializeActivity_D3D12(NGFX_SystemProfiling_InitializeActivity_D3D12_Params* params) { return NGFX_SystemProfiling_InitializeActivity_D3D12(params); }
NGFX_Result ngfx_sys_SystemProfiling_StartProfiling_D3D12(NGFX_SystemProfiling_StartProfiling_D3D12_Params* params) { return NGFX_SystemProfiling_StartProfiling_D3D12(params); }
NGFX_Result ngfx_sys_SystemProfiling_StopProfiling_D3D12(NGFX_SystemProfiling_StopProfiling_D3D12_Params* params) { return NGFX_SystemProfiling_StopProfiling_D3D12(params); }
#  endif
#  if defined(NGFX_SYS_VULKAN)
NGFX_Result ngfx_sys_SystemProfiling_Inject_Vulkan(NGFX_SystemProfiling_Inject_Vulkan_Params* params) { return NGFX_SystemProfiling_Inject_Vulkan(params); }
NGFX_Result ngfx_sys_SystemProfiling_InitializeActivity_Vulkan(NGFX_SystemProfiling_InitializeActivity_Vulkan_Params* params) { return NGFX_SystemProfiling_InitializeActivity_Vulkan(params); }
NGFX_Result ngfx_sys_SystemProfiling_StartProfiling_Vulkan(NGFX_SystemProfiling_StartProfiling_Vulkan_Params* params) { return NGFX_SystemProfiling_StartProfiling_Vulkan(params); }
NGFX_Result ngfx_sys_SystemProfiling_StopProfiling_Vulkan(NGFX_SystemProfiling_StopProfiling_Vulkan_Params* params) { return NGFX_SystemProfiling_StopProfiling_Vulkan(params); }
#  endif
#endif
