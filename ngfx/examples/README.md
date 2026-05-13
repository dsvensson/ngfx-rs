# `hello_compute_trace`

Minimal `ash` + Vulkan compute pipeline (writes `idx * idx + 1` to a storage buffer) instrumented with a GPU Trace
capture via the `ngfx` crate.

## Workflow

1. **Run the example standalone.** It will set up Vulkan, build the compute pipeline + command buffer, then print
   `Ready for NVIDIA Nsight Graphics attach` and block in `activate_vulkan`.
2. **Open Nsight Graphics** and use **Connect → Attach to Process** to attach to the running `hello_compute_trace.exe`.
3. Once attached, `activate_vulkan` returns, the dispatch runs inside a trace window, and the example exits. The
   `.gputrace` file lands in
   `Documents\NVIDIA Nsight Graphics\GpuTraces\`; open it from the Nsight UI.

## Build

```
cargo build --example hello_compute_trace --release
```

The shader (`hello_compute_trace.comp`) is pre-compiled to
`hello_compute_trace.spv` and embedded via `include_bytes!`, so no shader toolchain is needed at build time. If you
change the GLSL, recompile with:

```
glslc --target-env=vulkan1.2 -O hello_compute_trace.comp -o hello_compute_trace.spv
```

## Run

The example needs to know where Nsight Graphics is installed. Either:

```
set NGFX_INSTALLATION_PATH=C:\Program Files\NVIDIA Corporation\Nsight Graphics 2026.1.0
cargo run --example hello_compute_trace --release
```

or pass the path as the first argument:

```
cargo run --example hello_compute_trace --release -- "C:\Program Files\NVIDIA Corporation\Nsight Graphics 2026.1.0"
```

## Nsight Graphics host configuration

In the GPU Trace activity settings, set:

| Field           | Value            |
|-----------------|------------------|
| **Start After** | `NGFX SDK Start` |
| **Stop After**  | `NGFX SDK Stop`  |

Without these the host will use its default frame/duration triggers and the
`session.start_vulkan()` / `session.stop_vulkan()` calls in the example will have no effect — the trace will either not
start or not stop where you expect.

### PM Bandwidth Limit (MB/sec)

The default value (often `5000`) is fine for short traces. For workloads that span many seconds or capture many
streaming multiprocessors at once you may need to lower it — otherwise the sampling buffer fills, samples are dropped,
and the trace ends early or shows gaps. A safe starting point for long traces is `500`–`1000` MB/sec.

## What the example does

```
1. Resolve install path (NGFX_INSTALLATION_PATH or argv[1])
2. gpu_trace::Session::inject(install, settings)   // BEFORE VkInstance
3. ash Entry / Instance / PhysicalDevice / Device / Queue
4. session.initialize()
5. Build storage buffer, descriptor set, shader, compute pipeline, command buffer
6. print "Ready for NVIDIA Nsight Graphics attach"
7. session.activate(queue)   // blocks until Nsight host attaches
8. session.start()
9. vulkan::frame_boundary(queue)
10. submit + queue_wait_idle
11. vulkan::frame_boundary(queue)
12. session.stop(queue, StopFlag::ImmediateCollection)
13. session.wait_for_status(Status::Active, 10_000)
14. Read back, print first/last 8 outputs, tear down Vulkan
```

The two `frame_boundary` calls bracket the dispatch — compute-only apps have no swapchain, so we manufacture synthetic
frame boundaries for the SDK.
