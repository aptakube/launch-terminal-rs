# launch-terminal

A rust crate to launch Terminal windows on macOS, Windows and Linux

# Why?

There's a of different terminals out there, and they all have different ways of being launched. This crate aims to provide a simple way to launch a terminal window on any platform.

This is currently used by [Aptakube](https://aptakube.com).

# Supported Terminals

## macOS

- Apple Terminal
- iTerm2
- Warp
- Ghostty
- Kitty
- Tabby
- WezTerm

## Windows

- Default Windows Terminal

## Linux

- GNOME Terminal
- Konsole
- Kitty
- Ghostty


# Do you want to add support for another terminal?

To contribute, you need:

- Install `Rust` and `Node.js`
- Clone this repository
- Run `npm install`
- Run `npm run dev`

This will start a demo application that will allow you to test the terminal launchers. You can then add support for new terminals by modifying the files in `lib` folder based on the existing implementations.