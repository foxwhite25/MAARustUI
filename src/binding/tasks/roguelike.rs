use std::marker::PhantomData;

use serde::Deserialize;
use serde::Serialize;

use crate::binding::tasks::{Paused, Running, State, StoppedTask};

pub enum RogueLikeTheme {
    /// 傀影与猩红血钻
    Phantom,
    /// 水月与深蓝之树
    Mizuki
}

impl AsRef<str> for RogueLikeTheme {
    fn as_ref(&self) -> &str {
        match self {
            RogueLikeTheme::Phantom => "Phantom",
            RogueLikeTheme::Mizuki => "Mizuki",
        }
    }
}

pub enum RogueLikeMode {
    /// 刷积分，尽可能稳定的打更多层数
    MostFloors,
    /// 刷源石锭，第一层投资完就退出
    FarmMoney,
}

fn is_zero(v: &usize) -> bool {
    *v == 0
}

pub enum RogueLikeSquad {
    /// 指挥分队
    Leader,
    /// 集群分队
    Gathering,
    /// 后勤分队
    Support,
    /// 矛头分队
    Spearhead,
    /// 突击战术分队
    TacticalAssault,
    /// 堡垒战术分队
    TacticalFortification,
    /// 远程战术分队
    TacticalRanged,
    /// 破坏战术分队
    TacticalDestruction,
    /// 研究分队
    Research,
    /// 高规格分队
    FirstClass,
    /// 心胜于物分队
    MindOverMatter,
    /// 物尽其用分队
    Resourceful,
    /// 以人为本分队
    PeopleOriented,
}

impl AsRef<str> for RogueLikeSquad {
    fn as_ref(&self) -> &str {
        match self {
            RogueLikeSquad::Leader => "指挥分队",
            RogueLikeSquad::Gathering => "集群分队",
            RogueLikeSquad::Support => "后勤分队",
            RogueLikeSquad::Spearhead => "矛头分队",
            RogueLikeSquad::TacticalAssault => "突击战术分队",
            RogueLikeSquad::TacticalFortification => "堡垒战术分队",
            RogueLikeSquad::TacticalRanged => "远程战术分队",
            RogueLikeSquad::TacticalDestruction => "破坏战术分队",
            RogueLikeSquad::Research => "研究分队",
            RogueLikeSquad::FirstClass => "高规格分队",
            RogueLikeSquad::MindOverMatter => "心胜于物分队",
            RogueLikeSquad::Resourceful => "物尽其用分队",
            RogueLikeSquad::PeopleOriented => "以人为本分队",
        }
    }
}

pub enum RogueLikeRoles {
    /// 随心所欲
    AsYourHeartDesires,
    /// 先手必胜
    FireMoveAdvance,
    /// 稳扎稳打
    SlowAndSteayWinsTheRace,
    /// 取长补短
    OvercomingYourWeakness,
}

impl AsRef<str> for RogueLikeRoles {
    fn as_ref(&self) -> &str {
        match self {
            RogueLikeRoles::AsYourHeartDesires => "随心所欲",
            RogueLikeRoles::FireMoveAdvance => "先手必胜",
            RogueLikeRoles::SlowAndSteayWinsTheRace => "稳扎稳打",
            RogueLikeRoles::OvercomingYourWeakness => "取长补短",
        }
    }
}

/// 无限刷肉鸽
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RogueLike<T: State> {
    #[serde(skip)]
    _phantom: PhantomData<T>,
    #[serde(skip)]
    id: Option<usize>,

    theme: String,
    mode: usize,
    #[serde(skip_serializing_if = "is_zero")]
    starts_count: usize,

    investment_enabled: bool,
    #[serde(skip_serializing_if = "is_zero")]
    investment_count: usize,
    stop_when_investment_full: bool,

    #[serde(skip_serializing_if = "String::is_empty")]
    squad: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    roles: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    core_char: String,
    use_support: bool,
    use_nonfriend_support: bool,

    refresh_trader_with_dice: bool,
}

impl<T: State> RogueLike<T> {
    pub fn new() -> Self {
        RogueLike {
            _phantom: PhantomData,
            id: None,
            theme: "Phantom".to_string(),
            mode: 0,
            starts_count: 0,
            investment_enabled: true,
            investment_count: 0,
            stop_when_investment_full: false,
            squad: "指挥分队".to_string(),
            roles: "取长补短".to_string(),
            core_char: "".to_string(),
            use_support: false,
            use_nonfriend_support: false,
            refresh_trader_with_dice: false,
        }
    }

    /// 肉鸽名
    pub fn theme(mut self, theme: RogueLikeTheme) -> Self {
        self.theme = theme.as_ref().to_string();
        self
    }

    /// 模式
    pub fn mode(mut self, mode: RogueLikeMode) -> Self {
        self.mode = mode as usize;
        self
    }

    /// 开始探索次数，可选，默认 INT_MAX。达到后自动停止任务
    pub fn starts_count(mut self, starts_count: usize) -> Self {
        self.starts_count = starts_count;
        self
    }

    /// 是否投资源石锭，默认开
    pub fn investment_enabled(mut self, investment_enabled: bool) -> Self {
        self.investment_enabled = investment_enabled;
        self
    }

    /// 投资源石锭次数，可选，默认 INT_MAX。达到后自动停止任务
    pub fn investment_count(mut self, investment_count: usize) -> Self {
        self.investment_count = investment_count;
        self
    }

    /// 投资满了自动停止任务，可选，默认 false
    pub fn stop_when_investment_full(mut self, stop_when_investment_full: bool) -> Self {
        self.stop_when_investment_full = stop_when_investment_full;
        self
    }

    /// 开局分队，可选，例如 "突击战术分队" 等，默认 "指挥分队"
    pub fn squad(mut self, squad: RogueLikeSquad) -> Self {
        self.squad = squad.as_ref().to_string();
        self
    }

    /// 开局职业组，可选，例如 "先手必胜" 等，默认 "取长补短"
    pub fn roles(mut self, roles: RogueLikeRoles) -> Self {
        self.roles = roles.as_ref().to_string();
        self
    }

    /// 开局干员名，可选，仅支持单个干员中！文！名！。默认识别练度自动选择
    pub fn core_char(mut self, core_char: &str) -> Self {
        self.core_char = core_char.to_string();
        self
    }

    /// 开局干员是否为助战干员，是否可以是非好友助战干员，默认 false
    pub fn use_support(mut self, use_support: bool, use_nonfriend_support: bool) -> Self {
        self.use_support = use_support;
        self.use_nonfriend_support = use_nonfriend_support;
        self
    }

    /// 是否用骰子刷新商店购买特殊商品，目前支持水月肉鸽的指路鳞，可选，默认 false
    pub fn refresh_trader_with_dice(mut self, refresh_trader_with_dice: bool) -> Self {
        self.refresh_trader_with_dice = refresh_trader_with_dice;
        self
    }
}

impl RogueLike<Paused> {
    pub fn new_paused() -> Self {
        Self::new()
    }

    pub fn start(self) -> RogueLike<Running> {
        RogueLike {
            _phantom: PhantomData,
            id: None,
            theme: self.theme,
            mode: self.mode,
            starts_count: self.starts_count,
            investment_enabled: self.investment_enabled,
            investment_count: self.investment_count,
            stop_when_investment_full: self.stop_when_investment_full,
            squad: self.squad,
            roles: self.roles,
            core_char: self.core_char,
            use_support: self.use_support,
            use_nonfriend_support: self.use_nonfriend_support,
            refresh_trader_with_dice: self.refresh_trader_with_dice,
        }
    }
}

impl<'a> StoppedTask<'a> for RogueLike<Paused> {
    fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }

    fn name(&self) -> &'static str {
        "Roguelike"
    }
}
