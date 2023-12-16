type FocalLength = u8;

struct Lens {
    label: String,
    focal_length: FocalLength,
}

struct Box {
    lenses: Vec<Lens>,
}

enum Command {
    Set(String, FocalLength),
    Remove(String),
}


fn main() {
    let mut boxes = (0..256).map(|_| Box::new()).collect::<Vec<_>>();
    let mut result: u64 = 0;
    for cmd in std::fs::read_to_string("../../input").expect("input file not found").trim_end().split(',') {
        Command::parse(cmd).exec(&mut boxes);
    }
    for (box_number, r#box) in boxes.iter().enumerate() {
        for (slot_number, lens) in r#box.lenses.iter().enumerate() {
            result += (box_number + 1) as u64 * (slot_number + 1) as u64 * lens.focal_length as u64;
        }
    }
    println!("result: {result}");
}

impl Command {
    fn parse(cmd: &str) -> Self {
        if cmd.ends_with('-') {
            Self::Remove(cmd[0..(cmd.len()-1)].to_string())
        } else {
            let (label, focal_length) = cmd.split_once('=').expect("invalid command '{cmd}', is neither Remove nor Set");
            let label = label.to_string();
            let focal_length: FocalLength = focal_length.parse().expect("failed to parse focal length of lens");
            Self::Set(label, focal_length)
        }
    }

    fn exec(&self, boxes: &mut [Box]) {
        match self {
            Self::Set(label, focal_length) => boxes[hash(label) as usize].set_lens(Lens{label: label.to_string(), focal_length: *focal_length}),
            Self::Remove(label) => boxes[hash(label) as usize].remove_lens(label),
        }
    }
}


impl Box {
    fn new() -> Self {
        Self { lenses: Vec::new() }
    }

    fn remove_lens(&mut self, label: &str) {
        self.lenses.retain(|lens| lens.label != label);
    }

    fn set_lens(&mut self, newlens: Lens) {
        if let Some(index) = self.lenses.iter().position(|oldlens| oldlens.label == newlens.label) {
            self.lenses[index] = newlens;
        } else {
            self.lenses.push(newlens);
        }
    }
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
