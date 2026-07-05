# ttp

A terminal-based typing practice app for focused keyboard practice.

## About

`ttp` is a small Rust TUI typing test app inspired by browser-based typing trainers. It is designed for people who spend time in the terminal and want a fast, keyboard-first way to run typing tests and review local results.

## Current Features

- Terminal-based typing test UI
- Time-based test modes
- Word-count test modes
- WPM, accuracy, and mistake tracking
- Result screen after completed tests
- Local SQLite history storage
- Profile / Stats page with recent history, overall stats, today's stats, and personal bests
- Command-based mode switching
- Multilingual typing language modes
- Layout-independent navigation/control keys where supported by the current input handling
- Config persistence for the selected test mode and selected language mode
- Vim-inspired normal, typing, and command input modes

## Test Modes

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

Supported mode commands:

```text
:10  :25  :50  :100
:15s :30s :60s :120s
```

## Language Modes

v0.3.1 adds multilingual typing language modes:

- `english`
- `english-ext`
- `english-hard`
- `russian`
- `russian-ext`
- `russian-hard`
- `azerbaijani`
- `azerbaijani-ext`
- `azerbaijani-hard`
- `spanish`
- `spanish-ext`
- `spanish-hard`

Supported language commands:

```text
:english
:english-ext
:english-hard
:russian
:russian-ext
:russian-hard
:azerbaijani
:azerbaijani-ext
:azerbaijani-hard
:spanish
:spanish-ext
:spanish-hard
```

Short aliases such as `:en`, `:ru`, `:az`, and `:es` are not available yet.

## Controls

Normal mode:

- `s` start typing with the current mode
- `p` open the Profile / Stats page
- `q` quit
- `:` enter command mode
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

Navigation/control shortcuts are handled separately from typed text input so controls can keep working across supported keyboard layouts while typing still uses the actual produced characters.

## Profile / Stats

Press `p` to open the Profile / Stats page. It includes:

- Today's tests completed, highest WPM, and average WPM
- Personal bests for each supported time and word-count mode
- Overall test count, WPM, and accuracy statistics
- The 15 most recent completed tests, newest first

For v0.3.1, personal bests are still based on the `english` basic language mode only.

## Installation / Build

Requirements:

- Rust toolchain with Cargo
- A terminal emulator with ANSI color support

Build from source:

```bash
cargo build
```

Run with Cargo:

```bash
cargo run
```

Install locally with Cargo:

```bash
cargo install --path .
```

Check the installed version:

```bash
ttp --version
```

Expected version output:

```text
ttp v0.3.1
```

## Storage

`ttp` stores the last selected test mode and language mode in a small TOML config file and completed test results in SQLite.

Preferred paths:

- Config: `~/.config/ttp/config.toml`
- Database: `~/.local/share/ttp/ttp.db`

If those paths are unavailable, `ttp` falls back to local files under `./.ttp/`.

## Project Status

The project is currently at v0.3.1 and remains in active development.

v0.3.1 adds multilingual language modes, language-mode persistence, and layout-independent controls. Future versions may expand word lists, add language-aware history/profile views, broader configuration, themes, UI customization, and richer statistics.

## Screenshots

Screenshots will be added later.

## Development Notes

`ttp` is a learning and personal project focused on building a clean, reliable terminal typing trainer.
