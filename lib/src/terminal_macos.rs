use std::collections::HashMap;
use std::process::{Command, Stdio};

use std::fs::{self, File};
use std::os::unix::fs::PermissionsExt;
use std::{env::temp_dir, io::Write, path::PathBuf};

use crate::{Error, Terminal};

pub(crate) fn open(
    terminal: Terminal,
    command: &str,
    env_vars: HashMap<String, String>,
) -> Result<(), Error> {
    return match terminal {
        Terminal::AppleTerminal => open_with_app("terminal", command, env_vars),
        Terminal::ITerm2 => open_with_app("iterm", command, env_vars),
        Terminal::Warp => open_with_app("warp", command, env_vars),
        Terminal::Ghostty => open_with_app("ghostty", command, env_vars),
        Terminal::Kitty => open_with_app("kitty", command, env_vars),
        Terminal::WezTerm => open_with_wezterm(command, env_vars),
        _ => return Err(Error::NotSupported),
    };
}

pub(crate) fn is_installed(terminal: Terminal) -> Result<bool, Error> {
    let app_name = match terminal {
        Terminal::AppleTerminal => "Terminal",
        Terminal::Warp => "Warp",
        Terminal::ITerm2 => "iTerm",
        Terminal::WezTerm => "WezTerm",
        Terminal::Ghostty => "Ghostty",
        Terminal::Kitty => "Kitty",
        _ => return Err(Error::NotSupported),
    };

    Command::new("osascript")
        .args(["-e", format!("id of application \"{}\"", app_name)])
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .status()
}

fn open_with_app(app: &str, command: &str, env_vars: HashMap<String, String>) -> Result<(), Error> {
    let path = write_temp_script(command, env_vars)?;
    Command::new("open")
        .envs(env_vars)
        .args(["-a", app, path])
        .spawn()?;
    Ok(())
}

fn open_with_wezterm(command: &str, env_vars: HashMap<String, String>) -> Result<(), Error> {
    let path = write_temp_script(command, env_vars)?;

    match Command::new("open").arg("-na").arg("wezterm").arg("--args").arg("start").arg("--").arg(path).spawn() {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IOError(err)),
    }
}

fn write_temp_script(command: &str, env_vars: HashMap<String, String>) -> Result<PathBuf, Error> {
    let dir = temp_dir();
    let path = dir.join("run-in-terminal.sh");

    let mut f = File::create(&path)?;

    let content = if command.is_empty() {
        format!("#!/usr/bin/env sh\n\nexec $SHELL")
    } else {
        format!("#!/usr/bin/env sh\n\n{}\nexec $SHELL", command)
    };

    f.write_all(content.as_bytes()).and_then(|_| f.flush())?;

    let permissions = fs::Permissions::from_mode(0o755);
    fs::set_permissions(&path, permissions)?;

    Ok(path)
}
