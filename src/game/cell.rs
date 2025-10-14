#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Alive,
    Dead,
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        matches!(self, Cell::Alive)
    }

    #[allow(dead_code)]
    pub fn is_dead(&self) -> bool {
        matches!(self, Cell::Dead)
    }

    pub fn toggle(&mut self) {
        *self = match self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        };
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Dead
    }
}

impl From<bool> for Cell {
    fn from(alive: bool) -> Self {
        if alive {
            Cell::Alive
        } else {
            Cell::Dead
        }
    }
}