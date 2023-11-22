use crate::enums::Addressable;
use crate::enums::{AddressNames, Extension, Instruction, Label, Parameter, Register, Variable};
use std::io::Read;
use std::str::FromStr;

pub enum LineType {
    String(String),
    Bin(u32),
}

#[derive(PartialEq, Debug)]
pub enum LineCategory {
    DATA = 0b00,
    CODE = 0b01,
    NONE,
}

impl LineType {
    pub fn translate(
        &self,
        category: &LineCategory,
        variable_names: &mut AddressNames,
        label_names: &mut AddressNames,
    ) -> Instruction {
        // export categories
        if *category == LineCategory::CODE {
            match self {
                LineType::String(line) => {
                    // check if line is a label
                    if line.ends_with(":") {
                        let lbl_name = line.replace(":", "");
                        return Instruction::LABEL(Label::from_str(lbl_name.as_str(), label_names));
                    }
                    let mut line = line.split(" ");
                    let instruction = line.next().unwrap();
                    match instruction {
                        "LDA" => {
                            let register = line.next().unwrap();
                            let parameter = line.next().unwrap(); // get the parameter
                            return Instruction::LDA(
                                Register::from_str(register).unwrap(),
                                Parameter::from_str(parameter, variable_names),
                            )
                            .into();
                        }
                        _ => panic!("Invalid instruction"),
                    }
                }
                LineType::Bin(line) => {
                    let opcode = line >> 27; // get the instruction opcode
                    match opcode {
                        0b00000 => {
                            let register = ((line >> 25) & 0b11) as u8; // get the register
                            let parameter = ((line >> 13) & 0b111111111111) as u32; // get the parameter
                            return Instruction::LDA(
                                Register::from(register),
                                Parameter::from(parameter),
                            )
                            .into();
                        }
                        _ => panic!("Invalid instruction"),
                    }
                }
            }
        } else if *category == LineCategory::DATA {
            match self {
                LineType::String(line) => {
                    let (name, value) = line.split_at(line.find(" ").unwrap());
                    let value = value.trim();
                    let value =
                        Into::<u32>::into(value.parse::<i32>().unwrap() as u32) & 0b1111111111; // parse the value and convert it to 10 bits integer
                    return Instruction::VARIABLE(Variable::new(name, variable_names), value);
                }
                LineType::Bin(line) => {
                    let name = line >> 22; // get first 10 bits for variable name
                    let value = (line >> 12) & 0b1111111111; // get next 10 bits for variable value
                    return Instruction::VARIABLE(Variable::from(name as u16), value);
                }
            }
        }
        return Instruction::HLT;
    }

    pub fn is_category(&self) -> bool {
        match self {
            LineType::String(line) => {
                if line.starts_with("#DATA") {
                    return true;
                } else if line.starts_with("#CODE") {
                    return true;
                }
                return false;
            }
            LineType::Bin(line) => {
                // first 30 bits are 1
                if line >> 2 == 0b111111111111111111111111111111 {
                    return true;
                }
                return false;
            }
        }
    }

    pub fn get_category(&self) -> LineCategory {
        if !self.is_category() {
            panic!("Line is not a category");
        }
        match self {
            LineType::String(line) => {
                if line.starts_with("#DATA") {
                    return LineCategory::DATA;
                } else if line.starts_with("#CODE") {
                    return LineCategory::CODE;
                }
                return LineCategory::NONE;
            }
            LineType::Bin(line) => {
                // first 30 bits are 1
                if line & 0b11 == 0b00 {
                    return LineCategory::DATA;
                } else if line & 0b11 == 0b01 {
                    return LineCategory::CODE;
                } else {
                    panic!("Invalid category");
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            LineType::String(line) => {
                if line.is_empty() {
                    return true;
                }
                return false;
            }
            LineType::Bin(line) => {
                if line == &0 {
                    return true;
                }
                return false;
            }
        }
    }
}

pub struct BWFile {
    pub filename: String,
    pub extension: Extension,
}

impl BWFile {
    pub fn new(filename: String) -> BWFile {
        let extension = match filename.split(".").last() {
            Some("bw") => Extension::BW,
            Some("bin") => Extension::BIN,
            _ => panic!("Invalid file extension"),
        };

        BWFile {
            filename,
            extension,
        }
    }

    fn read_as_bin(&self) -> Vec<u32> {
        let mut file: std::fs::File = std::fs::File::open(&self.filename).expect("File not found");
        // if file is a .bwin file, read it as binary
        // in binary mode, each instruction is 32 bytes long so we need to read 32 bytes at a time
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer).expect("Error reading file");
        let buffer32 = buffer
            .chunks(4)
            .map(|chunk| {
                if chunk.len() != 4 {
                    panic!("Invalid file format");
                }
                let res: u32 = (chunk[0] as u32) << 24
                    | (chunk[1] as u32) << 16
                    | (chunk[2] as u32) << 8
                    | (chunk[3] as u32);
                // convert the 4 bytes into a u32
                return res;
            })
            .collect::<Vec<u32>>();
        return buffer32;
    }

    pub fn read_as_text(&self) -> Vec<String> {
        let mut file: std::fs::File = std::fs::File::open(&self.filename).expect("File not found");
        // if file is a .bw file, read it as text
        let mut buffer: String = String::new();
        file.read_to_string(&mut buffer)
            .expect("Error reading file");
        let buffer = buffer
            .split("\n")
            .map(|chunk| {
                return chunk.to_string();
            })
            .collect::<Vec<String>>();
        return buffer;
    }

    // return a vector of u32 or String depending on the file extension
    pub fn read(&self) -> Vec<LineType> {
        match self.extension {
            Extension::BW => {
                let buffer = self.read_as_text();
                let buffer = buffer
                    .iter()
                    .map(|line| {
                        return LineType::String(line.to_string());
                    })
                    .collect::<Vec<LineType>>();
                return buffer;
            }
            Extension::BIN => {
                let buffer = self.read_as_bin();
                let buffer = buffer
                    .iter()
                    .map(|line| {
                        return LineType::Bin(*line);
                    })
                    .collect::<Vec<LineType>>();
                return buffer;
            }
        }
    }
}
