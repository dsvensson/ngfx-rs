/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * Bindgen entry header for ngfx-sys. Declares a flat C surface of trampolines
 * that forward to the inline functions defined by the Nsight Graphics SDK.
 * shim.c is the sole TU that materializes g_NGFX_Globals.
 */

#ifndef NGFX_SYS_SHIM_H
#define NGFX_SYS_SHIM_H

#include <stdint.h>
#ifndef __cplusplus
#  include <stdbool.h>
#endif

#include "NGFX_Types.h"

#ifdef NGFX_SYS_VULKAN
#  include "NGFX_Vulkan_Types.h"
#endif
#ifdef NGFX_SYS_D3D12
#  include "NGFX_D3D12_Types.h"
#endif
#ifdef NGFX_SYS_OPENGL
#  include "NGFX_OpenGL_Types.h"
#endif
#ifdef NGFX_SYS_CUDA
#  include "NGFX_CUDA_Types.h"
#endif
#ifdef NGFX_SYS_CUDART
#  include "NGFX_CUDART_Types.h"
#endif

#ifdef NGFX_SYS_GRAPHICS_CAPTURE
#  if defined(NGFX_SYS_D3D12)
#    include "NGFX_GraphicsCapture_D3D12_Types.h"
#  endif
#  if defined(NGFX_SYS_VULKAN)
#    include "NGFX_GraphicsCapture_Vulkan_Types.h"
#  endif
#endif

#ifdef NGFX_SYS_GPU_TRACE
#  include "NGFX_GPUTrace_Common_Types.h"
#  if defined(NGFX_SYS_D3D12)
#    include "NGFX_GPUTrace_D3D12_Types.h"
#  endif
#  if defined(NGFX_SYS_VULKAN)
#    include "NGFX_GPUTrace_Vulkan_Types.h"
#  endif
#  if defined(NGFX_SYS_OPENGL)
#    include "NGFX_GPUTrace_OpenGL_Types.h"
#  endif
#  if defined(NGFX_SYS_CUDA)
#    include "NGFX_GPUTrace_CUDA_Types.h"
#  endif
#  if defined(NGFX_SYS_CUDART)
#    include "NGFX_GPUTrace_CUDART_Types.h"
#  endif
#endif

#ifdef NGFX_SYS_SYSTEM_PROFILING
#  include "NGFX_SystemProfiling_Common_Types.h"
#  if defined(NGFX_SYS_D3D12)
#    include "NGFX_SystemProfiling_D3D12_Types.h"
#  endif
#  if defined(NGFX_SYS_VULKAN)
#    include "NGFX_SystemProfiling_Vulkan_Types.h"
#  endif
#endif

#ifdef __cplusplus
extern "C" {
#endif

struct NGFX_Globals;
struct NGFX_Globals* ngfx_sys_get_globals(void);

/* ---- Core lifecycle ---------------------------------------------------- */

void        ngfx_sys_SetLibraryLoadFn(NGFX_LoadLibraryFnPtr fn);
void*       ngfx_sys_LoadLib_NoVerification(const NGFX_PathChar* libName);
void        ngfx_sys_FreeLib(void* lib);
void*       ngfx_sys_GetLibHandle(const NGFX_PathChar* libName);
void*       ngfx_sys_GetProc(void* lib, const char* name, void* defaultProc);

NGFX_Result ngfx_sys_EnumerateInstallations(NGFX_InstallationInfo* installations,
                                            uint32_t maxInstallations,
                                            uint32_t* numInstallations);
void        ngfx_sys_FreeInstallations(NGFX_InstallationInfo* installations,
                                       uint32_t numInstallations);

NGFX_Result ngfx_sys_LoadInjectionLib(NGFX_ActivityType activityType,
                                      const NGFX_PathChar* installationPath,
                                      void* settings,
                                      void** loadedLib);
NGFX_Result ngfx_sys_GetActivityLibHandle(NGFX_ActivityType* activityType,
                                          bool injectionLib,
                                          void** libHandle);
NGFX_Result ngfx_sys_InjectActivity(NGFX_ActivityType activityType,
                                    const NGFX_PathChar* installationPath,
                                    void* settings);
NGFX_Result ngfx_sys_IsActivityInjected(NGFX_ActivityType activityType, bool* injected);
NGFX_Result ngfx_sys_IsActivityInitialized(NGFX_ActivityType activityType, bool* initialized);

/* ---- Frame boundary --------------------------------------------------- */

#ifdef NGFX_SYS_VULKAN
NGFX_Result ngfx_sys_FrameBoundary_Vulkan(NGFX_FrameBoundary_Vulkan_Params* params);
NGFX_Result ngfx_sys_DLSS_FG_PresentBoundary_Vulkan(NGFX_DLSS_FG_PresentBoundary_Vulkan_Params* params);
#endif
#ifdef NGFX_SYS_D3D12
NGFX_Result ngfx_sys_FrameBoundary_D3D12(NGFX_FrameBoundary_D3D12_Params* params);
NGFX_Result ngfx_sys_DLSS_FG_PresentBoundary_D3D12(NGFX_DLSS_FG_PresentBoundary_D3D12_Params* params);
#endif
#ifdef NGFX_SYS_OPENGL
NGFX_Result ngfx_sys_FrameBoundary_OpenGL(NGFX_FrameBoundary_OpenGL_Params* params);
#endif
#ifdef NGFX_SYS_CUDA
NGFX_Result ngfx_sys_FrameBoundary_CUDA(NGFX_FrameBoundary_CUDA_Params* params);
#endif
#ifdef NGFX_SYS_CUDART
NGFX_Result ngfx_sys_FrameBoundary_CUDART(NGFX_FrameBoundary_CUDART_Params* params);
#endif

/* ---- Graphics Capture -------------------------------------------------- */

#ifdef NGFX_SYS_GRAPHICS_CAPTURE
NGFX_Result ngfx_sys_GraphicsCapture_InjectionSettings_SetDefaults(NGFX_GraphicsCapture_InjectionSettings* settings);
#  if defined(NGFX_SYS_D3D12)
NGFX_Result ngfx_sys_GraphicsCapture_Inject_D3D12(NGFX_GraphicsCapture_Inject_D3D12_Params* params);
NGFX_Result ngfx_sys_GraphicsCapture_InitializeActivity_D3D12(NGFX_GraphicsCapture_InitializeActivity_D3D12_Params* params);
NGFX_Result ngfx_sys_GraphicsCapture_RequestCapture_D3D12(NGFX_GraphicsCapture_RequestCapture_D3D12_Params* params);
NGFX_Result ngfx_sys_GraphicsCapture_StartCapture_D3D12(NGFX_GraphicsCapture_StartCapture_D3D12_Params* params);
NGFX_Result ngfx_sys_GraphicsCapture_StopCapture_D3D12(NGFX_GraphicsCapture_StopCapture_D3D12_Params* params);
#  endif
#  if defined(NGFX_SYS_VULKAN)
NGFX_Result ngfx_sys_GraphicsCapture_Inject_Vulkan(NGFX_GraphicsCapture_Inject_Vulkan_Params* params);
NGFX_Result ngfx_sys_GraphicsCapture_InitializeActivity_Vulkan(NGFX_GraphicsCapture_InitializeActivity_Vulkan_Params* params);
NGFX_Result ngfx_sys_GraphicsCapture_RequestCapture_Vulkan(NGFX_GraphicsCapture_RequestCapture_Vulkan_Params* params);
NGFX_Result ngfx_sys_GraphicsCapture_StartCapture_Vulkan(NGFX_GraphicsCapture_StartCapture_Vulkan_Params* params);
NGFX_Result ngfx_sys_GraphicsCapture_StopCapture_Vulkan(NGFX_GraphicsCapture_StopCapture_Vulkan_Params* params);
#  endif
#endif

/* ---- GPU Trace --------------------------------------------------------- */

#ifdef NGFX_SYS_GPU_TRACE
NGFX_Result ngfx_sys_GPUTrace_IsInjected(bool* injected);
NGFX_Result ngfx_sys_GPUTrace_IsInitialized(bool* initialized);
NGFX_Result ngfx_sys_GPUTrace_GetStatus(NGFX_GPUTrace_GetStatus_Params* params);
NGFX_Result ngfx_sys_GPUTrace_WaitForStatus(NGFX_GPUTrace_WaitForStatus_Params* params);
NGFX_Result ngfx_sys_GPUTrace_InjectionSettings_SetDefaults(NGFX_GPUTrace_InjectionSettings* settings);
#  if defined(NGFX_SYS_D3D12)
NGFX_Result ngfx_sys_GPUTrace_Inject_D3D12(NGFX_GPUTrace_Inject_D3D12_Params* params);
NGFX_Result ngfx_sys_GPUTrace_InitializeActivity_D3D12(NGFX_GPUTrace_InitializeActivity_D3D12_Params* params);
NGFX_Result ngfx_sys_GPUTrace_ActivateTrace_D3D12(NGFX_GPUTrace_ActivateTrace_D3D12_Params* params);
NGFX_Result ngfx_sys_GPUTrace_StartTrace_D3D12(NGFX_GPUTrace_StartTrace_D3D12_Params* params);
NGFX_Result ngfx_sys_GPUTrace_StopTrace_D3D12(NGFX_GPUTrace_StopTrace_D3D12_Params* params);
#  endif
#  if defined(NGFX_SYS_VULKAN)
NGFX_Result ngfx_sys_GPUTrace_Inject_Vulkan(NGFX_GPUTrace_Inject_Vulkan_Params* params);
NGFX_Result ngfx_sys_GPUTrace_InitializeActivity_Vulkan(NGFX_GPUTrace_InitializeActivity_Vulkan_Params* params);
NGFX_Result ngfx_sys_GPUTrace_ActivateTrace_Vulkan(NGFX_GPUTrace_ActivateTrace_Vulkan_Params* params);
NGFX_Result ngfx_sys_GPUTrace_StartTrace_Vulkan(NGFX_GPUTrace_StartTrace_Vulkan_Params* params);
NGFX_Result ngfx_sys_GPUTrace_StopTrace_Vulkan(NGFX_GPUTrace_StopTrace_Vulkan_Params* params);
#  endif
#  if defined(NGFX_SYS_OPENGL)
NGFX_Result ngfx_sys_GPUTrace_Inject_OpenGL(NGFX_GPUTrace_Inject_OpenGL_Params* params);
NGFX_Result ngfx_sys_GPUTrace_InitializeActivity_OpenGL(NGFX_GPUTrace_InitializeActivity_OpenGL_Params* params);
NGFX_Result ngfx_sys_GPUTrace_ActivateTrace_OpenGL(NGFX_GPUTrace_ActivateTrace_OpenGL_Params* params);
NGFX_Result ngfx_sys_GPUTrace_StartTrace_OpenGL(NGFX_GPUTrace_StartTrace_OpenGL_Params* params);
NGFX_Result ngfx_sys_GPUTrace_StopTrace_OpenGL(NGFX_GPUTrace_StopTrace_OpenGL_Params* params);
#  endif
#  if defined(NGFX_SYS_CUDA)
NGFX_Result ngfx_sys_GPUTrace_Inject_CUDA(NGFX_GPUTrace_Inject_CUDA_Params* params);
NGFX_Result ngfx_sys_GPUTrace_InitializeActivity_CUDA(NGFX_GPUTrace_InitializeActivity_CUDA_Params* params);
NGFX_Result ngfx_sys_GPUTrace_ActivateTrace_CUDA(NGFX_GPUTrace_ActivateTrace_CUDA_Params* params);
NGFX_Result ngfx_sys_GPUTrace_StartTrace_CUDA(NGFX_GPUTrace_StartTrace_CUDA_Params* params);
NGFX_Result ngfx_sys_GPUTrace_StopTrace_CUDA(NGFX_GPUTrace_StopTrace_CUDA_Params* params);
#  endif
#  if defined(NGFX_SYS_CUDART)
NGFX_Result ngfx_sys_GPUTrace_Inject_CUDART(NGFX_GPUTrace_Inject_CUDART_Params* params);
NGFX_Result ngfx_sys_GPUTrace_InitializeActivity_CUDART(NGFX_GPUTrace_InitializeActivity_CUDART_Params* params);
NGFX_Result ngfx_sys_GPUTrace_ActivateTrace_CUDART(NGFX_GPUTrace_ActivateTrace_CUDART_Params* params);
NGFX_Result ngfx_sys_GPUTrace_StartTrace_CUDART(NGFX_GPUTrace_StartTrace_CUDART_Params* params);
NGFX_Result ngfx_sys_GPUTrace_StopTrace_CUDART(NGFX_GPUTrace_StopTrace_CUDART_Params* params);
#  endif
#endif

/* ---- System Profiling -------------------------------------------------- */

#ifdef NGFX_SYS_SYSTEM_PROFILING
NGFX_Result ngfx_sys_SystemProfiling_InjectionSettings_SetDefaults(NGFX_SystemProfiling_InjectionSettings* settings);
#  if defined(NGFX_SYS_D3D12)
NGFX_Result ngfx_sys_SystemProfiling_Inject_D3D12(NGFX_SystemProfiling_Inject_D3D12_Params* params);
NGFX_Result ngfx_sys_SystemProfiling_InitializeActivity_D3D12(NGFX_SystemProfiling_InitializeActivity_D3D12_Params* params);
NGFX_Result ngfx_sys_SystemProfiling_StartProfiling_D3D12(NGFX_SystemProfiling_StartProfiling_D3D12_Params* params);
NGFX_Result ngfx_sys_SystemProfiling_StopProfiling_D3D12(NGFX_SystemProfiling_StopProfiling_D3D12_Params* params);
#  endif
#  if defined(NGFX_SYS_VULKAN)
NGFX_Result ngfx_sys_SystemProfiling_Inject_Vulkan(NGFX_SystemProfiling_Inject_Vulkan_Params* params);
NGFX_Result ngfx_sys_SystemProfiling_InitializeActivity_Vulkan(NGFX_SystemProfiling_InitializeActivity_Vulkan_Params* params);
NGFX_Result ngfx_sys_SystemProfiling_StartProfiling_Vulkan(NGFX_SystemProfiling_StartProfiling_Vulkan_Params* params);
NGFX_Result ngfx_sys_SystemProfiling_StopProfiling_Vulkan(NGFX_SystemProfiling_StopProfiling_Vulkan_Params* params);
#  endif
#endif

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* NGFX_SYS_SHIM_H */
