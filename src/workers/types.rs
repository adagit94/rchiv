use std::sync::{Arc, Mutex, mpsc::Receiver};

pub(super) type AtomicTaskReceiver<T> = Arc<Mutex<Receiver<T>>>;