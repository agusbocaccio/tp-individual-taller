#[derive(Debug)]
pub enum GameError {
    CouldNotOpenFile,
    CouldNotReadFile,
    BombCellCantHaceAdjacentBombs
}
