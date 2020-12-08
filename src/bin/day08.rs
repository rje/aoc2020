mod cpu;

use cpu::{run_program, ExitCondition};
use parsing::day08::{self, Instruction, Operation};

fn main() {
    let data = day08::parse("data/day08/problem_1.txt");
    println!("Data len: {}", data.len());

    solve_problem_1(&data);
    solve_problem_2(&data);
}

fn solve_problem_1(program: &Vec<Instruction>) {
    println!("Solving problem 1");
    match cpu::run_program(program) {
        ExitCondition::Loop(val) => {
            println!("Acc final: {}", val);
        }
        _ => {}
    }
}

fn solve_problem_2(program: &Vec<Instruction>) {
    println!("Solving problem 2");
    for i in 0..program.len() {
        let result = run_flipped_instr(program.clone(), i);
        match result {
            ExitCondition::Halt(val) => {
                println!("Acc val on halt: {}", val);
                return;
            }
            _ => {}
        }
    }
}

fn run_flipped_instr(mut program: Vec<Instruction>, instr_idx: usize) -> ExitCondition {
    let instr = &mut program[instr_idx];
    instr.op = flip_op(instr.op);
    run_program(&program)
}

fn flip_op(to_flip: Operation) -> Operation {
    match to_flip {
        Operation::Nop => Operation::Jmp,
        Operation::Jmp => Operation::Nop,
        _ => to_flip,
    }
}
