use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


struct Card {
    name: String,
    winning_numbers: Vec<u8>,
    drawn_numbers: Vec<u8>,
}

fn main() {
    let filename = "../../exp2";
    let file = File::open(filename).unwrap_or_else(|_| panic!("input file '{filename}' does not exist"));
    let reader = BufReader::new(file);
    let mut result = 0;
    for ln in reader.lines() {
        let card = Card::parse(&ln.unwrap());
        let points = card.get_points();
        println!("{card} => {points}");
        result += points;
    }
    println!("sum of winning points: {result}");
}

impl Card {
    fn parse(ln: &str) -> Self {
        let (name, numbers) = ln.split_once(": ").expect("missing separator ': '");
        let (winning_numbers, drawn_numbers) = numbers.split_once(" | ").expect("missing separator ' | '");
        Self {
            name: name.to_string(),
            winning_numbers: Self::parse_numbers(winning_numbers),
            drawn_numbers: Self::parse_numbers(drawn_numbers),
        }
    }
    fn parse_numbers(numbers: &str) -> Vec<u8> {
        numbers.trim().replace("  ", " ").split(' ').map(|n| n.parse().expect("failed to parse number")).collect()
    }

    fn get_points(&self) -> u32 {
        let mut count: u8 = 0;
        for n in &self.drawn_numbers {
            if self.winning_numbers.contains(n) {
                count += 1;
            }
        }
        (1 << count) >> 1
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?} | {:?}", self.name, self.winning_numbers, self.drawn_numbers)
    }
}
