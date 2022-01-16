use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct UnknownCommandError {
    command_name: String,
}

impl Display for UnknownCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "no handler is associated to command informed: {}", self.command_name)
    }
}

impl UnknownCommandError {
    fn new(command_name: String) -> UnknownCommandError {
        UnknownCommandError { command_name }
    }
}

impl Error for UnknownCommandError {}