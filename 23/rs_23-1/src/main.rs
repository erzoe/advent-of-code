// takes 10 seconds to run

type CorType = u8;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    N, S, W, E
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Tile {
    Forest,
    Path,
    Slope(Direction),
}

struct Map {
    map: Vec<Vec<Tile>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Cor {
    row: CorType,
    col: CorType,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Step {
    cor: Cor,
    direction: Direction,
}

#[derive(Clone, Debug)]
struct Hike {
    steps: Vec<Step>,
}


fn main() {
    let map = Map::read("../../input");
    let mut unfinished_hikes = vec![Hike::new(map.get_start())];
    let mut finished_hikes = Vec::new();
    while !unfinished_hikes.is_empty() {
        let mut hike = unfinished_hikes.pop().unwrap();
        loop {
            let mut next_steps = map.get_next(hike.steps.last().unwrap());
            next_steps.retain(|next_step| hike.steps.iter().all(|old_step| old_step.cor != next_step.cor));
            for s in next_steps.iter().skip(1) {
                unfinished_hikes.push({
                    let mut cloned_hike = hike.clone();
                    cloned_hike.step(*s);
                    cloned_hike
                })
            }
            if let Some(s) = next_steps.get(0) {
                hike.step(*s);
                if map.is_end(&s.cor) {
                    finished_hikes.push(hike);
                    break;
                }
            } else {
                // dead end, drop current route, continue with next possibility
                break;
            }
        }
    }

    finished_hikes.sort_by_key(|hike| hike.steps.len());
    //println!("longest hike:");
    let longest_hike = finished_hikes.last().expect("no path found");
    //map.print_hike(&longest_hike);
    println!("longest hike has {} steps", longest_hike.steps.len());
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

    fn get_start(&self) -> Step {
        Step {
            cor: Cor {
                row: 1,
                col: self.map[0].iter().position(|tile| tile == &Tile::Path).expect("there is no entrance to the hiking trail").try_into().unwrap(),
            },
            direction: Direction::S,
        }
    }
    fn get_next(&self, step: &Step) -> Vec<Step> {
        let mut out = Vec::new();
        for d in [Direction::S, Direction::E, Direction::W, Direction::N] {
            if d == step.direction.opposite() {
                continue;
            }
            let next_cor = step.cor + d;
            let next_tile = self.get(&next_cor);
            if next_tile == Tile::Path || next_tile == Tile::Slope(d) {
                out.push(Step{ cor: next_cor, direction: d });
            }
        }
        out
    }

    fn is_end(&self, cor: &Cor) -> bool {
        cor.row == self.rows() - 1
    }

    fn get(&self, cor: &Cor) -> Tile {
        self.map[cor.row as usize][cor.col as usize]
    }
    fn rows(&self) -> CorType {
        self.map.len() as CorType
    }
    fn cols(&self) -> CorType {
        self.map[0].len() as CorType
    }

    fn print_hike(&self, hike: &Hike) {
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                print!("{}", {
                    if let Some(step) = hike.steps.iter().filter(|s| s.cor == Cor{row, col}).next() {
                        let tile = self.get(&Cor{row, col});
                        if tile == Tile::Forest {
                            'X'
                        } else if let Tile::Slope(_) = tile {
                            tile.to_symbol()
                        } else {
                        match step.direction {
                            Direction::N => '↑',
                            Direction::S => '↓',
                            Direction::W => '←',
                            Direction::E => '→',
                        }}
                    } else {
                        self.get(&Cor{row, col}).to_symbol()
                    }
                });
            }
            println!();
        }
    }
}

impl Tile {
    fn parse(symbol: char) -> Self {
        match symbol {
            '#' => Self::Forest,
            '.' => Self::Path,
            '^' => Self::Slope(Direction::N),
            'v' => Self::Slope(Direction::S),
            '<' => Self::Slope(Direction::W),
            '>' => Self::Slope(Direction::E),
            _ => panic!("unknown tile '{}'", symbol),
        }
    }

    fn to_symbol(self) -> char {
        match self {
            Self::Forest => '#',
            Self::Path => '.',
            Self::Slope(Direction::N) => '^',
            Self::Slope(Direction::S) => 'v',
            Self::Slope(Direction::W) => '<',
            Self::Slope(Direction::E) => '>',
        }
    }
}

// I don't need to check the size of the map because it's completely surrounded by forest
impl std::ops::Add<Direction> for Cor {
    type Output = Cor;
    fn add(self, other: Direction) -> Self::Output {
        match other {
            Direction::N => Cor { row: self.row - 1, ..self },
            Direction::S => Cor { row: self.row + 1, ..self },
            Direction::W => Cor { col: self.col - 1, ..self },
            Direction::E => Cor { col: self.col + 1, ..self },
        }
    }
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
            Direction::E => Direction::W,
        }
    }
}

impl Hike {
    fn new(step: Step) -> Self {
        Self { steps: vec![step] }
    }

    fn step(&mut self, step: Step) {
        self.steps.push(step);
    }
}
