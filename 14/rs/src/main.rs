use std::collections::HashMap;
use std::fmt;

type CorType = u8;
type ResultType = u16;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Cor {
    row: CorType,
    col: CorType,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Rock {
    Round,
    Cube,
}

struct Platform {
    rows: CorType,
    cols: CorType,
    platform: HashMap<Cor, Rock>,
}


fn main() {
    let mut platform = Platform::read("../../exp");
    println!("read in:");
    println!("{}", platform);
    println!();

    platform.tilt_north();
    println!("tilted north:");
    println!("{}", platform);
    println!();

    println!("result: {}", platform.calc_weight_north());
}


impl Platform {
    fn read(filename: &str) -> Self {
        let mut platform = HashMap::new();
        let mut rows: CorType = 0;
        let mut cols: CorType = 0;
        for (row, ln) in std::fs::read_to_string(filename).unwrap_or_else(|_| panic!("missing input file '{filename}'")).lines().enumerate() {
            rows = (row + 1).try_into().unwrap();
            for (col, symbol) in ln.chars().enumerate() {
                if let Some(rock) = Rock::parse(symbol) {
                    platform.insert(Cor::new(row, col), rock);
                }
                cols = (col + 1).try_into().unwrap();
            }
        }
        Self {platform, rows, cols}
    }

    fn tilt_north(&mut self) {
        for col in 0..self.cols {
            for row in 1..self.rows {
                if self.get(Cor{row, col}) == Some(Rock::Round) {
                    let new_row = self.find_free_row_north(row, col);
                    self.platform.remove(&Cor{row, col});
                    self.platform.insert(Cor{row: new_row, col}, Rock::Round);
                }
            }
        }
    }

    fn find_free_row_north(&self, row: CorType, col: CorType) -> CorType {
        for new_row in (0..row).rev() {
            if let Some(_rock) = self.get(Cor{row: new_row, col}) {
                return new_row + 1;
            }
        }
        0
    }

    fn get(&self, cor: Cor) -> Option<Rock> {
        self.platform.get(&cor).copied()
    }

    fn calc_weight_north(&self) -> ResultType {
        let mut result: ResultType = 0;
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.get(Cor{row, col}) == Some(Rock::Round) {
                    result += (self.rows - row) as ResultType;
                }
            }
        }
        result
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", match self.get(Cor{row, col}) {
                    None => '.',
                    Some(Rock::Cube) => '#',
                    Some(Rock::Round) => 'O',
                })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Rock {
    fn parse(symbol: char) -> Option<Self> {
        match symbol {
            '.' => None,
            'O' => Some(Self::Round),
            '#' => Some(Self::Cube),
            _ => panic!("unknown type of rock '{symbol}'"),
        }
    }
}

impl Cor {
    fn new(row: usize, col: usize) -> Self {
        Self {
            row: row.try_into().unwrap(),
            col: col.try_into().unwrap(),
        }
    }
}
