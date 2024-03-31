use crate::res::{BevyPlat, VenxTasks};
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

pub(super) fn submit_tasks(mut q: Query<(&mut BevyPlat, &mut VenxTasks)>) {
    for (mut plat, mut tasks) in &mut q {
        for (_task_type, future) in plat.poll_tasks() {
            let task = AsyncComputeTaskPool::get().spawn(async { future.await });
            //tasks.push(task);
            todo!();
        }
    }
}

pub(super) fn poll_tasks(mut q: Query<&mut VenxTasks>) {
    for mut tasks in &mut q {
        tasks.retain(|task| !task.is_finished())
    }
}
