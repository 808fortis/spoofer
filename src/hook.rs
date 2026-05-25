use std::cell::Cell;
use std::ptr;
use std::sync::OnceLock;
use std::sync::atomic::Ordering;

use libc::{self, c_char, c_int};

use crate::SpoofCfg;

const H_BRAND: u64 = 0x853125a8179488b4;
const H_DEVICE: u64 = 0xbf514b56a99442b9;
const H_MANUFACTURER: u64 = 0x8e0d40508711b26a;
const H_MODEL: u64 = 0x853b8f3473e8cc30;
const H_PRODUCT_FP: u64 = 0x8a6cf1ae3db75295;
const H_PRODUCT_PRODUCT: u64 = 0xc7d0c1232cd2eeaa;
const H_BUILD_FP: u64 = 0xbfe94d2aa3c9bc62;
const H_BUILD_PRODUCT: u64 = 0xaf2f85e8fcdc43c9;

static ORIG: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);
static INSTALLED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

unsafe fn orig(name: *const c_char, value: *mut c_char) -> c_int {
    std::mem::transmute::<usize, unsafe extern "C" fn(*const c_char, *mut c_char) -> c_int>(
        ORIG.load(std::sync::atomic::Ordering::Relaxed),
    )(name, value)
}

static SPOOF_CFG: OnceLock<&'static SpoofCfg> = OnceLock::new();

thread_local! {
    static REENTRY: Cell<bool> = const { Cell::new(false) };
}

pub fn set_spoof_props(cfg: &'static SpoofCfg) {
    let _ = SPOOF_CFG.set(cfg);
}

pub fn has_spoof_props() -> bool {
    SPOOF_CFG.get().is_some()
}

#[inline(always)]
unsafe fn passthru(name: *const c_char, value: *mut c_char) -> c_int {
    REENTRY.with(|r| r.set(false));
    orig(name, value)
}

#[inline(always)]
unsafe fn prop_index(name: *const c_char) -> Option<usize> {
    let mut h = 0xcbf29ce484222325u64;
    let mut p = name;
    loop {
        let b = *p as u8;
        if b == 0 { break; }
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
        p = p.offset(1);
    }
    match h {
        H_BRAND => Some(0),
        H_DEVICE => Some(1),
        H_MANUFACTURER => Some(2),
        H_MODEL => Some(3),
        H_PRODUCT_FP => Some(4),
        H_PRODUCT_PRODUCT => Some(5),
        H_BUILD_FP => Some(6),
        H_BUILD_PRODUCT => Some(7),
        _ => None,
    }
}

unsafe extern "C" fn handler(name: *const c_char, value: *mut c_char) -> c_int {
    if REENTRY.with(|r| r.replace(true)) {
        return orig(name, value);
    }

    let cfg = match SPOOF_CFG.get() {
        Some(c) => c,
        None => return passthru(name, value),
    };

    let idx = match prop_index(name) {
        Some(i) => i,
        None => return passthru(name, value),
    };

    let bytes = cfg.0[idx].as_bytes();
    ptr::copy_nonoverlapping(bytes.as_ptr(), value as *mut u8, bytes.len());
    *value.add(bytes.len()) = 0;
    REENTRY.with(|r| r.set(false));
    bytes.len() as c_int
}

pub unsafe fn install_hook() {
    if INSTALLED.swap(true, Ordering::AcqRel) {
        return;
    }
    if !crate::trampoline::install(
        b"__system_property_get\0",
        handler as usize,
        &ORIG,
    ) {
        INSTALLED.store(false, Ordering::Release);
    }
}
