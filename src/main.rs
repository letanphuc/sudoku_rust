use std::env::args;
use std::io::BufRead;
use std::{fs::File, io};

#[derive(Debug, Clone, Copy)]

struct Cell {
    value: i8,
}

impl Cell {
    fn valid(&self) -> bool {
        self.value != -1
    }
    fn new(value: i8) -> Cell {
        Cell { value }
    }

    fn as_char(&self) -> char {
        match self.value {
            1..=9 => std::char::from_digit(self.value as u32, 10).unwrap(),
            _ => '_',
        }
    }

    fn as_string(&self) -> String {
        self.as_char().to_string()
    }
}
#[derive(Debug, Clone, Copy)]
struct Position {
    row: usize,
    column: usize,
}

struct Sudoku {
    data: [[Cell; 9]; 9],
    try_count: i128,
    rows: Vec<Vec<Position>>,
    colums: Vec<Vec<Position>>,
    regions: Vec<Vec<Position>>,
}

impl Sudoku {
    fn new() -> Sudoku {
        Sudoku {
            data: [[Cell::new(-1); 9]; 9],
            try_count: 0,
            rows: Sudoku::rows(),
            colums: Sudoku::columns(),
            regions: Sudoku::regions(),
        }
    }

    fn solve(&mut self) -> bool {
        let (pos, values) = self.find_best_position();
        if values.is_empty() {
            return self.is_ok();
        } else {
            // println!("Try on {:?} wi {:?}", &pos, &values);
            self.try_count += 1;
            for v in values {
                self.data[pos.row][pos.column].value = v;
                if self.solve() {
                    return true;
                }
            }
            // println!("Roll back here!!!!");
            self.data[pos.row][pos.column].value = -1;
        }
        false
    }

    fn find_best_position(&mut self) -> (Position, Vec<i8>) {
        let mut current_min = 9_usize;
        let mut current_position = Position { row: 0, column: 0 };
        let mut current_values = vec![];

        for row in 0..9 {
            for col in 0..9 {
                let pos = Position { row, column: col };
                let cell = self.get(&pos);
                if !cell.valid() {
                    let values = self.get_available_values(&pos);
                    // println!("{:?} -> {:?} values = {:?}", &pos, values.len(), values);

                    if values.len() == 1 {
                        return (pos, values);
                    } else if values.len() < current_min {
                        current_min = values.len();
                        current_position = pos;
                        current_values = values;
                    }
                }
            }
        }
        (current_position, current_values)
    }

    fn get_available_values(&self, pos: &Position) -> Vec<i8> {
        let (row, col, region) = Sudoku::get_zone(pos);

        let mut cells: Vec<Position> = self.rows[row].clone();
        cells.extend(self.colums[col].clone());
        cells.extend(self.regions[region].clone());

        let cells: Vec<i8> = cells
            .into_iter()
            .map(|p| -> i8 { self.data[p.row][p.column].value })
            .collect();
        (1..10).filter(|v| -> bool { !cells.contains(v) }).collect()
    }

    fn get_zone(pos: &Position) -> (usize, usize, usize) {
        let tmp_x = pos.row / 3;
        let tmp_y = pos.column / 3;
        (pos.row, pos.column, tmp_x * 3 + tmp_y)
    }

    fn print(&self) {
        println!("Status: OK? {}", self.is_ok());
        for r in self.data {
            let out: Vec<String> = r
                .into_iter()
                .map(|num| -> String { num.as_string() })
                .collect();
            println!("{}", out.join(" "));
        }
        println!();
    }
    fn from_file(file_name: &str) -> Sudoku {
        let mut out = Sudoku::new();
        let file = File::open(file_name).unwrap();
        let lines = io::BufReader::new(file).lines();
        for (line_num, line) in lines.enumerate() {
            if let Ok(content) = line {
                for (i, c) in content.chars().enumerate() {
                    out.data[line_num][i] = match c {
                        '1'..='9' => Cell::new(c.to_digit(10).unwrap() as i8),
                        _ => Cell::new(-1),
                    }
                }
            }
        }

        out
    }

    fn get(&self, pos: &Position) -> &Cell {
        &(self.data[pos.row][pos.column])
    }

    fn rows() -> Vec<Vec<Position>> {
        (0..9)
            .map(|row| -> Vec<Position> {
                (0..9)
                    .map(|col| -> Position { Position { row, column: col } })
                    .collect()
            })
            .collect()
    }

    fn columns() -> Vec<Vec<Position>> {
        (0..9)
            .map(|col| -> Vec<Position> {
                (0..9)
                    .map(|row| -> Position { Position { row, column: col } })
                    .collect()
            })
            .collect()
    }

    fn regions() -> Vec<Vec<Position>> {
        (0..9)
            .map(|r| -> Vec<Position> {
                let start_x = (r / 3) * 3;
                let start_y = (r % 3) * 3;
                (0..9)
                    .map(|c| -> Position {
                        let x = start_x + c / 3;
                        let y = start_y + c % 3;
                        Position { row: x, column: y }
                    })
                    .collect()
            })
            .collect()
    }

    fn is_ok(&self) -> bool {
        let check_a_zone = |zone: &Vec<Vec<Position>>| -> bool {
            let expected: Vec<i8> = (1..10).collect();
            for row in zone {
                let mut r: Vec<i8> = row.iter().map(|p| -> i8 { self.get(p).value }).collect();
                r.sort_unstable();
                if r != expected {
                    return false;
                }
            }
            true
        };

        check_a_zone(&self.rows) && check_a_zone(&self.colums) && check_a_zone(&self.regions)
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let file_name = &args[1];
    let mut s = Sudoku::from_file(file_name.as_str());
    s.print();

    s.solve();
    s.print();
    println!("end, try count = {}", s.try_count);
}
