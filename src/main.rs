use rfd::FileDialog;
use std::fs::File;
use std::io::prelude::*;

mod instruction;
mod machine;
const HERZ: u16 = 500;

fn main() {
    let path = FileDialog::new().pick_file().unwrap();

    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(reason) => panic!("Couldn't open {}: {}", display, reason),
        Ok(file) => file,
    };

    let mut buffer = Vec::<u8>::new();
    _ = file.read_to_end(&mut buffer);

    let mut machine = machine::machine::Machine::new();
    machine.set_ram(0, buffer);
    machine.general_registers[1] = 5;


    machine.execute(Some(HERZ as u32));
}
