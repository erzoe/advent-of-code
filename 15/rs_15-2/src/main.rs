fn main() {
    let mut result: u32 = 0;
    for cmd in std::fs::read_to_string("../../input").expect("input file not found").trim_end().split(',') {
        let hash_value = hash(cmd);
        result += hash_value as u32;
        println!("'{cmd}': {hash_value}");
    }
    println!("result: {result}");
}

fn hash(ln: &str) -> u8 {
    let mut out: u8 = 0;
    for c in ln.chars() {
        let mut tmp: u32 = out as u32 + c as u32;
        tmp *= 17;
        out = (tmp & 0xFF) as u8;
    }
    out
}
