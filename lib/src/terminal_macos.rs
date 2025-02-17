use std::process::{Command, Stdio};

use crate::{Error, Terminal};

pub(crate) fn open(terminal: Terminal, command: &str) -> Result<(), Error> {
    return match terminal {
        Terminal::AppleTerminal => open_apple_terminal(command),
        Terminal::ITerm2 => open_iterm2(command),
        _ => return Err(Error::NotSupported),
    }
}

pub(crate) fn is_installed(terminal: Terminal) -> Result<bool, Error> {
    let app_name = match terminal {
        Terminal::AppleTerminal => "Terminal",
        Terminal::ITerm2 => "iTerm",
        _ => return Err(Error::NotSupported),
    };


    let found = match Command::new("osascript")
        .arg("-e")
        .arg(format!("id of application \"{}\"", app_name))
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .status()
    {
        Ok(status) => status.success(),
        Err(err) => return Err(Error::IOError(err)),
    };

   return Ok(found)
}


fn open_apple_terminal(command: &str) -> Result<(), Error> {
    let script = format!(
        r#"tell application "Terminal"
        do script "{}"
    activate
  end"#, command);

    return run_osascript(script);
}

fn open_iterm2(command: &str) -> Result<(), Error> {
    let script = format!(
        r#"tell application "iTerm"
    set newWindow to (create window with default profile)
    tell current session of newWindow
        write text "{}"
    end tell
end tell"#,
        command
    );

    return run_osascript(script);
}

fn run_osascript(script: String) -> Result<(), Error> {
    let mut cmd = Command::new("osascript");
    for line in script.lines() {
        cmd.arg("-e").arg(line);
    }

    match cmd.spawn() {
        Ok(_) => return Ok(()),
        Err(err) => return Err(Error::IOError(err)),
    }
}