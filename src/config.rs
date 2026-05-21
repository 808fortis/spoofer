use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

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

const PROP_MAX: usize = 91;

pub fn parse_config(data: &str) -> Result<ModuleConfig, serde_json::Error> {
    serde_json::from_str(data)
}

pub fn build_package_map(config: ModuleConfig) -> HashMap<String, Arc<HashMap<String, String>>> {
    let total: usize = config.devices.values().map(|d| d.target.len()).sum();
    let mut map: HashMap<String, Arc<HashMap<String, String>>> = HashMap::with_capacity(total);

    for (_name, device) in config.devices {
        let props = Arc::new(sanitize_props(device.props));
        for pkg in device.target {
            map.insert(pkg, Arc::clone(&props));
        }
    }

    map
}

fn sanitize_props(props: HashMap<String, String>) -> HashMap<String, String> {
    let mut out = HashMap::with_capacity(props.len());
    for (k, v) in props {
        let truncated = if v.len() > PROP_MAX {
            v[..PROP_MAX].to_string()
        } else {
            v
        };
        out.insert(k, truncated);
    }
    out
}
