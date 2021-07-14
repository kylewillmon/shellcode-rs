use std::ptr;
use std::ops::{Deref, DerefMut};

mod bindings {
    windows::include_bindings!();
}

use bindings::{
    Windows::Win32::System::Memory::{
        VirtualAlloc,
        MEM_COMMIT,
        PAGE_EXECUTE_READWRITE
    }
};

pub struct ExecBuf {
    ptr: *mut u8,
    len: usize,
}

impl ExecBuf {
    pub fn alloc(size: usize) -> Self {
        let buf = unsafe {
            VirtualAlloc(ptr::null_mut(), size, MEM_COMMIT, PAGE_EXECUTE_READWRITE) as *mut u8
        };

        Self {
            ptr: buf,
            len: size,
        }
    }
}

impl Deref for ExecBuf {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.ptr, self.len)
        }
    }
}

impl DerefMut for ExecBuf {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr, self.len)
        }
    }
}