use parsing::day14::{self, MaskBit, MemCommand};
use std::collections::HashMap;

fn main() {
    let data = day14::parse("data/day14/problem_1.txt");
    //let data = day14::parse("data/day14/test2.txt");
    solve_problem_1(&data);
    solve_problem_2(&data);
}

fn solve_problem_1(data: &Vec<MemCommand>) {
    println!("Solving problem 1");

    let mut mask = MaskBit::get_blank();
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for command in data {
        match command {
            MemCommand::Mask(val) => mask = val.to_vec(),
            MemCommand::SetValue(addr, data) => {
                memory.insert(*addr, get_masked_value(&mask, *data));
            }
        }
    }

    let result: u64 = memory.values().sum();
    println!("Final result: {}", result);
}

fn get_masked_value(mask: &Vec<MaskBit>, data: u64) -> u64 {
    let mut result = data;

    for idx in 0..mask.len() {
        match mask[idx] {
            MaskBit::One => result |= 1 << idx,
            MaskBit::Zero => result &= !(1 << idx),
            _ => {}
        }
    }
    return result;
}

fn solve_problem_2(data: &Vec<MemCommand>) {
    println!("Solving problem 2");

    let mut mask = MaskBit::get_blank();
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for command in data {
        match command {
            MemCommand::Mask(val) => mask = val.to_vec(),
            MemCommand::SetValue(addr, data) => {
                let addrs = get_addrs_to_write(&mask, *addr);
                addrs.iter().for_each(|to_write| {
                    memory.insert(*to_write, *data);
                });
            }
        }
    }

    let result: u64 = memory.values().sum();
    println!("Final result: {}", result);
}

fn get_addrs_to_write(mask: &Vec<MaskBit>, addr: u64) -> Vec<u64> {
    let mut to_return: Vec<u64> = Vec::new();

    let count = mask.iter().filter(|&bit| *bit == MaskBit::Pass).count();
    let variants = (2 as u64).pow(count as u32);

    for i in 0..variants {
        let mut base = addr;
        let mut pass_count = 0;
        for mask_idx in 0..mask.len() {
            match mask[mask_idx] {
                MaskBit::Zero => {}
                MaskBit::One => base |= 1 << mask_idx,
                MaskBit::Pass => {
                    let is_set = (i & (1 << pass_count)) > 0;
                    pass_count += 1;
                    match is_set {
                        true => base |= 1 << mask_idx,
                        false => base &= !(1 << mask_idx),
                    }
                }
            }
        }
        to_return.push(base);
    }

    return to_return;
}
