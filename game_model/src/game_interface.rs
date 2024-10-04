use anyhow::Result;

#[allow(dead_code)]
pub enum CellState {
    Empty,
    Player,
    Target,
    Obstacle,
}

pub enum GameCommand {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

pub trait GameModelInterface {
    fn cell_state(&self, i: u16, j: u16) -> CellState;
    fn update(&mut self, time: i64) -> Result<()>;
    fn action(&mut self, act: GameCommand) -> Result<()>;
}
