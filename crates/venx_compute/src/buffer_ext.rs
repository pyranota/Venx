use async_trait::async_trait;
use bytemuck::{Pod, Zeroable};
use wgpu::{Buffer, BufferViewMut};

#[async_trait]
pub trait BufferRW {
    async fn read<T: Pod + Zeroable>(&self) -> Vec<T>;

    async fn write(&self) -> BufferViewMut<'async_trait>;

    // async fn read_range() -> BufferView<'async_trait>;

    // async fn write_range() -> BufferViewMut<'async_trait>;
}

#[async_trait]
impl BufferRW for Buffer {
    async fn read<T: Pod + Zeroable>(&self) -> Vec<T> {
        // Note that we're not calling `.await` here.
        let buffer_slice = self.slice(..);

        // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        // Awaits until `buffer_future` can be read from
        //let res = receiver.receive().await;
        if let Some(Ok(())) = receiver.receive().await {
            let data = buffer_slice.get_mapped_range();
            let res = bytemuck::cast_slice(&data).to_vec();

            return res;
        }
        todo!()
    }

    async fn write(&self) -> BufferViewMut<'async_trait> {
        // Note that we're not calling `.await` here.
        let buffer_slice: wgpu::BufferSlice<'_> = self.slice(..);

        // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Write, move |v| sender.send(v).unwrap());

        // Awaits until `buffer_future` can be read from
        //let res = receiver.receive().await;
        if let Some(Ok(())) = receiver.receive().await {
            let data = buffer_slice.get_mapped_range_mut();

            return data;
        }
        todo!()
    }
}
