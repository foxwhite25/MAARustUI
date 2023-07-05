use std::marker::PhantomData;

use serde::Deserialize;
use serde::Serialize;

use crate::binding::tasks::{Paused, Running, State, StoppedTask};

/// 领取日常奖励
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Award<T: State> {
    #[serde(skip)]
    _phantom: PhantomData<T>,
    #[serde(skip)]
    id: Option<usize>,
}

impl<T: State> Award<T> {
    pub fn new() -> Self {
        Award {
            _phantom: PhantomData,
            id: None,
        }
    }
}

impl Award<Paused> {
    pub fn new_paused() -> Self {
        Self::new()
    }

    pub fn start(self) -> Award<Running> {
        Award {
            _phantom: PhantomData,
            id: self.id,
        }
    }
}

impl<'a> StoppedTask<'a> for Award<Paused> {
    fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }

    fn name(&self) -> &'static str {
        "Award"
    }
}
