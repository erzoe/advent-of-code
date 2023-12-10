use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

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

struct Field {
    shape: Option<Shape>,
    distance: Option<u8>,
    cor: Cor,
}

struct FieldParser {
    cor: Cor,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Cor {
    row: i16,
    col: i16,
}

struct Map {
    map: Vec<Vec<Option<Field>>>,
    rows: usize,
    cols: usize,
}

fn main() {
    let file = File::open("../../exp").expect("failed to find input file");
    let reader = BufReader::new(file);
    let map = Map::parse(reader.lines().map(|ln| ln.unwrap()));
    let furthest_field = map.map.iter().flat_map(|row| row.iter()).filter(|f| f.is_some()).map(|f| f.as_ref().unwrap()).max_by_key(|f| f.distance.unwrap_or(0)).unwrap();
    println!("Furthest field: {:?}, {}", furthest_field.cor, furthest_field.distance.unwrap());
}

impl Map {
    fn parse<'a>(lines: impl Iterator<Item=String>) -> Self {
        let mut map: Vec<Vec<Option<Field>>> = Vec::new();
        let mut field_parser = FieldParser::new();
        for ln in lines {
            map.push( ln.chars().map(|symbol| field_parser.parse(symbol)).collect() );
        }

        let rows = map.len();
        let cols = map[0].len();
        for i in 1..rows {
            if map[i].len() != cols {
                panic!("row {i} has a different length: {} instead of {cols}", map[i].len());
            }
        }

        let mut out = Self { map, rows, cols };
        let start_cor = out.map.iter().flat_map(|row| row.iter()).filter(|f| f.is_some()).map(|f| f.as_ref().unwrap()).find(|f| f.distance == Some(0)).expect("failed to find start field").cor;
        let start: &mut Field = out.get_mut(start_cor);
        let connected_to_start = out.get_connected(start.cor);
        start.shape = Some(out.get_shape_from_neighbours(start.cor));

        out
    }

    fn get(&self, cor: Cor) -> Option<&Field> {
        if cor.row < 0 || cor.col < 0 {
            return None;
        }
        if let Some(row) = self.map.get(cor.row as usize) {
            if let Some(field) = row.get(cor.col as usize) {
                return field.as_ref();
            }
        }
        None
    }

    fn get_mut(&mut self, cor: Cor) -> &mut Field {
        self.map.get_mut(cor.row as usize).as_mut().unwrap().get_mut(cor.col as usize).as_mut().unwrap().as_mut().unwrap()
    }

    fn get_connected(&self, cor: Cor) -> Vec<&Field> {
        let mut out = Vec::new();
        if let Some(field) = self.get(cor.n()) {
            if field.is_pointing_south() {
                out.push(field);
            }
        }
        if let Some(field) = self.get(cor.s()) {
            if field.is_pointing_north() {
                out.push(field);
            }
        }
        if let Some(field) = self.get(cor.w()) {
            if field.is_pointing_east() {
                out.push(field);
            }
        }
        if let Some(field) = self.get(cor.e()) {
            if field.is_pointing_west() {
                out.push(field);
            }
        }
        out
    }
    fn get_shape_from_neighbours(&self, cor: Cor) -> Shape {
        let mut directions = Vec::new();
        for d in [Direction::N, Direction::S, Direction::W, Direction::E] {
            if let Some(field) = self.get(cor.step(d)) {
                if field.is_pointing(d.opposite()) {
                    directions.push(d);
                }
            }
        }
        assert_eq!(directions.len(), 2, "start field does not have exactly two connecting neighbours");
        Shape::from_directions(directions[0], directions[1])

        //if directions.contains(Direction::N) && directions.contains(Direction::S) { Shape::NS }
        //else if directions.contains(Direction::N) && directions.contains(Direction::W) { Shape::NW }
        //else if directions.contains(Direction::N) && directions.contains(Direction::E) { Shape::NE }
        //else if directions.contains(Direction::N) && directions.contains(Direction::S) { Shape::NS }

    }
}

impl FieldParser {
    fn new() -> Self {
        Self { cor: Cor{row: 0, col: 0} }
    }

    fn next_row(&mut self) {
        self.cor.row += 1;
        self.cor.col = 0;
    }

    fn parse(&mut self, symbol: char) -> Option<Field> {
        if symbol == '.' {
            return None;
        }
        let out = Field{
            shape: if symbol == 'S' {None} else {Some(Shape::parse(symbol))},
            distance: None,
            cor: self.cor,
        };
        self.cor.col += 1;
        Some(out)
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
    fn is_pointing_north(&self) -> bool {
        if let Some(s) = self.shape {
            s == Shape::NS || s == Shape::NW || s == Shape::NE
        } else {
            false
        }
    }
    fn is_pointing_south(&self) -> bool {
        if let Some(s) = self.shape {
            s == Shape::NS || s == Shape::SW || s == Shape::SE
        } else {
            false
        }
    }
    fn is_pointing_west(&self) -> bool {
        if let Some(s) = self.shape {
            s == Shape::WE || s == Shape::SW || s == Shape::NW
        } else {
            false
        }
    }
    fn is_pointing_east(&self) -> bool {
        if let Some(s) = self.shape {
            s == Shape::WE || s == Shape::SE || s == Shape::SE
        } else {
            false
        }
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

//impl<'a> IntoIterator for &'a Map {
//    type Item = &'a Field;
//    type IntoIter = impl Iterator<Item=Self::Item>;
//
//    fn into_iter(self) -> Self::IntoIter {
//        self.map.iter().flat_map(|row| row.iter()).filter(|f| f.is_some()).map(|f| f.as_ref().unwrap())
//    }
//}
