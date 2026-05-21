use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;

static SPOOF_PROPS: OnceLock<Arc<HashMap<String, String>>> = OnceLock::new();

pub fn set_spoof_props(props: Arc<HashMap<String, String>>) {
    let _ = SPOOF_PROPS.set(props);
}

pub fn has_spoof_props() -> bool {
    SPOOF_PROPS.get().is_some()
}

mod arch {
    use super::*;
    use std::cell::Cell;
    use std::ffi::CStr;
    use std::ptr;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use libc::{self, c_char, c_int, c_void};

    type PropGet = unsafe extern "C" fn(*const c_char, *mut c_char) -> c_int;

    static ORIG_FUNC: AtomicUsize = AtomicUsize::new(0);
    static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);

    thread_local! {
        static REENTRY: Cell<bool> = const { Cell::new(false) };
    }

    #[inline(always)]
    fn get_orig() -> PropGet {
        let ptr = ORIG_FUNC.load(Ordering::Relaxed);
        unsafe { std::mem::transmute::<usize, PropGet>(ptr) }
    }

    #[inline(always)]
    fn call_orig(name: *const c_char, value: *mut c_char) -> c_int {
        REENTRY.with(|r| r.set(false));
        unsafe { get_orig()(name, value) }
    }

    unsafe extern "C" fn hook_handler(name: *const c_char, value: *mut c_char) -> c_int {
        if REENTRY.with(|r| r.replace(true)) {
            return unsafe { get_orig()(name, value) };
        }

        let prop_name = match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return call_orig(name, value),
        };

        if let Some(val) = SPOOF_PROPS.get().and_then(|m| m.get(prop_name)) {
            let bytes = val.as_bytes();
            ptr::copy_nonoverlapping(bytes.as_ptr(), value as *mut u8, bytes.len());
            *value.add(bytes.len()) = 0;
            REENTRY.with(|r| r.set(false));
            return bytes.len() as c_int;
        }

        call_orig(name, value)
    }

    #[inline(always)]
    fn make_jump_stub(target: usize) -> [u8; 16] {
        let mut stub = [0u8; 16];
        stub[..4].copy_from_slice(&0x58000050u32.to_le_bytes());
        stub[4..8].copy_from_slice(&0xD61F0200u32.to_le_bytes());
        stub[8..16].copy_from_slice(&target.to_le_bytes());
        stub
    }

    #[inline(always)]
    fn flush_cache(start: *mut c_void, end: *mut c_void) {
        extern "C" {
            fn __clear_cache(start: *mut c_void, end: *mut c_void);
        }
        unsafe { __clear_cache(start, end) };
    }

    pub unsafe fn install_hook() {
        if HOOK_INSTALLED.load(Ordering::Acquire) {
            return;
        }
        HOOK_INSTALLED.store(true, Ordering::Release);

        let target_name = b"__system_property_get\0";
        let target = libc::dlsym(libc::RTLD_DEFAULT, target_name.as_ptr() as *const c_char);

        if target.is_null() {
            HOOK_INSTALLED.store(false, Ordering::Release);
            return;
        }

        let page_size = libc::sysconf(libc::_SC_PAGESIZE) as usize;
        let page_start = (target as usize) & !(page_size - 1);
        let page_end = ((target as usize) + 16 + page_size - 1) & !(page_size - 1);

        if libc::mprotect(
            page_start as *mut c_void,
            page_end - page_start,
            libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
        ) != 0 {
            HOOK_INSTALLED.store(false, Ordering::Release);
            return;
        }

        let mut orig: [u8; 16] = [0u8; 16];
        ptr::copy_nonoverlapping(target as *const u8, orig.as_mut_ptr(), 16);

        let trampoline = libc::mmap(
            ptr::null_mut(),
            32,
            libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );

        if trampoline == libc::MAP_FAILED {
            HOOK_INSTALLED.store(false, Ordering::Release);
            return;
        }

        ptr::copy_nonoverlapping(orig.as_ptr(), trampoline as *mut u8, 16);

        let jmp = make_jump_stub((target as usize) + 16);
        ptr::copy_nonoverlapping(jmp.as_ptr(), (trampoline as usize + 16) as *mut u8, 16);

        ORIG_FUNC.store(trampoline as usize, Ordering::Relaxed);

        let stub = make_jump_stub(hook_handler as usize);
        ptr::copy_nonoverlapping(stub.as_ptr(), target as *mut u8, 16);

        flush_cache(target as *mut c_void, (target as usize + 16) as *mut c_void);
        flush_cache(trampoline, (trampoline as usize + 32) as *mut c_void);
    }
}

pub use arch::install_hook;
