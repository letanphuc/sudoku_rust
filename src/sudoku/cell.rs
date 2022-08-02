use super::position::Position;

#[derive(Clone)]
pub struct Cell {
    pub value: i8,
    pub neighbors: Vec<Position>,
}

impl Cell {
    pub fn valid(&self) -> bool {
        self.value != -1
    }

    pub fn as_char(&self) -> char {
        match self.value {
            1..=9 => std::char::from_digit(self.value as u32, 10).unwrap(),
            _ => '_',
        }
    }

    pub fn as_string(&self) -> String {
        self.as_char().to_string()
    }
}
