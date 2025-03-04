use std::{env::temp_dir, fs::File, io::Write, path::PathBuf, process::{Command, Stdio}};

use crate::{Error, Terminal};

pub(crate) fn open(terminal: Terminal, command: &str) -> Result<(), Error> {
    return match terminal {
        Terminal::GNOMETerminal => open_gnome_terminal(command),
        Terminal::Konsole => open_konsole(command),
        Terminal::Kitty => open_kitty(command),
        Terminal::Ghostty => open_ghostty(command),
        _ => return Err(Error::NotSupported),
    }
}

pub(crate) fn is_installed(terminal: Terminal) -> Result<bool, Error> {
    let bin = match terminal {
        Terminal::GNOMETerminal => "gnome-terminal",
        Terminal::Konsole => "konsole",
        Terminal::Kitty => "kitty",
        Terminal::Ghostty => "ghostty",
        _ => return Err(Error::NotSupported),
    };
    
    return Ok(binary_exists(bin))
}

fn open_ghostty(command: &str) -> Result<(), Error> {
    let path = write_temp_script(command)?;

    let mut cmd = Command::new("ghostty");

    cmd.arg("-e")
        .arg("sh")
        .arg(path)
        .env_remove("PYTHONHOME")
        .env_remove("PYTHONPATH");

    match cmd.spawn() {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IOError(err)),
    }
}

fn open_kitty(command: &str) -> Result<(), Error> {
    let path = write_temp_script(command)?;

    let mut cmd = Command::new("kitty");

    cmd.arg("--working-directory")
        .arg(temp_dir())
        .arg("sh")
        .arg(path)
        .env_remove("PYTHONHOME")
        .env_remove("PYTHONPATH");

    match cmd.spawn() {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IOError(err)),
    }
}

fn open_konsole(command: &str) -> Result<(), Error> {
    let path = write_temp_script(command)?;

    let mut cmd = Command::new("konsole");

    cmd.arg("--workdir")
        .arg(temp_dir())
        .arg("-e")
        .arg(format!("sh {}", path.to_string_lossy()))
        .env_remove("PYTHONHOME")
        .env_remove("PYTHONPATH");

    match cmd.spawn() {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IOError(err)),
    }
}

fn open_gnome_terminal(command: &str) -> Result<(), Error> {
    let mut cmd = Command::new("gnome-terminal");

    cmd.arg("--")
        .arg("bash")
        .arg("-c")
        .arg(format!("{}; exec $SHELL", command))
        .env_remove("PYTHONHOME")
        .env_remove("PYTHONPATH");

    match cmd.spawn() {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IOError(err)),
    }
}

fn binary_exists(name: &str) -> bool {
    match Command::new("which")
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .arg(name)
        .status()
    {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}


pub fn write_temp_script(command: &str) -> Result<PathBuf, Error> {
    let dir = temp_dir();
    let path = dir.join("run-in-terminal.sh");

    let mut f = match File::create(&path) {
        Ok(f) => f,
        Err(err) => return Err(Error::IOError(err)),
    };

    let content = format!("#!/usr/bin/env sh\n\n{}\nexec $SHELL", command);

    match f.write_all(content.as_bytes()).and_then(|_| f.flush()) {
        Ok(_) => Ok(path),
        Err(err) => return Err(Error::IOError(err)),
    }
}
