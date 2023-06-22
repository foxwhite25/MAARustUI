use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InstanceOptionKey {
    TouchMode = 2,
    DeploymentWithPause = 3,
    AdbLiteEnabled = 4,
    KillAdbOnExit = 5,
}

pub enum TouchMode {
    MiniTouch,
    MAATouch,
    ADB,
}

impl From<&str> for TouchMode {
    fn from(s: &str) -> Self {
        match s {
            "minitouch" => Self::MiniTouch,
            "maatouch" => Self::MAATouch,
            "adb" => Self::ADB,
            _ => panic!("Invalid touch mode: {}", s),
        }
    }
}

impl From<String> for TouchMode {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

pub struct MAAOption {
    touch_mode: TouchMode,
    deployment_with_pause: bool,
    adb_lite_enabled: bool,
    kill_adb_on_exit: bool,
}

impl Default for MAAOption {
    fn default() -> Self {
        Self {
            touch_mode: TouchMode::MiniTouch,
            deployment_with_pause: false,
            adb_lite_enabled: false,
            kill_adb_on_exit: false,
        }
    }
}

impl MAAOption {
    pub fn with_touch_mode(mut self, touch_mode: TouchMode) -> Self {
        self.touch_mode = touch_mode;
        self
    }

    pub fn with_deployment_with_pause(mut self, deployment_with_pause: bool) -> Self {
        self.deployment_with_pause = deployment_with_pause;
        self
    }

    pub fn with_adb_lite_enabled(mut self, adb_lite_enabled: bool) -> Self {
        self.adb_lite_enabled = adb_lite_enabled;
        self
    }

    pub fn with_kill_adb_on_exit(mut self, kill_adb_on_exit: bool) -> Self {
        self.kill_adb_on_exit = kill_adb_on_exit;
        self
    }

    pub fn to_map(&self) -> HashMap<InstanceOptionKey, &str> {
        let mut map = HashMap::new();
        map.insert(
            InstanceOptionKey::TouchMode,
            match self.touch_mode {
                TouchMode::MiniTouch => "minitouch",
                TouchMode::MAATouch => "maatouch",
                TouchMode::ADB => "adb",
            },
        );
        map.insert(
            InstanceOptionKey::DeploymentWithPause,
            match self.deployment_with_pause {
                true => "1",
                false => "0",
            },
        );
        map.insert(
            InstanceOptionKey::AdbLiteEnabled,
            match self.adb_lite_enabled {
                true => "1",
                false => "0",
            },
        );
        map.insert(
            InstanceOptionKey::KillAdbOnExit,
            match self.kill_adb_on_exit {
                true => "1",
                false => "0",
            },
        );
        map
    }
}
