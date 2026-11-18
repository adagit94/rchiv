use std::pin::Pin;

pub(super) type BoxedAsyncTask = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

// type AsyncTask = Future<Output = ()>;

struct AsyncWorker {
  tasks: [256; Option<>]
}