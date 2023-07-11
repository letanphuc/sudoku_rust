mod sudoku;

use glob::glob;
use rayon::prelude::*;
use sudoku::game::Sudoku;

#[macro_use]
extern crate log;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let files: Vec<_> = glob("./data/problem_*.json")
        .expect("Failed to read glob pattern")
        .into_iter()
        .map(|f| f.unwrap())
        .collect();

    let _ = files.into_par_iter().for_each(|p| {
        let file = p.into_os_string().into_string().unwrap();
        let mut s = Sudoku::from_json(file.as_str()).unwrap();
        s.solve();
        s.print();
    });

    Ok(())
}
