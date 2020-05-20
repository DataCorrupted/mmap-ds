use super::InitMemoryMappedFile;
use caring::Shared;
use interprocess_traits::ProcSync;
use std::{fs::OpenOptions, io::Write, mem, os::unix::io::IntoRawFd, path::PathBuf};

#[derive(Debug)]
pub struct ToyStruct {
    pub secret: u64,
}
impl ToyStruct {
    pub fn new(secret: u64) -> Self {
        ToyStruct { secret: secret }
    }
}

impl InitMemoryMappedFile for ToyStruct {
    fn new_file(path: PathBuf) -> Result<Shared<ToyStruct>, failure::Error> {
        let mut f = OpenOptions::new().read(true).write(true).open(path)?;
        let _ = f.write(&[0; mem::size_of::<ToyStruct>()]);
        let fd = f.into_raw_fd();
        Ok(unsafe { Shared::from_raw_fd(fd)? })
    }
    fn get_file(path: PathBuf) -> Result<Shared<ToyStruct>, failure::Error> {
        let f = OpenOptions::new().write(true).read(true).open(path)?;
        let fd = f.into_raw_fd();
        Ok(unsafe { Shared::from_raw_fd(fd)? })
    }
}

unsafe impl ProcSync for ToyStruct {}
