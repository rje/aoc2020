use crate::parsing_error::ParseError;
use crate::util::get_lines;

pub enum ShipInstruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Forward(i32),
    Left(i32),
    Right(i32),
}

impl ShipInstruction {
    pub fn from_string(line: String) -> Result<ShipInstruction, ParseError> {
        let code = line.chars().nth(0).unwrap();
        let num = line[1..].parse::<i32>().unwrap();
        match code {
            'N' => Ok(ShipInstruction::North(num)),
            'S' => Ok(ShipInstruction::South(num)),
            'E' => Ok(ShipInstruction::East(num)),
            'W' => Ok(ShipInstruction::West(num)),
            'F' => Ok(ShipInstruction::Forward(num)),
            'L' => Ok(ShipInstruction::Left(num)),
            'R' => Ok(ShipInstruction::Right(num)),
            _ => Err(ParseError::new("Error parsing instruction!")),
        }
    }
}

pub fn parse(path: &str) -> Vec<ShipInstruction> {
    get_lines(path)
        .map(|line| ShipInstruction::from_string(line).expect("invalid line"))
        .collect()
}
