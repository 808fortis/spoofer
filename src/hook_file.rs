use std::cell::Cell;
use std::ffi::CStr;
use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::atomic::Ordering;

use libc::{self, c_char, c_int, c_uint, c_void};

use crate::config::SpoofFiles;

hook_state!(c_int, c_int, *const c_char, c_int, c_uint);

static SPOOF_FILES: OnceLock<Arc<SpoofFiles>> = OnceLock::new();

thread_local! {
    static REENTRY: Cell<u32> = const { Cell::new(0) };
}

const MEMFD_NAME: &[u8] = b"spoof\0";

pub fn set_spoof_files(files: Arc<SpoofFiles>) {
    let _ = SPOOF_FILES.set(files);
}

pub fn has_spoof_files() -> bool {
    SPOOF_FILES.get().is_some()
}

unsafe fn memfd_fill(content: &str) -> c_int {
    let fd = libc::memfd_create(MEMFD_NAME.as_ptr() as *const c_char, 0);
    if fd < 0 {
        return -1;
    }
    let b = content.as_bytes();
    libc::write(fd, b.as_ptr() as *const c_void, b.len());
    libc::lseek(fd, 0, libc::SEEK_SET);
    fd
}

fn match_path<'a>(path: &'a str, files: &'a SpoofFiles) -> Option<&'a str> {
    if path == "/proc/cpuinfo" || path.ends_with("/cpuinfo") {
        return files.cpuinfo.as_deref();
    }
    if path.ends_with("/cpu/possible") {
        return files.sys_cpu_possible.as_deref();
    }
    if path.ends_with("/cpu/present") {
        return files.sys_cpu_present.as_deref();
    }
    if path.ends_with("/cpu/online") {
        return files.sys_cpu_online.as_deref();
    }
    if path.ends_with("kernel_max") {
        return files.sys_cpu_kernel_max.as_deref();
    }
    None
}

unsafe extern "C" fn handler(
    dirfd: c_int,
    pathname: *const c_char,
    flags: c_int,
    mode: c_uint,
) -> c_int {
    if REENTRY.with(|r| r.get()) > 0 {
        return orig()(dirfd, pathname, flags, mode);
    }

    REENTRY.with(|r| r.set(r.get() + 1));

    let path = match CStr::from_ptr(pathname).to_str() {
        Ok(s) => s,
        Err(_) => {
            let ret = orig()(dirfd, pathname, flags, mode);
            REENTRY.with(|r| r.set(r.get() - 1));
            return ret;
        }
    };

    let result = match SPOOF_FILES.get() {
        Some(files) => match match_path(path, files) {
            Some(content) => memfd_fill(content),
            None => orig()(dirfd, pathname, flags, mode),
        },
        None => orig()(dirfd, pathname, flags, mode),
    };

    REENTRY.with(|r| r.set(r.get() - 1));
    result
}

pub unsafe fn install_hook() {
    if INSTALLED.swap(true, Ordering::AcqRel) {
        return;
    }
    if !crate::trampoline::install(b"openat\0", handler as usize, &ORIG) {
        INSTALLED.store(false, Ordering::Release);
    }
}
