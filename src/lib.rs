mod config;
mod hook;

use std::collections::HashMap;
use std::ffi::CStr;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;
use std::sync::OnceLock;

use libc::c_void;

static PACKAGE_MAP: OnceLock<Arc<HashMap<String, Arc<HashMap<String, String>>>>> = OnceLock::new();

#[repr(C)]
pub struct AppSpecializeArgs {
    pub rva: *mut c_void,
    pub pid: i32,
    pub app_data_dir: *const libc::c_char,
    pub nice_name: *const libc::c_char,
    pub process_type: *mut i64,
    pub mount_ns: *const libc::c_char,
    pub se_info: *const libc::c_char,
    pub se_name: *const libc::c_char,
    pub mount_mode: *mut i32,
    pub is_child_zygote: *mut i32,
    pub uid: *mut i64,
}

#[repr(C)]
pub struct ZygiskModule {
    pub on_module_loaded: Option<unsafe extern "C" fn(i32) -> i32>,
    pub pre_app_specialize: Option<unsafe extern "C" fn(i32, *mut c_void)>,
    pub post_app_specialize: Option<unsafe extern "C" fn(i32, *const c_void)>,
    pub on_process_loaded: Option<unsafe extern "C" fn(i32)>,
}

unsafe extern "C" fn on_module_loaded(_id: i32) -> i32 {
    let path = find_module_path();
    if let Some(p) = path {
        let cfg_path = Path::new(&p).join("config.json");
        if let Ok(data) = fs::read_to_string(&cfg_path) {
            if let Ok(cfg) = config::parse_config(&data) {
                let map = config::build_package_map(cfg);
                let _ = PACKAGE_MAP.set(Arc::new(map));
            }
        }
    }
    4
}

unsafe extern "C" fn pre_app_specialize(_id: i32, args: *mut c_void) {
    let args = args as *const AppSpecializeArgs;
    if args.is_null() { return; }

    let ptr = (*args).nice_name;
    if ptr.is_null() { return; }

    let name = match CStr::from_ptr(ptr).to_str() {
        Ok(s) => s,
        Err(_) => return,
    };

    if let Some(ref map) = PACKAGE_MAP.get() {
        if let Some(props) = map.get(name) {
            hook::set_spoof_props(Arc::clone(props));
        }
    }
}

unsafe extern "C" fn post_app_specialize(_id: i32, _args: *const c_void) {
    if hook::has_spoof_props() {
        hook::install_hook();
    }
}

unsafe extern "C" fn on_process_loaded(_id: i32) {}

#[no_mangle]
pub static zygisk_module: ZygiskModule = ZygiskModule {
    on_module_loaded: Some(on_module_loaded),
    pre_app_specialize: Some(pre_app_specialize),
    post_app_specialize: Some(post_app_specialize),
    on_process_loaded: Some(on_process_loaded),
};

fn find_module_path() -> Option<String> {
    let f = fs::File::open("/proc/self/maps").ok()?;
    for line in BufReader::new(f).lines() {
        let line = line.ok()?;
        if !line.contains("/zygisk/") { continue; }
        let path = line.split_whitespace().last()?;
        if !path.ends_with(".so") { continue; }
        let dir = Path::new(path).parent()?.parent()?;
        return Some(dir.to_str()?.to_string());
    }
    None
}
