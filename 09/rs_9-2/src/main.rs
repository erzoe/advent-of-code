use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::ops::{Add,Sub};
use std::fmt::Debug;

fn main() {
    let file = File::open("../../input").expect("input file does not exist");
    let reader = BufReader::new(file);
    let mut result: i32 = 0;
    for ln in reader.lines() {
        let data = ln.unwrap().split_whitespace().map(|item| item.parse().expect("failed to parse number")).collect::<Vec<i32>>();
        let prediction = predict_past(&data);
        println!("{} {:?}", prediction, data);
        result += prediction;
    }
    println!("result: {}", result);
}

fn predict_past<T>(data: &[T]) -> T
where
    T: Add<Output=T> + Sub<Output=T> + PartialEq + Default + Copy + Debug,
{
    let mut diffs = Vec::new();
    diffs.push(data.to_owned());
    let zero = Default::default();
    while !diffs.last().unwrap().iter().all(|item| *item == zero) {
        diffs.push( diffs.last().unwrap().iter().zip(diffs.last().unwrap().iter().skip(1)).map(|(current, next)| *next - *current).collect() );
    }
    let n = diffs.len();
    let mut last: T = zero;
    for i in (0..n).rev() {
        let new = *diffs[i].first().unwrap() - last;
        last = new;
        diffs[i].insert(0, last);
        //println!("    {:?}", diffs[i]);
    }
    *diffs.first().unwrap().first().unwrap()
}
