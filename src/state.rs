use std::{sync::Mutex, rc::Rc};

use crate::types::{Runnable, RunnableParams};

#[derive(Default)]
pub struct State {
    runnable: Mutex<Runnable>,
}

impl State {
    pub fn rc() -> Rc<State> {
        Default::default()
    }

    pub fn set_runnable(&self, runnable: Runnable) {
        let mut _runnable = self.runnable.lock().unwrap();
        *_runnable = runnable;
    }

    pub fn set_params(&self, params: RunnableParams) {
        let mut runnable = self.runnable.lock().unwrap();
        runnable.params = params;
    }

    pub fn get_runnable(&self) -> Runnable {
        self.runnable.lock().unwrap().clone()
    }
}
