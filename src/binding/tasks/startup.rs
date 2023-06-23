use std::marker::PhantomData;
use crate::binding::tasks::{ClientType, Paused, State, StoppedTask};
use serde::Deserialize;
use serde::Serialize;

/// 开始唤醒
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartUp<T: State> {
    #[serde(skip)]
    _phantom: PhantomData<T>,

    client_type: String,
    start_game_enabled: bool,
}

impl<T: State> StartUp<T> {
    pub fn new() -> Self {
        StartUp {
            _phantom: PhantomData,
            client_type: String::new(),
            start_game_enabled: false,
        }
    }

    /// 设置客户端版本，可选，默认为空
    pub fn set_client_type(mut self, client_type: ClientType) -> Self {
        self.client_type = client_type.as_ref().to_string();
        self
    }

    /// 是否自动启动客户端，可选，默认不启动
    pub fn set_start_game_enabled(mut self, start_game_enabled: bool) -> Self {
        self.start_game_enabled = start_game_enabled;
        self
    }
}

impl<'a> StoppedTask<'a> for StartUp<Paused> {
    fn name(&self) -> &'static str {
        "StartUp"
    }
}