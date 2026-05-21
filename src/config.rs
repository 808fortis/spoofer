use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Deserialize, Clone)]
pub struct DeviceConfig {
    pub target: Vec<String>,
    #[serde(default = "default_true")]
    pub cpu_spoof: bool,
    pub cpu_cores: Option<u32>,
    pub cpu_implementer: Option<String>,
    pub cpu_architecture: Option<String>,
    pub cpu_variant: Option<String>,
    pub cpu_part: Option<String>,
    pub cpu_revision: Option<String>,
    pub cpu_features: Option<String>,
    pub cpu_bogomips: Option<String>,
    pub sys_cpu_possible: Option<String>,
    pub sys_cpu_present: Option<String>,
    pub sys_cpu_online: Option<String>,
    pub sys_cpu_kernel_max: Option<String>,
    #[serde(flatten)]
    pub props: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct ModuleConfig {
    #[serde(flatten)]
    pub devices: HashMap<String, DeviceConfig>,
}

#[derive(Clone)]
pub struct SpoofFiles {
    pub cpuinfo: Option<String>,
    pub sys_cpu_possible: Option<String>,
    pub sys_cpu_present: Option<String>,
    pub sys_cpu_online: Option<String>,
    pub sys_cpu_kernel_max: Option<String>,
}

pub struct SpoofBundle {
    pub props: Arc<HashMap<String, String>>,
    pub files: Option<Arc<SpoofFiles>>,
}

const PROP_MAX: usize = 91;

fn default_true() -> bool {
    true
}

pub fn parse_config(data: &str) -> Result<ModuleConfig, serde_json::Error> {
    serde_json::from_str(data)
}

pub fn build_package_map(
    config: ModuleConfig,
) -> HashMap<String, SpoofBundle> {
    let total: usize = config.devices.values().map(|d| d.target.len()).sum();
    let mut out: HashMap<String, SpoofBundle> = HashMap::with_capacity(total);

    for (_name, dev) in config.devices {
        let props = Arc::new(sanitize_props(dev.props));
        let files = dev.cpu_spoof.then(|| {
            let cpuinfo = gen_cpuinfo(
                dev.cpu_cores,
                &dev.cpu_implementer,
                &dev.cpu_architecture,
                &dev.cpu_variant,
                &dev.cpu_part,
                &dev.cpu_revision,
                &dev.cpu_features,
                &dev.cpu_bogomips,
            );
            Arc::new(SpoofFiles {
                cpuinfo,
                sys_cpu_possible: dev.sys_cpu_possible,
                sys_cpu_present: dev.sys_cpu_present,
                sys_cpu_online: dev.sys_cpu_online,
                sys_cpu_kernel_max: dev.sys_cpu_kernel_max,
            })
        });
        let spoof = SpoofBundle { props, files };
        for pkg in dev.target {
            out.insert(pkg, SpoofBundle {
                props: Arc::clone(&spoof.props),
                files: spoof.files.clone(),
            });
        }
    }

    out
}

fn gen_cpuinfo(
    cores: Option<u32>,
    implementer: &Option<String>,
    architecture: &Option<String>,
    variant: &Option<String>,
    part: &Option<String>,
    revision: &Option<String>,
    features: &Option<String>,
    bogomips: &Option<String>,
) -> Option<String> {
    let n = cores?;
    let imp = implementer.as_deref()?;
    let arch = architecture.as_deref()?;
    let var = variant.as_deref()?;
    let p = part.as_deref()?;
    let rev = revision.as_deref()?;
    let feat = features.as_deref()?;
    let bogo = bogomips.as_deref()?;

    let mut out = String::with_capacity(n as usize * 192);
    for i in 0..n {
        if i > 0 { out.push('\n'); }
        out.push_str("processor\t: ");
        out.push_str(&i.to_string());
        out.push_str("\nBogoMIPS\t: ");   out.push_str(bogo);
        out.push_str("\nFeatures\t: ");   out.push_str(feat);
        out.push_str("\nCPU implementer\t: "); out.push_str(imp);
        out.push_str("\nCPU architecture\t: "); out.push_str(arch);
        out.push_str("\nCPU variant\t: "); out.push_str(var);
        out.push_str("\nCPU part\t: ");   out.push_str(p);
        out.push_str("\nCPU revision\t: "); out.push_str(rev);
    }
    Some(out)
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
