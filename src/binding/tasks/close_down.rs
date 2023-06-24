use std::marker::PhantomData;

use serde::Deserialize;
use serde::Serialize;

use crate::binding::tasks::{Paused, Running, State, StoppedTask};

/// 开始唤醒
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloseDown<T: State> {
    #[serde(skip)]
    _phantom: PhantomData<T>,
    #[serde(skip)]
    id: Option<usize>,
}

impl<T: State> CloseDown<T> {
    pub fn new() -> Self {
        CloseDown {
            _phantom: PhantomData,
            id: None,
        }
    }
}

impl CloseDown<Paused> {
    pub fn new_paused() -> Self {
        Self::new()
    }

    pub fn start(self) -> CloseDown<Running> {
        CloseDown {
            _phantom: PhantomData,
            id: self.id,
        }
    }
}

impl<'a> StoppedTask<'a> for CloseDown<Paused> {
    fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }

    fn name(&self) -> &'static str {
        "CloseDown"
    }
}
