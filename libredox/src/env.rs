use core::mem::size_of;
use core::ptr;

use collections::string::*;
use collections::vec::Vec;

use syscall::call::*;

static mut _args: *mut Vec<String> = 0 as *mut Vec<String>;

/// Arguments
pub fn args<'a>() -> &'a Vec<String> {
    unsafe { &*_args }
}

/// Initialize arguments
pub unsafe fn args_init(args: Vec<String>) {
    _args = sys_alloc(size_of::<Vec<String>>()) as *mut Vec<String>;
    ptr::write(_args, args);
}

/// Destroy arguments
pub unsafe fn args_destroy() {
    drop(ptr::read(_args));
    sys_unalloc(_args as usize);
    _args = 0 as *mut Vec<String>;
}
