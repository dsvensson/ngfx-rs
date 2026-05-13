# ngfx-rs

Rust bindings for the **NVIDIA Nsight Graphics SDK** — instrument a Vulkan application to drive GPU Trace captures, Graphics Captures, and System
Profiling activities from inside the process.

## Quick example

A minimal GPU Trace around a Vulkan workload:

```rust
use ngfx::gpu_trace;

// 1. Inject GPU Trace BEFORE creating the VkInstance.
let mut settings = gpu_trace::Settings::in_app().max_duration_ms(30_000);
let session = gpu_trace::Session::inject_vulkan(&install_path, &mut settings)?;

// 2. Standard ash setup (entry → instance → device → queue).
let entry    = ash::Entry::load()?;
let instance = entry.create_instance(&info, None)?;
let device   = instance.create_device(pdev, &info, None)?;
let queue    = device.get_device_queue(family, 0);

// 3. Bring the activity online.
session.initialize_vulkan()?;
session.activate_vulkan(queue)?;   // blocks until Nsight host attaches

// 4. Trace around the workload. Compute-only apps emit synthetic frame
//    boundaries since there's no swapchain.
session.start_vulkan()?;
ngfx::vulkan::frame_boundary(queue)?;
device.queue_submit(queue, &submits, vk::Fence::null())?;
device.queue_wait_idle(queue)?;
ngfx::vulkan::frame_boundary(queue)?;
session.stop_vulkan(queue, gpu_trace::StopFlag::ImmediateCollection)?;
session.wait_for_status(gpu_trace::Status::Active, 10_000)?;
```

A runnable version is in
[`ngfx/examples/hello_compute_trace.rs`](ngfx/examples/hello_compute_trace.rs);
see [its README](ngfx/examples/README.md) for the attach workflow.

## Relationship to NVIDIA

This project is an **independent** set of Rust bindings; it is not affiliated
with or endorsed by NVIDIA. "Nsight Graphics" and "NVIDIA" are trademarks of
NVIDIA Corporation. The vendored SDK headers under
[`ngfx-sdk/`](ngfx-sdk/) are copyright NVIDIA and distributed under their
original Apache-2.0 license (see [`ngfx-sdk/0.9.0/LICENSE.txt`](ngfx-sdk/0.9.0/LICENSE.txt)).

## License

`ngfx-rs` is licensed under the Apache License, Version 2.0. See
[`LICENSE`](LICENSE) for the full text.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in this work shall be licensed under the same terms,
without any additional conditions.
