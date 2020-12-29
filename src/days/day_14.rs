use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

use super::DayRunner;

lazy_static! {
    static ref PARSE_MEM_REGEX: Regex =
        Regex::new(r"mem\[(?P<address>\d+)\] *= *(?P<value>\d+)").unwrap();
}

pub fn runner(data: Vec<String>) -> DayRunner {
    DayRunner::new(data, Some(part_one), Some(part_two))
}

fn part_one(data: &[String]) {
    let mut decoder = Decoder::new();

    if let Err(e) = decoder.load(&data) {
        println!("An error occurred while loading instructions: {}", e);
        return;
    }

    decoder.run();

    println!("Sum: {}", decoder.calculate_sum());
}

fn part_two(data: &[String]) {
    todo!("do something and print the result");
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Mask(String),
    Mem(usize, usize),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            return Ok(Instruction::Mask(
                s.chars()
                    .filter(|&c| c == 'X' || c == '0' || c == '1')
                    .collect(),
            ));
        } else if s.starts_with("mem") {
            if let Some(caps) = PARSE_MEM_REGEX.captures(s) {
                let address = &caps["address"].parse().ok();
                let value = &caps["value"].parse().ok();

                if address.is_none() {
                    return Err(format!("Invalid address: {}", &caps["address"]));
                }

                if value.is_none() {
                    return Err(format!("Invalid value: {}", &caps["value"]));
                }

                return Ok(Instruction::Mem(address.unwrap(), value.unwrap()));
            }
        }

        Err(format!("Unrecognized instruction: {}", s))
    }
}

struct Decoder {
    memory: HashMap<usize, usize>,
    instructions: Vec<Instruction>,
    mask: String,
}

impl Decoder {
    fn new() -> Self {
        Self {
            memory: HashMap::new(),
            instructions: vec![],
            mask: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".into(),
        }
    }

    fn load(&mut self, instructions: &[String]) -> Result<(), String> {
        for s in instructions {
            let instruction = s.parse()?;
            self.instructions.push(instruction);
        }

        Ok(())
    }

    fn run(&mut self) {
        for instruction in &self.instructions {
            match instruction {
                Instruction::Mask(mask) => self.mask = mask.clone(),
                Instruction::Mem(address, value) => {
                    let masked_value = self.apply_mask(*value);
                    self.memory.insert(*address, masked_value);
                }
            }
        }
    }

    fn apply_mask(&self, value: usize) -> usize {
        let s: String = format!("{:036b}", value)
            .chars()
            .zip(self.mask.chars())
            .map(|(vc, mc)| match mc {
                'X' => vc,
                _ => mc,
            })
            .collect();

        usize::from_str_radix(&s, 2).unwrap()
    }

    fn calculate_sum(&self) -> usize {
        self.memory.values().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_14_parses_instructions() -> Result<(), String> {
        let raw_instructions = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string(),
        ];
        let mut decoder = Decoder::new();

        decoder.load(&raw_instructions)?;

        assert_eq!(
            Instruction::Mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()),
            decoder.instructions[0]
        );
        assert_eq!(Instruction::Mem(8, 11), decoder.instructions[1]);
        assert_eq!(Instruction::Mem(7, 101), decoder.instructions[2]);
        assert_eq!(Instruction::Mem(8, 0), decoder.instructions[3]);

        Ok(())
    }

    #[test]
    fn day_14_executes_mask_instruction() -> Result<(), String> {
        let raw_instructions = vec!["mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()];
        let mut decoder = Decoder::new();

        decoder.load(&raw_instructions)?;
        decoder.run();

        assert_eq!(
            "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            decoder.mask
        );

        Ok(())
    }

    #[test]
    fn day_14_executes_mem_instruction() -> Result<(), String> {
        let raw_instructions = vec!["mem[8] = 11".to_string()];
        let mut decoder = Decoder::new();

        decoder.load(&raw_instructions)?;
        decoder.run();

        assert_eq!(11, decoder.memory[&8]);

        Ok(())
    }

    #[test]
    fn day_14_executes_mem_instruction_with_applied_mask() -> Result<(), String> {
        let raw_instructions = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
        ];
        let mut decoder = Decoder::new();

        decoder.load(&raw_instructions)?;
        decoder.run();

        assert_eq!(73, decoder.memory[&8]);
        assert_eq!(101, decoder.memory[&7]);

        Ok(())
    }

    #[test]
    fn day_14_calculates_sum_of_memory_values() -> Result<(), String> {
        let raw_instructions = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string(),
        ];
        let mut decoder = Decoder::new();

        decoder.load(&raw_instructions)?;
        decoder.run();

        assert_eq!(165, decoder.calculate_sum());

        Ok(())
    }
}
