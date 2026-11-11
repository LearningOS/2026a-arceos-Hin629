#![no_std]

use allocator::{AllocError, BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const SIZE: usize> {
    start: usize,
    end: usize,
    b_pos: usize,
    p_pos: usize,
    live_byte_allocs: usize,
}

impl<const SIZE: usize> EarlyAllocator<SIZE> {
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            b_pos: 0,
            p_pos: 0,
            live_byte_allocs: 0,
        }
    }
}

impl<const SIZE: usize> BaseAllocator for EarlyAllocator<SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.b_pos = start;
        self.p_pos = self.end;
        self.live_byte_allocs = 0;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> allocator::AllocResult {
        if size == 0 || self.start == self.end {
            return Err(AllocError::InvalidParam);
        }
        let end = start.checked_add(size).ok_or(AllocError::InvalidParam)?;

        if start < self.end && end > self.start {
            return Err(AllocError::MemoryOverlap);
        }

        if start == self.end && self.p_pos == self.end {
            self.end = end;
            self.p_pos = end;
        } else if end == self.start && self.b_pos == self.start && self.live_byte_allocs == 0 {
            self.start = start;
            self.b_pos = start;
        } else {
            return Err(AllocError::InvalidParam);
        }
        Ok(())
    }
}

impl<const SIZE: usize> ByteAllocator for EarlyAllocator<SIZE> {
    fn alloc(
        &mut self,
        layout: core::alloc::Layout,
    ) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        if layout.size() == 0 {
            return Err(AllocError::InvalidParam);
        }

        let align_mask = layout.align() - 1;
        let pos = self
            .b_pos
            .max(1) // 避免从地址 0 （空指针）开始分配
            .checked_add(align_mask)
            .ok_or(AllocError::NoMemory)?
            & !align_mask;
        let end = pos.checked_add(layout.size()).ok_or(AllocError::NoMemory)?;
        if end > self.p_pos {
            return Err(AllocError::NoMemory);
        }

        let ptr = core::ptr::NonNull::new(pos as *mut u8).ok_or(AllocError::NoMemory)?;
        self.b_pos = end;
        self.live_byte_allocs += 1;
        Ok(ptr)
    }

    fn dealloc(&mut self, _pos: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {
        self.live_byte_allocs -= 1;
        if self.live_byte_allocs == 0 {
            self.b_pos = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
        self.b_pos - self.start
    }

    fn available_bytes(&self) -> usize {
        self.p_pos - self.b_pos
    }
}

impl<const SIZE: usize> PageAllocator for EarlyAllocator<SIZE> {
    const PAGE_SIZE: usize = SIZE;

    fn alloc_pages(
        &mut self,
        num_pages: usize,
        align_pow2: usize,
    ) -> allocator::AllocResult<usize> {
        if !SIZE.is_power_of_two() || num_pages == 0 || !align_pow2.is_power_of_two() {
            return Err(AllocError::InvalidParam);
        }

        let size = num_pages.checked_mul(SIZE).ok_or(AllocError::NoMemory)?;
        let align_mask = align_pow2.max(SIZE) - 1;
        let pos = self.p_pos.checked_sub(size).ok_or(AllocError::NoMemory)? & !align_mask;
        if pos < self.b_pos {
            return Err(AllocError::NoMemory);
        }

        self.p_pos = pos;
        Ok(pos)
    }

    fn dealloc_pages(&mut self, _pos: usize, _num_pages: usize) {
        // Page allocations stay reserved for the lifetime of the allocator.
    }

    fn total_pages(&self) -> usize {
        if !SIZE.is_power_of_two() {
            return 0;
        }
        (self.end / SIZE).saturating_sub(self.start.div_ceil(SIZE))
    }

    fn used_pages(&self) -> usize {
        if !SIZE.is_power_of_two() {
            return 0;
        }
        self.end / SIZE - self.p_pos / SIZE
    }

    fn available_pages(&self) -> usize {
        if !SIZE.is_power_of_two() {
            return 0;
        }
        (self.p_pos / SIZE).saturating_sub(self.b_pos.div_ceil(SIZE))
    }
}