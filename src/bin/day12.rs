use parsing::day12::{self, ShipInstruction};

fn main() {
    //let data = day12::parse("data/day12/test1.txt");
    let data = day12::parse("data/day12/problem_1.txt");
    solve_problem_1(&data);
    solve_problem_2(&data);
}

#[derive(Debug)]
struct Ship {
    pub x: i32,
    pub y: i32,
    pub angle: i32,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            angle: 0,
        }
    }
    pub fn move_ship(self: &mut Self, instr: &ShipInstruction) {
        match instr {
            ShipInstruction::North(val) => self.y += val,
            ShipInstruction::South(val) => self.y -= val,
            ShipInstruction::East(val) => self.x += val,
            ShipInstruction::West(val) => self.x -= val,
            ShipInstruction::Left(val) => self.update_angle(*val),
            ShipInstruction::Right(val) => self.update_angle(-*val),
            ShipInstruction::Forward(val) => self.move_forward(*val),
        }
    }

    fn update_angle(self: &mut Self, amt: i32) {
        let mut new_angle = self.angle + amt;
        while new_angle < 0 {
            new_angle += 360;
        }
        while new_angle >= 360 {
            new_angle -= 360;
        }
        self.angle = new_angle;
    }

    fn move_forward(self: &mut Self, units: i32) {
        match self.angle {
            0 => self.x += units,
            180 => self.x -= units,
            90 => self.y += units,
            270 => self.y -= units,
            _ => {}
        }
    }
}

fn solve_problem_1(data: &Vec<ShipInstruction>) {
    println!("Solving problem 1");
    let mut ship = Ship::new();
    data.iter().for_each(|instr| ship.move_ship(instr));
    println!("{:?}", ship);
    println!("Dist: {}", ship.x.abs() + ship.y.abs());
}

struct Waypoint {
    pub x: i32,
    pub y: i32,
}

impl Waypoint {
    pub fn update_pos(self: &mut Self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

fn solve_problem_2(data: &Vec<ShipInstruction>) {
    println!("Solving problem 1");
    let mut waypoint = Waypoint { x: 10, y: 1 };
    let mut ship = Ship {
        x: 0,
        y: 0,
        angle: 0,
    };

    data.iter().for_each(|instr| match instr {
        ShipInstruction::North(val) => waypoint.y += val,
        ShipInstruction::South(val) => waypoint.y -= val,
        ShipInstruction::East(val) => waypoint.x += val,
        ShipInstruction::West(val) => waypoint.x -= val,
        ShipInstruction::Left(val) => match val {
            90 => waypoint.update_pos(-waypoint.y, waypoint.x),
            180 => waypoint.update_pos(-waypoint.x, -waypoint.y),
            270 => waypoint.update_pos(waypoint.y, -waypoint.x),
            _ => {}
        },
        ShipInstruction::Right(val) => match val {
            90 => waypoint.update_pos(waypoint.y, -waypoint.x),
            180 => waypoint.update_pos(-waypoint.x, -waypoint.y),
            270 => waypoint.update_pos(-waypoint.y, waypoint.x),
            _ => {}
        },
        ShipInstruction::Forward(val) => {
            ship.x += val * waypoint.x;
            ship.y += val * waypoint.y;
        }
    });

    println!("{:?}", ship);
    println!("Dist: {}", ship.x.abs() + ship.y.abs());
}
