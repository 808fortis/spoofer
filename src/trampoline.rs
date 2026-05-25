use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering, compiler_fence};
use libc::{self, c_char, c_void};

#[inline(always)]
pub fn jump_stub(target: usize) -> [u8; 16] {
    unsafe {
        let mut stub: [u8; 16] = std::mem::zeroed();
        ptr::copy_nonoverlapping(
            &0x58000050u32.to_le() as *const u32 as *const u8,
            stub.as_mut_ptr(),
            4,
        );
        ptr::copy_nonoverlapping(
            &0xD61F0200u32.to_le() as *const u32 as *const u8,
            stub.as_mut_ptr().add(4),
            4,
        );
        ptr::copy_nonoverlapping(
            &target as *const usize as *const u8,
            stub.as_mut_ptr().add(8),
            8,
        );
        stub
    }
}

#[inline(always)]
pub fn icache_sync(start: *mut c_void, end: *mut c_void) {
    extern "C" {
        fn __clear_cache(s: *mut c_void, e: *mut c_void);
    }
    unsafe { __clear_cache(start, end) };
}

pub unsafe fn install(
    sym: &[u8],
    handler: usize,
    orig_out: &AtomicUsize,
) -> bool {
    let target = libc::dlsym(libc::RTLD_DEFAULT, sym.as_ptr() as *const c_char);
    if target.is_null() {
        return false;
    }

    let ps = libc::sysconf(libc::_SC_PAGESIZE) as usize;
    let ps_addr = (target as usize) & !(ps - 1);
    let pe_addr = ((target as usize) + 16 + ps - 1) & !(ps - 1);

    if libc::mprotect(
        ps_addr as *mut c_void,
        pe_addr - ps_addr,
        libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
    ) != 0 {
        return false;
    }

    let mut orig = [0u8; 16];
    ptr::copy_nonoverlapping(target as *const u8, orig.as_mut_ptr(), 16);

    let tramp = libc::mmap(
        ptr::null_mut(),
        32,
        libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
        libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
        -1,
        0,
    );
    if tramp == libc::MAP_FAILED {
        return false;
    }

    ptr::copy_nonoverlapping(orig.as_ptr(), tramp as *mut u8, 16);

    let jmp = jump_stub((target as usize) + 16);
    ptr::copy_nonoverlapping(jmp.as_ptr(), (tramp as usize + 16) as *mut u8, 16);

    orig_out.store(tramp as usize, Ordering::Release);
    compiler_fence(Ordering::SeqCst);

    let stub = jump_stub(handler);
    ptr::copy_nonoverlapping(stub.as_ptr(), target as *mut u8, 16);

    compiler_fence(Ordering::SeqCst);
    icache_sync(target as *mut c_void, (target as usize + 16) as *mut c_void);
    icache_sync(tramp, (tramp as usize + 32) as *mut c_void);

    true
}
