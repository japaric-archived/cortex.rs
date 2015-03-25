//! Memory initialization

/// Initializes the .data section
pub fn initialize_data() {
    extern {
        static __DATA_END__: ();
        static __DATA_START__: ();
        static __TEXT_END__: ();
    }

    let mut src = &__TEXT_END__ as *const () as *mut u32;
    let mut dst = &__DATA_START__ as *const () as *mut u32;
    let end = &__DATA_END__ as *const () as *mut u32;

    unsafe {
        while dst < end {
                *dst = *src;
                src = src.offset(1);
                dst = dst.offset(1);
        }
    }
}

/// Zeroes the .bss section
pub fn zero_bss() {
    extern {
        static __BSS_END__: ();
        static __BSS_START__: ();
    }

    let mut bss = &__BSS_START__ as *const () as *mut u32;
    let end = &__BSS_END__ as *const () as *mut u32;

    unsafe {
        while bss < end {
            *bss = 0;
            bss = bss.offset(1);
        }
    }
}

