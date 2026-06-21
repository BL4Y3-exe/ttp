# ttp

A terminal-based typing trainer for focused keyboard practice.

## About

`ttp` is a small Rust TUI typing practice application inspired by browser-based typing trainers. It is designed for people who spend time in the terminal and want a fast, keyboard-first way to run typing tests and review local results.

## Features

- Terminal-based typing tests with English text
- Time modes and word-count modes
- WPM, accuracy, and mistake tracking
- Centered, left-aligned typing area with a maximum of three visible lines
- Line-based text scrolling while typing
- Timer or word-progress counter above the typing text
- Block caret rendering that follows wrapped text
- Local SQLite result storage
- Profile / Stats page with today's statistics, personal bests, overall statistics, and recent history
- Line-based Profile / Stats scrolling with `j` and `k`
- Vim-inspired normal, typing, and command input modes

## Modes

Time-based tests:

- `15s`
- `30s`
- `60s`
- `120s`

Word-count tests:

- `10` words
- `25` words
- `50` words
- `100` words

Time modes show the remaining time. Word-count modes show completed words, such as `3/25`.

## Controls

Normal mode:

- `s` start typing with the current mode
- `p` open the Profile / Stats page
- `q` quit
- `:` enter command mode to change the test mode
- `Esc` stay in normal mode and clear an on-screen error

Profile / Stats page, normal mode:

- `j` scroll down one line
- `k` scroll up one line

Typing mode:

- Character keys enter typing input
- `Backspace` deletes the previous typed character
- `Esc` aborts the current attempt without saving it

Command mode:

- `Enter` apply the command
- `Backspace` delete a command character
- `Esc` cancel command mode

Supported mode commands:

```text
:10  :25  :50  :100
:15s :30s :60s :120s
```

## Profile / Stats

Press `p` to open the Profile / Stats page. It includes:

- Today's tests completed, highest WPM, and average WPM
- Personal bests for each supported time and word-count mode
- Overall test count, WPM, and accuracy statistics
- The 15 most recent completed tests, newest first

## Installation / Build

Requirements:

- Rust toolchain with Cargo
- A terminal emulator with ANSI color support

Build the project:

```bash
cargo build
```

Run the application:

```bash
cargo run
```

## Storage

`ttp` stores the last selected mode in a small TOML config file and completed test results in SQLite.

Preferred paths:

- Config: `~/.config/ttp/config.toml`
- Database: `~/.local/share/ttp/ttp.db`

If those paths are unavailable, `ttp` falls back to local files under `./.ttp/`.

## Project Status

The project is currently around v0.2.2 and remains in active development.

Future versions may add broader config-file support, themes, UI customization, caret options, an activity grid, and speed or accuracy graphs. These features are not implemented yet.

## Screenshots

Screenshots will be added later.

## Development Notes

`ttp` is a learning and personal project focused on building a clean, reliable terminal typing trainer.
