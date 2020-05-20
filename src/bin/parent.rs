use mmap_ds::AsMutRef;
use mmap_ds::InitMemoryMappedFile;
use mmap_ds::toy_struct::ToyStruct;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

fn main() -> Result<(), failure::Error> {
    println!("Parent: started");
    let data = ToyStruct::new_file(PathBuf::new().join("temp"))?;
    let data_ref = data.as_mut_ref();
    *data_ref = ToyStruct::new(42);
    println!("Parent: I put the secret: {:?}", data_ref);
    println!("Parent: I started child, let's wait him for 500ms");
    let _child = Command::new("./target/debug/child")
        .spawn()
        .expect("failed to execute child");
    std::thread::sleep(Duration::from_millis(500));
    println!("Parent: I got the new secret: {:?}", data_ref);
    Ok(())
}
