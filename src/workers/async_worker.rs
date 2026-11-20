use core::{
    array::from_fn,
    future::PollFn,
    pin::{Pin, pin},
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

use crate::workers::{tasks::BoxedAsyncTask, types::AtomicTaskReceiver};

// type AsyncTask = Future<Output = ()>;

const MAX_TASKS: usize = 256;

// struct AsyncTask([Option<Pin<BoxedAsyncTask>>; MAX_TASKS]);

struct AsyncTask {
    waker: Waker,
}

#[macro_export]
macro_rules! async_worker {
    ($name: ident, $queue_size: literal$(, $visibility: vis)?) => {
       $($visibility)? struct $name {
        queue: [Option<AsyncTask>; $queue_size]
       }

       impl $name {
        $($visibility)? fn new() -> Self {
            Self {queue: from_fn::<Option<AsyncTask>, $queue_size, fn(usize) -> Option<AsyncTask>>(|_| None)}
        }
       }
    };
}

async_worker!(W, 10, pub);

fn f() {
    let w = W::new();
}
