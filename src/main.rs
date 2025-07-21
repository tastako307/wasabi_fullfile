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
    
    loop{}
}

#[repr(C)]
struct EfiBootServicesTable{
    _reserved0:[u64,40],
    pub bootservices: &'static EfiBootServicesTable,
}
const _: () = assert!(offset_of!(EfiBootServicesTable, locale_protocol)==320);
// use core::panic::PanicInfo;
#[repr(C)]
struct EfiSystemTable{
    _reserved0: [u64; 12],
    pub boot_services: &'static EfiBootServicesTable,
}
const _: () = assert!(offset_of!(EfiSystemTable,boot_services)== 96) ;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
const EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID: EfiGuid = EfiGuid {
    data0: 0x9042a9de,
    data1: 0x23dc,
    data2: 0x4a38,
    data3: [0x96, 0xfb, 0x7a, 0xde, 0xd0,0x80,0x51, 0x6a], 
};