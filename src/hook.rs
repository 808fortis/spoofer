use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;

static SPOOF_PROPS: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

thread_local! {
    static REENTRY: AtomicBool = const { AtomicBool::new(false) };
}

pub fn set_spoof_props(props: HashMap<String, String>) {
    *SPOOF_PROPS.lock().unwrap() = Some(props);
}

pub fn has_spoof_props() -> bool {
    SPOOF_PROPS.lock().unwrap().is_some()
}

#[cfg(target_arch = "aarch64")]
mod arch {
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Mutex;
    use libc::{self, c_char, c_int, c_void};

    type PropGetFunc = unsafe extern "C" fn(*const c_char, *mut c_char) -> c_int;

    static ORIG_FUNC: AtomicUsize = AtomicUsize::new(0);

    fn get_orig() -> PropGetFunc {
        let ptr = ORIG_FUNC.load(Ordering::Relaxed);
        unsafe { std::mem::transmute::<usize, PropGetFunc>(ptr) }
    }

    fn call_orig(name: *const c_char, value: *mut c_char) -> c_int {
        super::REENTRY.with(|r| r.store(false, Ordering::Release));
        unsafe { get_orig()(name, value) }
    }

    unsafe extern "C" fn hook_handler(name: *const c_char, value: *mut c_char) -> c_int {
        if super::REENTRY.with(|r| r.swap(true, Ordering::AcqRel)) {
            return unsafe { get_orig()(name, value) };
        }

        let prop_name = match std::ffi::CStr::from_ptr(name).to_str() {
            Ok(s) => s.to_owned(),
            Err(_) => return call_orig(name, value),
        };

        let spoofed = super::SPOOF_PROPS.lock().unwrap().as_ref().and_then(|m| m.get(&prop_name).cloned());

        if let Some(ref val) = spoofed {
            let bytes = val.as_bytes();
            let len = bytes.len().min(91);
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), value as *mut u8, len);
            *value.add(len) = 0;
            super::REENTRY.with(|r| r.store(false, Ordering::Release));
            return len as c_int;
        }

        call_orig(name, value)
    }

    fn make_jump_stub(target: usize) -> [u8; 16] {
        let mut stub = [0u8; 16];
        stub[..4].copy_from_slice(&0x58000050u32.to_le_bytes());
        stub[4..8].copy_from_slice(&0xD61F0200u32.to_le_bytes());
        stub[8..16].copy_from_slice(&target.to_le_bytes());
        stub
    }

    fn flush_cache(start: *mut c_void, end: *mut c_void) {
        extern "C" {
            fn __clear_cache(start: *mut c_void, end: *mut c_void);
        }
        unsafe { __clear_cache(start, end) };
    }

    static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);

    pub unsafe fn install_hook() {
        if HOOK_INSTALLED.load(Ordering::Acquire) {
            return;
        }
        HOOK_INSTALLED.store(true, Ordering::Release);

        let target_name = b"__system_property_get\0";
        let target = libc::dlsym(libc::RTLD_DEFAULT, target_name.as_ptr() as *const c_char);

        if target.is_null() {
            return;
        }

        let page_size = libc::sysconf(libc::_SC_PAGESIZE) as usize;
        let page_start = (target as usize) & !(page_size - 1);
        let page_end = ((target as usize) + 16 + page_size - 1) & !(page_size - 1);

        libc::mprotect(
            page_start as *mut c_void,
            page_end - page_start,
            libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
        );

        let mut orig_bytes = [0u8; 16];
        std::ptr::copy_nonoverlapping(target as *const u8, orig_bytes.as_mut_ptr(), 16);

        let trampoline = libc::mmap(
            std::ptr::null_mut(),
            32,
            libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );

        if trampoline == libc::MAP_FAILED {
            return;
        }

        std::ptr::copy_nonoverlapping(orig_bytes.as_ptr(), trampoline as *mut u8, 16);

        let jump_back = make_jump_stub((target as usize) + 16);
        std::ptr::copy_nonoverlapping(jump_back.as_ptr(), (trampoline as usize + 16) as *mut u8, 16);

        ORIG_FUNC.store(trampoline as usize, Ordering::Relaxed);

        let hook_addr = hook_handler as usize;
        let jump_stub = make_jump_stub(hook_addr);
        std::ptr::copy_nonoverlapping(jump_stub.as_ptr(), target as *mut u8, 16);

        flush_cache(target as *mut c_void, (target as usize + 16) as *mut c_void);
        flush_cache(trampoline, (trampoline as usize + 32) as *mut c_void);
    }
}

#[cfg(target_arch = "aarch64")]
pub use arch::install_hook;

#[cfg(not(target_arch = "aarch64"))]
pub unsafe fn install_hook() {}
