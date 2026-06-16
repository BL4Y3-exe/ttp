# ttp

A terminal typing practice app inspired by Monkeytype.

Status: early v0.1 development / usable local MVP.

## Features

- Terminal-based speed typing
- Word modes: 10, 25, 50, 100
- Time modes: 15s, 30s, 60s, 120s
- WPM, accuracy, and mistakes
- Vim-inspired controls
- Command mode for switching test modes
- Local SQLite result saving
- Recent history page
- Config with last selected mode
- English-only word list for now

## Run

```bash
cargo run
```

## Controls

Normal mode:

- `s` start typing with the current test
- `r` restart with new text
- `p` open recent history
- `q` quit
- `:` enter command mode
- `Esc` return to Speed-test page

Typing mode:

- text keys type characters
- `Backspace` deletes previous input
- `Esc` aborts the attempt without saving

Command mode:

- `:10`, `:25`, `:50`, `:100`
- `:15s`, `:30s`, `:60s`, `:120s`

## Storage

Preferred paths:

- Config: `~/.config/ttp/config.toml`
- Database: `~/.local/share/ttp/ttp.db`

If those paths are unavailable, `ttp` falls back to local files under `./.ttp/`.

## Roadmap

Possible v0.2+ ideas include themes, personal bests, better stats, cursor trail, and practice mode later.
