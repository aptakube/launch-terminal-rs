use std::{fmt, io};
use serde::{Deserialize, Serialize};

#[cfg(target_os = "macos")]
mod terminal_macos;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Terminal {
    AppleTerminal,
    ITerm2,
    WindowsDefault,
    GNOMETerminal
}

#[derive(Debug)]
pub enum Error {
    NotSupported,
    IOError(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn open(terminal: Terminal, command: &str) -> Result<(), Error> {
    #[cfg(target_os = "macos")]
    return terminal_macos::open(terminal, command);

    #[allow(unreachable_code)]
    {
        return Err(Error::NotSupported);
    }
}


pub fn is_installed(terminal: Terminal) -> Result<bool, Error> {
    #[cfg(target_os = "macos")]
    return terminal_macos::is_installed(terminal);

    #[allow(unreachable_code)]
    {
        return Err(Error::NotSupported);
    }
}
