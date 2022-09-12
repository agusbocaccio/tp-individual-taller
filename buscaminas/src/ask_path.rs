use std::io;

use crate::errors::GameError;

/// Asks the path to the user so it can be received through the standard input.
/// It returns a String without the final \n in case of success, otherwise it will return a GameError.
///
/// # Errors
/// * GameError::CouldNotReadStandardInput
/// 
/// # Examples
///
/// ```
/// use buscaminas::ask_path::ask_path;
///
/// assert!(ask_path().is_ok());
/// ```
/// 
pub fn ask_path() -> Result<String, GameError> {
    let mut path = String::new();
    println!("Please enter the file path:");
    match io::stdin().read_line(&mut path) {
        Ok(_) => (),
        Err(_) => return Err(GameError::CouldNotReadStandardInput),
    };

    Ok(path.trim().to_string())
}
