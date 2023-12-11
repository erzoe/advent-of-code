#[macro_use]
extern crate macros;

definitions!();

fn main() {
    let mut nodes = START_NODES;
    let mut step: u32 = 0;
    let number_directions = DIRECTIONS.len();
    while !nodes.iter().all(|n| n.is_goal()) {
        let direction = DIRECTIONS[step as usize % number_directions];
        for i in 0..nodes.len() {
            nodes[i] = nodes[i].next(direction);
        }
        step += 1;
    }
    println!("steps: {:?}", step);
}
