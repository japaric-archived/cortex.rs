use core::fmt::Arguments;

#[lang="stack_exhausted"]
extern fn stack_exhausted() {
    ::abort()
}

#[lang="eh_personality"]
extern fn eh_personality() {
    ::abort()
}

#[lang="panic_fmt"]
pub fn panic_fmt(_fmt: &Arguments, _file_line: &(&'static str, usize)) -> ! {
    ::abort()
}
