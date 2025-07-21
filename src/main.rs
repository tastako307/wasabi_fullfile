#![no_std]
#![no_main]

#[no_mangle]
fn efi_main(_image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    // println!("Hello, world!");
    let efi_graphic_output_protocol = locale_graphic_protocol(efi_system_table).uwrap();
    let vram_add = efi_graphic_output_protocol.mode.frame_buffer_base;
    let vram_byte_size = efi_graphic_output_protocol.mode.frame_buffer_size;
    let vram = unsafe {
        slice::from_row_parts_mut(vram_addr as *mut u32, vram_byte_size / size_of::<u32>())
    };
    for e in vram{
        *e = 0xffffff
    } 
    // println!("Hello World")
    loop{}
}

#[repr(C)]
struct EfiBootServicesTable{
    _reserved0:[u64; 40],
    locale_protocol: extern "win64" fn(
        protocol: *const EfiVoid,
        registration: *const EfiVoid,
        interface: *mut *mut EfiVoid,
    ) -> EfiStatus,
    // pub bootservices: &'static EfiBootServicesTable,
}
const _: () = assert!(offset_of!(EfiBootServicesTable, locale_protocol)==320);
// use core::panic::PanicInfo;
#[repr(C)]
struct EfiSystemTable{
    _reserved0: [u64; 12],
    pub boot_services: &'static EfiBootServicesTable,
}
const _: () = assert!(offset_of!(EfiSystemTable,boot_services)== 96) ;

// #[repr(C)]
// struct EfiBootServicesTable {
//     _reserved0: [u64; 12],
//     pub boot_servicess: &'static EfiBootServicesTable,
// }
// const _: () = assert!(offset_of!(EfiSystemTable, boot_services))

const EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID: EfiGuid = EfiGuid {
    data0: 0x9042a9de,
    data1: 0x23dc,
    data2: 0x4a38,
    data3: [0x96, 0xfb, 0x7a, 0xde, 0xd0,0x80,0x51, 0x6a], 
};

#[repr[C]]
#[device(Clone, Copy, PartialEq, Eq, Debug)]
struct EfiGuid {
    pub data0: u32,
    pub data1: u16,
    pub data2: u16,
    pub data3: [u8; 8],
}

#[repr[C]]
#[device(Debug)]
struct EfiGraphicsOutputProtocolMode<'a>{
    reserved: [u64; 3],
    pub mode: &'a EfiGraphocsOutputProtocolMode<'a>,
}


#[repr[C]]
#[derive(Debug)]
struct EfiGraphicsOutputProtocolMode<'a> {
    pub max_mode: u32,
    pub mode:u32,
    pub info: &'a EfiGraphicsOutputprotcolPixcelInfo,
    pub size_of_info: u64,
    pub frame_buffer_base:usize,
    pub frame_buffer_size: usize,
}

struct EfiGraphicsOutputprotcolPixcelInfo {
    version: u32,
    pub horizonal_resolution: u32,
    pub vertical_resolution: u32,
    _padding0: [u32; 5],
    pub pixcels_per_line: u32, 
}

const _: () = assert!(size_of::<EfiGraphicsOutputprotcolPixcelInfo>()==36);

fn locate_grafic_protocol<'a>{
    let mut graphic_output_purotocol = null_mut::<EfiGraphicsOutputProtocol>();
    let status = (efi_system_table.boot_services.localeprotocol)(
        &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
        null_mut::<EfiVoid>(),
        &mut graphic_output_protocol as *mut *mut EfiGraphicsOutputProtocol
    );
    if status != EfiStatus::Success{
        return Err("Failed to locate graphics outputprotocol")
    }
    Ok(unsafe { &*garaphic_output_protocol })

}

use core::mem::offset_of;
use core::mem::size_of;
use core::panic::PanicInfo;
use core::ptr::null_mut;
use core::slice;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
};
