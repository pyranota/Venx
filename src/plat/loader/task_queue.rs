use std::{future::Future, pin::Pin};

use venx_core::plat::chunk::chunk::Chunk;

use crate::plat::VenxPlat;

pub type VenxTask = (
    TaskType,
    Pin<Box<dyn Future<Output = ()> + Send + 'static + Sync>>,
);

#[derive(Default)]
// TODO: Use queue?
// TODO: Find some way to drop task
// TODO: Make optional task runner
/// Allows integration take care of all processes and tasks going in user-time
pub(super) struct TaskQueue(Vec<VenxTask>);

pub enum TaskType {
    GenerateMesh,
    LoadChunk,
    Debug,
}

pub enum TaskOutput {
    Mesh(Box<()>),
    Chunk(Box<Chunk>),
    ChunksBatch(Box<Vec<Chunk>>),
}

impl VenxPlat {
    pub fn poll_tasks(&mut self) -> Vec<VenxTask> {
        let mut foo = true;
        vec![(
            TaskType::Debug,
            Box::pin(async move {
                foo = false;
            }),
        )]
    }
}
