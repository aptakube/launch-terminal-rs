use crate::{Error, Terminal};
use shlex::Shlex;
use std::collections::HashMap;
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
    let path = write_temp_script(command, env_vars)?;
    Command::new("wt.exe")
        .current_dir(path.parent().unwrap())
        .arg("wsl")
        .arg("--")
        .arg(format!("./{:?}", path.file_name().unwrap()))
        .spawn()?;
    Ok(())
}

fn open_with_cmd(command: &str, env_vars: HashMap<String, String>) -> Result<(), Error> {
    let args: Vec<String> = Shlex::new(command).collect();
    let mut cmd = Command::new("cmd.exe");
    cmd.current_dir(cwd()).arg("/c").arg("start").arg("cmd").arg("/k").args(args);
    for (key, value) in env_vars.iter() {
        cmd.env(key, value);
    }
    cmd.spawn()?;
    Ok(())
}

fn stringify_env_vars(env_vars: HashMap<String, String>) -> String {
    env_vars
        .iter()
        .map(|(key, value)| format!("{}='{}'", key, value))
        .collect::<Vec<String>>()
        .join(" ")
}

fn write_temp_script(command: &str, env_vars: HashMap<String, String>) -> Result<PathBuf, Error> {
    let dir = temp_dir();
    let path = dir.join("run-in-terminal.sh");

    let mut f = File::create(&path)?;

    let set_env = stringify_env_vars(env_vars);
    let content = if command.is_empty() {
        format!("#!/usr/bin/env sh\n\n{} exec $SHELL", set_env)
    } else {
        format!("#!/usr/bin/env sh\n\n{} {}\n{} exec $SHELL", set_env, command, set_env)
    };
    f.write_all(content.as_bytes()).and_then(|_| f.flush())?;
    Ok(path)
}

fn cwd() -> PathBuf {
    home_dir().unwrap_or(temp_dir())
}
