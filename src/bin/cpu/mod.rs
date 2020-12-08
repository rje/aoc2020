use parsing::day08::{Instruction, Operation};

pub enum ExitCondition {
    Halt(i32),
    Loop(i32),
}

pub fn run_program(program: &Vec<Instruction>) -> ExitCondition {
    let mut acc = 0;
    let mut visited: Vec<bool> = vec![false; program.len()];
    let mut pc: i32 = 0;
    while visited[pc as usize] == false {
        visited[pc as usize] = true;
        let instr = &program[pc as usize];
        match instr.op {
            Operation::Acc => {
                acc += instr.val;
                pc += 1;
            }
            Operation::Jmp => {
                pc += instr.val;
            }
            _ => {
                pc += 1;
            }
        }
        if pc >= program.len() as i32 {
            return ExitCondition::Halt(acc);
        }
    }
    return ExitCondition::Loop(acc);
}
