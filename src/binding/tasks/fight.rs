use std::collections::HashMap;
use std::marker::PhantomData;

use serde::Deserialize;
use serde::Serialize;

use crate::binding::tasks::{ClientType, Paused, Running, Server, State, StoppedTask};

fn is_zero(v: &usize) -> bool {
    *v == 0
}

/// 刷理智
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Fight<T: State> {
    #[serde(skip)]
    _phantom: PhantomData<T>,
    #[serde(skip)]
    id: Option<usize>,

    /* Stage */
    #[serde(skip_serializing_if = "String::is_empty")]
    stage: String,

    /* Conditions */
    #[serde(skip_serializing_if = "is_zero")]
    medicine: usize,
    #[serde(skip_serializing_if = "is_zero")]
    expiring_medicine: usize,
    #[serde(skip_serializing_if = "is_zero")]
    stone: usize,
    #[serde(skip_serializing_if = "is_zero")]
    times: usize,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    drop: HashMap<String, usize>,

    report_to_penguin: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    penguin_id: String,

    server: String,
    client_type: String,

    #[serde(rename = "DrGrandet")]
    dr_grandet: bool,
}

impl<T: State> Fight<T> {
    pub fn new() -> Self {
        Fight {
            _phantom: PhantomData,
            id: None,
            stage: String::new(),
            medicine: 0,
            expiring_medicine: 0,
            stone: 0,
            times: 0,
            drop: HashMap::new(),
            report_to_penguin: false,
            penguin_id: String::new(),
            server: String::from("CN"),
            client_type: String::new(),
            dr_grandet: false,
        }
    }

    /// 设定最大使用理智药数量
    pub fn use_medicine(mut self, medicine: usize) -> Self {
        self.medicine = medicine;
        self
    }

    /// 设定最大使用 48 小时内过期理智药数量
    pub fn use_expiring_medicine(mut self, expiring_medicine: usize) -> Self {
        self.expiring_medicine = expiring_medicine;
        self
    }

    /// 设定最大使用源石数量
    pub fn use_stone(mut self, stone: usize) -> Self {
        self.stone = stone;
        self
    }

    /// 设定最大作战次数
    pub fn stop_with_times(mut self, times: usize) -> Self {
        self.times = times;
        self
    }

    /// 设定指定掉落数量，可选，默认不指定。
    ///
    /// Key 可参考 `resource/item_index.json` 文件
    ///
    /// # 例子
    /// ```
    /// use std::collections::HashMap;
    /// use maa_rust_ui::binding::tasks::{Fight, Running};
    ///
    /// fn set() -> Fight<Running> {
    ///     Fight::new().stop_when_drop(HashMap::from([("30012", 1)]))
    /// }
    /// ```
    ///
    /// 表示指定掉落 30012 一次
    pub fn stop_when_drop(mut self, drop: HashMap<&str, usize>) -> Self {
        let drop = drop.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
        self.drop = drop;
        self
    }

    /// 上报到企鹅物流，设定设定企鹅物流 ID，可选，默认 false
    pub fn report_to_penguin(mut self, penguin_id: String) -> Self {
        self.report_to_penguin = true;
        self.penguin_id = penguin_id;
        self
    }

    /// 设定服务器，可选，默认 "CN", 会影响掉落识别及上传
    pub fn server(mut self, server: Server) -> Self {
        self.server = server.as_ref().to_string();
        self
    }

    /// 设定客户端版本，可选，默认为空。用于游戏崩溃时重启并连回去继续刷，若为空则不启用该功能
    pub fn client_type(mut self, client_type: ClientType) -> Self {
        self.client_type = client_type.as_ref().to_string();
        self
    }

    /// 设定节省理智碎石模式，可选，默认 `false`，仅在可能产生碎石效果时生效。
    /// 在碎石确认界面等待，直到当前的 1 点理智恢复完成后再立刻碎石
    pub fn dr_grandet(mut self, dr_grandet: bool) -> Self {
        self.dr_grandet = dr_grandet;
        self
    }
}

impl Fight<Paused> {
    /// 关卡名，可选，默认为空，识别当前/上次的关卡。不支持运行中设置
    ///
    /// 支持全部主线关卡，如 `1-7`、`S3-2`等
    ///
    /// 可在关卡结尾输入`Normal/Hard`表示需要切换标准与磨难难度
    ///
    /// 另支持少部分资源关卡，如 `CE-6` 等 (请参考 C# 集成示例)
    pub fn stage(mut self, stage: &str) -> Self {
        self.stage = stage.to_string();
        self
    }

    pub fn new_paused() -> Self {
        Self::new()
    }

    pub fn start(self) -> Fight<Running> {
        Fight {
            _phantom: PhantomData,
            id: self.id,
            stage: self.stage,
            medicine: self.medicine,
            expiring_medicine: self.expiring_medicine,
            stone: self.stone,
            times: self.times,
            drop: self.drop,
            report_to_penguin: self.report_to_penguin,
            penguin_id: self.penguin_id,
            server: self.server,
            client_type: self.client_type,
            dr_grandet: self.dr_grandet,
        }
    }
}

impl<'a> StoppedTask<'a> for Fight<Paused> {
    fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }

    fn name(&self) -> &'static str {
        "Fight"
    }
}
