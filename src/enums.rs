#![allow(non_camel_case_types)]

use core::fmt;
use std::str;
pub type u2 = u8; // Register size (4 possible registers)
pub type u3 = u8; // Variable and Label size (8 possible labels per program)
pub type u12 = u32; // Parameter type + value size (4096 possible parameters per program)

pub enum Instruction {
    LDA(Register, Parameter),
    STR(Register, Parameter),
    PUSH(Parameter),
    POP(Register),
    AND(Register, Parameter),
    OR(Register, Parameter),
    NOT(Register),
    ADD(Register, Parameter),
    SUB(Register, Parameter),
    DIV(Register, Parameter),
    MUL(Register, Parameter),
    MOD(Register, Parameter),
    INC(Register),
    DEC(Register),
    BEQ(Parameter, Parameter, u3),
    BNE(Parameter, Parameter, u3),
    BBG(Parameter, Parameter, u3),
    BSM(Parameter, Parameter, u3),
    JMP(u3),
    HLT,
    VARIABLE(Variable, u32),
    LABEL(Label),
}

impl Into<u32> for Instruction {
    fn into(self) -> u32 {
        match self {
            Instruction::LDA(r, p) => {
                let mut res: u32 = 0b00000; // 5 bits for the instruction
                res = res << 2;
                res = res | r as u32; // 2 bits for the register
                res = res << 12;
                res = res | Into::<u12>::into(p); // 12 bits for the parameter
                res = res << 13; // 13 bits to get to 32 bits
                return res;
            }
            Instruction::STR(r, p) => {
                let mut res: u32 = 0b00001;
                res = res << 2;
                res = res | r as u32; // 2 bits for the register
                res = res << 12;
                res = res | Into::<u12>::into(p); // 12 bits for the parameter
                res = res << 13; // 13 bits to get to 32 bits
                return res;
            }
            Instruction::PUSH(p) => {
                let mut res: u32 = 0b00010;
                res = res << 12;
                res = res | Into::<u12>::into(p); // 12 bits for the parameter
                res = res << 15; // 15 bits to get to 32 bits
                return res;
            }
            Instruction::POP(r) => {
                let mut res: u32 = 0b00011;
                res = res << 2;
                res = res | r as u32; // 2 bits for the register
                res = res << 25; // 25 bits to get to 32 bits
                return res;
            }
            Instruction::AND(r, p) => {
                let mut res: u32 = 0b00100;
                res = res << 2;
                res = res | r as u32; // 2 bits for the register
                res = res << 12;
                res = res | Into::<u12>::into(p); // 12 bits for the parameter
                res = res << 13; // 13 bits to get to 32 bits
                return res;
            }
            Instruction::OR(r, p) => {
                let mut res: u32 = 0b00101;
                res = res << 2;
                res = res | r as u32; // 2 bits for the register
                res = res << 12;
                res = res | Into::<u12>::into(p); // 12 bits for the parameter
                res = res << 13; // 13 bits to get to 32 bits
                return res;
            }
            Instruction::NOT(r) => {
                let mut res: u32 = 0b00110;
                res = res << 2;
                res = res | r as u32; // 2 bits for the register
                res = res << 25; // 25 bits to get to 32 bits
                return res;
            }
            Instruction::ADD(r, p) => {
                let mut res: u32 = 0b00111;
                res = res << 2;
                res = res | r as u32; // 2 bits for the register
                res = res << 12;
                res = res | Into::<u12>::into(p); // 12 bits for the parameter
                res = res << 13; // 13 bits to get to 32 bits
                return res;
            }
            Instruction::SUB(r, p) => {
                let mut res: u32 = 0b0100;
                res = res << 2;
                res = res | r as u32; // 2 bits for the register
                res = res << 12;
                res = res | Into::<u12>::into(p); // 12 bits for the parameter
                res = res << 13; // 13 bits to get to 32 bits
                return res;
            }
            Instruction::DIV(r, p) => {
                let mut res: u32 = 0b01001;
                res = res << 2;
                res = res | r as u32; // 2 bits for the register
                res = res << 12;
                res = res | Into::<u12>::into(p); // 12 bits for the parameter
                res = res << 13; // 13 bits to get to 32 bits
                return res;
            }
            Instruction::MUL(r, p) => {
                let mut res: u32 = 0b01010;
                res = res << 2;
                res = res | r as u32; // 2 bits for the register
                res = res << 12;
                res = res | Into::<u12>::into(p); // 12 bits for the parameter
                res = res << 13; // 13 bits to get to 32 bits
                return res;
            }
            Instruction::MOD(r, p) => {
                let mut res: u32 = 0b01011;
                res = res << 2;
                res = res | r as u32; // 2 bits for the register
                res = res << 12;
                res = res | Into::<u12>::into(p); // 12 bits for the parameter
                res = res << 13; // 13 bits to get to 32 bits
                return res;
            }
            Instruction::INC(r) => {
                let mut res: u32 = 0b01100;
                res = res << 2;
                res = res | r as u32; // 2 bits for the register
                res = res << 25; // 25 bits to get to 32 bits
                return res;
            }
            Instruction::DEC(r) => {
                let mut res: u32 = 0b01101;
                res = res << 2;
                res = res | r as u32; // 2 bits for the register
                res = res << 25; // 25 bits to get to 32 bits
                return res;
            }
            Instruction::BEQ(p1, p2, lbl) => {
                let mut res: u32 = 0b01110;
                res = res << 12;
                res = res | Into::<u12>::into(p1); // 12 bits for the parameter
                res = res << 12;
                res = res | Into::<u12>::into(p2); // 12 bits for the parameter
                res = res << 3;
                res = res | (lbl as u32 & 0b111); // 3 bits for the label
                return res;
            }
            Instruction::BNE(p1, p2, lbl) => {
                let mut res: u32 = 0b01111;
                res = res << 12;
                res = res | Into::<u12>::into(p1); // 12 bits for the parameter
                res = res << 12;
                res = res | Into::<u12>::into(p2); // 12 bits for the parameter
                res = res << 3;
                res = res | (lbl as u32 & 0b111); // 3 bits for the label
                return res;
            }
            Instruction::BSM(p1, p2, lbl) => {
                let mut res: u32 = 0b10000;
                res = res << 12;
                res = res | Into::<u12>::into(p1); // 12 bits for the parameter
                res = res << 12;
                res = res | Into::<u12>::into(p2); // 12 bits for the parameter
                res = res << 3;
                res = res | (lbl as u32 & 0b111); // 3 bits for the label
                return res;
            }
            Instruction::BBG(p1, p2, lbl) => {
                let mut res: u32 = 0b10001;
                res = res << 12;
                res = res | Into::<u12>::into(p1); // 12 bits for the parameter
                res = res << 12;
                res = res | Into::<u12>::into(p2); // 12 bits for the parameter
                res = res << 3;
                res = res | (lbl as u32 & 0b111); // 3 bits for the label
                return res;
            }
            Instruction::JMP(i) => {
                let mut res: u32 = 0b10010;
                res = res << 3;
                res = res | (i as u32 & 0b111); // 3 bits for the label
                res = res << 24; // 24 bits to get to 32 bits
                return res;
            }
            Instruction::HLT => {
                let mut res: u32 = 0b10011;
                res = res << 27; // 27 bits to get to 32 bits
                return res;
            }
            Instruction::VARIABLE(v, i) => {
                // first 10 bits are variable name
                let mut res: u32 = Into::<u16>::into(v) as u32;
                // next 10 bits are the value
                res = res << 10;
                res = res | (i & 0b1111111111);
                // shift left 12 bits to get to 32 bits
                res = res << 12;
                return res;
            }
            Instruction::LABEL(l) => {
                let mut res = 0b11110 as u32; // 5 bits for the instruction
                res = res << 3;
                res = res | (Into::<u3>::into(l) as u32 & 0b111); // 3 bits for the label
                res = res << 24; // 24 bits to get to 32 bits
                return res;
            }
        }
    }
}

pub enum Parameter {
    Register(Register),
    Variable(u16),
    Constant(u12),
}

impl Parameter {
    pub fn from_str(s: &str, variable_names: &mut AddressNames) -> Self {
        // check if the string is a register
        match s.parse::<Register>() {
            Ok(_) => return Parameter::Register(s.parse::<Register>().unwrap()),
            Err(_) => (),
        }
        // check if the string is a constant
        match s.parse::<i32>() {
            Ok(_) => {
                return Parameter::Constant(s.parse::<i32>().unwrap() as u12);
            }
            Err(_) => (),
        }
        // else it's a variable
        return Parameter::Variable(Variable::from_str(s, variable_names).into());
    }
}

impl From<u12> for Parameter {
    fn from(i: u12) -> Self {
        let param_type = i >> 10;
        match param_type {
            0b00 => {
                let register = ((i >> 8) & 0b11) as u2;
                return Parameter::Register(Register::from(register));
            }
            0b01 => {
                let variable = (i >> 7) & 0b111;
                return Parameter::Variable(variable as u16);
            }
            0b10 => {
                let constant = i & 0b1111111111;
                return Parameter::Constant(constant);
            }
            _ => panic!("Invalid parameter"),
        }
    }
}

impl Into<u12> for Parameter {
    fn into(self) -> u12 {
        match self {
            Parameter::Register(r) => {
                let mut res: u32 = 0b00; // 2 bits for the parameter type
                res = res << 2;
                res = res | r as u32; // 2 bits for the register
                res = res << 8; // shift left 8 bits to get to 12 bits
                return res;
            }
            Parameter::Variable(i) => {
                let mut res: u32 = 0b01; // 2 bits for the parameter type
                res = res << 10;
                res = res | (i as u32 & 0b1111111111); // 10 bits for the variable
                return res;
            }
            Parameter::Constant(i) => {
                let mut res: u32 = 0b10; // 2 bits for the parameter type
                res = res << 10;
                res = res | (i & 0b1111111111); // 10 bits for the constant
                return res;
            }
        }
    }
}

pub enum Register {
    T0,
    T1,
    T2,
    T3,
}

// impt from_str for Register
impl str::FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "T0" => Ok(Register::T0),
            "T1" => Ok(Register::T1),
            "T2" => Ok(Register::T2),
            "T3" => Ok(Register::T3),
            _ => Err(()),
        }
    }
}

impl From<u2> for Register {
    fn from(i: u2) -> Self {
        match i {
            0b00 => Register::T0,
            0b01 => Register::T1,
            0b10 => Register::T2,
            0b11 => Register::T3,
            _ => panic!("Invalid register"),
        }
    }
}

//TODO: Change structure of Variable and Label so that we can choose the name of the variable in string format

pub struct AddressNames(Vec<String>);

impl AddressNames {
    pub fn new() -> Self {
        AddressNames(Vec::new())
    }

    fn add(&mut self, s: &str) -> u16 {
        self.0.push(s.to_string());
        if (self.0.len() as u16 - 1) > 0b1111111111 {
            panic!("Too many variables");
        }
        if self.0.iter().filter(|x| *x == s).count() > 1 {
            panic!("Variable already exists");
        }
        // check if variable name is valid
        if s.chars().any(|c| !c.is_alphanumeric()) {
            panic!("Invalid variable name");
        }
        // check if variable name is not a register
        match s.parse::<Register>() {
            Ok(_) => panic!("Variable name cannot be a register"),
            Err(_) => (),
        }
        return self.0.len() as u16 - 1;
    }

    fn contains(&self, s: &str) -> bool {
        self.0.contains(&s.to_string())
    }
}

pub trait Addressable {
    fn new(s: &str, address_names: &mut AddressNames) -> Self;

    fn from_str(s: &str, address_names: &mut AddressNames) -> Self;
}

pub struct Variable {
    pub name: u16,
}

impl Addressable for Variable {
    fn new(s: &str, address_names: &mut AddressNames) -> Self {
        // check that the variable name does not exist
        if address_names.contains(s) {
            panic!("Address already exists");
        }
        return Variable {
            name: address_names.add(s),
        };
    }

    fn from_str(s: &str, address_names: &mut AddressNames) -> Self {
        {
            // check that the variable name exists
            if !address_names.contains(s) {
                panic!("Address does not exist");
            }
            return Variable {
                name: address_names.0.iter().position(|x| x == s).unwrap() as u16,
            };
        }
    }
}

impl From<u16> for Variable {
    fn from(i: u16) -> Self {
        Variable { name: i as u16 }
    }
}

impl Into<u16> for Variable {
    fn into(self) -> u16 {
        self.name
    }
}

pub struct Label {
    pub name: u16,
}

impl Addressable for Label {
    fn new(s: &str, label_names: &mut AddressNames) -> Self {
        // check that the label name does not exist
        if label_names.contains(s) {
            panic!("Label already exists");
        }
        return Label {
            name: label_names.add(s),
        };
    }

    fn from_str(s: &str, label_names: &mut AddressNames) -> Self {
        {
            // check that the label name exists
            if !label_names.contains(s) {
                return Label::new(s, label_names);
            }
            return Label {
                name: label_names.0.iter().position(|x| x == s).unwrap() as u16,
            };
        }
    }
}

impl From<u3> for Label {
    fn from(i: u3) -> Self {
        Label {
            name: (i & 0b111) as u16,
        }
    }
}

impl Into<u3> for Label {
    fn into(self) -> u3 {
        self.name as u3
    }
}

#[derive(PartialEq)]
pub enum Extension {
    BW,
    BIN,
}

impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Extension::BW => write!(f, "bw"),
            Extension::BIN => write!(f, "bin"),
        }
    }
}
