use crate::enums::{AddressNames, Extension, Instruction, Label, Parameter, Register, Variable};
use regex::Regex;
use std::io::Read;
use std::str::FromStr;

// define macros to convert a string to a Instruction with a parameter and a register
macro_rules! str_to_instruction {
    ($instruction:ident, $line:ident, $variable_names: ident) => {
        let register = $line.next().unwrap();
        let parameter = $line.next().unwrap();
        return Instruction::$instruction(
            Register::from_str(register).unwrap(),
            Parameter::from_str(parameter, $variable_names),
        )
    };
    ($instruction:ident, $line:ident, $variable_names: ident, $label_names: ident) => {
        let parameter_1 = $line.next().unwrap();
        let parameter_2 = $line.next().unwrap();
        let label = $line.next().unwrap();
        return Instruction::$instruction(
            Parameter::from_str(parameter_1, $variable_names),
            Parameter::from_str(parameter_2, $variable_names),
            Label::from_str(label, $label_names),
        );
    };
}

macro_rules! bin_to_instruction {
    ($instruction:ident, $line:ident) => {
        let register = ($line >> 25) & 0b11; // get the register
        let parameter = ($line >> 13) & 0b111111111111; // get the parameter
        return Instruction::$instruction(Register::from(register as u8), Parameter::from(parameter))
    };
    // overload macro for instructions with 2 parameters and a label
    ($instruction:ident, $line:ident, $_:ident) => {
        let parameter_1 = ($line >> 15) & 0b111111111111; // get the first parameter
        let parameter_2 = ($line >> 3) & 0b111111111111; // get the second parameter
        let label = $line & 0b111; // get the label
        return Instruction::$instruction(
            Parameter::from(parameter_1),
            Parameter::from(parameter_2),
            Label::from(label as u8),
        );
    };
}

pub enum LineType {
    String(String),
    Bin(u32),
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
                    // remove leading and trailing whitespaces
                    let line = line.trim();
                    // check if line is a label
                    if line.ends_with(":") {
                        let lbl_name = line.replace(":", "");
                        return Instruction::LABEL(Label::from_str(lbl_name.as_str(), label_names));
                    }
                    let mut line = line.split(" ");
                    let instruction = line.next().unwrap();
                    match instruction {
                        "LDA" => {
                            str_to_instruction!(LDA, line, variable_names);
                        }
                        "STR" => {
                            let variable = line.next().unwrap();
                            let parameter = line.next().unwrap();
                            return Instruction::STR(
                                Variable::from_str(variable, variable_names),
                                Parameter::from_str(parameter, variable_names),
                            );
                        }
                        "PUSH" => {
                            let parameter = line.next().unwrap();
                            return Instruction::PUSH(Parameter::from_str(
                                parameter,
                                variable_names,
                            ));
                        }
                        "POP" => {
                            let register = line.next().unwrap();
                            return Instruction::POP(Register::from_str(register).unwrap());
                        }
                        "OR" => {
                            str_to_instruction!(OR, line, variable_names);
                        }
                        "NOT" => {
                            let register = line.next().unwrap();
                            return Instruction::NOT(Register::from_str(register).unwrap());
                        }
                        "AND" => {
                            str_to_instruction!(AND, line, variable_names);
                        }
                        "ADD" => {
                            str_to_instruction!(ADD, line, variable_names);
                        }
                        "SUB" => {
                            str_to_instruction!(SUB, line, variable_names);
                        }
                        "MUL" => {
                            str_to_instruction!(MUL, line, variable_names);
                        }
                        "DIV" => {
                            str_to_instruction!(DIV, line, variable_names);
                        }
                        "MOD" => {
                            str_to_instruction!(MOD, line, variable_names);
                        }
                        "INC" => {
                            let register = line.next().unwrap();
                            return Instruction::INC(Register::from_str(register).unwrap());
                        }
                        "DEC" => {
                            let register = line.next().unwrap();
                            return Instruction::DEC(Register::from_str(register).unwrap());
                        }
                        "BEQ" => {
                            str_to_instruction!(BEQ, line, variable_names, label_names);
                        }
                        "BNE" => {
                            str_to_instruction!(BNE, line, variable_names, label_names);
                        }
                        "BBG" => {
                            str_to_instruction!(BBG, line, variable_names, label_names);
                        }
                        "BSM" => {
                            str_to_instruction!(BSM, line, variable_names, label_names);
                        }
                        "JMP" => {
                            let label = line.next().unwrap();
                            return Instruction::JMP(Label::from_str(label, label_names));
                        }
                        "SRL" => {
                            let register = line.next().unwrap();
                            let constant = line.next().unwrap();
                            return Instruction::SRL(
                                Register::from_str(register).unwrap(),
                                constant.parse::<u16>().unwrap(),
                            );
                        }
                        "SRR" => {
                            let register = line.next().unwrap();
                            let constant = line.next().unwrap();
                            return Instruction::SRR(
                                Register::from_str(register).unwrap(),
                                constant.parse::<u16>().unwrap(),
                            );
                        }
                        "HLT" => {
                            return Instruction::HLT;
                        }
                        "IN" => {
                            let parameter = line.next().unwrap();
                            return Instruction::IN(Parameter::from_str(parameter, variable_names));
                        }
                        "OUT" => {
                            let parameter = line.next().unwrap();
                            return Instruction::OUT(Parameter::from_str(
                                parameter,
                                variable_names,
                            ));
                        }
                        _ => panic!("Invalid instruction"),
                    }
                }
                LineType::Bin(line) => {
                    let opcode = line >> 27; // get the instruction opcode
                    match opcode {
                        0b00000 => {
                            bin_to_instruction!(LDA, line);
                        }
                        0b00001 => {
                            let variable = ((line >> 17) & 0b1111111111) as u16; // get the variable name
                            let parameter = ((line >> 5) & 0b111111111111) as u32; // get the parameter
                            return Instruction::STR(
                                Variable::from(variable),
                                Parameter::from(parameter),
                            );
                        }
                        0b00010 => {
                            let parameter = ((line >> 15) & 0b111111111111) as u32; // get the parameter
                            return Instruction::PUSH(Parameter::from(parameter));
                        }
                        0b00011 => {
                            let register = ((line >> 25) & 0b11) as u8; // get the register
                            return Instruction::POP(Register::from(register));
                        }
                        0b00100 => {
                            bin_to_instruction!(AND, line);
                        }
                        0b00101 => {
                            bin_to_instruction!(OR, line);
                        }
                        0b00110 => {
                            let register = ((line >> 25) & 0b11) as u8; // get the register
                            return Instruction::NOT(Register::from(register));
                        }
                        0b00111 => {
                            bin_to_instruction!(ADD, line);
                        }
                        0b01000 => {
                            bin_to_instruction!(SUB, line);
                        }
                        0b01001 => {
                            bin_to_instruction!(DIV, line);
                        }
                        0b01010 => {
                            bin_to_instruction!(MUL, line);
                        }
                        0b01011 => {
                            bin_to_instruction!(MOD, line);
                        }
                        0b01100 => {
                            let register = ((line >> 25) & 0b11) as u8; // get the register
                            return Instruction::INC(Register::from(register));
                        }
                        0b01101 => {
                            let register = ((line >> 25) & 0b11) as u8; // get the register
                            return Instruction::DEC(Register::from(register));
                        }
                        0b01110 => {
                            bin_to_instruction!(BEQ, line, line);
                        }
                        0b01111 => {
                            bin_to_instruction!(BNE, line, line);
                        }
                        0b10000 => {
                            bin_to_instruction!(BSM, line, line);
                        }
                        0b10001 => {
                            bin_to_instruction!(BBG, line, line);
                        }
                        0b10010 => {
                            let label = (line >> 24) & 0b111; // get the label
                            return Instruction::JMP(Label::from(label as u8));
                        }
                        0b10011 => {
                            let register = ((line >> 25) & 0b11) as u8; // get the register
                            let constant = ((line >> 15) & 0b1111111111) as u16; // get the constant
                            return Instruction::SRL(Register::from(register), constant);
                        }
                        0b10100 => {
                            let register = ((line >> 25) & 0b11111) as u8; // get the register
                            let constant = ((line >> 15) & 0b1111111111) as u16; // get the constant
                            return Instruction::SRR(Register::from(register), constant);
                        }
                        0b10101 => {
                            return Instruction::HLT;
                        }
                        0b10110 => {
                            let parameter = ((line >> 15) & 0b111111111111) as u32; // get the parameter
                            return Instruction::IN(Parameter::from(parameter));
                        }
                        0b10111 => {
                            let parameter = ((line >> 15) & 0b111111111111) as u32; // get the parameter
                            return Instruction::OUT(Parameter::from(parameter));
                        }
                        0b11110 => {
                            let label = (line >> 24) & 0b111; // get the label
                            return Instruction::LABEL(Label::from(label as u8));
                        }
                        _ => panic!("Invalid instruction"),
                    }
                }
            }
        } else if *category == LineCategory::DATA {
            match self {
                LineType::String(line) => {
                    // check if line is a array
                    if Regex::new(r"^[a-zA-Z0-9]+\[\d+\]").unwrap().is_match(line) {
                        let (name, value) = line.split_at(line.find(" ").unwrap());
                        let value = value.trim();
                        let value =
                            Into::<u32>::into(value.parse::<i32>().unwrap() as u32) & 0b1111111111; // parse the value and convert it to 10 bits integer
                                                                                                    // split the name to get the variable name and the array length
                        let (name, length) = name.split_at(name.find("[").unwrap());
                        let length = length.replace("[", "").replace("]", "");
                        let length = length.parse::<u16>().unwrap();
                        return Instruction::ARRAY(
                            Variable::new(name, variable_names),
                            value,
                            length,
                        );
                    } else {
                        let (name, value) = line.split_at(line.find(" ").unwrap());
                        let value = value.trim();
                        let value =
                            Into::<u32>::into(value.parse::<i32>().unwrap() as u32) & 0b1111111111; // parse the value and convert it to 10 bits integer
                        return Instruction::VARIABLE(Variable::new(name, variable_names), value);
                    }
                }
                LineType::Bin(line) => {
                    let data_type = (line >> 31) as usize; // get first bit for variable type
                    let name = (line >> 21) & 0b1111111111; // get next 10 bits for variable name
                    if data_type == 0 {
                        let value = (line >> 11) & 0b1111111111; // get next 10 bits for variable value
                        return Instruction::VARIABLE(Variable::from(name as u16), value);
                    } else {
                        let length = (line >> 11) & 0b1111111111; // get next 10 bits for array length
                        let value = (line >> 1) & 0b1111111111; // get last 10 bits for array value
                        return Instruction::ARRAY(
                            Variable::from(name as u16),
                            value,
                            length as u16,
                        );
                    }
                }
            }
        } else {
            panic!("Invalid category");
        }
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
                // first 5 bits are 1
                let line = line >> 27;
                if line == 0b11111 {
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
                // first 5 bits are 1 and next 2 bits are category number
                let line = line >> 25;
                // next 2 bits are category number
                let line = line & 0b11;
                if line == 0b00 {
                    return LineCategory::DATA;
                } else if line == 0b01 {
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

    pub fn is_comment(&self) -> bool {
        match self {
            LineType::String(line) => {
                // remove whitespaces
                let line = line.trim();
                if line.starts_with(";") {
                    return true;
                }
                return false;
            }
            LineType::Bin(_) => {
                return false;
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum LineCategory {
    DATA = 0b00,
    CODE = 0b01,
    NONE,
}

impl Into<u32> for LineCategory {
    fn into(self) -> u32 {
        match self {
            LineCategory::DATA => {
                let mut res: u32 = 0b11111;
                res <<= 2;
                res |= 0b00; // 2 next bits are category number
                res <<= 25; // shift 27 bits to get 32 bits
                return res;
            }
            LineCategory::CODE => {
                let mut res: u32 = 0b11111;
                res <<= 2;
                res |= 0b01; // 2 next bits are category number
                res <<= 25; // shift 27 bits to get 32 bits
                return res;
            }
            LineCategory::NONE => panic!("Invalid category"),
        }
    }
}

pub struct CoFile {
    pub filename: String,
    pub extension: Extension,
}

impl CoFile {
    pub fn new(filename: String) -> CoFile {
        let extension = match filename.split(".").last() {
            Some("co") => Extension::CO,
            Some("bin") => Extension::BIN,
            _ => panic!("Invalid file extension"),
        };

        CoFile {
            filename,
            extension,
        }
    }

    fn read_as_bin(&self) -> Vec<u32> {
        let mut file: std::fs::File = std::fs::File::open(&self.filename).expect("File not found");
        // if file is a .bin file, read it as binary
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
            Extension::CO => {
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

    pub fn export(&self) -> String {
        let buffer = self.read();
        let mut variable_names = AddressNames::new();
        let mut label_names = AddressNames::new();
        let mut res: Vec<u32> = Vec::new();
        let mut current_category: LineCategory = LineCategory::NONE;
        for line in buffer {
            if line.is_category() {
                current_category = line.get_category();
                res.push(current_category.into());
                continue;
            }
            if current_category == LineCategory::NONE {
                panic!("Invalid category");
            }
            if line.is_empty() || line.is_comment() {
                continue;
            }
            let instruction =
                line.translate(&current_category, &mut variable_names, &mut label_names);
            res.push(instruction.into());
        }
        let res = res
            .iter()
            .map(|line| {
                return format!("{:032b}", line);
            })
            .collect::<Vec<String>>()
            .join("");
        return res;
    }
}
