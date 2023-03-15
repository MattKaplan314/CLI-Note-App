use std::fmt::{Formatter, Display};
use std::fmt;
use std::write;
use std::str::{self, Utf8Error};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Request<'a> {
    pub exepath: &'a str,
    pub command: &'a str,
    pub path: &'a str,
    pub note: &'a str,
}


// rewrite this to work with &mut vec<String>
impl<'a> TryFrom<&'a [u8]> for Request<'a> {
   type Error = ParseError;

   fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
    let request = str::from_utf8(value)?;
    let (exepath, request) = next_word(request).ok_or(ParseError::InvalidRequest)?;
    let (command, request) = next_word(request).ok_or(ParseError::InvalidCommand)?;

    if command == "help" {
        let path = "";
        let note = "";
        return Ok(Request {exepath, command, path, note})
    }

    let (path, request) = next_word(request).ok_or(ParseError::StupidUser)?;
    let note = request; 
    Ok(Request {exepath, command, path, note }) 
   } 
} 

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidRequest
    }
}

impl<'a> Display for Request<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "exepath: {} command: {} path: {}
                 note: {}", self.exepath, 
                  self.command, self.path, self.note)
    }
}



fn next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' {
            return Some((&request[..i], &request[i+1..]));
        }
    }
    None
}

#[derive(Debug)]
pub enum ParseError {
    InvalidCommand,
    InvalidPath,
    InvalidRequest,
    StupidUser,
    StupidDeveloper,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidCommand => "Invalid Command!",
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidPath => "Invalid Path",
            Self::StupidUser => "Your dumb please learn to use my software",
            Self::StupidDeveloper => "nobodys perfect fuckhead cry about it"
        }
    }
}
    
