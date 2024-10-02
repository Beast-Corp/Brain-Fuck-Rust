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
    let mut line = 1;
    let mut column = 0;

    for (pos, ch) in code.chars().enumerate() {
        column += 1;
        match ch {
            '+' => instructions.push(Instruction::Increment),
            '-' => instructions.push(Instruction::Decrement),
            '>' => instructions.push(Instruction::MoveRight),
            '<' => instructions.push(Instruction::MoveLeft),
            '.' => instructions.push(Instruction::Output),
            ',' => instructions.push(Instruction::Input),
            '[' => {
                instructions.push(Instruction::LoopStart);
                bracket_stack.push((line, column));
            }
            ']' => {
                instructions.push(Instruction::LoopEnd);
                if bracket_stack.pop().is_none() {
                    return Err(BrainfuckError::UnmatchedBracket { line, column });
                }
            }
            '\n' => {
                line += 1;
                column = 0;
            }
            _ => {} // Ignore other characters
        }
    }

    if let Some((line, column)) = bracket_stack.first() {
        return Err(BrainfuckError::UnmatchedBracket {
            line: *line,
            column: *column,
        });
    }

    Ok(instructions)
}
