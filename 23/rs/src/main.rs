enum Direction {
    N, S, W, E
}
enum Tile {
    Forest,
    Path,
    Slope(Direction),
}
struct Map {
    map: Vec<Vec<Tile>>,
}


fn main() {
    let map = Map::read("../../exp");
    println!("Hello, world!");
}


impl Map {
    fn read(filename: &str) -> Self {
        Self::parse(
            std::fs::read_to_string(filename)
                .unwrap_or_else(|_| panic!("input file missing '{}'", filename))
                .lines(),
        )
    }
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        let mut map = Vec::new();
        for ln in lines {
            map.push(ln.chars().map(Tile::parse).collect());
        }
        Self { map }
    }
}

impl Tile {
    fn parse(symbol: char) -> Self {
        match symbol {
            '#' => Self::Forest,
            '.' => Self::Path,
            '^' => Self::Slope(Direction::N),
            'v' => Self::Slope(Direction::S),
            '>' => Self::Slope(Direction::W),
            '<' => Self::Slope(Direction::E),
            _ => panic!("unknown tile '{}'", symbol),
        }
    }
}
