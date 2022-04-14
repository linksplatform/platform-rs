use std::cmp::max;

use std::fs::{File};
use std::io;
use std::mem::ManuallyDrop;

use std::ptr::NonNull;

use memmap2::{MmapMut, MmapOptions};

use crate::{Mem, ResizeableBase, ResizeableMem};

pub struct FileMappedMem {
    base: ResizeableBase,
    pub(in crate) file: File,
    mapping: ManuallyDrop<MmapMut>, // TODO: `MaybeUninit`
}

impl FileMappedMem {
    pub fn from_file(file: File) -> io::Result<Self> {
        let capacity = ResizeableBase::MINIMUM_CAPACITY;
        let mapping = unsafe { MmapOptions::new().map_mut(&file)? };

        let len = file.metadata()?.len() as usize;
        let to_reserve = max(len, capacity);

        let mut new = Self {
            base: ResizeableBase {
                used: 0,
                reserved: 0,
                ptr: NonNull::slice_from_raw_parts(NonNull::dangling(), 0),
            },
            mapping: ManuallyDrop::new(mapping),
            file,
        };

        new.reserve_mem(to_reserve).map(|_| new)
    }

    pub fn new(file: File) -> std::io::Result<Self> {
        Self::from_file(file)
    }

    unsafe fn map(&mut self, capacity: usize) -> std::io::Result<NonNull<[u8]>> {
        let mapping = MmapOptions::new().len(capacity).map_mut(&self.file)?;
        self.mapping = ManuallyDrop::new(mapping);
        Ok(NonNull::from(self.mapping.as_mut()))
    }

    unsafe fn unmap(&mut self) {
        // TODO: WARNING! self.mapping must be initialized
        ManuallyDrop::drop(&mut self.mapping)
    }
}

impl Mem for FileMappedMem {
    fn get_ptr(&self) -> NonNull<[u8]> {
        self.base.get_ptr()
    }

    fn set_ptr(&mut self, ptr: NonNull<[u8]>) {
        self.base.set_ptr(ptr)
    }
}

impl ResizeableMem for FileMappedMem {
    fn use_mem(&mut self, capacity: usize) -> std::io::Result<usize> {
        self.base.use_mem(capacity)
    }

    fn used_mem(&self) -> usize {
        self.base.used_mem()
    }

    fn reserve_mem(&mut self, capacity: usize) -> std::io::Result<usize> {
        let reserved = self.base.reserve_mem(capacity)?;

        unsafe {
            self.unmap();
        }
        // TODO: file.set_len
        //  self.file.set_len(capacity as u64)?;

        // TODO: hack for parody on `self.file.set_len(capacity.max(`file len`))`
        // self.file.seek(SeekFrom::Start(capacity as u64))?;
        // self.file.seek(SeekFrom::Start(0))?;

        // TODO: current impl
        let file_len = self.file.metadata()?.len();
        self.file.set_len(file_len.max(capacity as u64))?;

        let ptr = unsafe { self.map(capacity) }?;
        self.set_ptr(ptr);

        Ok(reserved)
    }

    fn reserved_mem(&self) -> usize {
        self.base.reserved_mem()
    }
}

impl Drop for FileMappedMem {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.mapping);
        }
        let used = self.used_mem();
        // TODO: maybe remove `unwrap()` and ignore error
        self.file.set_len(used as u64);
    }
}
