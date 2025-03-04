use std::process::Command;

use shlex::Shlex;

use crate::{Error, Terminal};

pub(crate) fn open(terminal: Terminal, command: &str) -> Result<(), Error> {
    return match terminal {
        Terminal::WindowsDefault => open_with_cmd(command),
        _ => return Err(Error::NotSupported),
    }
}

pub(crate) fn is_installed(terminal: Terminal) -> Result<bool, Error> {
    return match terminal {
        Terminal::WindowsDefault => Ok(true),
        _ => return Err(Error::NotSupported),
    };
}

fn open_with_cmd(command: &str) -> Result<(), Error> {
    let args: Vec<String> = Shlex::new(command).collect();
    let mut cmd = Command::new("cmd.exe");

    cmd
        .arg("/c")
        .arg("start")
        .arg("cmd")
        .arg("/k")
        .args(args);

    match cmd.spawn() {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IOError(err)),
    }
}