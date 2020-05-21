use std::marker::{Sized, PhantomData};
use std::path::PathBuf;

struct LinkedList<T: Sized> {
    size: usize,
    len: usize,
    fd: usize,
    head: Option<usize>,
    phantom: PhantomData<T>,
}

struct Item<T: Sized> {
    item: T,
    next: Option<usize>,
}

impl<T: Sized> Item<T> {
    pub fn new(item: T) -> Item<T>{
        Item{
            item: item,
            next: None,
        }
    }
}

impl<T: Sized> LinkedList<T> {
    pub fn new(path: PathBuf) -> LinkedList<T> {
        unimplemented!()
    }
    pub fn resize(&self, size: usize){
        unimplemented!()
    }
    pub fn insert_after(&self, item: T, pos: usize) {
        unimplemented!();
    }
    pub fn remove(pos: usize) {
        unimplemented!();
    }
    pub fn nth_file(idx: usize) -> &'static Item<T> {
        unimplemented!()
    }
    pub fn nth_mut_file(idx: usize) -> &'static mut Item<T> {
        unimplemented!()
    }
    pub fn nth_list(idx: usize) -> &'static Item<T> {
        unimplemented!()
    }
    pub fn nth_mut_list(idx: usize) -> &'static mut Item<T> {
        unimplemented!()
    }
}

impl<T: Sized> std::ops::Index<usize> for LinkedList<T> {
    type Output = Item<T>;
    fn index(&self, index: usize) -> &Self::Output {
        unimplemented!()
    }
}

impl<T: Sized> std::ops::IndexMut<usize> for LinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unimplemented!()
    }
}
