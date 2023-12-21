type HeatLoss = u8;
type CumulatedHeatLoss = u32;
type CorType = u8;

struct HeatLossMap {
    map: Vec<Vec<HeatLoss>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Cor {
    row: CorType,
    col: CorType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    N, S, W, E
}

#[derive(Clone, Debug)]
struct Step {
    cor: Cor,
    direction: Direction,
    heat_loss: CumulatedHeatLoss,
    same_direction_counter: u8,
}

struct Ways {
    steps: Vec<Vec<Vec<Step>>>,
}


fn main() {
    let map = HeatLossMap::parse("../../exp");
    let mut ways = Ways::new(&map);
    let mut steps = vec![Step::first()];
    while !steps.is_empty() {
        for step in steps.clone() {
            steps = Vec::new();
            for next_step in map.next_steps(&step) {
                if ways.add(next_step.clone()) {
                    steps.push(next_step.clone());
                    print!("[YES] ");
                } else {
                    print!("[NO ] ");
                }
                println!("{:?}", next_step);
            }
            println!();
        }
        println!("==========");
    }
    println!("{}", ways);
    let result = ways.steps.last().unwrap().last().unwrap().iter().map(|step| step.heat_loss).min().unwrap();
    println!("min heat loss: {}", result);
}


impl HeatLossMap {
    fn parse(filename: &str) -> Self {
        let mut map = Vec::new();
        for ln in std::fs::read_to_string(filename).unwrap_or_else(|_| panic!("input file not found '{}'", filename)).lines() {
            map.push(ln.chars().map(|c| c.to_string().parse().expect("failed to parse heat loss number")).collect());
        }
        Self { map }
    }

    fn rows(&self) -> CorType {
        self.map.len().try_into().unwrap()
    }

    fn cols(&self) -> CorType {
        self.map[0].len().try_into().unwrap()
    }

    fn is_end(&self, cor: &Cor) -> bool {
        cor.row == self.rows() - 1 && cor.col == self.cols() - 1
    }

    fn next_steps(&self, last_step: &Step) -> Vec<Step> {
        let mut out = Vec::new();
        if self.is_end(&last_step.cor) {
            return out;
        }
        if last_step.cor.row + 1 < self.rows() {
            out.push(self.next_step(last_step, Direction::S));
        }
        if last_step.cor.col + 1 < self.cols() {
            out.push(self.next_step(last_step, Direction::E));
        }
        if last_step.cor.row > 0 {
            out.push(self.next_step(last_step, Direction::N));
        }
        if last_step.cor.col > 0 {
            out.push(self.next_step(last_step, Direction::W));
        }
        out.retain(|s| s.same_direction_counter < 3);

        out
    }

    fn next_step(&self, last_step: &Step, direction: Direction) -> Step {
        let new_cor = last_step.cor + direction;
        Step {
            direction,
            cor: new_cor,
            heat_loss: last_step.heat_loss + self.get_heat_loss(&new_cor) as CumulatedHeatLoss,
            same_direction_counter: if direction == last_step.direction {
                last_step.same_direction_counter + 1
            } else {
                last_step.same_direction_counter
            },
        }
    }

    fn get_heat_loss(&self, cor: &Cor) -> HeatLoss {
        self.map[cor.row as usize][cor.col as usize]
    }
}

impl std::ops::Add<Direction> for Cor {
    type Output = Cor;

    fn add(self, rhs: Direction) -> Cor {
        match rhs {
            Direction::S => Cor { row: self.row + 1, ..self },
            Direction::E => Cor { col: self.col + 1, ..self },
            Direction::N => Cor { row: self.row - 1, ..self },
            Direction::W => Cor { col: self.col - 1, ..self },
        }
    }
}

impl Step {
    fn first() -> Self {
        Self {
            cor: Cor{ row: 0, col: 0 },
            direction: Direction::E,
            heat_loss: 0,
            same_direction_counter: 0,
        }
    }

    fn may_turn_out_better(&self, other: &Step) -> bool {
        self.heat_loss < other.heat_loss || self.direction != other.direction || self.same_direction_counter < other.same_direction_counter
    }
}

impl Ways {
    fn new(map: &HeatLossMap) -> Self {
        let mut steps = Vec::new();
        for _row in 0..map.rows() {
            steps.push((0..map.cols()).map(|_| Vec::<Step>::new()).collect());
        }
        Self { steps }
    }

    fn add(&mut self, step: Step) -> bool {
        let steps = &mut self.steps[step.cor.row as usize][step.cor.col as usize];
        if steps.iter().all(|s| step.may_turn_out_better(s)) {
            steps.push(step);
            return true;
        }
        false
    }
}

impl std::fmt::Display for Ways {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.steps {
            for cell in row {
                write!(f, "{} ", cell.len())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
