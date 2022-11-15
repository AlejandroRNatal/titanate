#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

use core::intrinsics;
use core::panic::PanicInfo;

mod vga_buffer;

#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests...", tests.len());
    for test in tests {
        test();
    }
}

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn buffer() {

    static HELLO: &[u8] = b"Hello World!";
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
      unsafe {
        *vga_buffer.offset(i as isize * 2) = byte;
          *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
      }
    }
    
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    //let msg = b"Hello, World!";
    
    //let mut cursor = Cursor {
      //  position: 0,
        //foreground: Color::BrightCyan,
        //background: Color::Black,
    //};
    
    //cursor.print(msg);
    println!("Hello from a raw Print Macro!");

    #[cfg(test)]
    test_main();


    loop {}
} 

