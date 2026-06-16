# PROMPT_02_TYPING_ENGINE.md — Implement Core Typing Engine

You are working on the `ttp` project.

Before starting, read and follow:

```txt
PROJECT_SPEC_V0.1.md
MASTER_PROMPT.md
```

This prompt continues after:

```txt
PROMPT_01_PROJECT_SETUP.md
```

The project should already have:

```txt
Cargo.toml
src/
src/app.rs
src/core/
src/ui/
src/storage/
src/theme/
```

Your task is to implement the core typing engine for `ttp`.

Do not focus on final UI in this step. The goal is to create reliable, testable core logic that later UI prompts can use.

## Goal of This Step

Implement the core logic for:

```txt
test modes
typing sessions
text generation
character input handling
backspace handling
test status
WPM calculation
accuracy calculation
mistakes count
test completion detection
unit tests for pure logic
```

The typing engine must be independent from Ratatui.

Do not use Ratatui types in core modules.

## Important Scope Rule

Do not implement full app UI in this step.

Do not implement:

```txt
SQLite result saving
history page
result page UI
full command mode integration
practice mode
themes
cursor trail
advanced animations
multiple languages
punctuation mode
numbers mode
```

This step is only for core typing logic.

## Required Core Files

Work mainly in:

```txt
src/core/mod.rs
src/core/test_session.rs
src/core/typing_engine.rs
src/core/text_generator.rs
src/core/scoring.rs
```

You may adjust the file structure slightly if needed, but keep the logic modular.

## Test Mode Model

Create a test mode enum.

Example:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestMode {
    Words(u16),
    Time(u16),
}
```

Supported modes for v0.1:

```txt
Words(10)
Words(25)
Words(50)
Words(100)

Time(15)
Time(30)
Time(60)
Time(120)
```

Add helper methods if useful:

```rust
impl TestMode {
    pub fn default() -> Self;
    pub fn label(&self) -> String;
    pub fn mode_type(&self) -> &'static str;
    pub fn mode_value(&self) -> u16;
    pub fn is_supported(&self) -> bool;
}
```

Expected labels:

```txt
10w
25w
50w
100w
15s
30s
60s
120s
```

Default mode:

```txt
30s
```

## Session Status Model

Create a session status enum.

Example:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStatus {
    Waiting,
    Running,
    Finished,
    Aborted,
}
```

Meaning:

```txt
Waiting  = Typing mode entered, but first key has not been typed yet
Running  = timer has started after first typed character
Finished = test completed
Aborted  = user pressed ESC or otherwise cancelled the attempt
```

## Typing Session Model

Create a `TypingSession` struct.

It should contain enough state for v0.1:

```rust
pub struct TypingSession {
    pub mode: TestMode,
    pub target_text: String,
    pub typed_input: String,
    pub current_index: usize,
    pub status: SessionStatus,
    pub started_at: Option<std::time::Instant>,
    pub finished_at: Option<std::time::Instant>,
    pub mistakes: usize,
    pub correct_chars: usize,
    pub incorrect_chars: usize,
}
```

You may add fields if needed, but avoid unnecessary complexity.

## Session Creation

Create a constructor:

```rust
impl TypingSession {
    pub fn new(mode: TestMode, target_text: String) -> Self;
}
```

New sessions should start as:

```txt
status: Waiting
typed_input: empty
current_index: 0
started_at: None
finished_at: None
mistakes: 0
correct_chars: 0
incorrect_chars: 0
```

## Character Input Handling

Implement a method like:

```rust
pub fn input_char(&mut self, ch: char)
```

Behavior:

```txt
- If status is Finished or Aborted, ignore input.
- If status is Waiting, set status to Running and set started_at to now.
- Compare typed char with expected char at current_index.
- Append typed char to typed_input.
- If correct, increment correct_chars.
- If incorrect, increment incorrect_chars and mistakes.
- Increment current_index.
- Check if the test should be completed.
```

For word-count modes:

```txt
Finish when current_index >= target_text.chars().count()
```

For time-based modes:

```txt
Do not finish only because current_index reaches target_text length.
The app should generate enough text, but if text is exhausted, input should not panic.
```

For time mode completion, implement a method that can be called by the app loop:

```rust
pub fn update_time_status(&mut self)
```

It should finish the session if:

```txt
elapsed_seconds >= selected time duration
```

## Backspace Handling

Implement:

```rust
pub fn backspace(&mut self)
```

Behavior for v0.1:

```txt
- If status is Finished or Aborted, ignore.
- If typed_input is empty, do nothing.
- Remove the last typed character.
- Move current_index back by 1.
- Recalculate correct_chars, incorrect_chars, and mistakes from typed_input and target_text.
```

For simplicity and correctness, recalculating counts from scratch after backspace is acceptable in v0.1.

Do not allow current_index to underflow.

## Abort Handling

Implement:

```rust
pub fn abort(&mut self)
```

Behavior:

```txt
- Set status to Aborted
- Set finished_at if started_at exists
- Do not calculate or save result here
```

Saving will be handled by later prompts.

## Completion Handling

Implement internal finish logic:

```rust
fn finish(&mut self)
```

Behavior:

```txt
- Set status to Finished
- Set finished_at to now
```

Do not save anything to storage in this method.

## Elapsed Time

Implement:

```rust
pub fn elapsed_seconds(&self) -> f64
```

Behavior:

```txt
- If started_at is None, return 0.0
- If finished_at exists, return duration between started_at and finished_at
- Otherwise return duration between started_at and now
```

## Result Model

Create a result struct, for example:

```rust
#[derive(Debug, Clone)]
pub struct TestResult {
    pub mode: TestMode,
    pub wpm: f64,
    pub accuracy: f64,
    pub mistakes: usize,
    pub correct_chars: usize,
    pub incorrect_chars: usize,
    pub total_typed_chars: usize,
    pub elapsed_seconds: f64,
}
```

Implement:

```rust
pub fn result(&self) -> Option<TestResult>
```

Behavior:

```txt
- Return Some(TestResult) only if status is Finished.
- Return None otherwise.
```

## Scoring Logic

In `src/core/scoring.rs`, implement pure functions:

```rust
pub fn calculate_wpm(correct_chars: usize, elapsed_seconds: f64) -> f64
```

Formula:

```txt
WPM = (correct characters / 5) / minutes
minutes = elapsed_seconds / 60
```

If elapsed time is zero or negative, return `0.0`.

Implement:

```rust
pub fn calculate_accuracy(correct_chars: usize, total_typed_chars: usize) -> f64
```

Formula:

```txt
Accuracy = correct characters / total typed characters * 100
```

If total typed chars is zero, return `0.0`.

It is okay to return floating point values.

UI formatting will be handled later.

## Text Generator

In `src/core/text_generator.rs`, implement simple English text generation.

For v0.1, use a built-in static English word list.

Example word list can contain 100–300 common English words.

Implement:

```rust
pub fn generate_text(mode: TestMode) -> String
```

Behavior:

For word-count mode:

```txt
- Generate exactly N words.
- Join them with a single space.
```

For time-based mode:

```txt
- Generate enough words for the selected time.
- For v0.1, a simple rule is okay:
  15s  -> 80 words
  30s  -> 160 words
  60s  -> 320 words
  120s -> 640 words
```

The generated text should be lowercase English words only for now.

No punctuation.

No numbers.

## Typing Engine Wrapper

If useful, create a lightweight wrapper in `src/core/typing_engine.rs`.

Example:

```rust
pub struct TypingEngine {
    pub session: TypingSession,
}
```

But do not over-engineer.

It is acceptable for `TypingSession` to be the main engine in v0.1.

## Unit Tests

Add unit tests for core logic.

Required tests:

```txt
calculate_wpm returns expected value
calculate_accuracy returns expected value
new session starts in Waiting status
first character starts the session
correct character increments correct_chars
incorrect character increments mistakes and incorrect_chars
backspace removes previous input
backspace recalculates counters correctly
word mode finishes after target text is completed
abort sets status to Aborted
generate_text returns correct word count for word modes
TestMode labels are correct
```

Tests should run with:

```bash
cargo test
```

## Integration with Existing App

Do minimal integration only.

You may update `App` to include:

```rust
pub current_mode: TestMode,
pub session: Option<TypingSession>,
```

But do not implement the full UI behavior yet.

The TUI may still show placeholder text.

A later prompt will connect the engine to the speed-test UI and controls.

## Code Quality Requirements

After implementation, run:

```bash
cargo fmt
cargo check
cargo test
```

All must pass.

Avoid unused imports.

Avoid panics in normal flow.

Keep code readable and simple.

## Expected Final Response

After completing this step, respond with:

```txt
Implemented core typing engine for ttp.

Files changed:
- src/core/mod.rs
- src/core/test_session.rs
- src/core/typing_engine.rs
- src/core/text_generator.rs
- src/core/scoring.rs
- src/app.rs
- ...

What works now:
- TestMode model
- TypingSession model
- character input handling
- backspace handling
- WPM calculation
- accuracy calculation
- simple English text generation
- unit tests for core logic

Commands to verify:
cargo fmt
cargo check
cargo test

Next recommended step:
PROMPT_03_TUI_SPEED_TEST.md
```

Do not claim that the full app UI or full v0.1 is complete.
