# MASTER_PROMPT.md — ttp Development Prompt

You are working on a Rust terminal application called `ttp`.

`ttp` is a terminal-based typing practice application inspired by Monkeytype. The goal is to create a fast, lightweight, polished, keyboard-focused TUI typing trainer for users who spend a lot of time in the terminal.

Before making any implementation decisions, read and follow:

```txt
PROJECT_SPEC_V0.1.md
```

This file is the source of truth for the current version of the project.

## Current Project State

At the beginning, the project directory may contain only:

```txt
PROJECT_SPEC_V0.1.md
```

You must create all required project files and folders yourself.

This includes, but is not limited to:

```txt
Cargo.toml
Cargo.lock
src/
src/main.rs
src/app.rs
src/event.rs
src/command.rs
src/ui/
src/core/
src/storage/
src/theme/
README.md
```

Create the project structure according to the specification and adjust it only when it improves maintainability without changing the v0.1 scope.

## Project Name

Temporary project name:

```txt
ttp
```

The binary name should be:

```bash
ttp
```

## Technical Stack

Use the following stack unless there is a strong technical reason not to:

```txt
Language: Rust
TUI framework: Ratatui
Terminal backend/events: Crossterm
CLI arguments: clap
Serialization: serde
Config format: TOML
Local database: SQLite
SQLite crate: rusqlite or sqlx
Time/date handling: chrono or time
Random generation: rand
```

Prefer stable, simple, maintainable dependencies.

Do not add unnecessary dependencies.

## Main Rule

Implement only `v0.1`.

Do not implement future features unless explicitly requested.

The following features are out of scope for v0.1:

```txt
Practice mode
Per-key statistics
Daily goals
Personal bests
Full dashboard
Activity grid
Line graphs
Keyboard heatmap
Multiple themes
Theme command
Cursor trail
Advanced animations
Multiple languages
Punctuation mode
Numbers mode
Online accounts
Leaderboards
Sync
```

The architecture may be prepared for these features, but they must not appear as user-facing functionality in v0.1.

## v0.1 Required Features

Implement:

```txt
Speed-test mode only
English typing language only
Word modes: 10, 25, 50, 100
Time modes: 15s, 30s, 60s, 120s
WPM calculation
Accuracy calculation
Mistakes count
Result screen after test completion
Local result saving
Basic history page with recent test results
Vim-inspired control system
Normal mode
Typing mode
Command mode
Basic config file
Last selected test mode saving
Simple clean theme
```

## Architecture Requirements

Keep the code modular.

Separate:

```txt
typing logic
TUI rendering
input event handling
command parsing
storage/database logic
config handling
theme/style definitions
```

The typing engine must not depend on Ratatui types.

The storage layer must not depend on UI code.

The command parser must be testable separately from the TUI.

The UI should render existing app state; it should not contain core business logic.

## Suggested Project Structure

Use this structure as the default starting point:

```txt
src/
  main.rs
  app.rs
  event.rs
  command.rs

  ui/
    mod.rs
    speed_test.rs
    result.rs
    history.rs
    components/
      mod.rs
      typing_area.rs
      top_panel.rs

  core/
    mod.rs
    typing_engine.rs
    test_session.rs
    text_generator.rs
    scoring.rs

  storage/
    mod.rs
    database.rs
    models.rs
    config.rs

  theme/
    mod.rs
    default.rs
```

You may slightly adjust this structure if needed, but keep the same separation of concerns.

## Control System

The app uses a Vim-inspired control system.

There are three input modes:

```txt
Normal mode
Typing mode
Command mode
```

### Normal Mode

Global controls from Normal mode:

```txt
s    go to Speed-test page and enter Typing mode
r    restart current/last test mode with new text and enter Typing mode
p    open History page
q    quit app
:    enter Command mode
ESC  return to Speed-test page in Normal mode, where applicable
```

### Typing Mode

Controls:

```txt
text keys    type characters
Backspace    delete previous typed character
ESC          abort current attempt and return to Normal mode
```

When `ESC` is pressed during Typing mode:

```txt
- abort the current attempt
- do not save the result
- return to Normal mode
- show the Speed-test page with visually disabled/pseudo-blurred text
```

### Command Mode

Command mode is available globally from Normal mode on all pages.

Supported commands:

```txt
:10
:25
:50
:100

:15s
:30s
:60s
:120s
```

After a valid command:

```txt
- update selected test mode
- save it as last selected mode
- generate new text
- open Speed-test page
- enter Typing mode
- wait for first typed character before starting timer
```

Do not implement:

```txt
:quit
:theme
:set_theme
```

Quitting is handled by `q`.

Themes are not part of v0.1.

## App Startup Behavior

On first launch:

```txt
Page: Speed-test
Input mode: Normal
Test mode: 30 seconds
```

On later launches:

```txt
Page: Speed-test
Input mode: Normal
Test mode: last selected mode
```

In Normal mode on the Speed-test page:

```txt
- typing text is visually disabled or pseudo-blurred
- center hint says exactly: press s to start typing
```

Do not add an extra bottom help bar in v0.1.

## Timer Behavior

Entering Typing mode must not start the timer.

The timer starts only when the first character is typed.

This applies to:

```txt
s
r
:10 / :25 / :50 / :100
:15s / :30s / :60s / :120s
```

## Scoring

Use character-based WPM:

```txt
WPM = (correct characters / 5) / minutes
```

Accuracy:

```txt
Accuracy = correct characters / total typed characters * 100
```

Mistakes:

```txt
number of incorrect typed characters
```

If no characters were typed, accuracy should be `0`.

## Storage

Use TOML for config and SQLite for completed test results.

Recommended paths:

```txt
Config:
~/.config/ttp/config.toml

Database:
~/.local/share/ttp/ttp.db
```

Config should store at least:

```txt
last_selected_mode
```

SQLite should store completed test results.

Suggested table:

```sql
CREATE TABLE IF NOT EXISTS test_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mode_type TEXT NOT NULL,
    mode_value INTEGER NOT NULL,
    wpm REAL NOT NULL,
    accuracy REAL NOT NULL,
    mistakes INTEGER NOT NULL,
    correct_chars INTEGER NOT NULL,
    incorrect_chars INTEGER NOT NULL,
    total_typed_chars INTEGER NOT NULL,
    elapsed_seconds REAL NOT NULL,
    created_at TEXT NOT NULL
);
```

## UI Requirements

The UI must be minimal, clean, and responsive.

Speed-test page should show:

```txt
current mode
language: english
typing text
caret/current position
```

In Normal mode:

```txt
- typing text is visually disabled or pseudo-blurred
- show only: press s to start typing
```

In Typing mode:

```txt
- correct typed characters are visually distinct
- incorrect typed characters are visually distinct
- not-yet-typed characters are dim
- current character/caret is visible
```

Result page should show:

```txt
WPM
Accuracy
Mistakes
Mode
Time
```

History page should show the latest 10–15 completed tests with:

```txt
Mode
WPM
Accuracy
Mistakes
Date/time
```

## Development Behavior

When working on the project:

1. Read `PROJECT_SPEC_V0.1.md`.
2. Follow this master prompt.
3. Do not implement features outside v0.1.
4. Create missing files and folders yourself.
5. Keep code modular and easy to extend.
6. Prefer simple, working implementation over over-engineering.
7. After each implementation step, explain:

   * what files were created or changed
   * how to run/test the current state
   * what remains to be done next

## Code Quality Requirements

The code should:

```txt
compile successfully
use clear module boundaries
avoid unnecessary complexity
handle errors reasonably
avoid panics where possible
be formatted with cargo fmt
pass cargo check
```

When possible, add small unit tests for pure logic such as:

```txt
scoring
command parsing
typing engine
test mode parsing
```

## Important UX Principle

The app should feel fast and keyboard-focused.

Do not sacrifice typing responsiveness for visual complexity.

For v0.1, prioritize:

```txt
instant startup
no input lag
correct typing behavior
clean screen
reliable controls
correct result saving
```

Visual polish is important, but not more important than responsiveness and correctness.
