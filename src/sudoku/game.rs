use super::cell::Cell;
use super::position::Position;
use array_macro::array;
use rayon::prelude::*;
use std::io::BufRead;
use std::{fs::File, io};

#[derive(Clone)]
pub struct Sudoku {
    data: [[Cell; 9]; 9],
    pub try_count: i128,
}

impl Sudoku {
    pub const ROWS: [[Position; 9]; 9] =
        array![r => array![c => Position{row: r, column: c}; 9]; 9];
    pub const COLUMNS: [[Position; 9]; 9] =
        array![c => array![r => Position{row: r, column: c}; 9]; 9];
    pub const REGIONS: [[Position; 9]; 9] = Sudoku::regions_arr();

    fn new() -> Sudoku {
        fn get_neighbors(row: usize, column: usize) -> Vec<Position> {
            let tmp_x = row / 3;
            let tmp_y = column / 3;
            let region = tmp_x * 3 + tmp_y;
            let mut neighbors = vec![];
            neighbors.extend_from_slice(&Sudoku::ROWS[row]);
            neighbors.extend_from_slice(&Sudoku::COLUMNS[column]);
            neighbors.extend_from_slice(&Sudoku::REGIONS[region]);
            neighbors
        }
        let data: [[Cell; 9]; 9] =
            array![r => array![c => Cell{value: -1, neighbors: get_neighbors(r, c)}; 9]; 9];
        Sudoku { data, try_count: 0 }
    }

    pub fn solve(&mut self) -> bool {
        let (pos, values) = self.find_best_position();
        info!("pos = {:?}, values = {:?}", &pos, &values);

        if values.is_empty() {
            self.is_ok()
        } else {
            debug!("Try on {:?} wi {:?}", &pos, &values);
            self.try_count += 1;

            let result: Vec<_> = values
                .into_par_iter()
                .filter_map(|v| {
                    let mut tmp = self.clone();
                    tmp.data[pos.row][pos.column].value = v;
                    let ret = tmp.solve();
                    if ret {
                        Some(tmp)
                    } else {
                        None
                    }
                })
                .collect();

            if let Some(b) = result.first() {
                *self = b.clone();
                true
            } else {
                false
            }
        }
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
                    debug!("{:?} -> {:?} values = {:?}", &pos, values.len(), values);

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
        let this_cell = self.get(pos);
        let neighbors = &this_cell.neighbors;

        let neighbors: Vec<i8> = neighbors
            .iter()
            .map(|p| -> i8 { self.data[p.row][p.column].value })
            .collect();

        (1..10)
            .filter(|v| -> bool { !neighbors.contains(v) })
            .collect()
    }

    pub fn print(&self) {
        println!(
            "Status: OK? {}, try count = {}",
            self.is_ok(),
            self.try_count
        );
        for r in &self.data {
            let out: Vec<String> = r.iter().map(|num| -> String { num.as_string() }).collect();
            println!("{}", out.join(" "));
        }
        println!();
    }

    pub fn from_file(file_name: &str) -> Sudoku {
        let mut out = Sudoku::new();
        let file = File::open(file_name).unwrap();
        let lines = io::BufReader::new(file).lines();
        for (line_num, line) in lines.enumerate() {
            if let Ok(content) = line {
                for (i, c) in content.chars().enumerate() {
                    out.data[line_num][i].value = match c {
                        '1'..='9' => c.to_digit(10).unwrap() as i8,
                        _ => -1_i8,
                    }
                }
            }
        }

        out
    }

    fn get(&self, pos: &Position) -> &Cell {
        &(self.data[pos.row][pos.column])
    }

    #[allow(dead_code)]
    const fn regions_arr() -> [[Position; 9]; 9] {
        let mut regions = [[Position { row: 0, column: 0 }; 9]; 9];
        let mut reg = 0;
        while reg < 9 {
            let start_x = (reg / 3) * 3;
            let start_y = (reg % 3) * 3;

            let mut index = 0;
            while index < 9 {
                let x = start_x + index / 3;
                let y = start_y + index % 3;

                regions[reg][index] = Position { row: x, column: y };

                index += 1;
            }

            reg += 1;
        }

        regions
    }

    fn is_ok(&self) -> bool {
        let check_a_zone = |zone: &[[Position; 9]; 9]| -> bool {
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

        check_a_zone(&Sudoku::ROWS)
            && check_a_zone(&Sudoku::COLUMNS)
            && check_a_zone(&Sudoku::REGIONS)
    }
}
