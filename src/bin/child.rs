use mmap_ds::toy_struct::ToyStruct;
use mmap_ds::AsMutRef;
use mmap_ds::InitMemoryMappedFile;
use std::path::PathBuf;

fn main() -> Result<(), failure::Error> {
    println!("Child: started");
    let data = ToyStruct::get_file(PathBuf::new().join("temp"))?;
    let data_ref = data.as_mut_ref();
    println!("Child: I got secret: {:?}", data_ref);
    data_ref.secret = 4242;
    println!("Child: I changed secret: {:?}", data_ref);
    println!("Child: I am out");
    Ok(())
}
