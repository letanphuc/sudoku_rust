mod sudoku;
use std::env::args;
use sudoku::game::Sudoku;

#[macro_use]
extern crate log;

fn main() {
    env_logger::init();
    let args: Vec<String> = args().collect();
    let file_name = &args[1];

    info!("Try to solve problem in file {}", &file_name);

    let mut s = Sudoku::from_file(file_name.as_str());

    s.print();

    s.solve();

    s.print();
}
