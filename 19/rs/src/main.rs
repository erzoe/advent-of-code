use macros::load;

load!("~/advent-calendar/19/exp");

fn main() {
    let mut result: u32 = 0;
    for part in PARTS {
        if r#in(&part) {
            println!("accepted {:?}", part);
            result += part.x + part.m + part.a + part.s;
        }
    }

    println!("result {}", result);
}
