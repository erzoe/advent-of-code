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

struct Tile {
    object: Object,
    is_energized: bool,
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
    let mut grid = Grid::read(std::path::Path::new("../../exp"));
    println!("{}", grid);

    grid.energize(Beam::start());
    grid.print_energized();
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

    fn energize(&mut self, beam: Beam) {
        self.get_mut(beam.cor).is_energized = true;
        let mut beams = vec![beam];
        while !beams.is_empty() {
            beams = beams.iter().flat_map(|b| self.move_beam(b)).collect();
            for beam in &beams {
                self.get_mut(beam.cor).is_energized = true;
            }
        }

    }

    fn get(&self, cor: Cor) -> &Tile {
        &self.tiles[cor.row as usize][cor.col as usize]
    }

    fn get_mut(&mut self, cor: Cor) -> &mut Tile {
        &mut self.tiles[cor.row as usize][cor.col as usize]
    }

    fn move_beam(&self, beam: &Beam) -> Vec<Beam> {
        let tile = self.get(beam.cor);
        let cor = beam.cor;
        let direction = beam.direction;
        match (tile.object, beam.direction) {
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
            (Object::MirrorNwSe, Direction::W) => self.next_cor(cor, Direction::S).map(|cor| Beam{cor, direction: Direction::N}).into_iter().collect(),

            // splitter
            (Object::SplitterVer, Direction::E|Direction::W) => self.next_cor(cor, Direction::S).map(|cor| Beam{cor, direction: Direction::S}).into_iter().chain(
                                                                self.next_cor(cor, Direction::N).map(|cor| Beam{cor, direction: Direction::N}).into_iter()).collect(),
            (Object::SplitterHor, Direction::N|Direction::S) => self.next_cor(cor, Direction::E).map(|cor| Beam{cor, direction: Direction::E}).into_iter().chain(
                                                                self.next_cor(cor, Direction::W).map(|cor| Beam{cor, direction: Direction::W}).into_iter()).collect(),
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

    fn print_energized(&self) {
        for row in &self.tiles {
            for tile in row {
                if tile.is_energized {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Tile {
    fn parse(symbol: char) -> Self {
        let object = match symbol {
            '.' => Object::Empty,
            '/' => Object::MirrorSwNe,
            '\\' => Object::MirrorNwSe,
            '|' => Object::SplitterVer,
            '-' => Object::SplitterHor,
            _ => panic!("invalid tile '{symbol}'"),
        };

        Self {
            object,
            is_energized: false,
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.object {
            Object::Empty       => write!(f, r"."),
            Object::MirrorSwNe  => write!(f, r"/"),
            Object::MirrorNwSe  => write!(f, r"\"),
            Object::SplitterVer => write!(f, r"|"),
            Object::SplitterHor => write!(f, r"-"),
        }
    }
}

impl Beam {
    fn start() -> Self {
        Self {
            cor: Cor::origin(),
            direction: Direction::E,
        }
    }
}

impl Cor {
    fn origin() -> Self {
        Self { row: 0, col: 0 }
    }
}
