use bevy::{
    prelude::*,
    render::{
        render_resource::Buffer,
        renderer::{RenderDevice, RenderQueue},
    },
    tasks::block_on,
};
use venx::plat::loader::external_buffer::ExternalBuffer;

#[derive(Component)]
pub(crate) struct VertexPool {
    indirect_buffer: PoolBuffer,
    vertex_buffer: PoolBuffer,
}

struct PoolBuffer {
    device: RenderDevice,
    queue: RenderQueue,
    buffer: Buffer,
    staging_buffer: Buffer,
}
impl ExternalBuffer for PoolBuffer {
    fn set(&self, bounds: (u64, u64), data: Vec<u8>) {
        let future = async move {
            let size = 2;

            let buffer_slice = self.staging_buffer.slice(..);

            // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
            let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(bevy::render::render_resource::MapMode::Write, move |v| {
                sender.send(v).unwrap()
            });

            // Awaits until `buffer_future` can be read from
            if let Some(Ok(())) = receiver.receive().await {
                let mut data = buffer_slice.get_mapped_range_mut();
                let draw_indirect_slice: &mut [u8] = bytemuck::cast_slice_mut(data.as_mut());

                //draw_indirect_slice = data;
            }

            self.staging_buffer.unmap();

            let mut encoder = self.device.create_command_encoder(
                &bevy::render::render_resource::CommandEncoderDescriptor {
                    label: Some(
                        "Encoder for copying data from indirect staging buffer to storage buffer",
                    ),
                },
            );

            encoder.copy_buffer_to_buffer(&self.staging_buffer, 0, &self.buffer, 0, size);

            self.queue.submit(Some(encoder.finish()));
        };
        block_on(future);
    }
}
