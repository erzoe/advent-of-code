use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;

static RE_LINE: once_cell::sync::Lazy::<Regex> = once_cell::sync::Lazy::<Regex>::new(|| Regex::new(r"(?<number>[0-9]+)|(?<empty>\.+)|(?<symbol>.)").unwrap());

#[derive(Debug, PartialEq, Eq)]
enum FieldType {
    Empty,
    Number,
    Symbol,
}

struct Symbol {
    symbol: char,
    column: usize,
}

struct Number {
    number: u32,
    start_column: usize,
    end_column: usize,
    is_part_number: bool,
}

struct Row {
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

struct EnginePlan {
    rows: Vec<Row>,
}

fn main() {
    let engine_plan = EnginePlan::read("../../exp");
    println!("sum of part numbers: {}", engine_plan.sum_part_numbers());
}

impl Number {
    fn is_adjacent(&self, symbol: &Symbol) -> bool {
        // using +1 on other side to prevent panic from 0-1
        symbol.column + 1 >= self.start_column && symbol.column < self.end_column + 1
    }
}

impl Row {

    fn parse(ln: &str) -> Self {
        let mut out = Self { numbers: Vec::new(), symbols: Vec::new() };
        let mut last_match = FieldType::Empty;
        for caps in RE_LINE.captures_iter(ln) {
            if let Some(m) = caps.name("symbol") {
                out.symbols.push(Symbol{symbol: m.as_str().chars().next().unwrap(), column: m.start()});
                last_match = FieldType::Symbol;
            } else if let Some(m) = caps.name("number") {
                out.numbers.push(Number{number: m.as_str().parse().unwrap(), start_column: m.start(), end_column: m.end(), is_part_number: last_match == FieldType::Symbol});
                last_match = FieldType::Number;
            } else {
                last_match = FieldType::Empty;
            }
        }
        out
    }
    fn check_part_numbers(&mut self, last_row: &mut Row) {
        for number in &mut self.numbers {
            if last_row.has_corresponding_symbol(number) {
                number.is_part_number = true;
            }
        }
        for symbol in &self.symbols {
            last_row.set_is_part_number_for_adjacent_numbers(symbol);
        }
    }

    fn has_corresponding_symbol(&self, number: &Number) -> bool {
        for symbol in &self.symbols {
            if number.is_adjacent(symbol) {
                return true;
            }
        }
        false
    }
    fn set_is_part_number_for_adjacent_numbers(&mut self, symbol: &Symbol)  {
        for number in &mut self.numbers {
            if number.is_adjacent(symbol) {
                number.is_part_number = true;
            }
        }
    }
}

impl EnginePlan {
    fn read(filename: &str) -> Self {
        Self::parse(BufReader::new(File::open(filename).unwrap_or_else(|_| {panic!("input file '{0}' does not exist", filename)})))
    }
    fn parse(reader: BufReader<File>) -> Self {
        let mut out = Self { rows: Vec::new() };
        for line in reader.lines() {
            let mut row = Row::parse(&line.unwrap());
            if let Some(last_row) = out.rows.last_mut() {
                row.check_part_numbers(last_row)
            }
            out.rows.push(row);
        }
        out
    }

    fn sum_part_numbers (&self) -> u32 {
        let mut sum: u32 = 0;
        for row in &self.rows {
            for number in &row.numbers {
                if number.is_part_number {
                    println!("number: {}", number.number);
                    sum += number.number;
                } else {
                    println!("number: {:>3} (ignored)", number.number);
                }
            }
        }
        sum
    }
}



// ========== tests ==========

#[cfg(test)]
mod tests {
    use crate::Number;
    use crate::Symbol;

    #[test]
    fn test_not_adjacent_symbol_left () {
        let s = Symbol{symbol: '*', column: 0};
        let n = Number{number: 42, start_column: 2, end_column: 4, is_part_number: false};
        assert!(!n.is_adjacent(&s));
    }

    #[test]
    fn test_adjacent_symbol_diagonal_left () {
        let s = Symbol{symbol: '*', column: 1};
        let n = Number{number: 42, start_column: 2, end_column: 4, is_part_number: false};
        assert!(n.is_adjacent(&s));
    }

    #[test]
    fn test_adjacent_symbol_above_start () {
        let s = Symbol{symbol: '*', column: 2};
        let n = Number{number: 42, start_column: 2, end_column: 4, is_part_number: false};
        assert!(n.is_adjacent(&s));
    }

    #[test]
    fn test_adjacent_symbol_above_end () {
        let s = Symbol{symbol: '*', column: 3};
        let n = Number{number: 42, start_column: 2, end_column: 4, is_part_number: false};
        assert!(n.is_adjacent(&s));
    }

    #[test]
    fn test_adjacent_symbol_diagonal_right () {
        let s = Symbol{symbol: '*', column: 4};
        let n = Number{number: 42, start_column: 2, end_column: 4, is_part_number: false};
        assert!(n.is_adjacent(&s));
    }

    #[test]
    fn test_not_adjacent_symbol_right () {
        let s = Symbol{symbol: '*', column: 5};
        let n = Number{number: 42, start_column: 2, end_column: 4, is_part_number: false};
        assert!(!n.is_adjacent(&s));
    }
}
