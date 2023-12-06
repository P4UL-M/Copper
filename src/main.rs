use std::fs::File;
use std::io::Write;
use std::str::FromStr;
mod enums;
mod file;
mod program;

enum Command {
    Run,
    Export,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "run" => Ok(Command::Run),
            "export" => Ok(Command::Export),
            _ => Err(()),
        }
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
const HELP_MESSAGE: &str = "Usage: copper <filename>\n\nOptions:\n\t-h, --help\t\tPrint this help message\n\t-V, --version\t\tPrint version information\n\t-v, --verbose\t\tVerbose mode\n\t-d, --debug\t\tDebug mode\n\nCommands:\n\trun\t\t\tRun the program\n\texport\t\t\tExport the program to a binary file\n\nExamples:\n\tcopper program.co\n\tcopper run program.co\n\tcopper export program.co\n";

fn main() {
    // get the name of the file from the command line
    let mut args: Vec<String> = std::env::args().collect();
    // remove the first argument (the name of the program)
    args.remove(0);
    let mut command = Command::Run;
    let command_str = if args.len() > 0 { &args[0] } else { "" };
    match Command::from_str(&command_str) {
        Ok(c) => {
            command = c;
            args.remove(0);
        }
        Err(_) => {}
    }
    // check if there is some parameter argument
    for arg in args.iter() {
        if arg == "-h" || arg == "--help" {
            println!("{}", HELP_MESSAGE);
            std::process::exit(0);
        }
        if arg == "-V" || arg == "--version" {
            println!("Version: {}", VERSION);
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
                println!("No filename given. Use -h or --help for help.");
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

    if program.verbose {
        println!("File: {}", file.filename);
        println!("Extension: {}", file.extension);
    }

    match command {
        Command::Export => {
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
        Command::Run => {
            let t1 = std::time::Instant::now();
            program.load(file);
            println!("Time to load: {:?}", t1.elapsed());
            let t2 = std::time::Instant::now();
            if std::env::var("DEBUG_MODE").is_ok() {
                program.run_debug();
            } else {
                program.run();
            }
            println!("Time to run: {:?}", t2.elapsed());
        }
    }
    println!("Program finished");
}
