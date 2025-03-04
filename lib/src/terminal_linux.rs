use crate::{Error, Terminal};
use std::{
    env::temp_dir,
    fs,
    fs::File,
    io::Write,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::{Command, Stdio},
};

pub(crate) fn open(terminal: Terminal, command: &str) -> Result<(), Error> {
    match terminal {
        Terminal::GNOMETerminal => open_gnome_terminal(command),
        Terminal::Konsole => open_konsole(command),
        Terminal::Kitty => open_kitty(command),
        Terminal::Ghostty => open_ghostty(command),
        Terminal::Warp => open_warp(command),
        _ => Err(Error::NotSupported),
    }
}

pub(crate) fn is_installed(terminal: Terminal) -> Result<bool, Error> {
    let bin = match terminal {
        Terminal::GNOMETerminal => "gnome-terminal",
        Terminal::Konsole => "konsole",
        Terminal::Kitty => "kitty",
        Terminal::Ghostty => "ghostty",
        Terminal::Warp => "warp-terminal",
        _ => return Err(Error::NotSupported),
    };
    Ok(binary_exists(bin))
}

fn open_warp(command: &str) -> Result<(), Error> {
    let path = write_temp_script(command)?.to_string_lossy().to_string();
    write_warp_launch_config(temp_dir().to_string_lossy().as_ref(), path.as_str())?;
    match Command::new("warp-terminal")
        .current_dir(temp_dir())
        .args(["warp://launch/aptakube.yaml"])
        .spawn()
    {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IOError(err)),
    }
}

fn open_ghostty(command: &str) -> Result<(), Error> {
    let path = write_temp_script(command)?.to_string_lossy().to_string();
    match Command::new("ghostty")
        .current_dir(temp_dir())
        .args(["-e", path.as_str()])
        .spawn()
    {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IOError(err)),
    }
}

fn open_kitty(command: &str) -> Result<(), Error> {
    let path = write_temp_script(command)?.to_string_lossy().to_string();
    match Command::new("kitty")
        .current_dir(temp_dir())
        .args([path.as_str()])
        .spawn()
    {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IOError(err)),
    }
}

fn open_konsole(command: &str) -> Result<(), Error> {
    let path = write_temp_script(command)?.to_string_lossy().to_string();
    match Command::new("konsole")
        .current_dir(temp_dir())
        .args(["-e", path.as_str()])
        .spawn()
    {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IOError(err)),
    }
}

fn open_gnome_terminal(command: &str) -> Result<(), Error> {
    let path = write_temp_script(command)?.to_string_lossy().to_string();
    match Command::new("gnome-terminal")
        .current_dir(temp_dir())
        .args(["--", path.as_str()])
        .spawn()
    {
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
    let content = format!("#!/usr/bin/env sh\n\n{}\nexec $SHELL", command);
    f.write_all(content.as_bytes())
        .and_then(|_| f.flush())
        .map_err(Error::IOError)?;
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).map_err(Error::IOError)?;
    Ok(path)
}
