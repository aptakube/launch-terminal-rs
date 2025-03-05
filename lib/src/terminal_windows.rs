use std::process::Command;
use std::collections::HashMap;

use shlex::Shlex;

use crate::{Error, Terminal};

pub(crate) fn open(terminal: Terminal, command: &str, env_vars: HashMap<String, String>) -> Result<(), Error> {
    return match terminal {
        Terminal::WindowsDefault => open_with_cmd(command, env_vars),
        _ => return Err(Error::NotSupported),
    }
}

pub(crate) fn is_installed(terminal: Terminal) -> Result<bool, Error> {
    return match terminal {
        Terminal::WindowsDefault => Ok(true),
        _ => return Err(Error::NotSupported),
    };
}

fn open_with_cmd(command: &str, env_vars: HashMap<String, String>) -> Result<(), Error> {
    let args: Vec<String> = Shlex::new(command).collect();
    let mut cmd = Command::new("cmd.exe");

    cmd
        .arg("/c")
        .arg("start")
        .arg("cmd")
        .arg("/k")
        .args(args);

    for (key, value) in env_vars.iter() {
        cmd.env(key, value);
    }

    match cmd.spawn() {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IOError(err)),
    }
}