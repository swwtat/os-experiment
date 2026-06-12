mod heap_allocator;
mod page_table;
mod memory_set;
pub use page_table::translated_byte_buffer;

use page_table::{PTEFlags};
pub use page_table::{PageTableEntry};

mod address;
mod frame_allocator;

use address::{VPNRange, StepByOne};
pub use address::{PhysAddr, VirtAddr, PhysPageNum, VirtPageNum};
pub use frame_allocator::{FrameTracker, frame_alloc};
pub use memory_set::{MemorySet, KERNEL_SPACE, MapPermission, remap_test};

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}

