use std::{io, process::{Child, Command}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Terminal {
    AppleTerminal,
    ITerm2,
    WindowsDefault,
    GNOMETerminal
}

pub fn open(terminal: Terminal, command: &str) -> io::Result<Child> {
    match terminal {
        Terminal::AppleTerminal => open_apple_terminal(command),
        Terminal::ITerm2 => open_iterm2(command),
        _ => unimplemented!()   
    }
}

fn open_apple_terminal(command: &str) -> io::Result<Child> {
    let script = format!(
        r#"tell application "Terminal"
        do script "{}"
    activate
  end"#, command);

    return run_osascript(script);
}

fn open_iterm2(command: &str) -> io::Result<Child> {
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

fn run_osascript(script: String) -> io::Result<Child> {
    let mut cmd = Command::new("osascript");
    for line in script.lines() {
        cmd.arg("-e").arg(line);
    }

    return cmd.spawn();
}