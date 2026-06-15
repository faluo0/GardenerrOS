mod heap_allocator;
mod page_table;
mod address;
mod frame_allocator;
mod memory_set;

pub use address::{PhysAddr, VirtAddr, PhysPageNum, VirtPageNum, VPNRange, StepByOne};
pub use frame_allocator::{FrameTracker, frame_alloc};
pub use page_table::{
    PageTable, PageTableEntry, PTEFlags,
    translated_byte_buffer, translated_str, translated_refmut,
};
pub use memory_set::{MemorySet, KERNEL_SPACE, MapPermission, MapArea, MapType};
pub use memory_set::remap_test;

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}
