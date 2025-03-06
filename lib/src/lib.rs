use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fmt, io};

#[cfg(target_os = "macos")]
mod terminal_macos;

#[cfg(target_os = "windows")]
mod terminal_windows;

#[cfg(target_os = "linux")]
mod terminal_linux;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Terminal {
    AppleTerminal,
    ITerm2,
    Warp,
    WindowsDefault,
    GNOMETerminal,
    Konsole,
    Kitty,
    Ghostty
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

pub fn open(
    terminal: Terminal,
    command: &str,
    env_vars: HashMap<String, String>,
) -> Result<(), Error> {
    #[cfg(target_os = "macos")]
    return terminal_macos::open(terminal, command, env_vars);

    #[cfg(target_os = "windows")]
    return terminal_windows::open(terminal, command);

    #[cfg(target_os = "linux")]
    return terminal_linux::open(terminal, command, env_vars);

    #[allow(unreachable_code)]
    {
        return Err(Error::NotSupported);
    }
}

pub fn is_installed(terminal: Terminal) -> Result<bool, Error> {
    #[cfg(target_os = "macos")]
    return terminal_macos::is_installed(terminal);

    #[cfg(target_os = "windows")]
    return terminal_windows::is_installed(terminal);

    #[cfg(target_os = "linux")]
    return terminal_linux::is_installed(terminal);

    #[allow(unreachable_code)]
    {
        return Err(Error::NotSupported);
    }
}
