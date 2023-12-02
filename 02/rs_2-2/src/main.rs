use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

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
        let min = game.get_min_set();
        let minpower = min.get_power();
        println!("Game {0}: {minpower}", game.id);
        result += minpower;
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

    fn get_min_set(&self) -> Subset {
        Subset {
            red: self.get_max_red(),
            green: self.get_max_green(),
            blue: self.get_max_blue(),
        }
    }
}

impl Subset {
    fn get_power(&self) -> u32 {
        self.red as u32 * self.green as u32 * self.blue as u32
    }
}
