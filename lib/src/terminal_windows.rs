use crate::{Error, Terminal};
use shlex::Shlex;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::{env::home_dir, env::temp_dir, fs::File, io::Write, path::PathBuf, process::Command};

pub(crate) fn open(
    terminal: Terminal,
    command: &str,
    env_vars: HashMap<String, String>,
) -> Result<(), Error> {
    return match terminal {
        Terminal::WindowsDefault => open_with_cmd(command, env_vars),
        Terminal::WSL => open_with_wsl(command, env_vars),
        _ => return Err(Error::NotSupported),
    };
}

pub(crate) fn is_installed(terminal: Terminal) -> Result<bool, Error> {
    return match terminal {
        Terminal::WindowsDefault => Ok(true),
        Terminal::WSL => Ok(true),
        _ => return Err(Error::NotSupported),
    };
}

fn open_with_wsl(command: &str, env_vars: HashMap<String, String>) -> Result<(), Error> {
    let path = write_temp_script(command)?;
    let mut cmd = Command::new("wt.exe");
    let dir = match path.parent() {
        Some(dir) => dir,
        None => new_error("Invalid path"),
    };
    let file_name = match path.file_name() {
        Some(file_name) => file_name,
        None => new_error("Invalid filename"),
    };
    cmd.current_dir(dir)
        .envs(env_vars)
        .args(["wsl", "--", format!("./{:?}", file_name)])
        .spawn()?;
    Ok(())
}

fn open_with_cmd(command: &str, env_vars: HashMap<String, String>) -> Result<(), Error> {
    let args: Vec<String> = Shlex::new(command).collect();
    let mut cmd = Command::new("cmd.exe");
    cmd.current_dir(cwd())
        .envs(env_vars)
        .args(["/c", "start", "cmd", "/k"].iter().chain(args))
        .spawn()?;
    Ok(())
}

fn write_temp_script(command: &str) -> Result<PathBuf, Error> {
    let dir = temp_dir();
    let path = dir.join("run-in-terminal.sh");

    let mut f = File::create(&path)?;

    let content = if command.is_empty() {
        format!("#!/usr/bin/env sh\n\n exec $SHELL")
    } else {
        format!("#!/usr/bin/env sh\n\n{} exec $SHELL", command)
    };
    f.write_all(content.as_bytes()).and_then(|_| f.flush())?;
    Ok(path)
}

fn cwd() -> PathBuf {
    home_dir().unwrap_or(temp_dir())
}

fn new_error(message: &str) -> Error {
    Error::IOError(io::Error::new(ErrorKind::Other, message))
}
