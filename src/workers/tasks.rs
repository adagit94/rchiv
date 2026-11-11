pub(super) type BoxedTask = Box<dyn FnOnce() + Send + 'static>;
pub(super) struct IndexedTaskResult<R>(pub usize, pub R);
