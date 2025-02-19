use rfd::FileDialog;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::path::PathBuf;
use colored::Colorize;

mod instruction;
mod machine;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut herz: Option<u32> = None;
    let mut input_path: PathBuf = Default::default();

    get_inputs(args, &mut input_path, &mut herz);

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&input_path) {
        Err(reason) => panic!("Couldn't open {}: {}", input_path.display(), reason),
        Ok(file) => file,
    };

    let mut buffer = Vec::<u8>::new();
    _ = file.read_to_end(&mut buffer);

    let mut machine = machine::machine::Machine::new();
    machine.set_ram(0, buffer);

    machine.execute(herz);
}


fn get_inputs(args: Vec<String>, input_path: &mut PathBuf, herz: &mut Option<u32>) {
    if args.contains(&"-v".to_string()){
        // Visual setup
        let dialog = FileDialog::new();
        if let Some(path) = dialog.pick_file(){
            *input_path = path;
        }
        return;
    }
    // Command line args setup
    *input_path = PathBuf::from(get_parameter("-f", args.clone()));
    if args.contains(&"-hz".to_string()){
        *herz = Some(get_parameter_uint("-hz", args.clone()) as u32);
    }
}

fn get_parameter(name: &str, args: Vec<String>) -> String {
    let mut parameter_position: Option<usize> = None;
    for i in 0..args.len() - 1{
        if let Some(argument) = args.get(i) {
            if argument == name{
                parameter_position = Some(i + 1);
            }
        }
    }

    if parameter_position.is_none() {
        let error = format!("Parameter {} expected but not found", name).red().to_string();
        panic!("{}", error);
    }

    if let Some(result) = args.iter().nth(parameter_position.unwrap()) { return result.to_string(); }
    let error = format!("Parameter {} has an expected value.", name).red().to_string();
    panic!("{}", error);
}

fn get_parameter_uint(name: &str, args: Vec<String>) -> u64 {
    let parameter_value = get_parameter(name, args);

    if let Some(parameter_value_u64) = parameter_value.parse::<u64>().ok(){ return parameter_value_u64; }
    let error = format!("Parameter {} should be a positive number but {} was found instead.", name, parameter_value).red().to_string();
    panic!("{}", error);
}