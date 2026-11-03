use std::{
    sync::mpsc::{Receiver, SendError, Sender, channel, sync_channel},
    thread,
};

type Accessor<S> = Box<dyn FnOnce(&mut S) + Send + 'static>;

/// StateManager is channel based. It run's a receive loop in discrete thread and allows mutable access to the underlaying state via function/closure accessor send via [`StateManager::access_state`] and [`StateManager::access_state_receive_result`] methods.
/// 
/// By dropping the StateManager and underlaying Sender in cascade, thread with receive loop terminates to free resources.
pub struct StateManager<S: Send + 'static>(Sender<Accessor<S>>);

impl<S: Send + 'static> StateManager<S> {
    /// Create instance and run a receive loop immediately in discrete thread. To terminate it, simply drop (either implicitly or explicitly) whole StateManager returned.
    pub fn new(initial_state: S) -> Self {
        let (s, r) = channel::<Accessor<S>>();

        thread::spawn(move || {
            let mut s: S = initial_state;

            loop {
                match r.recv() {
                    Ok(f) => f(&mut s),
                    // State with a sender dropped and disconnected, so just terminate unneccessary thread.
                    Err(_) => return,
                }
            }
        });

        Self(s)
    }

    /// Acquire mutable access to the underlaying state via function/closure accessor
    pub fn access_state(
        &self,
        f: impl FnOnce(&mut S) + Send + 'static,
    ) -> Result<(), SendError<Accessor<S>>> {
        self.0.send(Box::new(f))
    }

    /// Acquire mutable access to the underlaying state via function/closure accessor and send a result to the receiver being returned.
    pub fn access_state_receive_result<R: Send + 'static>(
        &self,
        f: impl FnOnce(&mut S) -> R + Send + 'static,
    ) -> Result<Receiver<R>, SendError<Box<dyn FnOnce(&mut S) + Send>>> {
        let (s, r) = sync_channel::<R>(0);

        match self.0.send(Box::new(move |state| {
            let _ = s.send(f(state));
        })) {
             Err(err) => Err(err),
             Ok(_) => Ok(r)
        }
    }
}
