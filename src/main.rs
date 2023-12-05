#![allow(dead_code)]
mod bwfile;
mod enums;
mod program;

fn main() {
    // get the name of the file from the command line
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: bw <filename>");
        std::process::exit(1);
    }
    let filename: &str = &args[1];

    let mut program = program::Program::new();

    // check if the file exists
    if !std::path::Path::new(filename).exists() {
        println!("File {} does not exist", filename);
        std::process::exit(1);
    }
    // create a new BWFile
    let bwfile = bwfile::BWFile::new(filename.to_string());

    println!("File: {}", bwfile.filename);
    println!("Extension: {}", bwfile.extension);

    // ask if the user wants to export the file or run it
    println!("What do you want to do?\n1. Export\n2. Run\n3. Run and debug\n4. Exit");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: u32 = input.trim().parse().unwrap();
    match input {
        1 => {
            let t3 = std::time::Instant::now();
            println!("{}", bwfile.export());
            println!("Time to export: {:?}", t3.elapsed());
            std::process::exit(0);
        }
        2 => {
            let t1 = std::time::Instant::now();
            program.load(bwfile);
            println!("Time to load: {:?}", t1.elapsed());
            let t2 = std::time::Instant::now();
            program.run();
            println!("Time to run: {:?}", t2.elapsed());
        }
        3 => {
            let t1 = std::time::Instant::now();
            program.load(bwfile);
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
