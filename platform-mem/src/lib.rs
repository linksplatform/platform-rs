#![feature(nonnull_slice_from_raw_parts)]
#![feature(allocator_api)]
#![feature(default_free_fn)]
#![feature(layout_for_ptr)]
#![feature(slice_ptr_get)]
#![feature(try_blocks)]
#![feature(slice_ptr_len)]

pub use alloc_mem::AllocMem;
pub use file_mapped_mem::FileMappedMem;
pub use global_mem::GlobalMem;
pub use mem_traits::{Mem, ResizeableMem};
pub use temp_file_mem::TempFileMem;

pub(crate) use resizeable_base::ResizeableBase;

mod alloc_mem;
mod file_mapped_mem;
mod global_mem;
mod mem_traits;
mod resizeable_base;
mod temp_file_mem;
