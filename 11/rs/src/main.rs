use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::fmt;

struct Galaxies {
    galaxies: Vec<Galaxy>,
    rows: usize,
    cols: usize,
}

struct Galaxy {
    row: usize,
    col: usize,
}

fn main() {
    let mut galaxies = Galaxies::read("../../exp");
    println!("read galaxies:");
    println!("{}", galaxies);

    galaxies.expand();
    println!("galaxies after expansion:");
    println!("{}", galaxies);

    let sum_of_distances: usize = galaxies.get_distances().iter().sum();
    println!("result: {}", sum_of_distances);
}

impl Galaxies {
    fn read(filename: &str) -> Self {
        let file = File::open(filename).expect("input file does not exist");
        let reader = BufReader::new(file);

        let mut galaxies = Vec::new();
        let mut row = 0;
        let mut col = 0;
        for ln in reader.lines() {
            for s in ln.unwrap().chars() {
                if s == '#' {
                    galaxies.push( Galaxy { row, col } )
                }
                col += 1;
            }
            row += 1;
        }

        Self {galaxies, rows: row, cols: col}
    }

    fn expand(&mut self) {
        let empty_rows = (0..self.rows).filter(|row| self.galaxies.iter().all(|g| g.row != *row)).collect::<Vec<usize>>();
        let empty_cols = (0..self.cols).filter(|col| self.galaxies.iter().all(|g| g.col != *col)).collect::<Vec<usize>>();

        for row in &empty_rows {
            for galaxy in &mut self.galaxies {
                if galaxy.row > *row {
                    galaxy.row += 1;
                }
            }
        }

        for col in &empty_cols {
            for galaxy in &mut self.galaxies {
                if galaxy.col > *col {
                    galaxy.col += 1;
                }
            }
        }

        self.rows += empty_rows.len();
        self.cols += empty_cols.len();
    }

    fn get_distances(&self) -> Vec<usize> {
        let mut out = Vec::new();
        for i in 0..self.galaxies.len() {
            for j in i+1..self.galaxies.len() {
                let g1 = &self.galaxies[i];
                let g2 = &self.galaxies[j];
                let d = g1.get_distance(g2);
                out.push(d);
                println!("distance between galaxy {i} and {j}: {d}")
            }
        }
        out
    }

    fn get(&self, row: usize, col: usize) -> Option<usize> {
        for (i, galaxy) in self.galaxies.iter().enumerate() {
            if galaxy.row == row && galaxy.col == col {
                return Some(i);
            }
        }
        None
    }
}

impl fmt::Display for Galaxies {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if let Some(galaxy) = self.get(row, col) {
                    write!(f, "{}", galaxy)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Galaxy {
    fn get_distance(&self, other: &Galaxy) -> usize {
        let d_row = if other.row > self.row {
            other.row - self.row
        } else {
            self.row - other.row
        };
        let d_col = if other.col > self.col {
            other.col - self.col
        } else {
            self.col - other.col
        };
        d_row + d_col
    }
}
