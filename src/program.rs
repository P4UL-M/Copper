use crate::bwfile::{BWFile, LineCategory, LineType};
use crate::enums::{AddressNames, Instruction, Label, Parameter, Register, Variable};
use indexmap::IndexMap;
use std::collections::HashMap;

// macro to get the value of a parameter
macro_rules! get_parameter {
    ($parameter:ident,$self:ident, $value:expr) => {
        match $parameter {
            Parameter::Variable(variable) => {
                $value = $self.get_variable(*variable);
            }
            Parameter::Constant(constant) => {
                $value = *constant;
            }
            Parameter::Register(register) => {
                $value = $self.get_register(*register);
            }
        }
    };
}

pub struct Program {
    pub instructions: Vec<Instruction>,
    registers: HashMap<Register, u32>,
    variable_names: AddressNames,
    label_names: AddressNames,
    stack: Vec<u32>,
    memory: IndexMap<Variable, u32>,
    counter: usize,
    verbose: bool,
}

impl Program {
    pub fn new() -> Program {
        Program {
            instructions: Vec::new(),
            registers: HashMap::new(),
            variable_names: AddressNames::new(),
            label_names: AddressNames::new(),
            stack: Vec::new(),
            memory: IndexMap::new(),
            counter: 0,
            verbose: std::env::var("RUST_LOG").is_ok(),
        }
    }

    pub fn run(&mut self) {
        while self.counter < self.instructions.len() {
            // let t1 = std::time::Instant::now();
            self.execute_instruction(self.counter);
            self.counter += 1;
        }
        println!("Registers: {:?}", self.registers);
        println!("Memory: {:?}", self.memory);
    }

    pub fn run_debug(&mut self) {
        println!("Starting debug mode\n");
        while self.counter < self.instructions.len() {
            println!("Instruction: {:?}", self.instructions[self.counter]);
            println!("Registers: {:?}", self.registers);
            println!("Memory: {:?}", self.memory);
            println!("Stack: {:?}", self.stack);
            println!("Counter: {}", self.counter);
            println!("Press enter to continue");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            self.execute_instruction(self.counter);
            self.counter += 1;
        }
        println!("Registers: {:?}", self.registers);
        println!("Memory: {:?}", self.memory);
    }

    pub fn load(&mut self, file: BWFile) {
        // read the file
        let buffer: Vec<LineType> = file.read();

        // parse the file
        let mut current_category: LineCategory = LineCategory::NONE;
        for line in buffer {
            if line.is_category() {
                current_category = line.get_category();
                continue;
            }
            if current_category == LineCategory::NONE {
                panic!("Invalid category");
            }
            if line.is_empty() {
                // skip empty lines
                continue;
            }
            if line.is_comment() {
                // skip comments
                continue;
            }
            let instruction: Instruction = line.translate(
                &current_category,
                &mut self.variable_names,
                &mut self.label_names,
            );
            self.add_instruction(instruction);
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::VARIABLE(variable, value) => {
                self.memory.insert(variable, value);
            }
            _ => {
                self.instructions.push(instruction);
            }
        }
    }

    pub fn get_variable(&self, name: Variable) -> u32 {
        if !self.memory.contains_key(&name) {
            if self.verbose {
                println!("Warning: Accessing uninitialized address {:?}!", name);
            }
            return 0;
        }
        *self.memory.get(&name).unwrap()
    }

    pub fn set_variable(&mut self, name: Variable, value: u32) {
        self.memory.insert(name, value);
    }

    pub fn get_register(&self, register: Register) -> u32 {
        *self.registers.get(&register).unwrap()
    }

    pub fn set_register(&mut self, register: Register, value: u32) {
        self.registers.insert(register, value);
    }

    pub fn find_label(&self, label: &Label) -> usize {
        // find the label in instructions
        let mut index: usize = 0;
        for instruction in &self.instructions {
            if let Instruction::LABEL(l) = instruction {
                if l == label {
                    return index;
                }
            }
            index += 1;
        }
        panic!("Label {:?} not found", label);
    }

    pub fn execute_instruction(&mut self, index: usize) {
        match &self.instructions[index] {
            Instruction::LDA(register, parameter) => {
                let value: u32;
                get_parameter!(parameter, self, value);
                self.set_register(*register, value);
            }
            Instruction::STR(variable, parameter) => {
                let value: u32;
                get_parameter!(parameter, self, value);
                self.set_variable(*variable, value);
            }
            Instruction::PUSH(parameter) => {
                let value: u32;
                get_parameter!(parameter, self, value);
                self.stack.push(value);
            }
            Instruction::POP(register) => {
                let value: Option<u32> = self.stack.pop();
                if value.is_none() {
                    panic!("Stack is empty");
                }
                self.set_register(*register, value.unwrap());
            }
            Instruction::AND(register, parameter) => {
                let value: u32;
                get_parameter!(parameter, self, value);
                let result: u32 = self.get_register(*register) & value;
                self.set_register(*register, result);
            }
            Instruction::OR(register, parameter) => {
                let value: u32;
                get_parameter!(parameter, self, value);
                let result: u32 = self.get_register(*register) | value;
                self.set_register(*register, result);
            }
            Instruction::ADD(register, parameter) => {
                let value: u32;
                get_parameter!(parameter, self, value);
                let result: u32 = self.get_register(*register) + value;
                self.set_register(*register, result);
            }
            Instruction::SUB(register, parameter) => {
                let value: u32;
                get_parameter!(parameter, self, value);
                let result: u32 = self.get_register(*register) - value;
                self.set_register(*register, result);
            }
            Instruction::MUL(register, parameter) => {
                let value: u32;
                get_parameter!(parameter, self, value);
                let result: u32 = self.get_register(*register) * value;
                self.set_register(*register, result);
            }
            Instruction::DIV(register, parameter) => {
                let value: u32;
                get_parameter!(parameter, self, value);
                let result: u32 = self.get_register(*register) / value;
                self.set_register(*register, result);
            }
            Instruction::MOD(register, parameter) => {
                let value: u32;
                get_parameter!(parameter, self, value);
                let result: u32 = self.get_register(*register) % value;
                self.set_register(*register, result);
            }
            Instruction::INC(register) => {
                let result: u32 = self.get_register(*register) + 1;
                self.set_register(*register, result);
            }
            Instruction::DEC(register) => {
                let result: u32 = self.get_register(*register) - 1;
                self.set_register(*register, result);
            }
            Instruction::BEQ(parameter_1, parameter_2, label) => {
                let value_1: u32;
                let value_2: u32;
                get_parameter!(parameter_1, self, value_1);
                get_parameter!(parameter_2, self, value_2);
                if value_1 == value_2 {
                    self.counter = self.find_label(label);
                }
            }
            Instruction::BNE(parameter_1, parameter_2, label) => {
                let value_1: u32;
                let value_2: u32;
                get_parameter!(parameter_1, self, value_1);
                get_parameter!(parameter_2, self, value_2);
                if value_1 != value_2 {
                    self.counter = self.find_label(label);
                }
            }
            Instruction::BBG(parameter_1, parameter_2, label) => {
                let value_1: u32;
                let value_2: u32;
                get_parameter!(parameter_1, self, value_1);
                get_parameter!(parameter_2, self, value_2);
                if value_1 > value_2 {
                    self.counter = self.find_label(label);
                }
            }
            Instruction::BSM(parameter_1, parameter_2, label) => {
                let value_1: u32;
                let value_2: u32;
                get_parameter!(parameter_1, self, value_1);
                get_parameter!(parameter_2, self, value_2);
                if value_1 < value_2 {
                    self.counter = self.find_label(label);
                }
            }
            Instruction::SRL(register, offset) => {
                let value: u32 = self.get_register(*register);
                let result: u32 = value << offset;
                self.set_register(*register, result);
            }
            Instruction::SRR(register, offset) => {
                let value: u32 = self.get_register(*register);
                let result: u32 = value >> offset;
                self.set_register(*register, result);
            }
            Instruction::JMP(label) => {
                self.counter = self.find_label(label);
            }
            Instruction::LABEL(_) => {}
            Instruction::HLT => {
                self.counter = self.instructions.len();
            }
            _ => {
                panic!("Instruction not implemented");
            }
        }
    }
}
