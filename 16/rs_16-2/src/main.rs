use std::fmt;

type CorType = u8;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Object {
    Empty,
    MirrorSwNe,
    MirrorNwSe,
    SplitterVer,
    SplitterHor,
}

#[derive(Clone)]
struct Tile {
    object: Object,
    beam_directions: Vec<Direction>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    N, S, W, E,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Cor {
    row: CorType,
    col: CorType,
}

#[derive(Clone)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
    rows: CorType,
    cols: CorType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Beam {
    cor: Cor,
    direction: Direction,
}


fn main() {
    let mut grid = Grid::read(std::path::Path::new("../../input"));
    //println!("{}", grid);

    let mut result: u32 = 0;
    for row in 0..grid.rows {
        for (col, direction) in [(0, Direction::W), (grid.cols-1, Direction::E)] {
            let n = grid.clone().energize(Beam::new(row, col, direction)).count_energized();
            if n > result {
                result = n;
                println!("found new max: {}", n)
            }
        }
    }
    for col in 0..grid.cols {
        for (row, direction) in [(0, Direction::S), (grid.rows-1, Direction::N)] {
            let n = grid.clone().energize(Beam::new(row, col, direction)).count_energized();
            if n > result {
                result = n;
                println!("found new max: {}", n)
            }
        }
    }
    println!("result: {}", result);
}


impl Grid {
    fn read(path: &std::path::Path) -> Self {
        Self::parse(std::fs::read_to_string(path).unwrap_or_else(|_| panic!("no such file '{}'", path.display())).lines())
    }
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        let mut tiles = Vec::new();
        for ln in lines {
            tiles.push(ln.chars().map(Tile::parse).collect::<Vec<_>>());
        }

        let cols = tiles[0].len();
        for row in tiles.iter().skip(1) {
            assert_eq!(row.len(), cols, "rows have inconsistent length");
        }
        let rows: CorType = tiles.len().try_into().unwrap();
        let cols: CorType = cols.try_into().unwrap();

        Self { tiles, rows, cols }
    }

    fn energize(&mut self, beam: Beam) -> &Self {
        self.energize_tile(&beam);
        let mut beams = vec![beam];
        while !beams.is_empty() {
            beams = beams.iter().flat_map(|b| self.move_beam(b)).collect();
        }

        self
    }

    fn get(&self, cor: Cor) -> &Tile {
        &self.tiles[cor.row as usize][cor.col as usize]
    }

    fn get_mut(&mut self, cor: Cor) -> &mut Tile {
        &mut self.tiles[cor.row as usize][cor.col as usize]
    }

    fn move_beam(&mut self, beam: &Beam) -> Vec<Beam> {
        let tile = self.get(beam.cor);
        let cor = beam.cor;
        let direction = beam.direction;
        let out: Vec<Beam> = match (tile.object, beam.direction) {
            // empty and ignored splitters
            (Object::Empty, direction) => self.next_cor(cor, direction).map(|cor| Beam{cor, direction}).into_iter().collect(),
            (Object::SplitterVer, Direction::N|Direction::S) => self.next_cor(cor, direction).map(|cor| Beam{cor, direction}).into_iter().collect(),
            (Object::SplitterHor, Direction::W|Direction::E) => self.next_cor(cor, direction).map(|cor| Beam{cor, direction}).into_iter().collect(),

            // mirror /
            (Object::MirrorSwNe, Direction::E) => self.next_cor(cor, Direction::N).map(|cor| Beam{cor, direction: Direction::N}).into_iter().collect(),
            (Object::MirrorSwNe, Direction::S) => self.next_cor(cor, Direction::W).map(|cor| Beam{cor, direction: Direction::W}).into_iter().collect(),
            (Object::MirrorSwNe, Direction::N) => self.next_cor(cor, Direction::E).map(|cor| Beam{cor, direction: Direction::E}).into_iter().collect(),
            (Object::MirrorSwNe, Direction::W) => self.next_cor(cor, Direction::S).map(|cor| Beam{cor, direction: Direction::S}).into_iter().collect(),

            // mirror \
            (Object::MirrorNwSe, Direction::E) => self.next_cor(cor, Direction::S).map(|cor| Beam{cor, direction: Direction::S}).into_iter().collect(),
            (Object::MirrorNwSe, Direction::S) => self.next_cor(cor, Direction::E).map(|cor| Beam{cor, direction: Direction::E}).into_iter().collect(),
            (Object::MirrorNwSe, Direction::N) => self.next_cor(cor, Direction::W).map(|cor| Beam{cor, direction: Direction::W}).into_iter().collect(),
            (Object::MirrorNwSe, Direction::W) => self.next_cor(cor, Direction::N).map(|cor| Beam{cor, direction: Direction::N}).into_iter().collect(),

            // splitter
            (Object::SplitterVer, Direction::E|Direction::W) => self.next_cor(cor, Direction::S).map(|cor| Beam{cor, direction: Direction::S}).into_iter().chain(
                                                                self.next_cor(cor, Direction::N).map(|cor| Beam{cor, direction: Direction::N}).into_iter()).collect(),
            (Object::SplitterHor, Direction::N|Direction::S) => self.next_cor(cor, Direction::E).map(|cor| Beam{cor, direction: Direction::E}).into_iter().chain(
                                                                self.next_cor(cor, Direction::W).map(|cor| Beam{cor, direction: Direction::W}).into_iter()).collect(),
        };
        out.into_iter().filter(|b| self.energize_tile(b)).collect()
    }

    fn energize_tile(&mut self, beam: &Beam) -> bool {
        let tile = self.get_mut(beam.cor);
        if !tile.beam_directions.contains(&beam.direction) {
            tile.beam_directions.push(beam.direction);
            true
        } else {
            false
        }
    }

    fn next_cor(&self, cor: Cor, direction: Direction) -> Option<Cor> {
        match direction {
            Direction::N => if cor.row == 0 {
                None
            } else {
                Some( Cor { row: cor.row-1, ..cor } )
            },
            Direction::W => if cor.col == 0 {
                None
            } else {
                Some( Cor { col: cor.col-1, ..cor } )
            },
            Direction::S => if cor.row + 1 >= self.rows {
                None
            } else {
                Some( Cor { row: cor.row+1, ..cor } )
            },
            Direction::E => if cor.col + 1 >= self.cols {
                None
            } else {
                Some( Cor { col: cor.col+1, ..cor } )
            },
        }
    }

    fn count_energized(&self) -> u32 {
        let mut count: u32 = 0;
        for row in &self.tiles {
            for tile in row {
                if tile.is_energized() {
                    count += 1;
                }
            }
        }
        count
    }

    fn print_energized(&self) {
        for row in &self.tiles {
            for tile in row {
                if tile.is_energized() {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn print_beams(&self) {
        for row in &self.tiles {
            for tile in row {
                print!("{}", tile.to_beam_symbol());
            }
            println!();
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{}", tile.to_object_symbol())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Tile {
    fn parse(symbol: char) -> Self {
        Self {
            object: Object::parse(symbol),
            beam_directions: Vec::new(),
        }
    }

    fn is_energized(&self) -> bool {
        !self.beam_directions.is_empty()
    }

    fn to_object_symbol(&self) -> char {
        self.object.to_symbol()
    }

    fn to_beam_symbol(&self) -> char {
        if self.object != Object::Empty {
            self.object.to_symbol()
        } else if self.beam_directions.len() == 1 {
            self.beam_directions[0].to_beam_symbol()
        } else if self.beam_directions.len() > 1 {
            self.beam_directions.len().to_string().chars().next().unwrap()
        } else {
            '.'
        }
    }
}

impl Direction {
    fn to_beam_symbol(self) -> char {
        match self {
            Self::N => '^',
            Self::S => 'v',
            Self::W => '<',
            Self::E => '>',
        }
    }
}

impl Object {
    fn parse(symbol: char) -> Self {
        match symbol {
            '.' => Object::Empty,
            '/' => Object::MirrorSwNe,
            '\\' => Object::MirrorNwSe,
            '|' => Object::SplitterVer,
            '-' => Object::SplitterHor,
            _ => panic!("invalid tile '{symbol}'"),
        }
    }

    fn to_symbol(self) -> char {
        match self {
            Object::Empty       => '.',
            Object::MirrorSwNe  => '/',
            Object::MirrorNwSe  => '\\',
            Object::SplitterVer => '|',
            Object::SplitterHor => '-',
        }
    }
}

impl Beam {
    fn new<T>(row: T, col: T, direction: Direction) -> Self where T: Into<CorType> {
        Self {
            direction,
            cor: Cor {
                row: row.try_into().unwrap(),
                col: col.try_into().unwrap(),
            },
        }
    }
}

impl Cor {
    fn origin() -> Self {
        Self { row: 0, col: 0 }
    }
}
