use super::InitMemoryMappedFile;
use caring::Shared;
use interprocess_traits::ProcSync;
use std::{fs::OpenOptions, io::Write, mem, os::unix::io::IntoRawFd, path::PathBuf};

pub struct Slice<T: Default, const N: usize> {
    slice: [T; N],
}
impl<T: Default, const N: usize> Slice<T, N> {
    pub fn new() -> Self {
        unsafe {
            let mut data: [T; N] = std::mem::zeroed();
            for i in 0..N {
                data[i] = Default::default();
            }
            Self { slice: data }
        }
    }
}

impl<T, const N: usize> InitMemoryMappedFile for Slice<T, N>
where T : Default + Sync {
    fn new_file(path: PathBuf) -> Result<Shared<Slice<T, N>>, failure::Error> {
        let mut f = OpenOptions::new().write(true).read(true).open(path)?;
        // Has to use vec! instead of slice here or the COMPILER crashes. 
        // TODO: file an issue later.
        let _ = f.write(&vec![0u8; mem::size_of::<Slice<T, N>>()]);
        let fd = f.into_raw_fd();
        Ok(unsafe { Shared::from_raw_fd(fd)? })
    }
    fn get_file(path: PathBuf) -> Result<Shared<Slice<T, N>>, failure::Error> {
        let f = OpenOptions::new().write(true).read(true).open(path)?;
        let fd = f.into_raw_fd();
        Ok(unsafe { Shared::from_raw_fd(fd)? })
    }
}

unsafe impl<T, const N: usize> ProcSync for Slice<T, N> 
where T : Default + Sync{}
