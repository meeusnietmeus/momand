# Momand
Moman is a wayland monitor manager (hence the name). This project is the daemon for moman which takes care of automatically turning
on or off your connected displays according to your configuration.

# Supported features
The main use cases are:
- automatically turn on laptop display if it's the only connected display left, after disconnecting your external monitor.

# Compositor support
The following wayland compositors are currently supported:
- Niri

# Configuration

## CLI
The daemon expects 2 arguments:
1. The path to your compositor configuration file where the outputs should be configured. Examples for niri: `outputs.kdl`, `config.kdl`, ...
2. The path to your configuration file (see next section)

>[!INFO] you can also use -h or --help to display this information

## Config file
As aforementioned, the CLI expects the path to your configuration file for moman.
You'll have to make this file yourself e.g. `~/.config/moman/config.kdl`.

A basic moman configuration can then look like this:
```kdl
builtin-monitor "eDP-1"
```
Configuration options can be found in the [unfinished project wiki]

>[!INFO] This configuration file can be manually managed or via [unfinished project]

## Running the daemon
I run mine via my niri config. I suggest you do the same for your compositor.
```kdl
spawn-sh-at-startup "momand ~/.config/niri/outputs.kdl ~/.config/moman/config.kdl"
```
