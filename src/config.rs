use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
pub struct DeviceConfig {
    pub target: Vec<String>,
    #[serde(flatten)]
    pub props: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct ModuleConfig {
    #[serde(flatten)]
    pub devices: HashMap<String, DeviceConfig>,
}

pub fn parse_config(data: &str) -> Result<ModuleConfig, serde_json::Error> {
    serde_json::from_str(data)
}

pub fn build_package_map(config: ModuleConfig) -> HashMap<String, HashMap<String, String>> {
    let mut map = HashMap::new();
    for (_name, device) in config.devices {
        for pkg in device.target {
            map.entry(pkg).or_insert_with(HashMap::new).extend(device.props.clone());
        }
    }
    map
}
