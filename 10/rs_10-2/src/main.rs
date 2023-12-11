use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Shape {
    NS,
    WE,
    NE,
    NW,
    SW,
    SE,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction{
    N,
    S,
    W,
    E
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Side{
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Cor {
    row: i16,
    col: i16,
}

struct Field {
    shape: Shape,
    distance: Option<u32>,
    is_inside: bool,
}

struct Map {
    map: HashMap<Cor, Field>,
    rows: usize,
    cols: usize,
}

fn main() {
    let file = File::open("../../exp2_complex").expect("failed to find input file");
    let reader = BufReader::new(file);
    let map = Map::parse(reader.lines().map(|ln| ln.unwrap()));
    println!("{}", map);
    map.print_distances();
    let (furthest_cor, furthest_field) = map.map.iter().filter(|(_c, f)| f.distance.is_some()).max_by_key(|(_c, f)| f.distance.unwrap()).unwrap();
    let fields_inside = map.map.iter().filter(|(_c, f)| f.is_inside).count();
    println!("Furthest field: {:?}, {}", furthest_cor, furthest_field.distance.unwrap());
    println!("Number fields inside: {}", fields_inside);
}

impl Map {
    fn parse(lines: impl Iterator<Item=String>) -> Self {
        let mut map = HashMap::<Cor, Field>::new();
        let mut cor = Cor::origin();
        let mut start = None;
        for ln in lines {
            cor.col = 0;
            for s in ln.chars() {
                match s {
                    'S' => { start = Some(cor); }
                    '.' => {}
                    _ => { map.insert(cor, Field::new(Shape::parse(s))); }
                }
                cor.col += 1;
            }
            cor.row += 1;
        }
        let start = start.expect("input did not contain a start field");

        map.insert(start, Field::new(Self::get_shape_from_neighbours(&map, start)));
        Self::count_distances(&mut map, start);
        Self::set_inside(&mut map, start);

        Self { map, rows: cor.row.try_into().unwrap(), cols: cor.col.try_into().unwrap() }
    }

    fn count_distances(map: &mut HashMap<Cor, Field>, start: Cor) {
        let mut distance = 0;
        map.entry(start).and_modify(|f| f.distance = Some(distance));
        let mut c1 = start;
        let mut c2 = start;
        let (mut d1, mut d2) = map[&start].shape.to_directions();
        loop {
            (c1, d1) = Self::get_next(map, c1, d1);
            (c2, d2) = Self::get_next(map, c2, d2);
            distance += 1;
            if map[&c1].distance.is_some() {
                break;
            }
            map.entry(c1).and_modify(|f| f.distance = Some(distance));
            if map[&c2].distance.is_some() {
                break;
            }
            map.entry(c2).and_modify(|f| f.distance = Some(distance));
        }
    }

    fn set_inside(map: &mut HashMap<Cor, Field>, start: Cor) {
        let mut coming_from = map[&start].shape.to_directions().0;
        let inside = Self::get_inside_side(map, start, coming_from);
        let mut cor_pipe = start;
        loop {
            (cor_pipe, coming_from) = Self::get_next(map, cor_pipe, coming_from);
            if cor_pipe == start {
                break;
            }

            let going_to = map[&cor_pipe].shape.to_other_direction(coming_from);
            let mut directions_pointing_inwards = Vec::new();
            let turn = Self::get_turn(coming_from, going_to.opposite());
            if turn.is_none() {
                directions_pointing_inwards.push(coming_from.opposite().turn(inside))
            } else if turn.unwrap() == inside.opposite() {
                directions_pointing_inwards.push(coming_from.opposite());
                directions_pointing_inwards.push(coming_from.opposite().turn(inside));
            };
            for d in directions_pointing_inwards {
                let mut cor_inner = cor_pipe;
                loop {
                    cor_inner = cor_inner.step(d);
                    if let Some(field) = map.get_mut(&cor_inner) {
                        if field.distance.is_some() {
                            // I have hit the other side of the pipe, that's the end of the inside
                            break;
                        } else {
                            field.is_inside = true;
                        }
                    } else {
                        map.insert(cor_inner, Field {
                            is_inside: true,
                            shape: Shape::NS,  // shape is irrelevant if is_inside is set
                            distance: None,
                        });
                    }
                }
            }
        }
    }
    fn get_inside_side(map: &HashMap<Cor, Field>, start: Cor, coming_from: Direction) -> Side {
        let mut left = 0;
        let mut right = 0;
        let mut d0 = coming_from;
        let mut d1: Direction;
        let mut c = start;
        loop {
            (c, d1) = Self::get_next(map, c, d0);
            if c == start {
                break;
            }
            match Self::get_turn(d0, d1) {
                Some(Side::Left) => {left += 1;}
                Some(Side::Right) => {right += 1;}
                None => {}
            }
            d0 = d1;
        }
        if left > right {
            Side::Left
        } else {
            Side::Right
        }
    }
    fn get_turn(coming_from_0: Direction, coming_from_1: Direction) -> Option<Side> {
        match (coming_from_0.opposite(), coming_from_1.opposite()) {
            (Direction::N, Direction::W) => Some(Side::Left),
            (Direction::W, Direction::S) => Some(Side::Left),
            (Direction::S, Direction::E) => Some(Side::Left),
            (Direction::E, Direction::N) => Some(Side::Left),

            (Direction::N, Direction::E) => Some(Side::Right),
            (Direction::E, Direction::S) => Some(Side::Right),
            (Direction::S, Direction::W) => Some(Side::Right),
            (Direction::W, Direction::N) => Some(Side::Right),

            (Direction::N, Direction::N) => None,
            (Direction::S, Direction::S) => None,
            (Direction::W, Direction::W) => None,
            (Direction::E, Direction::E) => None,

            (Direction::N, Direction::S) => panic!("180° turn is not possible"),
            (Direction::S, Direction::N) => panic!("180° turn is not possible"),
            (Direction::W, Direction::E) => panic!("180° turn is not possible"),
            (Direction::E, Direction::W) => panic!("180° turn is not possible"),
        }
    }

    fn get_next(map: &HashMap<Cor, Field>, cor: Cor, coming_from: Direction) -> (Cor, Direction) {
        let (d1, d2) = map[&cor].shape.to_directions();
        if d1 == coming_from {
            (cor.step(d2), d2.opposite())
        } else {
            (cor.step(d1), d1.opposite())
        }
    }

    fn get_shape_from_neighbours(map: &HashMap<Cor, Field>, cor: Cor) -> Shape {
        let mut directions = Vec::new();
        for d in [Direction::N, Direction::S, Direction::W, Direction::E] {
            if let Some(field) = map.get(&cor.step(d)) {
                if field.is_pointing(d.opposite()) {
                    directions.push(d);
                }
            }
        }
        assert_eq!(directions.len(), 2, "start field does not have exactly two connecting neighbours");
        Shape::from_directions(directions[0], directions[1])
    }

    fn print_distances(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if let Some(field) = self.map.get(&Cor{row: row.try_into().unwrap(), col: col.try_into().unwrap()}) {
                    if let Some(distance) = field.distance {
                        print!("{}", distance);
                    } else {
                        print!("?", );
                    }
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if let Some(field) = self.map.get(&Cor{row: row.try_into().unwrap(), col: col.try_into().unwrap()}) {
                    if field.is_inside {
                        write!(f, "I")?;
                    } else {
                        write!(f, "{}", field.shape.to_symbol())?;
                    }
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Shape {
    fn parse(symbol: char) -> Self {
        match symbol {
            '|' => Self::NS,
            '-' => Self::WE,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            _ => panic!("unexpected symbol '{}'", symbol),
        }
    }

    fn to_symbol(self) -> char {
        match self {
            Self::NS => '│',
            Self::WE => '─',
            Self::NE => '└',
            Self::NW => '┘',
            Self::SW => '┐',
            Self::SE => '┌',
        }
    }

    fn from_directions(d1: Direction, d2: Direction) -> Self {
        match (d1, d2) {
            (Direction::N, Direction::S) => Self::NS,
            (Direction::S, Direction::N) => Self::NS,

            (Direction::N, Direction::W) => Self::NW,
            (Direction::W, Direction::N) => Self::NW,

            (Direction::N, Direction::E) => Self::NE,
            (Direction::E, Direction::N) => Self::NE,

            (Direction::W, Direction::E) => Self::WE,
            (Direction::E, Direction::W) => Self::WE,

            (Direction::S, Direction::W) => Self::SW,
            (Direction::W, Direction::S) => Self::SW,

            (Direction::S, Direction::E) => Self::SE,
            (Direction::E, Direction::S) => Self::SE,

            (Direction::N, Direction::N) => panic!("two different directions must be given"),
            (Direction::S, Direction::S) => panic!("two different directions must be given"),
            (Direction::W, Direction::W) => panic!("two different directions must be given"),
            (Direction::E, Direction::E) => panic!("two different directions must be given"),
        }
    }

    fn to_directions(self) -> (Direction, Direction) {
        match self {
            Self::NS => (Direction::N, Direction::S),
            Self::NW => (Direction::N, Direction::W),
            Self::NE => (Direction::N, Direction::E),
            Self::WE => (Direction::W, Direction::E),
            Self::SW => (Direction::S, Direction::W),
            Self::SE => (Direction::S, Direction::E),
        }
    }

    fn to_other_direction(self, d: Direction) -> Direction {
        match (self, d) {
            (Self::NS, Direction::N) => Direction::S,
            (Self::NS, Direction::S) => Direction::N,

            (Self::NW, Direction::N) => Direction::W,
            (Self::NW, Direction::W) => Direction::N,

            (Self::NE, Direction::N) => Direction::E,
            (Self::NE, Direction::E) => Direction::N,

            (Self::WE, Direction::W) => Direction::E,
            (Self::WE, Direction::E) => Direction::W,

            (Self::SW, Direction::S) => Direction::W,
            (Self::SW, Direction::W) => Direction::S,

            (Self::SE, Direction::S) => Direction::E,
            (Self::SE, Direction::E) => Direction::S,

            (Self::NS, Direction::W) => panic!("{d:?} is not one of {self:?} directions"),
            (Self::NS, Direction::E) => panic!("{d:?} is not one of {self:?} directions"),
            (Self::WE, Direction::N) => panic!("{d:?} is not one of {self:?} directions"),
            (Self::WE, Direction::S) => panic!("{d:?} is not one of {self:?} directions"),
            (Self::NW, Direction::S) => panic!("{d:?} is not one of {self:?} directions"),
            (Self::NW, Direction::E) => panic!("{d:?} is not one of {self:?} directions"),
            (Self::SW, Direction::N) => panic!("{d:?} is not one of {self:?} directions"),
            (Self::SW, Direction::E) => panic!("{d:?} is not one of {self:?} directions"),
            (Self::SE, Direction::N) => panic!("{d:?} is not one of {self:?} directions"),
            (Self::SE, Direction::W) => panic!("{d:?} is not one of {self:?} directions"),
            (Self::NE, Direction::W) => panic!("{d:?} is not one of {self:?} directions"),
            (Self::NE, Direction::S) => panic!("{d:?} is not one of {self:?} directions"),
        }
    }
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::N => Self::S,
            Self::S => Self::N,
            Self::W => Self::E,
            Self::E => Self::W,
        }
    }

    fn turn(&self, side: Side) -> Self {
        match (self, side) {
            (Self::N, Side::Left) => Self::W,
            (Self::N, Side::Right) => Self::E,

            (Self::W, Side::Left) => Self::S,
            (Self::W, Side::Right) => Self::N,

            (Self::S, Side::Left) => Self::E,
            (Self::S, Side::Right) => Self::W,

            (Self::E, Side::Left) => Self::N,
            (Self::E, Side::Right) => Self::S,
        }
    }
}

impl Side {
    fn opposite(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl Field {
    fn new(shape: Shape) -> Self {
        Self { shape, distance: None, is_inside: false }
    }

    fn is_pointing_north(&self) -> bool {
        let s =self.shape;
        s == Shape::NS || s == Shape::NW || s == Shape::NE
    }
    fn is_pointing_south(&self) -> bool {
        let s =self.shape;
        s == Shape::NS || s == Shape::SW || s == Shape::SE
    }
    fn is_pointing_west(&self) -> bool {
        let s =self.shape;
        s == Shape::WE || s == Shape::SW || s == Shape::NW
    }
    fn is_pointing_east(&self) -> bool {
        let s =self.shape;
        s == Shape::WE || s == Shape::SE || s == Shape::NE
    }
    fn is_pointing(&self, d: Direction) -> bool {
        match d {
            Direction::N => self.is_pointing_north(),
            Direction::S => self.is_pointing_south(),
            Direction::W => self.is_pointing_west(),
            Direction::E => self.is_pointing_east(),
        }
    }
}

impl Cor {
    fn origin() -> Self {
        Self { row: 0, col: 0 }
    }

    fn n(&self) -> Cor { Cor{ row: self.row - 1, ..*self } }
    fn s(&self) -> Cor { Cor{ row: self.row + 1, ..*self } }
    fn w(&self) -> Cor { Cor{ col: self.col - 1, ..*self } }
    fn e(&self) -> Cor { Cor{ col: self.col + 1, ..*self } }
    fn step(&self, d: Direction) -> Cor {
        match d {
            Direction::N => self.n(),
            Direction::S => self.s(),
            Direction::W => self.w(),
            Direction::E => self.e(),
        }
    }
}
