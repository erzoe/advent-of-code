use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("../../exp").expect("input file does not exist");
    let reader = BufReader::new(file);
    let mut result = 0;
    for ln in reader.lines() {
        let data = ln.unwrap().split_whitespace().map(|item| item.parse().expect("failed to parse number")).collect::<Vec<u32>>();
        let prediction = predict(&data);
        println!("{:?} {}", data, prediction);
        result += prediction;
    }
    println!("result: {}", result);
}

fn predict(data: &[u32]) -> u32 {
    let mut diffs = Vec::new();
    diffs.push(data.to_owned());
    while !diffs.last().unwrap().iter().all(|item| *item == 0) {
        diffs.push( diffs.last().unwrap().iter().zip(diffs.last().unwrap().iter().skip(1)).map(|(current, next)| next - current).collect() );
    }
    let n = diffs.len();
    let mut last = 0;
    for i in (0..n).rev() {
        let new = diffs[i].last().unwrap() + last;
        last = new;
        diffs[i].push(last);
        println!("    {:?}", diffs[i]);
    }
    *diffs.first().unwrap().last().unwrap()
}
