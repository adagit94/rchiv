use std::pin::Pin;

use crate::workers::tasks::BoxedAsyncTask;



// type AsyncTask = Future<Output = ()>;

const MAX_TASKS: usize = 256;


// struct AsyncTask([Option<Pin<BoxedAsyncTask>>; MAX_TASKS]);


struct AsyncWorker {
  tasks: Vec<Option<Pin<BoxedAsyncTask>>>,
  index: usize,
  prolongation_step: usize
}

// struct S(u8);

impl AsyncWorker {
  fn new(initial_capacity: usize, prolongation_step: usize) -> Self {
    // let arr: [Option<S>; MAX_TASKS] = [None; 256];
    
    let mut tasks = Vec::with_capacity(initial_capacity);
    tasks.resize_with(initial_capacity, || None);

    Self {tasks, index: 0, prolongation_step}
  }
}
