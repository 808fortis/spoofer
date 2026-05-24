use std::cell::Cell;
use std::ffi::CStr;
use std::ptr;
use std::sync::OnceLock;
use std::sync::atomic::Ordering;

use libc::{self, c_char, c_int};

use crate::PropPair;

macro_rules! hook_state {
    ( $ret:ty $(, $arg:ty)* $(,)? ) => {
        static ORIG: std::sync::atomic::AtomicUsize =
            std::sync::atomic::AtomicUsize::new(0);
        static INSTALLED: std::sync::atomic::AtomicBool =
            std::sync::atomic::AtomicBool::new(false);

        fn orig() -> unsafe extern "C" fn($($arg),*) -> $ret {
            unsafe {
                std::mem::transmute::<usize, unsafe extern "C" fn($($arg),*) -> $ret>(
                    ORIG.load(std::sync::atomic::Ordering::Relaxed),
                )
            }
        }

        pub fn is_installed() -> bool {
            INSTALLED.load(std::sync::atomic::Ordering::Acquire)
        }
    };
}

hook_state!(c_int, *const c_char, *mut c_char);

static SPOOF_PROPS: OnceLock<&'static [PropPair]> = OnceLock::new();

thread_local! {
    static REENTRY: Cell<bool> = const { Cell::new(false) };
}

pub fn set_spoof_props(props: &'static [PropPair]) {
    let _ = SPOOF_PROPS.set(props);
}

pub fn has_spoof_props() -> bool {
    SPOOF_PROPS.get().is_some()
}

#[inline(always)]
unsafe fn call_orig(name: *const c_char, value: *mut c_char) -> c_int {
    REENTRY.with(|r| r.set(false));
    orig()(name, value)
}

unsafe extern "C" fn handler(name: *const c_char, value: *mut c_char) -> c_int {
    if REENTRY.with(|r| r.replace(true)) {
        return orig()(name, value);
    }

    let prop = match CStr::from_ptr(name).to_str() {
        Ok(s) => s,
        Err(_) => return call_orig(name, value),
    };

    if let Some(props) = SPOOF_PROPS.get() {
        for p in *props {
            if p.k == prop {
                let b = p.v.as_bytes();
                ptr::copy_nonoverlapping(b.as_ptr(), value as *mut u8, b.len());
                *value.add(b.len()) = 0;
                REENTRY.with(|r| r.set(false));
                return b.len() as c_int;
            }
        }
    }

    call_orig(name, value)
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
