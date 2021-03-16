
#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
///fn main() {
    
///}

#[macro_use]
mod console;
mod panic;
mod sbi;


global_asm!(include_str!("entry.asm"));






/// 覆盖 crt0 中的 _start 函数
/// 我们暂时将它的实现为一个死循环
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Hello DuShiyi!");
    panic!("end of rust_main")
}
