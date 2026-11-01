pub struct State<S> {
    state: S,
}

impl<S> State<S> {
    pub fn new(initial_state: S) -> Self {
        Self {
            state: initial_state,
        }
    }
}

pub trait StateTrait<S> {
    fn get(&self) -> &S;
    fn update(&mut self, updater: impl FnOnce(&S) -> S) -> &S;
}

impl<S> StateTrait<S> for State<S> {
    fn get(&self) -> &S {
        &self.state
    }

    fn update(&mut self, updater: impl FnOnce(&S) -> S) -> &S {
        self.state = updater(&self.state);
        self.get()
    }
}
