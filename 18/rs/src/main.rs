use std::ops::{Add, AddAssign};

use regex::Regex;
use once_cell::sync::Lazy;

static RE_DIG_INSTRUCTION: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<direction>[LRUD]) *(?<distance>[0-9]+) *\(#(?<red>[0-9A-Fa-f]{2})(?<green>[0-9A-Fa-f]{2})(?<blue>[0-9A-Fa-f]{2})\)").unwrap());

type CorType = i8;

enum Direction {
    Left, Right, Up, Down
}

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

struct DigInstruction {
    direction: Direction,
    distance: CorType,
    color: Color,
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct Cor {
    row: CorType,
    col: CorType,
}


fn main() {
    let instructions = std::fs::read_to_string("../../exp").expect("input file missing").lines().map(DigInstruction::parse).collect::<Vec<_>>();
    let mut cor = Cor::origin();
    let mut border = vec![cor];
    for instruction in instructions {
        cor += instruction;
        border.push(cor);
    }
    print_cors(&border);
}

fn get_top_left(cors: &[Cor]) -> Cor {
    Cor {
        row: cors.iter().map(|cor| cor.row).min().unwrap(),
        col: cors.iter().map(|cor| cor.col).min().unwrap(),
    }
}

fn get_bottom_right(cors: &[Cor]) -> Cor {
    Cor {
        row: cors.iter().map(|cor| cor.row).max().unwrap() + 1,
        col: cors.iter().map(|cor| cor.col).max().unwrap() + 1,
    }
}

fn print_cors(cors: &[Cor]) {
    let tl = get_top_left(cors);
    let br = get_bottom_right(cors);
    for row in tl.row..br.row {
        for col in tl.col..br.col {
            if cors.contains(&Cor{row, col}) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

impl DigInstruction {
    fn parse(ln: &str) -> Self {
        let caps = RE_DIG_INSTRUCTION.captures(ln).unwrap_or_else(|| panic!("invalid instruction line '{ln}'"));
        Self {
            direction: Direction::parse(caps.name("direction").unwrap().as_str()),
            distance: caps.name("distance").unwrap().as_str().to_string().parse().unwrap(),
            color: Color {
                red: u8::from_str_radix(caps.name("red").unwrap().as_str(), 16).expect("failed to parse hex number for red"),
                green: u8::from_str_radix(caps.name("green").unwrap().as_str(), 16).expect("failed to parse hex number for green"),
                blue: u8::from_str_radix(caps.name("blue").unwrap().as_str(), 16).expect("failed to parse hex number for blue"),
            },
        }
    }
}

impl Direction {
    fn parse(direction: &str) -> Self {
        match direction {
            "L" => Self::Left,
            "R" => Self::Right,
            "U" => Self::Up,
            "D" => Self::Down,
            _ => panic!("invalid direction '{direction}'"),
        }
    }
}

impl Cor {
    fn origin() -> Self {
        Self { row: 0, col: 0 }
    }
}

impl Add<DigInstruction> for Cor {
    type Output = Self;

    fn add(self, rhs: DigInstruction) -> Self::Output {
        match rhs.direction {
            Direction::Left => Self { col: self.col - rhs.distance, ..self },
            Direction::Right => Self { col: self.col + rhs.distance, ..self },
            Direction::Up => Self { row: self.row - rhs.distance, ..self },
            Direction::Down => Self { row: self.row + rhs.distance, ..self },
        }
    }
}

impl AddAssign<DigInstruction> for Cor {
    fn add_assign(&mut self, rhs: DigInstruction) {
        *self = *self + rhs;
    }
}
