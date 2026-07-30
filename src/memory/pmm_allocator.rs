use crate::errors::KernelError;
use crate::{is_used, set_bit};

pub const PAGE_SIZE: usize = 4096;

pub struct PmmBitmapAllocator {
    bitmap: &'static mut [u8],
    total_frames: usize,
    used_frames: usize,
}


impl PmmBitmapAllocator {


    pub fn new(bitmap: &'static mut [u8], total_frames: usize) -> Self {
        Self {
            bitmap,
            total_frames,
            used_frames: 0,
        }
    }

    pub fn allocate(&mut self) -> Result<Page, KernelError> {

        let total_frames = self.bitmap.len() * 8;

        for item in 0..total_frames {
            if !is_used(self.bitmap, item) {
                set_bit(self.bitmap, item);
                self.used_frames += 1;

                return Ok(Page::new(item));
            }
        }

        Err(KernelError::AllocateMemoryError)
    }
/*
    pub fn free(&self, page: Page) -> Result<(), KernelError> {

    }

    pub fn total_memory(&self) {

    }

    pub fn total_frames(&self) -> usize {

    }
*/
}

#[derive(Debug)]
pub struct Page {
    pub number: usize,
}

impl Page {

    pub fn new(number: usize) -> Page {
        Self {
            number
        }
    }

    pub fn address(&self) -> usize {
        self.number * PAGE_SIZE
    }
}