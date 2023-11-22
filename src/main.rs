#![allow(dead_code)]
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

    // read the file
    let buffer: Vec<LineType> = bwfile.read();

    // parse the file
    let mut current_category: LineCategory = LineCategory::NONE;
    for line in buffer {
        if line.is_category() {
            current_category = line.get_category();
            println!("Current category : {:?}", current_category);
            continue;
        }
        if current_category == LineCategory::NONE {
            panic!("Invalid category");
        }
        // check if line is not empty
        if line.is_empty() {
            continue;
        }
        let instruction: Instruction = line.translate(
            &current_category,
            &mut program.variable_names,
            &mut program.label_names,
        );
        // print the instruction in 32 bits (u32)
        println!(
            "Intruction in 32 bits is {:032b}",
            Into::<u32>::into(instruction)
        );
    }
}
