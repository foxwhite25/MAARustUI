use std::marker::PhantomData;

use serde::Deserialize;
use serde::Serialize;

use crate::binding::tasks::{Paused, State, StoppedTask};

pub enum ShopItem {
    LMD,
    RecruitmentPermit,
    ExpeditionPermit,
    CarbonStick,
    FurniturePart,
}

impl AsRef<str> for ShopItem {
    fn as_ref(&self) -> &str {
        match self {
            ShopItem::LMD => "龙门币",
            ShopItem::RecruitmentPermit => "招聘许可",
            ShopItem::ExpeditionPermit => "刷图许可",
            ShopItem::CarbonStick => "碳",
            ShopItem::FurniturePart => "家具",
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

    shopping: bool,
    buy_first: Vec<String>,
    blacklist: Vec<String>,

    force_shopping_if_credit_full: bool,
}

impl<T: State> Mall<T> {
    pub fn new() -> Self {
        Mall {
            _phantom: PhantomData,
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
    pub fn set_shopping(mut self, shopping: bool) -> Self {
        self.shopping = shopping;
        self
    }

    /// 设置优先购买的物品，可选，默认为空。不支持运行中设置
    pub fn set_buy_first(mut self, buy_first: Vec<ShopItem>) -> Self {
        self.buy_first = buy_first
            .into_iter()
            .map(|x| x.as_ref().to_string())
            .collect();
        self
    }

    /// 设置黑名单，可选，默认为空。不支持运行中设置
    pub fn set_blacklist(mut self, blacklist: Vec<ShopItem>) -> Self {
        self.blacklist = blacklist
            .into_iter()
            .map(|x| x.as_ref().to_string())
            .collect();
        self
    }

    /// 设置信用溢出时是否强制购物，可选，默认 false。不支持运行中设置
    pub fn set_force_shopping_if_credit_full(
        mut self,
        force_shopping_if_credit_full: bool,
    ) -> Self {
        self.force_shopping_if_credit_full = force_shopping_if_credit_full;
        self
    }
}

impl<'a> StoppedTask<'a> for Mall<Paused> {
    fn name(&self) -> &'static str {
        "Mall"
    }
}
