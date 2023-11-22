mod bwfile;
mod enums;
mod program;

use crate::{
    bwfile::{LineCategory, LineType},
    enums::Instruction,
};

fn main() {
    // get the name of the file from the command line
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: bw <filename>");
        std::process::exit(1);
    }
    let filename: &String = &args[1];

    let program = program::Program::new();

    // check if the file exists
    if !std::path::Path::new(filename).exists() {
        println!("File {} does not exist", filename);
        std::process::exit(1);
    }
    // create a new BWFile
    let bwfile = bwfile::BWFile::new(filename.to_string());

    println!("File: {}", bwfile.filename);
    println!("Extension: {}", bwfile.extension.to_string());

    // read the file
    let buffer: Vec<LineType> = bwfile.read();

    // parse the file
    let mut current_category: LineCategory = LineCategory::NONE;
    for line in buffer {
        if line.is_category() {
            current_category = line.get_category();
            continue;
        }
        match current_category {
            LineCategory::DATA => {
                continue;
            }
            LineCategory::CODE => {
                let instruction: Instruction = line.translate(&current_category);
                // print the instruction in 32 bits (u32)
                println!(
                    "Intruction in 32 bits is {:032b}",
                    Into::<u32>::into(instruction)
                );
            }
            LineCategory::NONE => {
                continue;
            }
        }

        // Expected output:
        // File: 20_bytes.bin
        // 111111111111111111111111111111 01
        // 00000 00 010010000000 0000000000000
        // 00000 01 010100000000 0000000000000
        // 00000 10 100000000001 0000000000000
        // 00000 11 001000000000 0000000000000
    }
}
