#![no_std]
#![no_main]
mod writer;
use writer::FrameBufferWriter;
use writer::custom_macro;

use bootloader_api::config::Mapping; //this is how we namespace in rust 

/* In Rust, namespacing is a way to organize and group related items like functions, structs, 
enums, and constants so that you can avoid name conflicts and make your 
code easier to understand and maintain.
Think of namespacing as putting things into folders. 
Just like you organize files into folders on your computer to avoid clutter, 
Rust uses modules (like mod) to group related items together. 
This way, even if two items have the same name, 
they can live in different "folders" (modules) without causing confusion. */
use x86_64::instructions::hlt;

//Use the entry_point macro to register the entry point function: 
//bootloader_api::entry_point!(kernel_main)
//optionally pass a custom config

pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
let mut config = bootloader_api::BootloaderConfig::new_default(); 
config.mappings.physical_memory = Some(Mapping::Dynamic); 
config.kernel_stack_size = 100 * 1024; // 100 KiB
config
};


bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);



fn my_entry_point(_boot_info: &'static mut bootloader_api::BootInfo) -> ! 
{
    let frame_buffer_info = _boot_info.framebuffer.as_mut().unwrap().info();
    let buffer = _boot_info.framebuffer.as_mut().unwrap(). buffer_mut();
    let mut frame_buffer_writer =FrameBufferWriter::new(buffer, frame_buffer_info); 
    //use core::fmt::Write;//below requires this
    
    custom_macro(
        &mut frame_buffer_writer,
        "Hello, world!! This is a test\n\tIndented Text\n\\cHot pink Text\n\\rBack to Yellow Text.",);

    loop {
        hlt(); //stop x86_64 from being unnecessarily busy while looping
    }
}


#[panic_handler]
fn  panic(_info: &core::panic::PanicInfo) -> ! {
    loop { hlt(); //avoids wasting of cpu cycles
    } }

