use std::io::{Read, Write};
use crate::error::BrainfuckError;
use crate::parser::Instruction;

const MEMORY_SIZE: usize = 30000;

pub struct Interpreter {
    memory: [u8; MEMORY_SIZE],
    pointer: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            pointer: 0,
        }
    }

    pub fn run(&mut self, instructions: &[Instruction]) -> Result<(), BrainfuckError> {
        let mut pc = 0;
        let mut loop_stack = Vec::new();

        while pc < instructions.len() {
            match instructions[pc] {
                Instruction::Increment => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
                }
                Instruction::Decrement => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
                }
                Instruction::MoveRight => {
                    self.pointer += 1;
                    if self.pointer >= MEMORY_SIZE {
                        return Err(BrainfuckError::PointerOutOfBounds);
                    }
                }
                Instruction::MoveLeft => {
                    if self.pointer == 0 {
                        return Err(BrainfuckError::PointerOutOfBounds);
                    }
                    self.pointer -= 1;
                }
                Instruction::Output => {
                    std::io::stdout().write_all(&[self.memory[self.pointer]])?;
                    std::io::stdout().flush()?;
                }
                Instruction::Input => {
                    let mut input = [0];
                    std::io::stdin().read_exact(&mut input)?;
                    self.memory[self.pointer] = input[0];
                }
                Instruction::LoopStart => {
                    if self.memory[self.pointer] == 0 {
                        let mut depth = 1;
                        while depth > 0 {
                            pc += 1;
                            if pc >= instructions.len() {
                                return Err(BrainfuckError::UnmatchedBracket(pc));
                            }
                            match instructions[pc] {
                                Instruction::LoopStart => depth += 1,
                                Instruction::LoopEnd => depth -= 1,
                                _ => {}
                            }
                        }
                    } else {
                        loop_stack.push(pc);
                    }
                }
                Instruction::LoopEnd => {
                    if self.memory[self.pointer] != 0 {
                        pc = loop_stack.last().copied().ok_or(BrainfuckError::UnmatchedBracket(pc))?;
                    } else {
                        loop_stack.pop();
                    }
                }
            }
            pc += 1;
        }

        Ok(())
    }
}
