use std::borrow::Cow;
use std::time::Duration;

pub struct GpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub compute_pipeline: wgpu::ComputePipeline,
}

impl GpuContext {
    /// Initializes a basic WGPU context for compute operations.
    /// This method is async and needs to be awaited or blocked on.
    pub async fn new() -> Option<Self> {
        let instance = wgpu::Instance::default();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok()?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Worm Engine Compute Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_defaults(),
                    memory_hints: wgpu::MemoryHints::Performance,
                    ..Default::default()
                },
            )
            .await
            .ok()?;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../../shaders/compute.wgsl"))),
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: None,
            module: &shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });

        Some(Self {
            device,
            queue,
            compute_pipeline,
        })
    }

    /// Prototype function to execute a compute shader pass over a flat array of floats.
    pub async fn execute_compute(&self, input_data: &[f32]) -> Vec<f32> {
        let data_bytes: &[u8] = bytemuck::cast_slice(input_data);

        // Create buffer on GPU and initialize it with input_data
        let storage_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Storage Buffer"),
            size: data_bytes.len() as u64,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        self.queue.write_buffer(&storage_buffer, 0, data_bytes);

        // Create a staging buffer to read the data back to CPU
        let staging_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: data_bytes.len() as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = self.compute_pipeline.get_bind_group_layout(0);
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Compute Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: storage_buffer.as_entire_binding(),
            }],
        });

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            cpass.set_pipeline(&self.compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            // Dispatch enough workgroups to cover all items (divided by 3 because we process Vector3d).
            // Workgroup size in shader is 64.
            let items = (input_data.len() / 3) as u32;
            let workgroups = (items + 63) / 64;
            cpass.dispatch_workgroups(workgroups, 1, 1);
        }

        // Copy from storage buffer back to staging buffer
        encoder.copy_buffer_to_buffer(
            &storage_buffer,
            0,
            &staging_buffer,
            0,
            data_bytes.len() as u64,
        );

        // Submit the commands to the queue
        let submission_index = self.queue.submit(Some(encoder.finish()));

        // Map the staging buffer so we can read it on CPU
        let buffer_slice = staging_buffer.slice(..);
        let (sender, receiver) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        // Wait for mapping to finish
        self.device.poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(Duration::from_secs(5)),
        }).unwrap();

        if let Ok(Ok(())) = receiver.await {
            let data = buffer_slice.get_mapped_range();
            let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
            drop(data); // Unmap buffer
            staging_buffer.unmap();
            result
        } else {
            panic!("Failed to read from staging buffer!")
        }
    }
}
