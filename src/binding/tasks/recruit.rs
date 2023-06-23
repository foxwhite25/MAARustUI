use std::collections::HashMap;
use std::marker::PhantomData;

use serde::Deserialize;
use serde::Serialize;

use crate::binding::tasks::{ClientType, Paused, Running, Server, State, StoppedTask};

fn is_zero(v: &usize) -> bool {
    *v == 0
}

/// 公开招募
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Recruit<T: State> {
    #[serde(skip)]
    _phantom: PhantomData<T>,

    refresh: bool,
    select: Vec<usize>,
    confirm: Vec<usize>,
    #[serde(skip_serializing_if = "is_zero")]
    times: usize,
    set_time: bool,

    expedite: bool,
    #[serde(skip_serializing_if = "is_zero")]
    expedite_times: usize,

    skip_robot: bool,
    recruitment_time: HashMap<String, usize>,

    report_to_penguin: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    penguin_id: String,
    report_to_yituliu: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    yituliu_id: String,

    server: String,
}

impl<T: State> Recruit<T> {
    pub fn new() -> Self {
        Recruit {
            _phantom: PhantomData,
            refresh: false,
            select: vec![3, 4, 5],
            confirm: vec![3, 5, 4, 6],
            times: 0,
            set_time: true,
            expedite: false,
            expedite_times: 0,
            skip_robot: true,
            recruitment_time: HashMap::from([
                ("3".to_string(), 540),
                ("4".to_string(), 540),
                ("5".to_string(), 540),
                ("6".to_string(), 540),
            ]),
            report_to_penguin: false,
            penguin_id: String::new(),
            report_to_yituliu: false,
            yituliu_id: String::new(),
            server: String::from("CN"),
        }
    }

    /// 是否刷新三星 Tags, 可选，默认 false
    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = refresh;
        self
    }

    /// 会去点击标签的 Tag 等级, 可选，默认 `[3,4,5]`。若仅公招计算，可设置为空数组
    pub fn select(mut self, select: Vec<usize>) -> Self {
        self.select = select;
        self
    }

    /// 会去点击确认的 Tag 等级, 可选，默认 `[3,5,4,6]`
    pub fn confirm(mut self, confirm: Vec<usize>) -> Self {
        self.confirm = confirm;
        self
    }

    /// 招募多少次，可选，默认 0。若仅公招计算，可设置为 0
    pub fn times(mut self, times: usize) -> Self {
        self.times = times;
        self
    }

    /// 是否设置招募时限。仅在 times 为 0 时生效，可选，默认 true
    pub fn set_time(mut self, set_time: bool) -> Self {
        self.set_time = set_time;
        self
    }

    /// 使用加急许可并设置次数，可选，默认 false
    pub fn expedite(mut self, expedite_times: usize) -> Self {
        self.expedite = true;
        self.expedite_times = expedite_times;
        self
    }

    /// 是否在识别到小车词条时跳过，可选，默认跳过
    pub fn skip_robot(mut self, skip_robot: bool) -> Self {
        self.skip_robot = skip_robot;
        self
    }

    /// Tag 等级（大于等于 3）和对应的希望招募时限，单位为分钟，默认值都为 540（即 09:00:00）
    pub fn recruitment_time(mut self, three: usize, four: usize, five: usize, six: usize) -> Self {
        self.recruitment_time = HashMap::from([
            ("3".to_string(), three),
            ("4".to_string(), four),
            ("5".to_string(), five),
            ("6".to_string(), six),
        ]);
        self
    }

    /// 是否向企鹅物流汇报招募结果，可选，默认 false
    pub fn report_to_penguin(mut self, penguin_id: String) -> Self {
        self.report_to_penguin = true;
        self.penguin_id = penguin_id;
        self
    }

    /// 是否向一图流汇报招募结果，可选，默认 false
    pub fn report_to_yituliu(mut self, yituliu_id: String) -> Self {
        self.report_to_yituliu = true;
        self.yituliu_id = yituliu_id;
        self
    }

    /// 服务器，可选，默认 CN
    pub fn server(mut self, server: Server) -> Self {
        self.server = server.as_ref().to_string();
        self
    }
}

impl<'a> StoppedTask<'a> for Recruit<Paused> {
    fn name(&self) -> &'static str {
        "Recruit"
    }
}
