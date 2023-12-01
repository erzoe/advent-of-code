use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use regex::{Regex, Captures};

fn main() {
    let pattern_digit = r"(one|two|three|four|five|six|seven|eight|nine|[0-9])";
    let re_first_digit = Regex::new(pattern_digit).unwrap();
    let re_last_digit = Regex::new(&format!(r".*{pattern_digit}.*?$")).unwrap();

    let file = File::open("../../input").unwrap();
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    for line in reader.lines() {
        let ln = line.unwrap();
        let d1 = to_number(re_first_digit.captures(&ln).unwrap());
        let d2 = to_number(re_last_digit.captures(&ln).unwrap());
        sum += (10*d1 + d2) as u32;
        println!("{d1}{d2}");
    }

        println!("sum: {sum}");
}

fn to_number(caps: Captures) -> u8 {
        let digit = caps.get(1).unwrap().as_str();
        match digit {
            "one"   => 1,
            "two"   => 2,
            "three" => 3,
            "four"  => 4,
            "five"  => 5,
            "six"   => 6,
            "seven" => 7,
            "eight" => 8,
            "nine"  => 9,
            _ => digit.parse().unwrap(),
        }
}
