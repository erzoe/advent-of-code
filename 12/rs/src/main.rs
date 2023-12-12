use regex::Regex;

fn main() {
    let mut result = 0;
    for ln in std::fs::read_to_string("../../input").expect("missing input file").lines() {
        let (broken, checksum) = ln.split_once(' ').expect("missing checksum in input line");
        let re = Regex::new(&(r"^\.*".to_string() + &checksum.split(',').map(|n| "#".repeat(n.parse().expect("failed to parse number"))).collect::<Vec<_>>().join(r"\.+") + r"\.*$")).expect("failed to build regex");
        let mut possibilites = 0;
        for possibility in get_possible_fixes(broken) {
            //println!("    {possibility}");
            if re.is_match(&possibility) {
                possibilites += 1;
            }
        }
        //println!("{broken} {checksum}: {possibilites}");
        result += possibilites;
    }
    println!("result: {result}");
}

fn get_possible_fixes(broken: &str) -> Vec<String> {
    let mut out = Vec::<String>::new();
    out.push("".to_string());
    for c in broken.chars() {
        match c {
            '.'|'#' => {
                for i in 0..out.len() {
                    out[i] += &c.to_string();
                }
            }
            '?' => {
                for i in 0..out.len() {
                    out.push(out[i].clone() + "#");
                    out[i] += ".";
                }
            }
            _ => {
                panic!("invalid character '{c}' in broken input");
            }
        }
    }
    out
}
