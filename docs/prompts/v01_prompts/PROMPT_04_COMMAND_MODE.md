# PROMPT_04_COMMAND_MODE.md — Implement Real Command Mode

You are working on the `ttp` project.

Before starting, read and follow:

```txt
PROJECT_SPEC_V0.1.md
MASTER_PROMPT.md
```

This prompt continues after:

```txt
PROMPT_01_PROJECT_SETUP.md
PROMPT_02_TYPING_ENGINE.md
PROMPT_03_TUI_SPEED_TEST.md
```

The project should already have:

```txt
- initial Rust project structure
- minimal Ratatui app loop
- App state with Page and InputMode
- core typing engine
- TestMode
- TypingSession
- scoring logic
- text generation
- Speed-test page connected to real typing session
- Result page showing real test result
- placeholder History page
```

Your task is to implement the real command mode behavior for `ttp`.

## Goal of This Step

Implement a working Vim-like command mode that allows the user to switch typing modes globally from Normal mode.

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

After executing a valid command, the app must:

```txt
- update the selected test mode
- generate a new typing session
- open the Speed-test page
- enter Typing mode
- wait for the first typed character before starting the timer
```

Do not implement commands outside v0.1.

## Important Scope Rule

Do not implement:

```txt
:quit
:theme
:set_theme
practice mode
themes
SQLite saving
real history page
personal bests
dashboard
cursor trail
multiple languages
punctuation mode
numbers mode
```

This step is only about command parsing and command execution for test mode switching.

## Required Files to Work On

You will likely need to modify:

```txt
src/command.rs
src/app.rs
src/event.rs
src/core/test_session.rs
src/core/text_generator.rs
src/ui/mod.rs
```

You may add helper modules only if needed.

## Command Parser

Implement command parsing in `src/command.rs`.

Create an enum similar to:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    SetMode(TestMode),
}
```

Create a parser function:

```rust
pub fn parse_command(input: &str) -> Result<Command, CommandError>
```

The parser should accept command input with or without a leading colon.

Both of these should work internally:

```txt
"30s"
":30s"
```

Supported word-count commands:

```txt
10
25
50
100
```

Supported time-based commands:

```txt
15s
30s
60s
120s
```

Expected mappings:

```txt
"10"   -> Command::SetMode(TestMode::Words(10))
"25"   -> Command::SetMode(TestMode::Words(25))
"50"   -> Command::SetMode(TestMode::Words(50))
"100"  -> Command::SetMode(TestMode::Words(100))

"15s"  -> Command::SetMode(TestMode::Time(15))
"30s"  -> Command::SetMode(TestMode::Time(30))
"60s"  -> Command::SetMode(TestMode::Time(60))
"120s" -> Command::SetMode(TestMode::Time(120))
```

Invalid commands should return an error.

Examples of invalid commands:

```txt
"5"
"20"
"90s"
"theme default"
"quit"
"abc"
""
```

## Command Error

Create a simple command error type.

Example:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandError {
    Empty,
    Unknown(String),
}
```

The exact design can differ, but it should be simple and testable.

## Command Execution

Add command execution logic to the app.

You may implement a method like:

```rust
impl App {
    pub fn execute_command(&mut self, input: &str) {
        // parse command
        // if valid, apply it
        // if invalid, store error message
    }
}
```

For a valid `Command::SetMode(mode)`:

```txt
- set app.current_mode = mode
- generate a new TypingSession using this mode
- set app.page = Page::SpeedTest
- set app.input_mode = InputMode::Typing
- clear app.command_input
- clear any previous command error
```

The timer must not start when the command is executed.

The timer should still start only on the first typed character.

## Command Mode Availability

Command mode must be available globally from Normal mode on all pages.

From Normal mode:

```txt
:
```

Behavior:

```txt
- switch input_mode to Command
- clear command_input
```

This should work from:

```txt
Speed-test page
Result page
History page
```

## Command Mode Key Handling

In Command mode:

```txt
Enter
```

Behavior:

```txt
- execute command_input
- if command is valid:
  - apply command
  - go to Speed-test page
  - enter Typing mode
- if command is invalid:
  - return to Normal mode
  - keep current page unchanged
  - show a small temporary error message
```

```txt
ESC
```

Behavior:

```txt
- clear command_input
- clear command error if appropriate
- return to Normal mode
```

```txt
Backspace
```

Behavior:

```txt
- remove previous character from command_input
```

```txt
regular character keys
```

Behavior:

```txt
- append character to command_input
```

Do not include the colon character in `command_input` unless the current implementation already does. Prefer storing only the text after `:`.

The UI may display it as:

```txt
:30s
```

even if internally `command_input` is:

```txt
30s
```

## Command Error Display

Add a simple optional field to `App`, for example:

```rust
pub command_error: Option<String>
```

When a command is invalid, show a small message somewhere unobtrusive, for example near the bottom:

```txt
unknown command: 90s
```

The app should not panic on invalid commands.

The error can disappear when:

```txt
- the user enters Command mode again
- the user starts a test
- the user presses ESC
- the next valid command runs
```

No advanced notification system is required.

## UI Requirements

When in Command mode, show a command line at the bottom:

```txt
:30s
```

Use the current `command_input`.

When there is an error, show it in a simple style.

Do not add a general help bar.

The Speed-test page in Normal mode should still show only:

```txt
press s to start typing
```

## Controls After This Prompt

After this implementation, controls should behave like this.

### Normal Mode

```txt
s    go to Speed-test page and enter Typing mode
r    restart current/last test mode with new text and enter Typing mode
p    open History page
q    quit app
:    enter Command mode
ESC  return to Speed-test page in Normal mode, where applicable
```

### Command Mode

```txt
Enter      execute command
ESC        cancel command mode and return to Normal mode
Backspace  delete previous character
characters append to command input
```

### Valid Mode Commands

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

All valid mode commands should immediately start a new test in Typing mode.

## Unit Tests

Add tests for command parsing.

Required tests:

```txt
parse "10" as Words(10)
parse ":10" as Words(10)
parse "25" as Words(25)
parse "100" as Words(100)

parse "15s" as Time(15)
parse ":30s" as Time(30)
parse "60s" as Time(60)
parse "120s" as Time(120)

reject empty command
reject unsupported word count
reject unsupported time value
reject unknown text command
```

Run:

```bash
cargo test
```

All existing tests must still pass.

## Code Quality Requirements

After implementation, run:

```bash
cargo fmt
cargo check
cargo test
```

All must pass.

Avoid unused imports.

Keep command parsing separate from UI rendering.

Do not hardcode command behavior inside Ratatui rendering functions.

## Expected Final Response

After completing this step, respond with:

```txt
Implemented real command mode for ttp.

Files changed:
- src/command.rs
- src/app.rs
- src/event.rs
- src/ui/mod.rs
- ...

What works now:
- Command mode is available globally from Normal mode
- :10 / :25 / :50 / :100 work
- :15s / :30s / :60s / :120s work
- valid commands switch mode and immediately enter Typing mode
- invalid commands show a small error message
- command parsing has unit tests

Commands to verify:
cargo fmt
cargo check
cargo test
cargo run

Next recommended step:
PROMPT_05_STORAGE_CONFIG.md
```

Do not claim that SQLite saving, real history, or full v0.1 is complete.
