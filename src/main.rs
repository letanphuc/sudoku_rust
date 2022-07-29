use std::io::BufRead;
use std::{cell, vec};
use std::{fs::File, io};

#[derive(Debug, Clone, Copy)]

struct Cell {
    value: i8,
    is_fixed: bool,
}

impl Cell {
    fn valid(&self) -> bool {
        self.value != -1
    }
    fn new(value: i8, is_fixed: bool) -> Cell {
        Cell { value, is_fixed }
    }

    fn to_char(&self) -> char {
        match self.value {
            1..=9 => std::char::from_digit(self.value as u32, 10).unwrap(),
            _ => '_',
        }
    }

    fn to_string(&self) -> String {
        self.to_char().to_string()
    }
}
#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

struct Sudoku {
    data: [[Cell; 9]; 9],
    empty_positions: Vec<Position>,
    try_count: i128,
    rows: Vec<Vec<Position>>,
    colums: Vec<Vec<Position>>,
    regions: Vec<Vec<Position>>,
}

impl Sudoku {
    fn new() -> Sudoku {
        Sudoku {
            data: [[Cell::new(-1, false); 9]; 9],
            empty_positions: Vec::new(),
            try_count: 0,
            rows: Sudoku::rows(),
            colums: Sudoku::columns(),
            regions: Sudoku::regions(),
        }
    }

    fn solve(&mut self) {
        for row in 0..9 {
            for col in 0..9 {
                let pos = Position { x: row, y: col };
                let cell = self.get(&pos);
                if !cell.valid() {
                    self.empty_positions.push(pos);
                }
            }
        }
        dbg!(&(self.empty_positions));
        self.solve_internal(0);
    }

    fn solve_internal(&mut self, index: usize) -> bool {
        if self.is_ok() {
            println!("Done, OK:");
            self.print();
            return true;
        }

        println!("Try {index} {}", self.try_count);
        self.try_count += 1;
        if self.empty_positions.len() > index {
            let pos = self.empty_positions[index];
            let (row, col, region) = Sudoku::get_zone(&pos);

            let mut cells: Vec<Position> = self.rows[row].clone();
            cells.extend(self.colums[col].clone());
            cells.extend(self.regions[region].clone());

            let cells: Vec<i8> = cells
                .into_iter()
                .map(|p| -> i8 { self.data[p.x][p.y].value })
                .collect();
            let cells: Vec<i8> = (1..10).filter(|v| -> bool { !cells.contains(v) }).collect();

            for try_value in cells {
                self.data[pos.x][pos.y].value = try_value;
                if self.solve_internal(index + 1) {
                    return true;
                }
            }
            self.data[pos.x][pos.y].value = -1;
        }
        false
    }

    fn get_zone(pos: &Position) -> (usize, usize, usize) {
        let tmp_x = pos.x / 3;
        let tmp_y = pos.y / 3;
        (pos.x, pos.y, tmp_x * 3 + tmp_y)
    }

    fn print(&self) {
        for r in self.data {
            let out: Vec<String> = r
                .into_iter()
                .map(|num| -> String { num.to_string() })
                .collect();
            println!("{}", out.join(" "));
        }
    }
    fn from_file(file_name: &str) -> Sudoku {
        let mut out = Sudoku::new();
        let file = File::open(file_name).unwrap();
        let lines = io::BufReader::new(file).lines();
        for (line_num, line) in lines.enumerate() {
            if let Ok(content) = line {
                for (i, c) in content.chars().enumerate() {
                    out.data[line_num][i] = match c {
                        '1'..='9' => Cell::new(c.to_digit(10).unwrap() as i8, true),
                        _ => Cell::new(-1, false),
                    }
                }
            }
        }

        out
    }

    fn get(&self, pos: &Position) -> &Cell {
        &(self.data[pos.x][pos.y])
    }

    fn rows() -> Vec<Vec<Position>> {
        (0..9)
            .map(|row| -> Vec<Position> {
                (0..9)
                    .map(|col| -> Position { Position { x: row, y: col } })
                    .collect()
            })
            .collect()
    }

    fn columns() -> Vec<Vec<Position>> {
        (0..9)
            .map(|col| -> Vec<Position> {
                (0..9)
                    .map(|row| -> Position { Position { x: row, y: col } })
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
                        Position { x, y }
                    })
                    .collect()
            })
            .collect()
    }

    fn is_ok(&self) -> bool {
        let expected: Vec<i8> = (1..10).collect();
        for row in Sudoku::rows() {
            let mut r: Vec<i8> = row
                .into_iter()
                .map(|p| -> i8 { self.get(&p).value })
                .collect();
            r.sort();
            if r != expected {
                return false;
            }
        }
        for cols in Sudoku::columns() {
            let mut r: Vec<i8> = cols
                .into_iter()
                .map(|p| -> i8 { self.get(&p).value })
                .collect();
            r.sort();
            if r != expected {
                return false;
            }
        }

        for regions in Sudoku::regions() {
            let mut r: Vec<i8> = regions
                .into_iter()
                .map(|p| -> i8 { self.get(&p).value })
                .collect();
            r.sort();
            if r != expected {
                return false;
            }
        }

        true
    }
}

fn main() {
    // let s = Sudoku::from_file("./src/data/example1_ok.txt");
    let mut s = Sudoku::from_file("./src/data/example1.txt");
    s.print();
    let rows = Sudoku::rows();
    let cols = Sudoku::columns();
    let regions = Sudoku::regions();

    s.solve();

    println!("ok = {}", s.is_ok());
    println!("end");
}
