use crate::error::BrainfuckError;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Increment,
    Decrement,
    MoveRight,
    MoveLeft,
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

pub fn parse(code: &str) -> Result<Vec<Instruction>, BrainfuckError> {
    let mut instructions = Vec::new();
    let mut bracket_stack = Vec::new();

    for (pos, ch) in code.chars().enumerate() {
        match ch {
            '+' => instructions.push(Instruction::Increment),
            '-' => instructions.push(Instruction::Decrement),
            '>' => instructions.push(Instruction::MoveRight),
            '<' => instructions.push(Instruction::MoveLeft),
            '.' => instructions.push(Instruction::Output),
            ',' => instructions.push(Instruction::Input),
            '[' => {
                instructions.push(Instruction::LoopStart);
                bracket_stack.push(pos);
            }
            ']' => {
                instructions.push(Instruction::LoopEnd);
                if bracket_stack.pop().is_none() {
                    return Err(BrainfuckError::UnmatchedBracket(pos));
                }
            }
            _ => {} // Ignore other characters
        }
    }

    if !bracket_stack.is_empty() {
        return Err(BrainfuckError::UnmatchedBracket(bracket_stack[0]));
    }

    Ok(instructions)
}
