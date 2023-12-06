use std::fs::File;
use std::io::Write;
mod enums;
mod file;
mod program;

fn main() {
    // get the name of the file from the command line
    let mut args: Vec<String> = std::env::args().collect();
    // remove the first argument (the name of the program)
    args.remove(0);
    // check if there is some parameter argument
    for arg in args.iter() {
        if arg == "-h" || arg == "--help" {
            println!("Usage: co <filename>");
            std::process::exit(0);
        }
        if arg == "-V" || arg == "--version" {
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }
        if arg == "-v" || arg == "--verbose" {
            println!("Verbose mode");
            std::env::set_var("RUST_LOG", "verbose");
        }
        if arg == "-d" || arg == "--debug" {
            println!("Debug mode");
            std::env::set_var("DEBUG_MODE", "debug");
        }
    }
    // get the filename (argument without a dash)
    let filename: String = loop {
        match args.pop() {
            Some(arg) => {
                if !arg.starts_with("-") {
                    break arg;
                }
            }
            None => {
                println!("Usage: co <filename>");
                std::process::exit(1);
            }
        }
    };
    let filename = filename.as_str();

    let mut program = program::Program::new();

    // check if the file exists
    if !std::path::Path::new(filename).exists() {
        println!("File {} does not exist", filename);
        std::process::exit(1);
    }
    // create a new CoFile
    let file = file::CoFile::new(filename.to_string());

    println!("File: {}", file.filename);
    println!("Extension: {}", file.extension);

    // ask if the user wants to export the file or run it
    println!("What do you want to do?\n1. Export\n2. Run\n3. Run and debug\n4. Exit");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: u32 = input.trim().parse().unwrap();
    match input {
        1 => {
            let t3 = std::time::Instant::now();
            let data = file.export();
            println!("Exported data: {}", data);
            let name = file.filename.clone().replace(".co", "");
            // create a new file with the same name but with the extension .bin and write the exported file to it
            let mut file = File::create(format!("{}.bin", name)).expect("creation failed");
            // cut the string into packages of 8 bits
            let bytes: Vec<u8> = data
                .chars()
                .collect::<Vec<char>>()
                .chunks(8)
                .map(|chunk| {
                    let s: String = chunk.iter().collect();
                    u8::from_str_radix(&s, 2).unwrap()
                })
                .collect();
            // write the bytes to the file
            file.write_all(&bytes).expect("write failed");
            println!("Time to export: {:?}", t3.elapsed());
            std::process::exit(0);
        }
        2 => {
            let t1 = std::time::Instant::now();
            program.load(file);
            println!("Time to load: {:?}", t1.elapsed());
            let t2 = std::time::Instant::now();
            program.run();
            println!("Time to run: {:?}", t2.elapsed());
        }
        3 => {
            let t1 = std::time::Instant::now();
            program.load(file);
            println!("Time to load: {:?}", t1.elapsed());
            let t2 = std::time::Instant::now();
            program.run_debug();
            println!("Time to run: {:?}", t2.elapsed());
        }
        4 => {
            std::process::exit(0);
        }
        _ => {
            println!("Invalid input");
            std::process::exit(1);
        }
    }
    println!("Program finished");
}
