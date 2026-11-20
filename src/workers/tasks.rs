use std::task::Poll;

pub(super) type BoxedTask = Box<dyn FnOnce() + Send + 'static>;

pub(super) type BoxedAsyncTask = Box<dyn Future<Output = Poll<()>> + Send + 'static>;
pub(super) struct IndexedTaskResult<R>(pub usize, pub R);
