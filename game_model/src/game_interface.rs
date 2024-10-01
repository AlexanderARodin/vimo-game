use anyhow::Result;
//type Result<T> = std::io::Result<T>;

#[allow(dead_code)]
pub enum CellState {
    Empty,
    Player,
    Target,
    Obstacle,
}

pub enum GameCommand {
    Up,
    Down,
    Left,
    Right,
}

pub enum GameState {
    Undef,
    Running(GameObjects),
    GameOver(String),
}

pub struct GameObjects {
    pub(super) player: Option<(u16,u16)>,
    pub(super) target: Option<(u16,u16)>,
    pub(super) obstacles: Vec<(u16,u16)>,
}

pub trait GameModelInterface {
    fn cell_state(&self, i: u16, j: u16) -> CellState;
    fn update(&mut self, time: i64) -> Result<()>;
    fn action(&mut self, act: GameCommand) -> Result<()>;
}
