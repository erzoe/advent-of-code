// running this code takes three and a half minutes


use std::ops::{Add, AddAssign};

use regex::Regex;
use once_cell::sync::Lazy;

static RE_DIG_INSTRUCTION: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<direction>[LRUD]) *(?<distance>[0-9]+) *\(#(?<red>[0-9A-Fa-f]{2})(?<green>[0-9A-Fa-f]{2})(?<blue>[0-9A-Fa-f]{2})\)").unwrap());

type CorType = i32;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Direction {
    Left, Right, Up, Down
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum RelativeDirection {
    Left, Right, Straight
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

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct Cor {
    row: CorType,
    col: CorType,
}


fn main() {
    let instructions = std::fs::read_to_string("../../input").expect("input file missing").lines().map(DigInstruction::parse).collect::<Vec<_>>();
    let mut cor = Cor::origin();
    let mut border = Vec::new();
    for instruction in instructions {
        for _ in 0..instruction.distance {
            cor += instruction.direction;
            border.push(cor);
        }
    }
    //print_cors(&border);
    //println!();

    let filled = fill(&border);
    //print_cors(&filled);
    //println!();

    println!("volume: {}m³", filled.len())
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

fn fill(cors: &[Cor]) -> Vec<Cor> {
    let directions = cors.iter().zip(cors.iter().skip(1)).map(|(this, last)| to_direction(last, this)).collect::<Vec<_>>();
    let mut relative_directions = directions.iter().zip(directions.iter().skip(1)).map(|(this, last)| to_relative_direction(last, this)).collect::<Vec<_>>();
    relative_directions.insert(0, RelativeDirection::Straight);
    let inside = if relative_directions.iter().filter(|d| **d==RelativeDirection::Left).count() > relative_directions.iter().filter(|d| **d==RelativeDirection::Right).count() {
        RelativeDirection::Left
    } else {
        RelativeDirection::Right
    };

    let mut out = Vec::new();
    for ((cor, direction), relative_direction) in cors.iter().zip(directions).zip(relative_directions) {
        let cor = *cor;
        out.push(cor);
        for rd in if relative_direction == inside {
            vec![]
        } else if relative_direction == RelativeDirection::Straight {
            vec![inside]
        } else {
            vec![inside, RelativeDirection::Straight]
        } {
            let ad = direction.turn(rd);
            let mut c = cor + ad;
            while !cors.contains(&c) {
                if !out.contains(&c) {
                    out.push(c);
                }
                c += ad;
            }
        }
    }
    out.push(cors[cors.len()-1]);
    out
}

fn to_direction(last: &Cor, this: &Cor) -> Direction {
    if this == last {
        panic!("can't get direction between two identical coordinates");
    }
    if this.col == last.col {
        if this.row > last.row {
            Direction::Down
        } else {
            Direction::Up
        }
    } else if this.row == last.row {
        if this.col > last.col {
            Direction::Right
        } else {
            Direction::Left
        }
    } else {
        panic!("is not a straight direction");
    }
}

fn to_relative_direction(last: &Direction, this: &Direction) -> RelativeDirection {
    match (last, this) {
        (Direction::Left, Direction::Left) => RelativeDirection::Straight,
        (Direction::Left, Direction::Up) => RelativeDirection::Right,
        (Direction::Left, Direction::Down) => RelativeDirection::Left,
        (Direction::Left, Direction::Right) => panic!("180° change not possible"),

        (Direction::Right, Direction::Right) => RelativeDirection::Straight,
        (Direction::Right, Direction::Down) => RelativeDirection::Right,
        (Direction::Right, Direction::Up) => RelativeDirection::Left,
        (Direction::Right, Direction::Left) => panic!("180° change not possible"),

        (Direction::Up, Direction::Up) => RelativeDirection::Straight,
        (Direction::Up, Direction::Right) => RelativeDirection::Right,
        (Direction::Up, Direction::Left) => RelativeDirection::Left,
        (Direction::Up, Direction::Down) => panic!("180° change not possible"),

        (Direction::Down, Direction::Down) => RelativeDirection::Straight,
        (Direction::Down, Direction::Left) => RelativeDirection::Right,
        (Direction::Down, Direction::Right) => RelativeDirection::Left,
        (Direction::Down, Direction::Up) => panic!("180° change not possible"),
    }
}

//fn fill(cors: &[Cor]) -> Vec<Cor> {
//    let mut out = Vec::new();
//    let tl = get_top_left(cors);
//    let br = get_bottom_right(cors);
//
//    #[derive(PartialEq, Eq)]
//    enum State { Searching, FoundStart, NotFilling, Filling }
//    for row in tl.row..br.row {
//        let mut state = State::Searching;
//        for col in tl.col..br.col {
//            let cor = Cor { row, col };
//            if cors.contains(&cor) {
//                out.push(cor);
//                match state {
//                    State::Searching => print!("S"),
//                    State::FoundStart => print!("I"),
//                    State::NotFilling => print!("N"),
//                    State::Filling => print!("F"),
//                }
//                state = match state {
//                    State::Searching => State::FoundStart,
//                    State::FoundStart => State::NotFilling,
//                    State::NotFilling => State::NotFilling,
//                    State::Filling => State::Searching,
//                };
//            } else {
//                state = match state {
//                    State::Searching => State::Searching,
//                    State::FoundStart => State::Filling,
//                    State::NotFilling => State::Searching,
//                    State::Filling => State::Filling,
//                };
//                if state == State::Filling {
//                    out.push(cor);
//                    print!("+");
//                } else {
//                    print!(".");
//                }
//            }
//                match state {
//                    State::Searching => print!("S "),
//                    State::FoundStart => print!("I "),
//                    State::NotFilling => print!("N "),
//                    State::Filling => print!("F "),
//                }
//        }
//        println!();
//    }
//    out
//}

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

    fn turn(&self, relative_direction: RelativeDirection) -> Self {
        match (self, relative_direction) {
            (Direction::Left, RelativeDirection::Straight) => Direction::Left,
            (Direction::Left, RelativeDirection::Left) => Direction::Down,
            (Direction::Left, RelativeDirection::Right) => Direction::Up,
            (Direction::Right, RelativeDirection::Straight) => Direction::Right,
            (Direction::Right, RelativeDirection::Left) => Direction::Up,
            (Direction::Right, RelativeDirection::Right) => Direction::Down,
            (Direction::Down, RelativeDirection::Straight) => Direction::Down,
            (Direction::Down, RelativeDirection::Left) => Direction::Right,
            (Direction::Down, RelativeDirection::Right) => Direction::Left,
            (Direction::Up, RelativeDirection::Straight) => Direction::Up,
            (Direction::Up, RelativeDirection::Left) => Direction::Left,
            (Direction::Up, RelativeDirection::Right) => Direction::Right,
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

impl Add<Direction> for Cor {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self + DigInstruction { distance: 1, direction: rhs, color: Color::black() }
    }
}

impl AddAssign<Direction> for Cor {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

impl Color {
    fn black() -> Self {
        Self { red: 0, green: 0, blue: 0 }
    }
}
