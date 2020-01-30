// no-prefer-dynamic

#![crate_type = "rlib"]
#![no_std]
#![feature(lang_items)]

use core::panic::PanicInfo;

#[lang = "panic_impl"]
fn panic_impl(info: &PanicInfo) -> ! {
    loop {}
}
#[lang = "eh_personality"]
fn eh_personality() {}

unsafe extern "C" fn eh_unwind_resume(_: *mut u8) -> ! {
    loop {}
}
#[lang = "eh_unwind_resume"]
static _RESUME: unsafe extern "C" fn(*mut u8) -> ! = eh_unwind_resume;
