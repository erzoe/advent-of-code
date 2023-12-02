use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

const AVAILABLE_RED: u8 = 12;
const AVAILABLE_GREEN: u8 = 13;
const AVAILABLE_BLUE: u8 = 14;

struct Subset {
    red: u8,
    green: u8,
    blue: u8,
}

struct Game {
    id: u8,
    sets: Vec<Subset>,
}

fn main() {
    let file = File::open("../../input").unwrap();
    let reader = BufReader::new(file);
    let mut result: u32 = 0;
    for line in reader.lines() {
        let ln = line.unwrap();
        let game = Game::new(&ln);
        if game.is_possible() {
            result += game.id as u32;
        }
    }

    println!("result: {result}");
}

impl Game {
    fn new(ln: &str) -> Game {
        let (name, sets) = ln.split_once(": ").expect("missing colon in line");

        let name = name.strip_prefix("Game ").expect("line does not start with \"Game \"");
        let id: u8 = name.parse().expect("failed to parse game id");

        Game{id, sets: Self::parse_sets(sets)}
    }

    fn parse_sets(sets: &str) -> Vec<Subset> {
        let mut out = Vec::<Subset>::new();
        for subset in sets.split("; ") {
            out.push(Self::parse_subset(subset));
        }
        out
    }

    fn parse_subset(subset: &str) -> Subset {
        let mut out = Subset{red: 0, green: 0, blue: 0};

        for balls in subset.split(", ") {
            let (number, color) = balls.split_once(' ').expect("failed to split balls into number and color");
            let number = number.parse::<u8>().expect("failed to parse number of balls");
            match color {
                "red" => out.red = number,
                "green" => out.green = number,
                "blue" => out.blue = number,
                _ => panic!("unknown color {color}"),
            }
        }

        out
    }

    fn get_max_red(&self) -> u8 {
        self.sets.iter().map(|s| s.red).max().unwrap()
    }
    fn get_max_green(&self) -> u8 {
        self.sets.iter().map(|s| s.green).max().unwrap()
    }
    fn get_max_blue(&self) -> u8 {
        self.sets.iter().map(|s| s.blue).max().unwrap()
    }

    fn is_possible(&self) -> bool {
        if self.get_max_red() > AVAILABLE_RED {
            return false;
        }
        if self.get_max_green() > AVAILABLE_GREEN {
            return false;
        }
        if self.get_max_blue() > AVAILABLE_BLUE {
            return false;
        }
        true
    }
}
