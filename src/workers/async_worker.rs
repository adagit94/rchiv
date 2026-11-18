use std::{
    pin::Pin, sync::{Arc, Mutex, mpsc::{Receiver, TryRecvError}}, thread::{self as th, JoinHandle},
};

use crate::workers::{tasks::BoxedAsyncTask, types::AtomicTaskReceiver};

// type AsyncTask = Future<Output = ()>;

const MAX_TASKS: usize = 256;

// struct AsyncTask([Option<Pin<BoxedAsyncTask>>; MAX_TASKS]);

pub(super) struct AsyncWorker {
    rcv: AtomicTaskReceiver<BoxedAsyncTask>,
    tasks: Vec<Option<Pin<BoxedAsyncTask>>>,
    index: usize,
}

// struct S(u8);

impl AsyncWorker {
    pub(super) fn new(rcv: AtomicTaskReceiver<BoxedAsyncTask>, q_cap: usize) -> JoinHandle<fn()> {
        return th::spawn(move || {
            let mut tasks = Vec::with_capacity(q_cap);

            tasks.resize_with(q_cap, || None);

            // loop {}

            loop {
                for task in tasks.into_iter() {
                    match task {
                        Some(fu) => {}
                        None => {
                            let lock_res = rcv.try_lock();

                            match lock_res {
                                Ok(rcv) => {
                                    let fu_res = rcv.try_recv();

                                    match fu_res {
                                      Ok(fu) => {},
                                      Err(err) => {
                                        match err {
                                          TryRecvError::Empty => {},
                                          TryRecvError::Disconnected => {},
                                        }
                                      }
                                    }
                                  
                                    drop(rcv);
                                }
                            }

                            match t {
                                Ok(t) => {
                                    t();
                                }
                                Err(err) => {
                                    println!("Receive of a task failed:\n{err}");
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}
