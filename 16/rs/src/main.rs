use std::fmt;

type CorType = u8;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    MirrorSwNe,
    MirrorNwSe,
    SplitterVer,
    SplitterHor,
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
    pos: Cor,
    direction: Direction,
}


fn main() {
    let grid = Grid::read(std::path::Path::new("../../exp"));
    println!("{}", grid);
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

    fn get(&self, cor: Cor) -> Tile {
        self.tiles[cor.row as usize][cor.col as usize]
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
        match symbol {
            '.' => Self::Empty,
            '/' => Self::MirrorSwNe,
            '\\' => Self::MirrorNwSe,
            '|' => Self::SplitterVer,
            '-' => Self::SplitterHor,
            _ => panic!("invalid tile '{symbol}'"),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty       => write!(f, r"."),
            Self::MirrorSwNe  => write!(f, r"/"),
            Self::MirrorNwSe  => write!(f, r"\"),
            Self::SplitterVer => write!(f, r"|"),
            Self::SplitterHor => write!(f, r"-"),
        }
    }
}
