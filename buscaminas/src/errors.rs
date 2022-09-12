///Error types
#[derive(Debug)]
pub enum GameError {
    ///This error happens when the entered file couldn't be opened.
    CouldNotOpenFile,
    ///This error happens when the entered file couldn't be read.
    CouldNotReadFile,
    ///This error happens when the standard input couldn't be read.
    CouldNotReadStandardInput,
    ///This error happens when the entered file has a different character than '*' or '.'.
    InvalidCharacter,
    ///This error happens when the file to write on couldn't be opened.
    CoulNotWriteFile,
    ///This error happens when the given field isn't reactangular.
    InvalidField,
}
