type CorType = u16;

#[derive(PartialEq, Eq, Clone, Debug)]
struct Cor {
    x: CorType,
    y: CorType,
    z: CorType,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Brick {
    name: char,
    c1: Cor,
    c2: Cor,
}

struct Pile {
    bricks: Vec<Brick>,
}


fn main() {
    let mut pile = Pile::read("../../input");
    //pile.print_x(); println!();
    //pile.print_y(); println!();

    //println!("after gravity has done it's thing:");
    pile.let_gravity_do_its_thing();
    //pile.print_x(); println!();
    //pile.print_y();

    let mut result = 0;
    for brick in &pile.bricks {
        if pile.all_supported_bricks_have_other_supporters(brick) {
            result += 1;
        }
    }
    println!("result: {}", result);
}


impl Pile {
    fn read(filename: &str) -> Self {
        Self::parse(std::fs::read_to_string(filename).unwrap_or_else(|_| panic!("no such file '{}'", filename)).lines())
    }

    fn parse<'a>(lines: impl Iterator<Item=&'a str>) -> Self {
        let mut name = 'A';
        Self {
            bricks: lines.map(|ln| {
                let (c1, c2) = ln.split_once('~').expect("invalid line for pile");
                let c1 = Cor::parse(c1);
                let c2 = Cor::parse(c2);
                let out = Brick { c1, c2, name };
                name = std::char::from_u32( name as u32 + 1 ).unwrap();
                out
            }).collect()
        }
    }

    fn let_gravity_do_its_thing(&mut self) {
        self.bricks.sort_by_key(|b| b.min_z());
        for (i, brick) in self.bricks.clone().iter().enumerate() {
            let z = self.bricks.iter().filter(|b| b.is_under(brick)).map(|b| b.max_z() + 1).max().unwrap_or(0);
            //println!("    lower {} by {} - {}", brick.name, brick.min_z(), z);
            self.bricks[i] = brick.lower(brick.min_z() - z);
        }
    }

    fn all_supported_bricks_have_other_supporters(&self, brick: &Brick) -> bool {
        for supported in self.bricks.iter().filter(|b| brick.supports(b)) {
            if !self.has_other_supporters(supported, brick) {
                return false;
            }
        }
        true
    }
    fn has_other_supporters(&self, brick: &Brick, supporter: &Brick) -> bool {
        for b in &self.bricks {
            if b != supporter && b.supports(brick) {
                return true;
            }
        }
        false
    }

    fn print_x(&self) {
        let max_z = self.bricks.iter().map(|b| b.max_z()).max().unwrap();
        let max_x = self.bricks.iter().map(|b| b.max_x()).max().unwrap();

        println!(" ^");

        for z in (0..=max_z).rev() {
            print!("{}", z);
            if z == max_z / 2 {
                print!("z ");
            } else {
                print!("| ")
            }

            for x in 0..=max_x {
                let brick = self
                    .bricks
                    .iter()
                    .filter(
                        |b| b.min_x() <= x && x <= b.max_x()
                        &&  b.min_z() <= z && z <= b.max_z()
                    ).min_by_key(|b| b.min_y())
                    .map(|b| b.name)
                    .unwrap_or(' ');
                print!("{}", brick);
            }
            println!();
        }

        println!("   -{:-^w$}>", "x", w=max_x as usize);
        print!("   ");
        for x in 0..=max_x {
            print!("{}", x.to_string().chars().next().unwrap());
        }
        println!();
    }
    fn print_y(&self) {
        let max_z = self.bricks.iter().map(|b| b.max_z()).max().unwrap();
        let max_y = self.bricks.iter().map(|b| b.max_y()).max().unwrap();

        println!(" ^");

        for z in (0..=max_z).rev() {
            print!("{}", z);
            if z == max_z / 2 {
                print!("z ");
            } else {
                print!("| ")
            }

            for y in 0..=max_y {
                let brick = self
                    .bricks
                    .iter()
                    .filter(
                        |b| b.min_y() <= y && y <= b.max_y()
                        &&  b.min_z() <= z && z <= b.max_z()
                    ).min_by_key(|b| b.min_y())
                    .map(|b| b.name)
                    .unwrap_or(' ');
                print!("{}", brick);
            }
            println!();
        }

        println!("   -{:-^w$}>", "y", w=max_y as usize);
        print!("   ");
        for x in 0..=max_y {
            print!("{}", x.to_string().chars().next().unwrap());
        }
        println!();
    }
}

impl Cor {
    fn parse(val: &str) -> Self {
        let mut c = val.split(',');
        let x: CorType = c.next().expect("missing x coordinate").parse().expect("failed to parse x coordinate");
        let y: CorType = c.next().expect("missing y coordinate").parse().expect("failed to parse y coordinate");
        let z: CorType = c.next().expect("missing z coordinate").parse().expect("failed to parse z coordinate");
        assert!(c.next().is_none(), "more than three dimensions given for coordinate");
        Self { x, y, z }
    }
}

impl Brick {
    fn min_z(&self) -> CorType {
        if self.c1.z < self.c2.z {
            self.c1.z
        } else {
            self.c2.z
        }
    }
    fn max_z(&self) -> CorType {
        if self.c1.z > self.c2.z {
            self.c1.z
        } else {
            self.c2.z
        }
    }

    fn min_y(&self) -> CorType {
        if self.c1.y < self.c2.y {
            self.c1.y
        } else {
            self.c2.y
        }
    }
    fn max_y(&self) -> CorType {
        if self.c1.y > self.c2.y {
            self.c1.y
        } else {
            self.c2.y
        }
    }

    fn min_x(&self) -> CorType {
        if self.c1.x < self.c2.x {
            self.c1.x
        } else {
            self.c2.x
        }
    }
    fn max_x(&self) -> CorType {
        if self.c1.x > self.c2.x {
            self.c1.x
        } else {
            self.c2.x
        }
    }

    fn is_under(&self, other: &Self) -> bool {
           self.min_x() <= other.max_x() && self.max_x() >= other.min_x()
        && self.min_y() <= other.max_y() && self.max_y() >= other.min_y()
        && self.max_z() < other.min_z()
    }

    fn supports(&self, other: &Self) -> bool {
        self.is_under(other) && self.max_z() + 1 == other.min_z()
    }

    fn lower(&self, z: CorType) -> Self {
        Self {
            c1: Cor { z: self.c1.z - z, ..self.c1 },
            c2: Cor { z: self.c2.z - z, ..self.c2 },
            name: self.name,
        }
    }
}
