use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

struct Record {
    time: u32,
    distance: u32,
}

struct Records {
    records: Vec<Record>,
}

fn main() {
    let records = Records::read("../../input");
    let mut result: u32 = 1;

    for r in records.records {
        let mut number_of_possibilities_that_would_beat_the_highscore = 0;
        for time_press_button_in_ms in 1..r.time {
            if calc_range_in_mm(r.time, time_press_button_in_ms) > r.distance {
                number_of_possibilities_that_would_beat_the_highscore += 1;
            }
        }
        println!("number_of_possibilities_that_would_beat_the_highscore: {number_of_possibilities_that_would_beat_the_highscore}");
        result *= number_of_possibilities_that_would_beat_the_highscore;
    }
    println!("result: {result}");
}

fn calc_range_in_mm(time_race_in_ms: u32, time_press_button_in_ms: u32) -> u32 {
    let time_driving_in_ms = time_race_in_ms - time_press_button_in_ms;
    let speed_in_mm_per_ms = time_press_button_in_ms;
    speed_in_mm_per_ms * time_driving_in_ms
}

impl Records {
    fn read(filename: &str) -> Self {
        let file = File::open(filename).unwrap_or_else(|_| panic!("input file missing {filename}"));
        let reader = BufReader::new(file);
        Self::parse(reader.lines().map(|line| line.unwrap()).collect())
    }
    fn parse(lines: Vec<String>) -> Self {
        let times = lines[0].strip_prefix("Time: ").expect("unexpected first line").trim();
        let distances = lines[1].strip_prefix("Distance: ").expect("unexpected second line").trim();
        let times: Vec<u32> = times.split_whitespace().map(|n| n.parse().expect("failed to parse number")).collect();
        let distances: Vec<u32> = distances.split_whitespace().map(|n| n.parse().expect("failed to parse number")).collect();

        if times.len() != distances.len() {
            panic!("different number of times and distances given");
        }

        Self { records: (0..times.len()).map(|i| Record{time: times[i], distance: distances[i]}).collect() }
    }
}
