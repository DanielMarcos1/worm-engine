use std::borrow::Cow;

use crate::geometry::vector::Vector3d;

pub async fn run_compute_shader(velocities: &mut [Vector3d], accelerations: &[Vector3d], dt: f32) {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .expect("Failed to create device");

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Compute Shader"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../../shaders/compute.wgsl"))),
    });

    let velocities_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Velocities Buffer"),
        size: (velocities.len() * std::mem::size_of::<Vector3d>()) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let accelerations_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Accelerations Buffer"),
        size: (accelerations.len() * std::mem::size_of::<Vector3d>()) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let dt_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("DT Buffer"),
        size: (std::mem::size_of::<f32>() * 4) as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // Ensure size aligns to WGSL standards
    let dt_aligned: [f32; 4] = [dt, 0.0, 0.0, 0.0];

    queue.write_buffer(&velocities_buffer, 0, bytemuck::cast_slice(velocities));
    queue.write_buffer(&accelerations_buffer, 0, bytemuck::cast_slice(accelerations));
    queue.write_buffer(&dt_buffer, 0, bytemuck::cast_slice(&dt_aligned));

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Compute Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Compute Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: velocities_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: accelerations_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: dt_buffer.as_entire_binding(),
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Compute Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main",
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        let workgroup_count = ((velocities.len() as u32) + 63) / 64;
        cpass.dispatch_workgroups(workgroup_count, 1, 1);
    }

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging Buffer"),
        size: velocities_buffer.size(),
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    encoder.copy_buffer_to_buffer(&velocities_buffer, 0, &staging_buffer, 0, velocities_buffer.size());

    queue.submit(Some(encoder.finish()));

    let buffer_slice = staging_buffer.slice(..);
    let (sender, receiver) = futures_channel::oneshot::channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

    device.poll(wgpu::MaintainBase::Wait);

    if let Ok(Ok(())) = receiver.await {
        let data = buffer_slice.get_mapped_range();
        velocities.copy_from_slice(bytemuck::cast_slice(&data));
        drop(data);
        staging_buffer.unmap();
    }
}
