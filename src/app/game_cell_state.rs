
#[allow(dead_code)]
pub enum GameCellState {
    Empty,
    Test,
}

pub trait GetGameCellState {
    fn get_game_cell_state(&self, i: u16, j: u16) -> GameCellState;
}
