#[derive(Debug)]
pub enum GameError {
    CouldNotOpenFile,
    CouldNotReadFile,
    EmptyFile,
    InvalidCharacter,
    CoulNotWriteFile,
    InvalidField,
}
