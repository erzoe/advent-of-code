use std::collections::HashSet;

type CorType = u8;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Garden,
    Rock,
    Start,
}

struct Map {
    rows: Vec<Vec<Tile>>,
    number_rows: CorType,
    number_cols: CorType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Cor {
    row: CorType,
    col: CorType,
}


fn main() {
    let map = Map::parse("../../input");
    let mut pos = HashSet::new();
    pos.insert(map.get_start_cor());

    for _step in 0..64 {
        pos = pos.iter().flat_map(|c| map.get_next_cors(*c)).collect();
        //map.print_positions(&pos);
        //println!();
    }

    println!("result: {}", pos.len());
}


impl Map {
    fn parse(filename: &str) -> Self {
        let mut rows = Vec::new();
        for ln in std::fs::read_to_string(filename).unwrap_or_else(|_| panic!("input file missing '{}'", filename)).lines() {
            rows.push(ln.chars().map(Tile::parse).collect::<Vec<_>>());
        }

        let number_rows = rows.len();
        let number_cols = rows[0].len();
        for row in &rows[1..] {
            assert_eq!(row.len(), number_cols);
        }
        let number_rows: CorType = number_rows.try_into().unwrap();
        let number_cols: CorType = number_cols.try_into().unwrap();

        Self { rows, number_rows, number_cols }
    }

    fn get_start_cor(&self) -> Cor {
        for (row, cols) in self.rows.iter().enumerate() {
            for (col, cell) in cols.iter().enumerate() {
                if cell == &Tile::Start {
                    return Cor {
                        row: row as CorType,
                        col: col as CorType,
                    };
                }
            }
        }
        panic!("no start tile");
    }

    fn get_next_cors(&self, cor: Cor) -> Vec<Cor> {
        let mut out = Vec::new();
        if cor.col > 0 {
            out.push(Cor{col: cor.col-1, ..cor});
        }
        if cor.col < self.number_cols - 1 {
            out.push(Cor{col: cor.col+1, ..cor});
        }
        if cor.row > 0 {
            out.push(Cor{row: cor.row-1, ..cor});
        }
        if cor.row < self.number_rows - 1 {
            out.push(Cor{row: cor.row+1, ..cor});
        }
        out.retain(|c| self.get(c) != Tile::Rock);
        out
    }

    fn get(&self, cor: &Cor) -> Tile {
        self.rows[cor.row as usize][cor.col as usize]
    }

    fn print_positions(&self, pos: &HashSet<Cor>) {
        for (row, cols) in self.rows.iter().enumerate() {
            for (col, cell) in cols.iter().enumerate() {
                let symbol = if pos.contains(&Cor{row: row as CorType, col: col as CorType}) {
                    'O'
                } else if cell == &Tile::Rock {
                    '#'
                } else {
                    '.'
                };
                print!("{}", symbol);
            }
            println!();
        }
    }
}

impl Tile {
    fn parse(symbol: char) -> Self {
        match symbol {
            '.' => Self::Garden,
            '#' => Self::Rock,
            'S' => Self::Start,
            _ => panic!("unknown tile '{}'", symbol),
        }
    }
}
