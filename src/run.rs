extern {
    fn tail_call_asm(p: *const u8);
    fn ret_call_asm(p: *const u8);
}

pub fn tail_call(p: *const u8) {
    unsafe {
        tail_call_asm(p)
    }
}

pub fn ret_call(p: *const u8) {
    unsafe {
        ret_call_asm(p)
    }
}