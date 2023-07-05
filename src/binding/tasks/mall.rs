use std::marker::PhantomData;

use serde::Deserialize;
use serde::Serialize;

use crate::binding::tasks::{Paused, Running, Server, State, StoppedTask};

pub enum ShopItem {
    LMD,
    RecruitmentPermit,
    ExpeditedPermit,
    CarbonStick,
    FurniturePart,
}

impl ShopItem {
    fn localize(&self, server: &Server) -> &'static str {
        match server {
            Server::CN => match self {
                ShopItem::LMD => "龙门币",
                ShopItem::RecruitmentPermit => "招聘",
                ShopItem::ExpeditedPermit => "加急",
                ShopItem::CarbonStick => "碳",
                ShopItem::FurniturePart => "家具",
            },
            Server::JP => match self {
                ShopItem::LMD => "龍門幣",
                ShopItem::RecruitmentPermit => "採用",
                ShopItem::ExpeditedPermit => "",
                ShopItem::CarbonStick => "炭素",
                ShopItem::FurniturePart => "家具",
            },
            Server::US => match self {
                ShopItem::LMD => "LMD",
                ShopItem::RecruitmentPermit => "Recruitment",
                ShopItem::ExpeditedPermit => "Expedited",
                ShopItem::CarbonStick => "Carbon",
                ShopItem::FurniturePart => "Furniture",
            },
            Server::KR => match self {
                ShopItem::LMD => "LMD",
                ShopItem::RecruitmentPermit => "채용 허가증",
                ShopItem::ExpeditedPermit => "탐색 허가증",
                ShopItem::CarbonStick => "탄소",
                ShopItem::FurniturePart => "가구 부품",
            },
        }
    }
}

/// 领取信用及商店购物
///
///  会先有序的按 `buy_first` 购买一遍，再从左到右并避开 `blacklist` 购买第二遍，在信用溢出时则会无视黑名单从左到右购买第三遍直到不再溢出
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Mall<T: State> {
    #[serde(skip)]
    _phantom: PhantomData<T>,
    #[serde(skip)]
    id: Option<usize>,

    shopping: bool,
    buy_first: Vec<String>,
    blacklist: Vec<String>,

    force_shopping_if_credit_full: bool,
}

impl<T: State> Mall<T> {
    pub fn new() -> Self {
        Mall {
            _phantom: PhantomData,
            id: None,
            shopping: false,
            buy_first: Vec::new(),
            blacklist: Vec::new(),
            force_shopping_if_credit_full: false,
        }
    }
}

impl Mall<Paused> {
    pub fn new_paused() -> Self {
        Self::new()
    }

    /// 设置是否购物，可选，默认 false。不支持运行中设置
    pub fn shopping(mut self, shopping: bool) -> Self {
        self.shopping = shopping;
        self
    }

    /// 设置优先购买的物品，可选，默认为空。不支持运行中设置
    pub fn buy_first(mut self, buy_first: Vec<ShopItem>, server: &Server) -> Self {
        self.buy_first = buy_first
            .into_iter()
            .map(|x| x.localize(server).to_string())
            .collect();
        self
    }

    /// 设置黑名单，可选，默认为空。不支持运行中设置
    pub fn blacklist(mut self, blacklist: Vec<ShopItem>, server: &Server) -> Self {
        self.blacklist = blacklist
            .into_iter()
            .map(|x| x.localize(server).to_string())
            .collect();
        self
    }

    /// 设置信用溢出时是否强制购物，可选，默认 false。不支持运行中设置
    pub fn force_buy_when_full(mut self, force_buy_when_full: bool) -> Self {
        self.force_shopping_if_credit_full = force_buy_when_full;
        self
    }

    pub fn start(self) -> Mall<Running> {
        Mall {
            _phantom: PhantomData,
            id: self.id,
            shopping: self.shopping,
            buy_first: self.buy_first,
            blacklist: self.blacklist,
            force_shopping_if_credit_full: self.force_shopping_if_credit_full,
        }
    }
}

impl<'a> StoppedTask<'a> for Mall<Paused> {
    fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }

    fn name(&self) -> &'static str {
        "Mall"
    }
}
