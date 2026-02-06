# Momand
`momand` is an automation daemon for Wayland monitor management. 
It monitors your hardware state and dynamically manages output configurations. 
It is specifically designed to handle transitions between docked (external monitor) and portable (builtin display) states.

# Core functionality
The main use cases are:
- automatically turn on laptop display if it's the only connected display left, after disconnecting your external monitor.

# Compositor support
`momand` interacts with compositor-specific configuration formats. Currently supported:

| Compositor | Configuration Format |
| :--- | :--- |
| **Niri** | KDL |

---
# Configuration

## CLI
The daemon expects 2 arguments:
1. The path to your compositor configuration file where the outputs should be configured. Examples for niri: `outputs.kdl`, `config.kdl`, ...
2. The path to your configuration file (see next section)

> [!TIP] 
> you can use -h or --help to display this information as well

## Configuration file
As aforementioned, the CLI expects the path to your configuration file for moman.
You'll have to make this file yourself e.g. `~/.config/moman/config.kdl`.

A basic moman configuration can then look like this:
```kdl
// The laptop display name. If this KDL node is used, the laptop display will turn on automatically when external monitors are disconnected.
// "eDP-1" is the default name for the built in display and as such the following will work too:
// builtin-monitor
builtin-monitor "eDP-1"
```
Configuration options can be found in the [unfinished project wiki](#)

> [!INFO] 
> This configuration file can be manually managed or via [unfinished project](#)

## Usage
I run mine via my niri config. I suggest you do the same for your compositor.
```kdl
spawn-sh-at-startup "momand ~/.config/niri/outputs.kdl ~/.config/moman/config.kdl"
```
