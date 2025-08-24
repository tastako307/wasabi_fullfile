#![no_std]
#![no_main]
#![feature(offset_of)]

use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use core::writeln;
use wasabi::graphics::draw_font_fg;
use wasabi::graphics::draw_test_pattern;
use wasabi::graphics::fill_rect;
use wasabi::graphics::Bitmap;
use wasabi::result::Result;
use wasabi::uefi::exit_from_efi_boot_services;
use wasabi::uefi::init_vram;
use wasabi::uefi::EfiHandle;
use wasabi::uefi::EfiMemoryType;
use wasabi::uefi::EfiSystemTable;
use wasabi::uefi::MemoryMapHolder;
use wasabi::uefi::VramTextWriter;

// Rustコンパイラくんに明示的にUseせよと怒られたので仕方なく
// use wasabi::uefi::EfiBootServicesTable;
// use wasabi::uefi::EfiGuid;
// use wasabi::uefi::EfiStatus;

pub fn hlt() {
    unsafe { asm!("hlt") }
}

#[no_mangle]
fn efi_main(image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    // Initialize the console output
    let mut vram = init_vram(efi_system_table).expect("init_vram failed");
    let vw = vram.width();
    let vh = vram.height();
    fill_rect(&mut vram, 0x000000, 0,0, vw, vh).expect("fill rect failed");
    draw_test_pattern(&mut vram);
    let mut w= VramTextWriter::new(&mut vram);
    for i in 0..4{
        writeln!(w, "i = {i}").unwrap();
    }
    let mut memory_map = MemoryMapHolder::new();
    let status = efi_system_table
        .boot_services()
        .get_memory_map(&mut memory_map);
    writeln!(w, "{status:?}").unwrap();
    let mut total_memory_pages = 0; 
    for e in memory_map.iter(){
        if e.memory_type() != EfiMemoryType:: CONVENTIONAL_MEMORY {
            continue;
        }
        total_memory_pages += e.number_of_pages();
        writeln!(w, "{e:?}").unwrap();
    }
    let total_memory_bytes = total_memory_pages * 4096 / 1024/ 1024;
    writeln!(w, "Total Memory: {total_memory_bytes} MiB").unwrap();
    exit_from_efi_boot_services(
        image_handle,
        efi_system_table,
        &mut memory_map,
    );
    writeln!(w,"Hello, Non-UEFI world!").unwrap();
    loop {
        hlt()
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        hlt()
    }
}


#[derive(Clone, Copy)]
struct VramBufferInfo{
    buf: *mut u8,
    width: i64,
    height: i64,
    pixels_per_line: i64,
}

impl Bitmap for VramBufferInfo {
    fn bytes_per_pixel(&self) -> i64 {
        4
    }
    fn pixels_per_line(&self) -> i64 {
        self.pixels_per_line
    }
    fn width(&self) -> i64{
        self.width
    }
    fn height(&self) -> i64 {
        self.height
    }
    fn buf_mut(&mut self) -> *mut u8 {
        self.buf
    }
}


