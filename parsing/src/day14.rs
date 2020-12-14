use crate::util::get_lines;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MaskBit {
    Zero,
    One,
    Pass,
}

impl MaskBit {
    pub fn get_blank() -> Vec<MaskBit> {
        vec![MaskBit::Pass; 36]
    }
}

#[derive(Debug, Clone)]
pub enum MemCommand {
    Mask(Vec<MaskBit>),
    SetValue(u64, u64),
}

impl MemCommand {
    pub fn from_string(line: &String) -> MemCommand {
        if line.starts_with("mem") {
            return MemCommand::mem_from_string(&line);
        } else {
            return MemCommand::mask_from_string(&line);
        }
    }

    pub fn mem_from_string(line: &String) -> MemCommand {
        lazy_static! {
            static ref MEM_REGEX: Regex =
                Regex::new(r"mem\[(\d+)\] = (\d+)").expect("Error creating regex");
        }
        let caps = MEM_REGEX.captures(line).expect("Error unwrapping captures");

        let addr = caps[1].parse::<u64>().expect("Error getting mem addr");
        let val = caps[2].parse::<u64>().expect("Error getting mem val");

        MemCommand::SetValue(addr, val)
    }

    pub fn mask_from_string(line: &String) -> MemCommand {
        lazy_static! {
            static ref MASK_REGEX: Regex =
                Regex::new(r"mask = ([X01]{36})").expect("Error creating regex");
        }
        let caps = MASK_REGEX
            .captures(line)
            .expect("Error unwrapping captures");

        let vals: Vec<MaskBit> = caps[1]
            .chars()
            .rev()
            .map(|c| match c {
                '0' => MaskBit::Zero,
                '1' => MaskBit::One,
                _ => MaskBit::Pass,
            })
            .collect();

        MemCommand::Mask(vals)
    }
}

pub fn parse(path: &str) -> Vec<MemCommand> {
    get_lines(path)
        .map(|line| MemCommand::from_string(&line))
        .collect()
}
