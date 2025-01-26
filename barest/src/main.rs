#![no_std]
#![no_main]





#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let framebuffer: *mut u8 = 0xb8000 as *mut u8;
    /*unsafe {
    framebuffer.offset(1).write_volatile(0x30); }*/
    for (i, &byte) in HELLO.iter().enumerate() { unsafe {
    *framebuffer.offset(i as isize * 2) = byte;
    *framebuffer.offset(i as isize * 2 + 1) = 0xb; }
    }
    loop{}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> !{
    loop{}
}

static HELLO: &[u8] = b"Hello World. I love you!";































/*const vga_buffer_addy: usize = 0xb8000; 
/*memory address of vga text buffer where text is written so its
displayed on the screen, this is hardware specific
*/
const screen_width: usize = 95;
const screen_height: usize = 30;
//just basic screen dimensions for defining screen limits

pub struct FrameBufferWriter 
{
    cursor_x: usize,
    cursor_y: usize,
    colour: u8, //i like pink, hence, the changing colour 
}
/*this struct tracks text positioning on the screen (x and y axis),
as well as the colour of the text, initially hardcoded cursor logic all over
the code, but this makes it more modular*/

pub fn new(colour: u8) -> Self 
{
Self {
    cursor_x: 0,
    cursor_y: 0, 
    colour,
}
}
//construtor  that creates a FrameBufferWriter with a curor at the top 
//left of the screen

pub fn write(&mut self, byte:u8) //this helps handle edge cases and directly interacts with memory hardware
{
    if self.cursor_x >= screen_width 
    {
        self.new_line(); //go to new line when at end of screen
    }

    if self.cursor_y >=screen_height
    {
        self.scroll(); //scroll sreen when at the bottom
    }

    let offset = (self.cursor_y * screen_width + self.cursor_x) *2;
    //calculates memeory location for character and colour in vga buffer
    unsafe  //this directly writes the character (byte) and its colour to vga memory
    {
        let vga_buffer = vga_buffer_addy as *mut u8;
        *vga_buffer.add(offset) = byte;
        *vga_buffer.add(offset + 1) = self.colour;
    }

    self.cursor_x = self.cursor_x + 1;
}


pub fn new_line(&mut self)
{
    self.cursor_x = 0;
    self.cursor_y +=1;
}
//this is basically my "return" or "enter" button


pub fn scroll(&mut self)
{
    let vga_buffer = vga_buffer_addy as *mut u8;

    for y in 1..screen_height {
        for x in 0..screen_width {
            let from = (y * screen_width + x) * 2;
            let to = ((y - 1) * screen_width + x) * 2;

            unsafe {
                *vga_buffer.add(to) = *vga_buffer.add(from);
                *vga_buffer.add(to + 1) = *vga_buffer.add(from + 1);
            }
        }
    }

    // Clear the last row
    for x in 0..screen_width {
        let offset = ((screen_height - 1) * screen_width + x) * 2;
        unsafe {
            *vga_buffer.add(offset) = b' ';
            *vga_buffer.add(offset + 1) = self.color;
        }
    }

    self.cursor_y = screen_height - 1;
}

pub fn write_string(&mut self, s: &str) {
    for byte in s.bytes() {
        match byte {
            b'\n' => self.new_line(),         // Handle newline
            b'\t' => {
                self.cursor_x += 4;          // Tab (4 spaces)
                if self.cursor_x >= SCREEN_WIDTH {
                    self.new_line();
                }
            }
            _ => self.write_byte(byte),      // Regular characters
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print_to_framebuffer(format_args!($($arg)*));
    };
}


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print_to_framebuffer(format_args!($($arg)*));
    };
}


lazy_static::lazy_static! {
    pub static ref WRITER: Mutex<FrameBufferWriter> = Mutex::new(FrameBufferWriter::new(0xb));
}


















pub extern "C" fn _start() -> ! {
    let framebuffer: *mut u8 = 0xb8000 as *mut u8;
    /*unsafe {
    framebuffer.offset(1).write_volatile(0x30); }*/
    for (i, &byte) in HELLO.iter().enumerate() { unsafe {
    *framebuffer.offset(i as isize * 2) = byte;
    *framebuffer.offset(i as isize * 2 + 1) = 0xb; }
    }
    loop{}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> !{
    loop{}
}

static HELLO: &[u8] = b"Hello World. I love you!";

*/

