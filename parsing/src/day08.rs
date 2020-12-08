use crate::util::get_lines;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Instruction {
    pub op: Operation,
    pub val: i32,
}

impl Instruction {
    fn new(line: &String) -> Instruction {
        let split: Vec<&str> = line.split_whitespace().collect();
        let op = match split[0] {
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            _ => Operation::Nop,
        };
        let val = split[1].parse::<i32>().unwrap();

        Instruction { op, val }
    }
}

pub fn parse(path: &str) -> Vec<Instruction> {
    get_lines(path)
        .map(|line| Instruction::new(&line))
        .collect()
}
