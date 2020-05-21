#![feature(const_generics)]

use caring::Shared;
use interprocess_traits::ProcSync;
use std::{marker::Sized, path::PathBuf};

pub mod slice;
pub mod toy_struct;
pub mod linked_list;

/// Initialize a memory mapped file.
///
/// The initial size would be the same as
/// `mem::size_of::<T>` where `T` is your type.
///
/// There is no generic implementation for `new_file(path)`
/// because you can't take a size of generic type even if
/// it is Sized, read [here](https://github.com/rust-lang/rust/issues/43408)
pub trait InitMemoryMappedFile: Sized + ProcSync {
    /// Create a new file and memory map it.
    ///
    /// Use it if you are parent as it would create a new file and write
    /// a bunch of zeros to it.
    fn new_file(path: PathBuf) -> Result<Shared<Self>, failure::Error>;

    /// From an existing file get a memory mapped value of it.
    ///
    /// Use it if you are child. It would only try to convert the content
    /// of the file or it fails.
    fn get_file(path: PathBuf) -> Result<Shared<Self>, failure::Error>;
}

pub trait AsMutRef<T> {
    fn as_mut_ref(&self) -> &mut T;
}

impl<T> AsMutRef<T> for Shared<T> {
    fn as_mut_ref(&self) -> &mut T {
        unsafe { &mut *Shared::as_mut_ptr(self) }
    }
}
