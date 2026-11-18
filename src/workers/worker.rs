use std::{
    thread::{self as th, JoinHandle},
};

use crate::workers::{tasks::BoxedTask, types::AtomicTaskReceiver};

pub(super) struct Worker();

impl Worker {
    pub(super) fn new(rcv: AtomicTaskReceiver<BoxedTask>) -> JoinHandle<fn()> {
        return th::spawn(move || {
            loop {
                let mg = rcv.lock().unwrap();
                let task = mg.recv();

                drop(mg);

                match task {
                    Ok(t) => {
                        t();
                    }
                    Err(err) => {
                        println!("Receive of the task failed:\n{err}");
                    }
                }
            }
        });
    }
}
