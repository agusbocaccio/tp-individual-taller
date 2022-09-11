#[derive(Debug)]
pub enum GameError {
    CouldNotOpenFile,
    CouldNotReadFile,
    CouldNotReadStandardInput,
    InvalidCharacter,
    CoulNotWriteFile,
    InvalidField,
}
