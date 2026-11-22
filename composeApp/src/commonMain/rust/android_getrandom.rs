//! Android 6 backport: Bionic only has `getrandom()` from Android 9 (API 28), and the
//! SHIELD Portable's kernel (3.10) predates the getrandom syscall (Linux 3.17).
//! Defining the symbol here makes the linker resolve every `getrandom` reference
//! (from the getrandom crate, std, ...) inside libcompose_app.so instead of importing
//! it from libc. It tries the syscall first and falls back to /dev/urandom.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

unsafe extern "C" {
    fn syscall(number: c_long, ...) -> c_long;
    fn __errno() -> *mut c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
}

#[cfg(target_arch = "arm")]
const SYS_GETRANDOM: c_long = 384;
#[cfg(target_arch = "aarch64")]
const SYS_GETRANDOM: c_long = 278;
#[cfg(target_arch = "x86")]
const SYS_GETRANDOM: c_long = 355;
#[cfg(target_arch = "x86_64")]
const SYS_GETRANDOM: c_long = 318;

const EINTR: c_int = 4;
const EIO: c_int = 5;
const ENOSYS: c_int = 38;
const O_RDONLY: c_int = 0;
const O_CLOEXEC: c_int = 0o2000000;

fn errno() -> c_int {
    unsafe { *__errno() }
}

fn set_errno(e: c_int) {
    unsafe { *__errno() = e }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn getrandom(buf: *mut c_void, buflen: usize, flags: c_uint) -> isize {
    let r = unsafe { syscall(SYS_GETRANDOM, buf, buflen, flags) };
    if r >= 0 || errno() != ENOSYS {
        return r as isize;
    }

    // Kernel without the syscall: read /dev/urandom instead.
    let fd = loop {
        let fd = unsafe { open(c"/dev/urandom".as_ptr(), O_RDONLY | O_CLOEXEC) };
        if fd >= 0 || errno() != EINTR {
            break fd;
        }
    };
    if fd < 0 {
        return -1;
    }
    let mut done = 0usize;
    while done < buflen {
        let n = unsafe { read(fd, buf.cast::<u8>().add(done).cast(), buflen - done) };
        if n < 0 {
            let e = errno();
            if e == EINTR {
                continue;
            }
            unsafe { close(fd) };
            set_errno(e);
            return -1;
        }
        if n == 0 {
            unsafe { close(fd) };
            set_errno(EIO);
            return -1;
        }
        done += n as usize;
    }
    unsafe { close(fd) };
    done as isize
}
