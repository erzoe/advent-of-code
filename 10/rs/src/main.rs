use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

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

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Cor {
    row: i16,
    col: i16,
}

struct Field {
    shape: Shape,
    distance: Option<u32>,
}

struct Map {
    map: HashMap<Cor, Field>,
    rows: usize,
    cols: usize,
}

fn main() {
    let file = File::open("../../exp_easy").expect("failed to find input file");
    let reader = BufReader::new(file);
    let map = Map::parse(reader.lines().map(|ln| ln.unwrap()));
    let (furthest_cor, furthest_field) = map.map.iter().max_by_key(|(_c, f)| f.distance.unwrap()).unwrap();
    println!("Furthest field: {:?}, {}", furthest_cor, furthest_field.distance.unwrap());
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

        Self { map, rows: cor.row.try_into().unwrap(), cols: cor.col.try_into().unwrap() }
    }

    fn count_distances(map: &mut HashMap<Cor, Field>, start: Cor) {
        let mut distance = 0;
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
    fn get_next(map: &HashMap<Cor, Field>, cor: Cor, coming_from: Direction) -> (Cor, Direction) {
        let (d1, d2) = map[&cor].shape.to_directions();
        if d1 == coming_from {
            (cor.step(d2), d2)
        } else {
            (cor.step(d1), d1)
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
}

impl Field {
    fn new(shape: Shape) -> Self {
        Self { shape, distance: None }
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
