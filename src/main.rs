#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::intrinsics;
use core::panic::PanicInfo;

use titan::{print, println};
use bootloader::{BootInfo, entry_point};

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    print!("{}", _info);
    titan::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    titan::test_panic_handler(_info)
}

entry_point!(kernel_main);

fn kernel_main(boot_info:&'static BootInfo) -> ! {
       titan::init();    
       #[cfg(test)]     
       test_main(); 
        titan::hlt_loop();
}

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    
    println!("Hello from a raw Print Macro!");
    
    titan::init(); 

    //x86_64::instructions::interrupts::int3(); 
    //unsafe {
    //    *(0xdeadbeef as *mut u64) = 69; //lil hacker ref deadbeef;
    //};

    #[cfg(test)]
    test_main();

    println!("[MAIN] No Crash after interrupt handler");
    titan::hlt_loop();
} 


#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}


