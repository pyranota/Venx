use async_trait::async_trait;
use bytemuck::{Pod, Zeroable};
use wgpu::{Buffer, BufferViewMut};

#[async_trait]
pub trait BufferRW {
    /// Needs unmap manually
    async fn read_manual<T: Pod + Zeroable>(&self) -> Vec<T>;

    /// Safe way to write buffer
    async fn write<T: Pod + Zeroable, F: FnOnce(&mut [T]) + Send + Sync>(&self, f: F);

    /// Safe way to write buffer
    async fn read<T: Pod + Zeroable, F: FnOnce(Vec<T>) + Send + Sync>(&self, f: F);

    /// Needs unmap manually
    async fn write_manual(&self) -> BufferViewMut<'async_trait>;
}

#[async_trait]
impl BufferRW for Buffer {
    async fn write<T: Pod + Zeroable, F: FnOnce(&mut [T]) + Send + Sync>(&self, f: F) {
        let mut data = self.write_manual().await;

        let a: &mut [T] = bytemuck::cast_slice_mut(data.as_mut());

        f(a);

        drop(a);

        self.unmap();
    }

    async fn read<T: Pod + Zeroable, F: FnOnce(Vec<T>) + Send + Sync>(&self, f: F) {
        let data: Vec<T> = self.read_manual().await;
        f(data);
        self.unmap();
    }

    async fn read_manual<T: Pod + Zeroable>(&self) -> Vec<T> {
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

    async fn write_manual(&self) -> BufferViewMut<'async_trait> {
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
