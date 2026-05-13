//! Minimal ash + Vulkan compute shader instrumented with a GPU Trace capture.
//!
//! Workflow: launch the example, then attach with Nsight Graphics
//! (Connect → Attach to Process). The example will print
//! `Ready for NVIDIA Nsight Graphics attach` and block in `activate_vulkan`
//! until the host connects.
//!
//! Run with:
//!   cargo run --example hello_compute_trace --release -- "C:\Program Files\NVIDIA Corporation\Nsight Graphics 2026.X"
//!
//! Or set `NGFX_INSTALLATION_PATH` instead of passing it as an arg.

use std::path::PathBuf;
use std::{error::Error, slice};

use ash::vk;
use ngfx::vulkan::{self, gpu_trace};

const SHADER_SPV: &[u8] = include_bytes!("hello_compute_trace.spv");
const NUM_ELEMENTS: u32 = 256;
const WORKGROUP_SIZE: u32 = 64;

fn main() -> Result<(), Box<dyn Error>> {
    let install_path: PathBuf = std::env::var_os("NGFX_INSTALLATION_PATH")
        .or_else(|| std::env::args_os().nth(1))
        .map(PathBuf::from)
        .ok_or("set NGFX_INSTALLATION_PATH or pass the Nsight Graphics install dir as arg #1")?;
    println!("Nsight Graphics install: {}", install_path.display());

    let mut settings = gpu_trace::Settings::in_app().max_duration_ms(30_000);
    let session = gpu_trace::Session::inject(&install_path, &mut settings)?;

    unsafe { run_compute(&session) }
}

unsafe fn run_compute(session: &gpu_trace::Session) -> Result<(), Box<dyn Error>> {
    unsafe {
        let entry = ash::Entry::load()?;

        let app_info = vk::ApplicationInfo::default()
            .application_name(c"ngfx-hello-compute")
            .api_version(vk::API_VERSION_1_2);
        let instance = entry.create_instance(
            &vk::InstanceCreateInfo::default().application_info(&app_info),
            None,
        )?;

        let pdev = *instance
            .enumerate_physical_devices()?
            .first()
            .ok_or("no Vulkan physical device")?;

        let queue_family = instance
            .get_physical_device_queue_family_properties(pdev)
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::COMPUTE))
            .ok_or("no compute queue family")? as u32;

        let queue_infos = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family)
            .queue_priorities(&[1.0])];
        let device = instance.create_device(
            pdev,
            &vk::DeviceCreateInfo::default().queue_create_infos(&queue_infos),
            None,
        )?;
        let queue = device.get_device_queue(queue_family, 0);

        session.initialize()?;

        let buffer_size = (NUM_ELEMENTS as u64) * 4;
        let buffer = device.create_buffer(
            &vk::BufferCreateInfo::default()
                .size(buffer_size)
                .usage(vk::BufferUsageFlags::STORAGE_BUFFER),
            None,
        )?;

        let mem_req = device.get_buffer_memory_requirements(buffer);
        let mem_props = instance.get_physical_device_memory_properties(pdev);
        let want = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
        let mem_type_idx = (0..mem_props.memory_type_count)
            .find(|&i| {
                mem_req.memory_type_bits & (1 << i) != 0
                    && mem_props.memory_types[i as usize]
                        .property_flags
                        .contains(want)
            })
            .ok_or("no host-visible coherent memory type")?;
        let memory = device.allocate_memory(
            &vk::MemoryAllocateInfo::default()
                .allocation_size(mem_req.size)
                .memory_type_index(mem_type_idx),
            None,
        )?;
        device.bind_buffer_memory(buffer, memory, 0)?;

        let dsl = device.create_descriptor_set_layout(
            &vk::DescriptorSetLayoutCreateInfo::default().bindings(&[
                vk::DescriptorSetLayoutBinding::default()
                    .binding(0)
                    .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                    .descriptor_count(1)
                    .stage_flags(vk::ShaderStageFlags::COMPUTE),
            ]),
            None,
        )?;
        let pool = device.create_descriptor_pool(
            &vk::DescriptorPoolCreateInfo::default()
                .max_sets(1)
                .pool_sizes(&[vk::DescriptorPoolSize::default()
                    .ty(vk::DescriptorType::STORAGE_BUFFER)
                    .descriptor_count(1)]),
            None,
        )?;
        let set_layouts = [dsl];
        let set = device.allocate_descriptor_sets(
            &vk::DescriptorSetAllocateInfo::default()
                .descriptor_pool(pool)
                .set_layouts(&set_layouts),
        )?[0];
        device.update_descriptor_sets(
            &[vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(0)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .buffer_info(&[vk::DescriptorBufferInfo::default()
                    .buffer(buffer)
                    .range(buffer_size)])],
            &[],
        );

        let words: Vec<u32> = SHADER_SPV
            .chunks_exact(4)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect();
        let sm = device
            .create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&words), None)?;
        let pipeline_layout = device.create_pipeline_layout(
            &vk::PipelineLayoutCreateInfo::default().set_layouts(&set_layouts),
            None,
        )?;
        let pipeline = device
            .create_compute_pipelines(
                vk::PipelineCache::null(),
                &[vk::ComputePipelineCreateInfo::default()
                    .stage(
                        vk::PipelineShaderStageCreateInfo::default()
                            .stage(vk::ShaderStageFlags::COMPUTE)
                            .module(sm)
                            .name(c"main"),
                    )
                    .layout(pipeline_layout)],
                None,
            )
            .map_err(|(_, e)| e)?[0];

        let cp = device.create_command_pool(
            &vk::CommandPoolCreateInfo::default().queue_family_index(queue_family),
            None,
        )?;
        let cb = device.allocate_command_buffers(
            &vk::CommandBufferAllocateInfo::default()
                .command_pool(cp)
                .command_buffer_count(1),
        )?[0];

        device.begin_command_buffer(cb, &vk::CommandBufferBeginInfo::default())?;
        device.cmd_bind_pipeline(cb, vk::PipelineBindPoint::COMPUTE, pipeline);
        device.cmd_bind_descriptor_sets(
            cb,
            vk::PipelineBindPoint::COMPUTE,
            pipeline_layout,
            0,
            &[set],
            &[],
        );
        device.cmd_dispatch(cb, NUM_ELEMENTS.div_ceil(WORKGROUP_SIZE), 1, 1);
        device.end_command_buffer(cb)?;

        println!("Ready for NVIDIA Nsight Graphics attach (Connect → Attach to Process)...");
        session.activate(queue)?;
        println!("Attached — running compute workload.");

        // Compute-only apps have no swapchain, so we bracket the dispatch with
        // synthetic frame boundaries for the SDK.
        session.start()?;
        vulkan::frame_boundary(queue)?;
        device.queue_submit(
            queue,
            &[vk::SubmitInfo::default().command_buffers(&[cb])],
            vk::Fence::null(),
        )?;
        device.queue_wait_idle(queue)?;
        vulkan::frame_boundary(queue)?;
        session.stop(queue, gpu_trace::StopFlag::ImmediateCollection)?;
        // Post-stop idle is Active (Activate makes Active the terminal state).
        session.wait_for_status(gpu_trace::Status::Active, 10_000)?;

        let ptr =
            device.map_memory(memory, 0, buffer_size, vk::MemoryMapFlags::empty())? as *const u32;
        let view = slice::from_raw_parts(ptr, NUM_ELEMENTS as usize);
        println!("first 8 outputs: {:?}", &view[..8]);
        println!("last  8 outputs: {:?}", &view[NUM_ELEMENTS as usize - 8..]);
        device.unmap_memory(memory);

        device.destroy_pipeline(pipeline, None);
        device.destroy_pipeline_layout(pipeline_layout, None);
        device.destroy_shader_module(sm, None);
        device.destroy_descriptor_pool(pool, None);
        device.destroy_descriptor_set_layout(dsl, None);
        device.destroy_command_pool(cp, None);
        device.destroy_buffer(buffer, None);
        device.free_memory(memory, None);
        device.destroy_device(None);
        instance.destroy_instance(None);

        Ok(())
    }
}
