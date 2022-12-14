#![feature(abi_x86_interrupt)]
use x86_64::structures::idt::InterruptDescriptorTable;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDesctiptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}


pub fn init_idt() {
    IDT.load();
}


extern "x86-interrupt" fn breakpoint_handler(
        stack_frame: InterruptStackFrame
    ) {
    println!("Exception: BREAKPOINT\n{:#?}", stack_frame);
}


