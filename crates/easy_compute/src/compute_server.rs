use std::{sync::Arc, thread::JoinHandle};

use wgpu::{util::DeviceExt, *};

pub struct ComputeServer {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Arc<Device>,
    pub queue: Queue,
    pub join_handle: JoinHandle<()>,
    // pub composer: Composer,
}

impl ComputeServer {
    pub async fn new() -> Self {
        // Instantiates instance of WebGPU
        let instance = wgpu::Instance::default();

        let mut gpu_limits = wgpu::Limits::default();
        gpu_limits.max_storage_buffer_binding_size = 1 << 30; // try to set the storage buffer limit to 16GiB

        gpu_limits.max_storage_buffer_binding_size = 1 << 30;

        gpu_limits.max_buffer_size = 1 << 30;

        gpu_limits.max_bind_groups = 6;

        // `request_adapter` instantiates the general connection to the GPU
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();

        // `request_device` instantiates the feature specific connection to the GPU, defining some parameters,
        //  `features` being the available features.
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: gpu_limits,
                },
                None,
            )
            .await
            .unwrap();

        let device = Arc::new(device);

        let _info = adapter.get_info();
        // skip this on LavaPipe temporarily

        let new_device = device.clone();

        let handle = std::thread::spawn(move || loop {
            new_device.poll(wgpu::Maintain::Poll);
        });

        // let composer = Composer::default();

        ComputeServer {
            instance,
            adapter,
            device,
            queue,
            join_handle: handle,
            //   composer,
        }
    }

    // pub async fn read_buffer<'a, C>(buffer: &'a Buffer, callback: &C)
    // where
    //     C: Fn(&BufferView<'a>),
    // {
    //     let data = buffer.read().await.unwrap();
    //     callback(&data);

    //     drop(data);
    //     buffer.unmap();
    // }

    pub fn new_buffer(&self, contents: &[u8]) -> Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Storage Buffer"),
                contents,
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_DST
                    | wgpu::BufferUsages::COPY_SRC,
            })
    }

    /// Size in **bytes**
    pub fn new_staging_buffer(&self, size: u64, read_only: bool) -> Buffer {
        let map_flag = if read_only {
            wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST
        } else {
            wgpu::BufferUsages::MAP_WRITE | wgpu::BufferUsages::COPY_SRC
            //| wgpu::BufferUsages::COPY_DST
        };

        self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size, //size: 32768 * 4 * locs.len() as u64,
            usage: map_flag,
            mapped_at_creation: false,
        })
    }

    pub async fn eval<'a, R, F>(&self, mut closure: F)
    where
        F: FnMut(&mut CommandEncoder) -> R,
    {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            closure(&mut encoder);
        }
        // Submits command encoder for processing
        self.queue.submit(Some(encoder.finish()));
    }
}
