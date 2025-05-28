use crate::{Error, Terminal};
use std::collections::HashMap;
use std::env::home_dir;
use std::{
    env::temp_dir,
    fs,
    fs::File,
    io::Write,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::{Command, Stdio},
};

type Launcher = fn(&mut Command, &str) -> Result<(), Error>;

pub(crate) fn open(
    terminal: Terminal,
    command: &str,
    env_vars: HashMap<String, String>,
) -> Result<(), Error> {
    let (mut cmd, launcher) = new_command(terminal, env_vars)?;
    if let Some(path) = write_temp_script(command)?.to_str() {
        launcher(&mut cmd, path)
    } else {
        Err(Error::IOError(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid path",
        )))
    }
}

pub(crate) fn is_installed(terminal: Terminal) -> Result<bool, Error> {
    let (bin, _) = command_name(terminal)?;
    Ok(binary_exists(bin))
}

fn command_name(terminal: Terminal) -> Result<(&'static str, Launcher), Error> {
    let map = match terminal {
        Terminal::GNOMETerminal => ("gnome-terminal", open_gnome_terminal as Launcher),
        Terminal::Konsole => ("konsole", open_konsole as Launcher),
        Terminal::Kitty => ("kitty", open_kitty as Launcher),
        Terminal::Ghostty => ("ghostty", open_ghostty as Launcher),
        Terminal::Warp => ("warp-terminal", open_warp as Launcher),
        _ => Err(Error::NotSupported)?,
    };
    Ok(map)
}

fn new_command(term: Terminal, env: HashMap<String, String>) -> Result<(Command, Launcher), Error> {
    let (bin, launcher) = command_name(term)?;
    let mut cmd = Command::new(bin);
    cmd.envs(env).current_dir(cwd());

    if std::env::var("APPIMAGE").is_ok() {
        // AppImage sets its own PYTHONHOME and PYTHONPATH variables and we don't want that to leak into the new terminal
        // If we don't remove them, some terminals like gnome-terminal and kitty won't launch on AppImage
        cmd.env_remove("PYTHONHOME").env_remove("PYTHONPATH");
    }

    Ok((cmd, launcher))
}

fn open_warp(command: &mut Command, path: &str) -> Result<(), Error> {
    write_warp_launch_config(cwd().to_string_lossy().as_ref(), path)?;
    match command.args(["warp://launch/aptakube.yaml"]).spawn() {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IOError(err)),
    }
}

fn open_ghostty(commnad: &mut Command, path: &str) -> Result<(), Error> {
    match commnad.args(["-e", path]).spawn() {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IOError(err)),
    }
}

fn open_kitty(command: &mut Command, path: &str) -> Result<(), Error> {
    match command.args([path]).spawn() {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IOError(err)),
    }
}

fn open_konsole(command: &mut Command, path: &str) -> Result<(), Error> {
    match command.args(["-e", path]).spawn() {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IOError(err)),
    }
}

fn open_gnome_terminal(command: &mut Command, path: &str) -> Result<(), Error> {
    match command.args(["--", path]).spawn() {
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

fn write_warp_launch_config(cwd: &str, script_path: &str) -> Result<(), Error> {
    // Find out where to store the launch config
    // https://docs.warp.dev/features/sessions/launch-configurations
    // ${XDG_DATA_HOME:-$HOME/.local/share}/warp-terminal/launch_configurations/
    let xdg_data_home = match std::env::var("XDG_DATA_HOME") {
        Ok(val) => val,
        Err(_) => {
            let home = match std::env::var("HOME") {
                Ok(val) => val,
                Err(_) => return Err(Error::NotSupported),
            };
            format!("{}/.local/share", home)
        }
    };
    let warp_dir = format!("{}/warp-terminal/launch_configurations", xdg_data_home);
    fs::create_dir_all(&warp_dir).map_err(Error::IOError)?;
    let file_path = PathBuf::from(format!("{}/aptakube.yaml", warp_dir));
    let mut f = File::create(&file_path).map_err(Error::IOError)?;
    let content = format!(
        r#"---
name: Aptakube
active_window_index: 0
windows:
  - active_tab_index: 0
    tabs:
      - layout:
          cwd: {}
          is_focused: true
          commands:
            - exec: {}"#,
        cwd, script_path
    );
    f.write_all(content.as_bytes())
        .and_then(|_| f.flush())
        .map_err(Error::IOError)?;
    Ok(())
}

fn write_temp_script(command: &str) -> Result<PathBuf, Error> {
    let dir = temp_dir();
    let path = dir.join("run-in-terminal.sh");

    let mut f = File::create(&path).map_err(Error::IOError)?;

    let content = if command.is_empty() {
        format!("#!/usr/bin/env sh\n\ncd $HOME\nexec $SHELL")
    } else {
        format!("#!/usr/bin/env sh\n\ncd $HOME\n{}\nexec $SHELL", command)
    };
    f.write_all(content.as_bytes())
        .and_then(|_| f.flush())
        .map_err(Error::IOError)?;
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).map_err(Error::IOError)?;
    Ok(path)
}

fn cwd() -> PathBuf {
    home_dir().unwrap_or(temp_dir())
}