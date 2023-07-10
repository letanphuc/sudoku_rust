mod sudoku;
use glob::glob;
use sudoku::game::Sudoku;

#[macro_use]
extern crate log;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    for entry in glob("./src/data/problem_*.json").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let file = path.into_os_string().into_string().unwrap();
                let mut s = Sudoku::from_json(file.as_str())?;
                s.print();
                s.solve();
                s.print();
            }
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(())
}
