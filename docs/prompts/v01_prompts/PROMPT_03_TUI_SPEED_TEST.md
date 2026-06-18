# PROMPT_03_TUI_SPEED_TEST.md — Connect Typing Engine to Speed-test TUI

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
- unit tests for core logic
```

Your task is to connect the existing core typing engine to the actual Speed-test TUI.

## Goal of This Step

Implement a usable Speed-test page where the user can:

```txt
- start typing with `s`
- see generated English text
- type characters
- see correct and incorrect typed characters
- see current caret position
- use Backspace
- abort with ESC
- finish a word-count test
- finish a time-based test when time runs out
- see a Result page after test completion
```

Do not implement SQLite saving or History page logic in this step.

## Important Scope Rule

Do not implement features outside v0.1.

Do not implement:

```txt
SQLite result saving
real history page
practice mode
themes
theme command
cursor trail
advanced animations
personal bests
dashboard
activity grid
multiple languages
punctuation mode
numbers mode
```

This prompt is only about connecting the Speed-test UI with the core typing session.

## Required Files to Work On

You will likely need to modify:

```txt
src/app.rs
src/event.rs
src/ui/mod.rs
src/ui/speed_test.rs
src/ui/result.rs
src/ui/components/typing_area.rs
src/core/test_session.rs
src/core/text_generator.rs
```

Add helper functions only if needed.

## App State Updates

Update `App` so it can hold the current typing session and last result.

The app should include fields similar to:

```rust
pub struct App {
    pub should_quit: bool,
    pub page: Page,
    pub input_mode: InputMode,
    pub command_input: String,

    pub current_mode: TestMode,
    pub session: Option<TypingSession>,
    pub last_result: Option<TestResult>,
}
```

Default startup state:

```txt
Page: SpeedTest
InputMode: Normal
current_mode: TestMode::default() // 30s
session: Some(new generated session)
last_result: None
```

On startup, generate a session for the default mode, but keep the app in Normal mode.

## Speed-test Page Behavior

### Normal Mode

When the app is on the Speed-test page in Normal mode:

```txt
- show generated typing text in visually disabled / pseudo-blurred style
- show center hint exactly:
  press s to start typing
```

Do not add a bottom help bar.

Pseudo-blur can be implemented in a simple way:

```txt
- dim the target text heavily
- optionally replace some visible characters with ░, ▒, ▓
```

Do not over-engineer the blur effect.

It only needs to visually communicate that typing is inactive.

### Typing Mode

When the app is in Typing mode:

```txt
- show the active target text
- already typed correct characters should be visually distinct
- already typed incorrect characters should be visually distinct
- not-yet-typed characters should be dim
- current character/caret should be visible
```

Use Ratatui `Span` / `Line` / `Text` to style characters.

Suggested simple styling:

```txt
correct typed chars    normal/bright
incorrect typed chars  red
current char/caret     underlined or reversed
remaining chars        dim gray
```

Do not implement cursor trail yet.

## Typing Area Requirements

Implement `src/ui/components/typing_area.rs` as a reusable renderer for target text + typed input.

It should be able to render:

```txt
- active typing text
- disabled/pseudo-blurred typing text
```

For v0.1, it is acceptable if wrapping is simple.

The typing text should appear centered or near the center of the screen.

It should display 2–3 rows if possible.

Do not spend too much effort on perfect Monkeytype-like sliding in this step.

Basic readable wrapping is enough.

## Control Behavior

Implement the following behavior.

### Normal Mode Controls

From Normal mode:

```txt
s
```

Behavior:

```txt
- go to Speed-test page
- ensure there is a current session
- enter Typing mode
- do not start timer yet
```

```txt
r
```

Behavior:

```txt
- generate a new session using current_mode
- go to Speed-test page
- enter Typing mode
- do not start timer yet
```

```txt
p
```

Behavior for now:

```txt
- open History page placeholder
```

```txt
q
```

Behavior:

```txt
- quit app
```

```txt
:
```

Behavior:

```txt
- enter Command mode
- clear command_input
```

```txt
ESC
```

Behavior:

```txt
- if on Result or History page, return to Speed-test page in Normal mode
- if already on Speed-test page, stay in Normal mode
```

### Typing Mode Controls

In Typing mode:

```txt
regular character keys
```

Behavior:

```txt
- send character to TypingSession::input_char
- if session finishes, store result in last_result and open Result page
```

```txt
Backspace
```

Behavior:

```txt
- call TypingSession::backspace
```

```txt
ESC
```

Behavior:

```txt
- abort current session
- return to Normal mode
- stay on Speed-test page
- do not save result
- show pseudo-blurred text again
```

### Command Mode Controls

Keep command mode simple in this prompt.

You may keep the existing placeholder behavior:

```txt
Enter
```

Behavior for now:

```txt
- clear command input
- return to Normal mode
```

Actual command parsing will be implemented in the next prompt.

Still allow typing characters into command_input, Backspace, and ESC.

## Time-based Test Completion

For time-based modes, the session should finish when selected time runs out.

The app event loop should periodically call:

```rust
session.update_time_status();
```

If the session becomes `Finished`:

```txt
- call result()
- store it in app.last_result
- open Result page
- set input mode to Normal
```

The TUI event loop should have a tick rate.

Suggested tick rate:

```txt
50ms or 100ms
```

Do not make the UI laggy.

Do not create a busy loop that uses too much CPU.

## Word-count Test Completion

For word-count modes, the session should finish when all target text characters are typed.

After finishing:

```txt
- calculate result
- store it in app.last_result
- open Result page
- set input mode to Normal
```

## Result Page

Implement a simple but real Result page using `app.last_result`.

Show:

```txt
WPM
Accuracy
Mistakes
Mode
Time
```

Example:

```txt
WPM: 82
Accuracy: 97%
Mistakes: 4
Mode: 30s
Time: 30.00s
```

Controls on Result page from Normal mode:

```txt
r    restart same mode with new text and enter Typing mode
s    start a new test in current/last mode and enter Typing mode
p    open History page placeholder
q    quit
:    enter Command mode
ESC  return to Speed-test page in Normal mode
```

## History Page Placeholder

History is still a placeholder in this prompt.

It may show:

```txt
ttp
history
recent results will appear here
```

Do not implement SQLite or real history yet.

Controls on History page from Normal mode:

```txt
s    go to Speed-test page and enter Typing mode
r    restart current/last mode with new text and enter Typing mode
q    quit
:    enter Command mode
ESC  return to Speed-test page in Normal mode
```

## Current Mode Display

On Speed-test page, show basic info:

```txt
mode: 30s
language: english
```

During Typing mode for time-based tests, optionally show remaining time.

During Typing mode for word-count tests, optionally show progress.

Keep it minimal.

## Timer Start Rule

Very important:

```txt
Entering Typing mode must not start the timer.
The timer starts only when the first typed character is entered.
```

This must work for:

```txt
s
r
future mode commands
```

## Error Handling

The app should not panic during normal typing.

Handle edge cases:

```txt
- typing when session is None should create a new session
- Backspace on empty input should do nothing
- input after Finished should be ignored
- ESC during Typing should abort safely
- small terminal sizes should not crash
```

## Tests

Add or update tests if useful.

At minimum, existing tests must still pass:

```bash
cargo test
```

Do not remove core unit tests from the previous prompt.

## Code Quality Requirements

After implementation, run:

```bash
cargo fmt
cargo check
cargo test
```

All must pass.

Avoid unused imports.

Keep UI logic separate from typing engine logic.

## Expected Final Response

After completing this step, respond with:

```txt
Connected typing engine to Speed-test TUI.

Files changed:
- src/app.rs
- src/event.rs
- src/ui/speed_test.rs
- src/ui/result.rs
- src/ui/components/typing_area.rs
- ...

What works now:
- Speed-test page uses real generated text
- s enters Typing mode
- typing input updates session state
- correct/incorrect characters are styled
- Backspace works
- ESC aborts attempt
- word-count tests can finish
- time-based tests can finish
- Result page shows real result

Commands to verify:
cargo fmt
cargo check
cargo test
cargo run

Next recommended step:
PROMPT_04_COMMAND_MODE.md
```

Do not claim that SQLite saving, real history, or full v0.1 is complete.
