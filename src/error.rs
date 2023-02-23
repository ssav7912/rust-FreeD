use std::{error::Error, fmt::Display};

use crate::common::*;

#[derive(Debug, Clone)]
pub struct DeserialiseError {
    pub description: String,
}
impl DeserialiseError {
    pub fn length_template(length: usize, payload: &str) -> String {
        return format!("Misformed data - the array must be exactly {} bytes for the payload type: {}", length, payload);
    }
}
impl std::fmt::Display for DeserialiseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

#[derive(Debug)]
pub struct InvalidCommand {
    pub allowedcommands: (Commands, Commands),
    pub payload: crate::payloads::Payloads,
}

impl Display for InvalidCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Only {} or {} commands may be used with the {} payload", self.allowedcommands.1, self.allowedcommands.1, self.payload)
    }   
}

impl Error for InvalidCommand {}