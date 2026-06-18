# PROMPT_01_PROJECT_SETUP.md — Create Initial Rust Project Structure

You are working on the `ttp` project.

Before starting, read and follow:

```txt
PROJECT_SPEC_V0.1.md
MASTER_PROMPT.md
```

The current project directory contains only:

```txt
PROJECT_SPEC_V0.1.md
```

Your task is to create the initial Rust project structure for `ttp` from scratch.

Do not implement the full typing trainer yet. This step is only for project setup, architecture foundation, dependencies, module structure, and a minimal working TUI app loop.

## Goal of This Step

Create a compilable Rust project with:

```txt
Cargo.toml
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

The project must compile with:

```bash
cargo check
```

It should also run with:

```bash
cargo run
```

When running, it should open a minimal terminal UI and show a placeholder Speed-test page with the app name `ttp`.

## Important Scope Rule

Do not implement full v0.1 functionality in this step.

Do not implement:

```txt
full typing engine
real WPM calculation
real accuracy calculation
SQLite saving
history page logic
result saving
command execution
practice mode
themes
cursor trail
advanced animations
```

Only create the foundation that later prompts can build on.

## Required Dependencies

Set up `Cargo.toml` with appropriate dependencies for the planned stack.

Use stable, common crates.

Recommended dependencies:

```toml
ratatui = "0.29"
crossterm = "0.28"
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
toml = "0.8"
rusqlite = { version = "0.32", features = ["bundled"] }
chrono = { version = "0.4", features = ["serde"] }
rand = "0.8"
dirs = "5"
anyhow = "1"
```

If newer compatible versions are available in the local environment, using them is acceptable.

Prefer `rusqlite` over `sqlx` for v0.1 because it is simpler for a local-only SQLite app.

## Required Project Structure

Create this structure:

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

You may add small helper files only if necessary.

## App State Requirements

Create basic app state types, but keep them simple.

In `src/app.rs`, define:

```rust
pub enum Page {
    SpeedTest,
    Result,
    History,
}

pub enum InputMode {
    Normal,
    Typing,
    Command,
}

pub struct App {
    pub should_quit: bool,
    pub page: Page,
    pub input_mode: InputMode,
    pub command_input: String,
}
```

You may add minimal fields if needed, but do not over-engineer.

Default startup state must be:

```txt
Page: SpeedTest
InputMode: Normal
should_quit: false
command_input: empty
```

## Minimal Event Handling

Create a minimal event loop using:

```txt
ratatui
crossterm
```

The app should:

```txt
enter alternate screen
enable raw mode
render the TUI
handle keyboard events
restore terminal on exit
```

For now, implement only minimal controls:

```txt
q    quit app, only from Normal mode
ESC  return to Normal mode from Command mode or Typing mode
:    enter Command mode from Normal mode
s    enter Typing mode from Normal mode
p    switch to History page from Normal mode
r    stay on Speed-test page and enter Typing mode from Normal mode
```

No real test restart logic is required yet.

Command mode should allow typing characters into `command_input`, but command execution can be a placeholder for now.

In Command mode:

```txt
Enter    clear command input and return to Normal mode
ESC      clear command input and return to Normal mode
Backspace remove previous command character
```

## Minimal UI Requirements

Create a minimal UI that renders based on the current page.

### Speed-test Page Placeholder

Show:

```txt
ttp
speed-test
press s to start typing
```

Also show current input mode somewhere small, for debugging during development:

```txt
mode: normal
```

This debug mode label is allowed during early development.

### History Page Placeholder

Show:

```txt
ttp
history
recent results will appear here
```

### Result Page Placeholder

Show:

```txt
ttp
result
test result will appear here
```

The Result page does not need to be reachable yet unless you add a temporary key for development. Do not over-focus on it in this setup step.

### Command Mode UI

When in Command mode, show a simple command line at the bottom:

```txt
:command_input
```

For example:

```txt
:30s
```

Do not implement real command parsing yet. That will be done in a later prompt.

## Placeholder Core Modules

Create placeholder files for the future core logic.

They should compile and contain minimal public structs/functions where useful.

Examples:

```rust
// core/scoring.rs
pub fn calculate_wpm(correct_chars: usize, elapsed_seconds: f64) -> f64 {
    if elapsed_seconds <= 0.0 {
        return 0.0;
    }

    let minutes = elapsed_seconds / 60.0;
    (correct_chars as f64 / 5.0) / minutes
}
```

It is okay to add small pure functions like this, but do not implement the full typing engine yet.

## Placeholder Storage Modules

Create placeholder storage modules:

```txt
storage/database.rs
storage/models.rs
storage/config.rs
```

Do not implement full SQLite logic yet.

It is enough to define placeholders like:

```rust
pub struct Database;
```

and simple TODO comments.

## README.md

Create a short initial `README.md` with:

```txt
# ttp

A terminal typing practice app inspired by Monkeytype.

Status: early v0.1 development.

## Run

cargo run
```

Do not overdo README yet.

## Code Quality Requirements

After implementing, make sure:

```bash
cargo fmt
cargo check
```

both pass.

Avoid unused imports where possible.

Avoid panics in normal flow.

Use `anyhow::Result` for fallible app startup/run logic.

## Expected Final Response

After completing this setup step, respond with:

```txt
Created initial Rust project structure for ttp.

Files created:
- Cargo.toml
- src/main.rs
- src/app.rs
- ...

How to run:
cargo run

How to check:
cargo check

What works now:
- minimal Ratatui app loop
- normal/typing/command mode switching
- placeholder speed-test page
- placeholder history page
- q to quit from normal mode

Next recommended step:
PROMPT_02_TYPING_ENGINE.md
```

Do not claim that full v0.1 is complete.
