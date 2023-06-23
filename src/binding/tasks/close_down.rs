use std::marker::PhantomData;
use crate::binding::tasks::{Paused, State, StoppedTask};
use serde::Deserialize;
use serde::Serialize;

/// 开始唤醒
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloseDown<T: State> {
    #[serde(skip)]
    _phantom: PhantomData<T>,
}

impl<T: State> CloseDown<T> {
    pub fn new() -> Self {
        CloseDown {
            _phantom: PhantomData,
        }
    }
}

impl<'a> StoppedTask<'a> for CloseDown<Paused> {
    fn name(&self) -> &'static str {
        "CloseDown"
    }
}