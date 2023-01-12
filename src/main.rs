#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use x86_64::VirtAddr;

use core::panic::PanicInfo;

use titan::{print, println};
use titan::task::{Task, executor::Executor, keyboard};

use bootloader::{BootInfo, entry_point};

extern crate alloc;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;

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
    use titan::allocator;
    use titan::memory::{self, BootInfoFrameAllocator};
    
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello World!");
    titan::init();    
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    
    // let page = Page::containing_address(VirtAddr::new(0));
    // memory::create_example_mapping(page,
    //                               &mut mapper,
    //                               &mut frame_allocator);

    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    //unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    // new
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    
    let x = Box::new(34);
    println!("Value on heap: {:p}", x);

    // create a dynamically sized vector
    //let mut vec = Vec::new();
    //for i in 0..500 {
    //    vec.push(i);
    //}
    //println!("vec at {:p}", vec.as_slice());

     //create a reference counted vector -> will be freed when count reaches 0
    //let reference_counted = Rc::new(vec![1, 2, 3]);
    //let cloned_reference = reference_counted.clone();
    //println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    //core::mem::drop(reference_counted);
    //println!("reference count is {} now", Rc::strong_count(&cloned_reference));
    
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    #[cfg(test)]     
    test_main(); 
    
    println!("No crash from Heap Allocation");
    titan::hlt_loop();

}

async fn async_num() -> u32 {
    42
}

async fn example_task() {
    let num = async_num().await;
    println!("async number: {}", num);
}

/*
#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    
    use titan::memory;
    use titan::memory::BootInfoFrameAllocator;
    use x86_64::{structures::paging::Page};

    println!("Hello from a raw Print Macro!");
    
    titan::init(); 

    //x86_64::instructions::interrupts::int3(); 
    //unsafe {
    //    *(0xdeadbeef as *mut u64) = 69; //lil hacker ref deadbeef;
    //};
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };


    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};
    
    #[cfg(test)]
    test_main();

    println!("[MAIN] No Crash after interrupt handler");
    titan::hlt_loop();
} 
*/

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}


