use std::{
    sync::{Arc, Mutex, mpsc::Receiver},
    thread::{self as th, JoinHandle},
};

use crate::workers::tasks::BoxedTask;

type AtomicTaskReceiver = Arc<Mutex<Receiver<BoxedTask>>>;

pub(super) struct Worker();

impl Worker {
    pub(super) fn new(r: AtomicTaskReceiver) -> JoinHandle<fn()> {
        return th::spawn(move || {
            loop {
                let mg = r.lock().unwrap();
                let t = mg.recv();

                drop(mg);

                match t {
                    Ok(t) => {
                        t();
                    }
                    Err(err) => {
                        println!("Receive of a task failed:\n{err}");
                    }
                }
            }
        });
    }
}
