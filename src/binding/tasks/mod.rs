use anyhow::Result;
use serde::{Deserialize, Serialize};

pub use close_down::*;
pub use fight::*;
pub use mall::*;
pub use recruit::*;
pub use startup::*;
pub use award::*;

use crate::binding::connection::MAAConnection;

mod close_down;
mod fight;
mod mall;
mod recruit;
mod startup;
mod award;
mod roguelike;

pub trait StoppedTask<'a>: Deserialize<'a> + Serialize {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn set_id(&mut self, id: usize);

    fn name(&self) -> &'static str;

    fn append_in(mut self, maa: &mut MAAConnection) -> Result<Self> {
        let id = maa.append_task(&self)?;
        self.set_id(id);
        Ok(self)
    }
}

#[derive(Copy, Clone)]
pub enum ClientType {
    Official,
    Bilibili,
    Twxy,
    YoStarEN,
    YoStarJP,
    YoStarKR,
}

impl AsRef<str> for ClientType {
    fn as_ref(&self) -> &str {
        match self {
            ClientType::Official => "Official",
            ClientType::Bilibili => "Bilibili",
            ClientType::Twxy => "twxy",
            ClientType::YoStarEN => "YoStarEN",
            ClientType::YoStarJP => "YoStarJP",
            ClientType::YoStarKR => "YoStarKR",
        }
    }
}

#[derive(Copy, Clone)]
pub enum Server {
    CN,
    JP,
    KR,
    US,
}

impl AsRef<str> for Server {
    fn as_ref(&self) -> &str {
        match self {
            Server::CN => "CN",
            Server::JP => "JP",
            Server::KR => "KR",
            Server::US => "US",
        }
    }
}

pub trait State {}

pub struct Running {}

pub struct Paused {}

impl State for Running {}

impl State for Paused {}
