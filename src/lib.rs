mod config;
mod hook;

use std::collections::HashMap;
use std::ffi::CStr;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

use libc::c_void;

static PACKAGE_MAP: OnceLock<HashMap<String, HashMap<String, String>>> = OnceLock::new();

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
    let module_path = find_module_path();
    if let Some(path) = module_path {
        let config_path = Path::new(&path).join("config.json");
        if let Ok(data) = fs::read_to_string(&config_path) {
            if let Ok(cfg) = config::parse_config(&data) {
                let pkg_map = config::build_package_map(cfg);
                let _ = PACKAGE_MAP.set(pkg_map);
            }
        }
    }
    4
}

unsafe extern "C" fn pre_app_specialize(_id: i32, args: *mut c_void) {
    let args = args as *const AppSpecializeArgs;
    if args.is_null() {
        return;
    }

    let nice_name_ptr = (*args).nice_name;
    if nice_name_ptr.is_null() {
        return;
    }

    let nice_name = match CStr::from_ptr(nice_name_ptr).to_str() {
        Ok(s) => s,
        Err(_) => return,
    };

    let map = match PACKAGE_MAP.get() {
        Some(m) => m,
        None => return,
    };

    if let Some(props) = map.get(nice_name) {
        hook::set_spoof_props(props.clone());
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
    let maps = fs::read_to_string("/proc/self/maps").ok()?;
    for line in maps.lines() {
        if !line.contains("/zygisk/") {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        let path = *parts.last()?;
        if !path.ends_with(".so") {
            continue;
        }
        let lib_path = Path::new(path);
        let module_dir = lib_path.parent()?.parent()?;
        return Some(module_dir.to_str()?.to_string());
    }
    None
}
