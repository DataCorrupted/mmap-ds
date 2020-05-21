use nix::{
    fcntl::OFlag,
    sys::mman::{mmap, munmap, MapFlags, ProtFlags},
};
use std::marker::{PhantomData, Sized};
use std::{
    fs::{File, OpenOptions},
    mem,
    os::unix::io::AsRawFd,
    path::PathBuf,
    ptr::{self, null_mut},
};

type FileItemIdx = usize;
type ListItemIdx = usize;

struct LinkedListMetaData {
    size: FileItemIdx,
    file_len: FileItemIdx,
    list_len: ListItemIdx,
    head: Option<ListItemIdx>,
}
struct LinkedList<T: Sized> {
    ptr: *mut u8,
    metadata: *mut LinkedListMetaData,
    _phantom: PhantomData<T>,
}

struct Item<T: Sized> {
    item: T,
    next: Option<usize>,
}

impl<T: Sized> Item<T> {
    pub fn new(item: T) -> Item<T> {
        Item {
            item: item,
            next: None,
        }
    }
}

impl<T: Sized> LinkedList<T> {
    pub fn empty_with_size(
        path: PathBuf,
        size: FileItemIdx,
    ) -> Result<LinkedList<T>, failure::Error> {
        let metadata = LinkedListMetaData {
            size: size,
            file_len: 0,
            list_len: 0,
            head: None,
        };
        let file = OpenOptions::new().write(true).read(true).open(path)?;
        let size = mem::size_of::<LinkedListMetaData>() + size * mem::size_of::<T>();
        let ptr = unsafe {
            mmap(
                ptr::null_mut(),
                size as usize,
                ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
                MapFlags::MAP_PRIVATE,
                file.as_raw_fd(),
                0,
            )?
        } as *mut u8;
        let metadata_ptr = ptr as *mut LinkedListMetaData;
        unsafe {
            *metadata_ptr = metadata;
        }
        Ok(Self {
            ptr: unsafe { ptr.add(mem::size_of::<LinkedListMetaData>()) },
            metadata: metadata_ptr,
            _phantom: PhantomData,
        })
    }

    pub fn from_file(path: PathBuf) -> Result<LinkedList<T>, failure::Error> {
        unimplemented!()
    }
    pub fn resize(&self, size: FileItemIdx) {
        unimplemented!()
    }
    pub fn insert_after(&self, item: T, pos: ListItemIdx) {
        unimplemented!();
    }
    pub fn remove(pos: ListItemIdx) {
        unimplemented!();
    }
    pub fn nth_file(idx: FileItemIdx) -> &'static Item<T> {
        unimplemented!()
    }
    pub fn nth_mut_file(idx: FileItemIdx) -> &'static mut Item<T> {
        unimplemented!()
    }
    pub fn nth_list(idx: ListItemIdx) -> &'static Item<T> {
        unimplemented!()
    }
    pub fn nth_mut_list(idx: ListItemIdx) -> &'static mut Item<T> {
        unimplemented!()
    }
}

impl<T: Sized> std::ops::Index<ListItemIdx> for LinkedList<T> {
    type Output = Item<T>;
    fn index(&self, index: ListItemIdx) -> &Self::Output {
        unimplemented!()
    }
}

impl<T: Sized> std::ops::IndexMut<ListItemIdx> for LinkedList<T> {
    fn index_mut(&mut self, index: ListItemIdx) -> &mut Self::Output {
        unimplemented!()
    }
}
