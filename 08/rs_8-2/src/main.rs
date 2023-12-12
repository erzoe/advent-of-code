#[macro_use]
extern crate macros;

definitions!();

//Warning: don't try to run this, it takes way too long and a u32 is not even big enough.
// I guess I could try to loop over every node individually until I find a repetition in (index_direction, node)
// and then only check the steps where the longest period has it's goals.

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
