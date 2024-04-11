use std::borrow::BorrowMut;

use bevy::{
    prelude::*,
    reflect::List,
    render::{
        render_resource::{encase::internal::BufferMut, Buffer},
        renderer::{RenderDevice, RenderQueue},
    },
    tasks::block_on,
};
use venx::plat::loader::external_buffer::ExternalBuffer;

pub struct PoolBuffer {
    pub device: RenderDevice,
    pub queue: RenderQueue,
    pub buffer: Buffer,
    pub staging_buffer: Buffer,
}
impl ExternalBuffer for PoolBuffer {
    fn set(&self, offset: u32, data: &[u8]) {
        let future = async move {
            let input_data_len = data.len();

            let buffer_slice = self.staging_buffer.slice(..);

            // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
            let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(bevy::render::render_resource::MapMode::Write, move |v| {
                sender.send(v).unwrap()
            });

            // Awaits until `buffer_future` can be read from
            if let Some(Ok(())) = receiver.receive().await {
                dbg!("Buffer is mapped");
                let mut mapped_data = buffer_slice.get_mapped_range_mut();
                for (staging_element, input_element) in
                    mapped_data.as_mut().iter_mut().zip(data.iter())
                {
                    *staging_element = *input_element;
                }
            }

            self.staging_buffer.unmap();

            let mut encoder = self.device.create_command_encoder(
                &bevy::render::render_resource::CommandEncoderDescriptor {
                    label: Some(
                        "Encoder for copying data from indirect staging buffer to storage buffer",
                    ),
                },
            );

            encoder.copy_buffer_to_buffer(
                &self.staging_buffer,
                0,
                &self.buffer,
                offset as u64,
                input_data_len as u64,
            );

            self.queue.submit(Some(encoder.finish()));
        };
        block_on(future);
    }
}
