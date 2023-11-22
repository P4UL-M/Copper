use crate::enums::{Instruction, Register};
use std::collections::HashMap;

pub struct Program {
    instructions: Vec<Instruction>,
    registers: HashMap<Register, i32>,
    stack: Vec<i32>,
    memory: Vec<i32>,
    counter: usize,
}

impl Program {
    pub fn new() -> Program {
        Program {
            instructions: Vec::new(),
            registers: HashMap::new(),
            stack: Vec::new(),
            memory: Vec::new(),
            counter: 0,
        }
    }

    pub fn run(&mut self) {}
}
