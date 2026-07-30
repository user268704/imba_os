#![no_std]
#![no_main]

pub mod errors;
pub mod memory;
pub mod fmt;

use core::arch::asm;
use core::panic::PanicInfo;

use crate::memory::pmm_allocator::{PmmBitmapAllocator, PAGE_SIZE};
use limine::memory_map::EntryType;
use limine::request::{MemoryMapRequest, RequestsEndMarker, RequestsStartMarker};
use limine::BaseRevision;

const COM1: u16 = 0x3F8;

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static REQUESTS_START_MARKER: RequestsStartMarker =
    RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static REQUESTS_END_MARKER: RequestsEndMarker =
    RequestsEndMarker::new();

#[used]
#[unsafe(link_section = ".requests")]
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    fmt::fmt::serial_init();

    if !BASE_REVISION.is_supported() {
        fmt::fmt::serial_write_str(
            "RustOS: unsupported Limine protocol revision\n"
        );

        halt();
    }

    serial_print!("Initializing... ");

    let (bitmap, total_frames) = init_memory();

    serial_print!("Memory initialization successful");

    let mut pmm_allocator = PmmBitmapAllocator::new(bitmap, total_frames);

    test_pmm(&mut pmm_allocator);

    halt();
}

pub fn test_pmm(pmm: &mut PmmBitmapAllocator) {

    let page1 = pmm.allocate();

    let page2 = pmm.allocate();

    let page3 = pmm.allocate();


    serial_println!(
        "Page1: {:?}",
        page1
    );

    serial_println!(
        "Page2: {:?}",
        page2
    );

    serial_println!(
        "Page3: {:?}",
        page3
    );
}

unsafe extern "C" {
    static _kernel_start: u8;
    static _kernel_end: u8;
}

fn kernel_end() -> usize {
    unsafe {
        &_kernel_end as *const u8 as usize
    }
}

fn kernel_start() -> usize {
    unsafe {
        &_kernel_start as *const u8 as usize
    }
}

fn init_memory() -> (&'static mut [u8], usize) {

    let memory = MEMORY_MAP_REQUEST
        .get_response()
        .unwrap();

    let mut max_address = 0usize;

    serial_print!("start mark frames");

    for item in memory.entries() {
        if item.entry_type == EntryType::USABLE {
            let end = (item.base + item.length) as usize;

            if end > max_address {
                max_address = end;
            }
        }
    }

    let total_frames = (max_address + PAGE_SIZE - 1) / PAGE_SIZE;
    let bitmap_size = (total_frames + 7) / 8;

    let bitmap_start = align_up(
        kernel_end(),
        PAGE_SIZE
    );

    let bitmap_end = align_up(bitmap_start + bitmap_size, PAGE_SIZE);

    let bitmap =
        unsafe {
            core::slice::from_raw_parts_mut(
                bitmap_start as *mut u8,
                bitmap_size
            )

        };

    for byte in bitmap.iter_mut() {
        *byte = 0xFF;
    }

    for item in memory.entries() {
        if item.entry_type == EntryType::USABLE {

            serial_print!("mark frame");

            let start = item.base as usize / PAGE_SIZE;
            let end =
                ((item.base + item.length) as usize + PAGE_SIZE - 1)
                    / PAGE_SIZE;


            for frame in start..end {
                clear_bit(bitmap, frame);
            }
        }
    }

    serial_print!("mark frames end");

    mark_used(bitmap, kernel_start(), kernel_end());
    mark_used(bitmap, bitmap_start, bitmap_end);

    (bitmap, total_frames)
}

fn mark_used(bitmap: &mut [u8], start: usize, end: usize) {

    let start = start / PAGE_SIZE;
    let end = end / PAGE_SIZE;

    for bit in start..=end {
        set_bit(bitmap, bit);
    }

}

fn set_bit(bitmap: &mut [u8], frame: usize) {

    let byte = frame / 8;
    let bit = frame % 8;

    bitmap[byte] |= 1 << bit;
}

fn is_used(bitmap: &[u8], frame: usize) -> bool {

    let byte = frame / 8;
    let bit = frame % 8;

    bitmap[byte] & (1 << bit) != 0
}

fn clear_bit(bitmap: &mut [u8], frame: usize) {
    let byte = frame / 8;
    let bit = frame % 8;

    bitmap[byte] &= !(1 << bit);
}


fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}


/*
 * Обработчик Rust panic.
 */
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {

    fmt::fmt::serial_init();

    serial_print!("panic");

    halt();
}

/*
 * Останавливаем процессор.
 *
 * cli запрещает маскируемые прерывания.
 * hlt переводит CPU в состояние ожидания.
 */
fn halt() -> ! {
    loop {
        unsafe {
            asm!(
                "cli",
                "hlt",
                options(nomem, nostack),
            );
        }
    }
}
