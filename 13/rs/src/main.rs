#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Orientation {
    Row,
    Col,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Terrain {
    Ash,
    Rock,
}

struct Map {
    rows: Vec<Vec<Terrain>>,
}

#[derive(PartialEq, Eq, Debug)]
struct Cor {
    row: usize,
    col: usize,
}


fn main() {
    let map = Map::parse(
        std::fs::read_to_string("../../exp_row")
            .expect("input file not found")
            .lines(),
    );

    let mirror_rows = map.get_mirrors(Orientation::Row);
    println!("mirror_rows: {mirror_rows:?}");

    let mirror_cols = map.get_mirrors(Orientation::Col);
    println!("mirror_cols: {mirror_cols:?}");

    let sum_mirror_rows: usize = mirror_rows.iter().sum();
    let sum_mirror_cols: usize = mirror_cols.iter().sum();
    let result = sum_mirror_rows * 100 + sum_mirror_cols;
    println!("result: {}", result);
}


impl Map {
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        let mut rows = Vec::<Vec<Terrain>>::new();
        for ln in lines {
            rows.push(ln.chars().map(Terrain::parse).collect());
        }
        let cols = rows[0].len();
        for row in &rows {
            assert_eq!(row.len(), cols);
        }
        Self { rows }
    }

    fn get_mirrors(&self, orientation: Orientation) -> Vec<usize> {
        let mut out = ( 1 .. self.size(orientation) ).collect::<Vec<_>>();
        for series in 0..self.size(orientation.other()) {
            self.check_mirrors(orientation, series, &mut out);
        }
        out
    }
    fn check_mirrors(&self, orientation: Orientation, series: usize, out: &mut Vec<usize>) {
        for i in out.clone() {
            if !self.is_mirror(orientation, series, i) {
                out.remove(out.iter().position(|x| *x == i).unwrap());
            }
        }
    }
    fn is_mirror(&self, orientation: Orientation, series: usize, index: usize) -> bool {
        for i in 1..*[index + 1, self.size(orientation)-index+1].iter().min().unwrap() {
            if self.getr(orientation, series, index - i) != self.getr(orientation, series, index + i - 1) {
                return false;
            }
        }
        true
    }

    fn size(&self, orientation: Orientation) -> usize {
        match orientation {
            Orientation::Row => self.rows.len(),
            Orientation::Col => self.rows[0].len(),
        }
    }
    fn getr(&self, orientation: Orientation, series: usize, index: usize) -> Terrain {
        let cor = match orientation {
            Orientation::Row => Cor { row: index, col: series },
            Orientation::Col => Cor { row: series, col: index },
        };
        self.get(cor)
    }
    fn get(&self, cor: Cor) -> Terrain {
        self.rows[cor.row][cor.col]
    }
}

impl Terrain {
    fn parse(symbol: char) -> Self {
        match symbol {
            '#' => Self::Rock,
            '.' => Self::Ash,
            _ => panic!("unknown terrrain '{symbol}'"),
        }
    }
}

impl Orientation {
    fn other(&self) -> Self {
        match self {
            Self::Row => Self::Col,
            Self::Col => Self::Row,
        }
    }
}
