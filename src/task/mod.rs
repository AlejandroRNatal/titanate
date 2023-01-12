
use core::task::{Context, Poll};

use core::{future::Future, pin::Pin, sync::atomic::{AtomicU64, Ordering}};
use alloc::boxed::Box;


pub mod simple_executor;
pub mod keyboard;
pub mod executor;

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskId(u64);

pub struct Task {
    id: TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>,
}


impl Task {
    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }

    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            id: TaskId::new(),
            future: Box::pin(future),
        }
    }
}

impl TaskId {
    pub fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}
