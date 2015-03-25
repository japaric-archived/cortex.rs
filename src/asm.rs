/// Breakpoint
pub fn bkpt() {
    unsafe {
        asm!("bkpt" :::: "volatile");
    }
}

/// NOP
pub fn nop() {
    unsafe {
        asm!("nop" :::: "volatile");
    }
}

/// Wait for interrput
pub fn wfi() {
    unsafe {
        asm!("wfi" :::: "volatile");
    }
}
