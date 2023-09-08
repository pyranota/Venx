use std::{sync::Arc, thread::JoinHandle};

use wgpu::{hal::empty::Encoder, util::DeviceExt, *};

use crate::buffer_ext::BufferRW;

pub struct ComputeServer {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Arc<Device>,
    pub queue: Queue,
    pub join_handle: JoinHandle<()>,
}

impl ComputeServer {
    pub async fn new() -> Self {
        // Instantiates instance of WebGPU
        let instance = wgpu::Instance::default();

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
                    limits: wgpu::Limits::downlevel_defaults(),
                },
                None,
            )
            .await
            .unwrap();

        let device = Arc::new(device);

        let info = adapter.get_info();
        // skip this on LavaPipe temporarily

        let new_device = device.clone();

        let handle = std::thread::spawn(move || loop {
            new_device.poll(wgpu::Maintain::Poll);
        });

        ComputeServer {
            instance,
            adapter,
            device,
            queue,
            join_handle: handle,
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

    pub fn new_module(&self, smd: ShaderModuleDescriptor) -> ShaderModule {
        self.device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: smd.source,
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
