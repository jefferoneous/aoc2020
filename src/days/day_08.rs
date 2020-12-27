use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

use super::DayRunner;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug, PartialEq, Eq)]
enum ParseError {
    UnrecognizedInstruction(String),
    InvalidArgument(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnrecognizedInstruction(instruction) => {
                write!(f, "Unrecognized instruction: {}", instruction)
            }
            Self::InvalidArgument(instruction) => write!(f, "Invalid argument: {}", instruction),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        match parts[0] {
            "nop" => {
                if let Ok(v) = parts[1].parse() {
                    Ok(Instruction::Nop(v))
                } else {
                    Err(ParseError::InvalidArgument(s.to_owned()))
                }
            }
            "acc" => {
                if let Ok(v) = parts[1].parse() {
                    Ok(Instruction::Acc(v))
                } else {
                    Err(ParseError::InvalidArgument(s.to_owned()))
                }
            }
            "jmp" => {
                if let Ok(v) = parts[1].parse() {
                    Ok(Instruction::Jmp(v))
                } else {
                    Err(ParseError::InvalidArgument(s.to_owned()))
                }
            }
            _ => Err(ParseError::UnrecognizedInstruction(s.to_owned())),
        }
    }
}

struct Processor {
    accumulator: i32,
    instruction_pointer: usize,
    instructions: Vec<Instruction>,
    addresses_visited: HashSet<usize>,
}

impl Processor {
    fn new() -> Self {
        Self {
            accumulator: 0,
            instruction_pointer: 0,
            instructions: vec![],
            addresses_visited: HashSet::new(),
        }
    }

    fn load<T: AsRef<str>>(&mut self, raw_code: &[T]) -> Result<(), ParseError> {
        let mut instructions: Vec<Instruction> = vec![];

        for instruction in raw_code {
            instructions.push(instruction.as_ref().parse()?);
        }

        self.instructions = instructions;

        Ok(())
    }

    fn run(&mut self) -> (bool, i32) {
        let mut encountered_infinite_loop = false;

        loop {
            self.addresses_visited.insert(self.instruction_pointer);

            match self.instructions.get(self.instruction_pointer) {
                Some(Instruction::Nop(_)) => self.instruction_pointer += 1,
                Some(Instruction::Acc(inc)) => {
                    self.accumulator += *inc;
                    self.instruction_pointer += 1;
                }
                Some(Instruction::Jmp(delta)) => {
                    let new_ptr = self.instruction_pointer as i32 + *delta;
                    if new_ptr < 0 || new_ptr > self.instructions.len() as i32 {
                        break;
                    } else {
                        self.instruction_pointer = new_ptr as usize;
                    }
                }
                None => break,
            }

            if self.addresses_visited.contains(&self.instruction_pointer) {
                encountered_infinite_loop = true;
                break;
            }
        }

        (encountered_infinite_loop, self.accumulator)
    }
}

pub fn part_one(data: &[String]) {
    let mut processor = Processor::new();
    if let Ok(()) = processor.load(data) {
        let (encountered_loop, result) = processor.run();
        println!(
            "Encountered infinite loop: {}\nAccumulator: {}",
            encountered_loop, result
        );
    } else {
        println!("Loading instructions failed");
    }
}

pub fn part_two(data: &[String]) {
    for index in 0..data.len() {
        let new_data: Vec<String> = data
            .iter()
            .enumerate()
            .map(|(i, line)| {
                if i == index {
                    if line.contains("nop") {
                        line.replace("nop", "jmp")
                    } else if line.contains("jmp") {
                        line.replace("jmp", "nop")
                    } else {
                        line.clone()
                    }
                } else {
                    line.clone()
                }
            })
            .collect();

        let mut processor = Processor::new();
        if let Ok(()) = processor.load(&new_data) {
            let (encountered_loop, result) = processor.run();
            if !encountered_loop {
                println!("Accumulator: {}", result);
            }
        } else {
            println!("Loading instructions failed");
        }
    }
}

pub fn runner(data: Vec<String>) -> DayRunner {
    DayRunner::new(data, Some(part_one), Some(part_two))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_08_parses_nop() -> Result<(), ParseError> {
        let instruction: Instruction = "nop +0".parse()?;

        assert_eq!(instruction, Instruction::Nop(0));

        Ok(())
    }

    #[test]
    fn day_08_parses_acc() -> Result<(), ParseError> {
        let instruction: Instruction = "acc +1".parse()?;

        assert_eq!(instruction, Instruction::Acc(1));

        Ok(())
    }

    #[test]
    fn day_08_parses_jmp() -> Result<(), ParseError> {
        let instruction: Instruction = "jmp +3".parse()?;

        assert_eq!(instruction, Instruction::Jmp(3));

        Ok(())
    }

    fn get_sample_instructions() -> Vec<String> {
        vec![
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        ]
    }

    #[test]
    fn day_08_correct_value_in_accumulator_upon_repeated_instruction() -> Result<(), ParseError> {
        let sample_instructions = get_sample_instructions();

        let mut processor = Processor::new();
        processor.load(&sample_instructions)?;

        assert_eq!(processor.run(), (true, 5));

        Ok(())
    }
}
