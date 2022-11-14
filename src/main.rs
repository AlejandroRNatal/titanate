#![no_std]
#![no_main]
#![feature(core_intrinsics)]

use core::intrinsics;

use core::panic::PanicInfo;

struct VGACell {
    is_blinking: u8,
    background_color: u32,
    is_bright: u8,
    character_color: u32,
    character: u8, 
}

#[allow(unused)]
#[derive(Clone, Copy)]
#[repr(u8)]
enum Color {
    Black=0,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    Gray,
    White,
    BrightBlue,
    BrightGreen,
    BrightCyan,
    BrightRed,
    BrightMagenta,
    Yellow,
    DarkGray,
}

struct Cursor {
    position: isize,
    foreground: Color,
    background: Color,
}

impl Cursor {
    fn color(&self) -> u8 {
        
        let fg = self.foreground as u8;
        let bg = (self.background as u8) << 4;

        fg | bg
    }

    
    fn print(&mut self, text: &[u8]) {
        
        let color = self.color();
        let buffer = 0xb8000 as *mut u8;

        
        for &character in text {
            unsafe {
                buffer.offset(self.position).write_volatile(character);
                buffer.offset(self.position + 1).write_volatile(color);
            }

            self.position += 2;
        }

    }

}

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// static HELLO: &[u8] = b"Hello World!";


#[no_mangle]
pub extern "C" fn _start() -> ! {
    //let vga_buffer = 0xb8000 as *mut u8;

    //for (i, &byte) in HELLO.iter().enumerate() {
      //  unsafe {
        //    *vga_buffer.offset(i as isize * 2) = byte;
          //  *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        //}
    //}
    
    let msg = b"Hello, World!";
    
    let mut cursor = Cursor {
        position: 0,
        foreground: Color::BrightCyan,
        background: Color::Black,
    };
    
    cursor.print(msg);

    loop {}
} 

