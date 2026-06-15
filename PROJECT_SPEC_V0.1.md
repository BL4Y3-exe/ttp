# ttp — Project Specification v0.1

## 1. Project Overview

`ttp` is a terminal-based typing practice application inspired by Monkeytype.

The main goal of the project is to create a fast, lightweight, polished, keyboard-focused TUI typing trainer for users who spend a lot of time in the terminal and want a pleasant typing experience without opening a browser.

The project is not intended to be a full Monkeytype clone at the beginning. Version `v0.1` focuses only on the core typing experience, local result saving, basic history, and a Vim-inspired control system.

Temporary project name:

```txt
ttp
```

Meaning:

```txt
terminal typing
```

The binary name should be:

```bash
ttp
```

## 2. Core Product Goal

The main goal of `v0.1` is:

```txt
Open terminal → run ttp → start a typing test → finish test → see result → save result locally → restart quickly
```

The most important quality requirements:

* instant startup
* no input lag
* clean and minimal TUI
* pleasant typing experience
* keyboard-first control
* simple but reliable local history
* modular codebase for future expansion

The first version should feel small but solid.

## 3. Technical Stack

The project should use the following stack:

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

Recommended storage paths:

```txt
Config:
~/.config/ttp/config.toml

Database:
~/.local/share/ttp/ttp.db
```

The exact path handling should be cross-platform where possible, but Linux support is the main priority for `v0.1`.

## 4. Scope of v0.1

### Included in v0.1

`v0.1` must include:

* Speed-test mode only
* English typing language only
* Word-count test modes:

  * 10 words
  * 25 words
  * 50 words
  * 100 words
* Time-based test modes:

  * 15 seconds
  * 30 seconds
  * 60 seconds
  * 120 seconds
* WPM calculation
* Accuracy calculation
* Mistakes count
* Result screen after test completion
* Local result saving
* Basic history page with recent test results
* Vim-inspired control system
* Normal mode
* Typing mode
* Command mode
* Basic config file
* Last selected test mode saving
* Simple clean theme

### Not included in v0.1

The following features should not be implemented in `v0.1`:

* Practice mode like Keybr
* Per-key statistics
* Daily goals
* Personal bests
* Full dashboard
* Activity grid
* Line graphs
* Keyboard heatmap
* Multiple themes
* Theme command
* Cursor trail
* Advanced animations
* Multiple languages
* Punctuation mode
* Numbers mode
* Online accounts
* Leaderboards
* Sync

These features may be added in later versions.

## 5. Typing Language

For `v0.1`, the only supported typing language is English.

The app should use a simple English word list.

No punctuation mode and no numbers mode are required in `v0.1`.

Future versions may add language files such as:

```txt
languages/
  english.toml
  russian.toml
  azerbaijani.toml
```

But this should not be implemented yet unless it does not complicate the architecture.

## 6. App Pages

The app has three main pages in `v0.1`:

```txt
Speed-test page
Result page
History page
```

### 6.1 Speed-test Page

This is the default page of the app.

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

The center of the screen contains the generated typing text.

When the user is in Normal mode, the typing text should appear visually disabled or pseudo-blurred.

A real blur effect is not possible in a terminal, so the app may imitate it using dim colors or Unicode block characters such as:

```txt
░ ▒ ▓
```

In the center, show only this hint:

```txt
press s to start typing
```

No extra help bar should be shown in `v0.1`.

When the user starts typing mode, the blur effect disappears and the normal typing text becomes active.

### 6.2 Result Page

After a test is completed, the app opens the Result page.

The Result page should show:

```txt
WPM
Accuracy
Mistakes
Mode
Elapsed time or selected duration
```

Example:

```txt
WPM: 82
Accuracy: 97%
Mistakes: 4
Mode: 30s
Time: 30.00s
```

The result must be saved locally immediately after the test is completed.

The Result page is always in Normal mode.

### 6.3 History Page

The History page shows recent completed test results.

For `v0.1`, this page is a simple recent history page, not a full statistics dashboard.

It should show the last 10–15 completed tests.

Each row should include:

```txt
Mode
WPM
Accuracy
Mistakes
Date/time
```

Example:

```txt
Recent tests

Mode       WPM     Accuracy    Mistakes    Date
30s        82      97%         4           today 18:42
25w        79      98%         2           today 18:35
60s        75      96%         9           yesterday
10w        91      100%        0           yesterday
```

The History page is always in Normal mode.

## 7. Input Modes

The app has three input modes:

```txt
Normal mode
Typing mode
Command mode
```

### 7.1 Normal Mode

Normal mode is the control/navigation mode.

In Normal mode, typed letters are interpreted as app commands, not as typing input.

Normal mode is active:

* on app start
* on Result page
* on History page
* after pressing `ESC` from Typing mode
* after cancelling Command mode

### 7.2 Typing Mode

Typing mode is the active test mode.

In Typing mode, typed letters are interpreted as typing input.

Typing mode is entered when the user starts or restarts a test.

The timer should not start immediately when entering Typing mode.

The timer starts only after the first typed character.

This allows the user to enter Typing mode, look at the text, prepare, and then start the real test with the first keypress.

### 7.3 Command Mode

Command mode is used for changing the typing test mode.

Command mode is available globally from Normal mode on all pages.

Command mode is entered by pressing:

```txt
:
```

The user types a command and presses `Enter`.

Command mode can be cancelled by pressing `ESC`.

## 8. Control System

The control system is inspired by Vim.

### 8.1 Global Normal Mode Controls

The following keys work from Normal mode:

```txt
s    go to Speed-test page and enter Typing mode
r    restart current/last test mode with new text and enter Typing mode
p    open History page
q    quit app
:    enter Command mode
ESC  return to Speed-test page in Normal mode, where applicable
```

### 8.2 Speed-test Page / Normal Mode

When the user is on the Speed-test page in Normal mode:

```txt
s    enter Typing mode with the current generated text
r    generate new text in the current mode and enter Typing mode
p    open History page
q    quit app
:    enter Command mode
```

The typing text is visually disabled/pseudo-blurred.

The center hint is:

```txt
press s to start typing
```

### 8.3 Speed-test Page / Typing Mode

When the user is in Typing mode:

```txt
text keys    type characters
Backspace    delete previous typed character
ESC          abort current attempt and return to Normal mode
```

If the user presses `ESC` during an active or prepared test:

```txt
- the current attempt is aborted
- the result is not saved
- the app returns to Normal mode
- the Speed-test page shows pseudo-blurred text again
```

### 8.4 Command Mode

Command mode is available from Normal mode on all pages.

Supported commands in `v0.1`:

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

Word-count commands:

```txt
:10     switch to 10 words
:25     switch to 25 words
:50     switch to 50 words
:100    switch to 100 words
```

Time-based commands:

```txt
:15s     switch to 15 seconds
:30s     switch to 30 seconds
:60s     switch to 60 seconds
:120s    switch to 120 seconds
```

After executing a valid mode command:

```txt
- update the selected test mode
- save it as the last selected mode
- generate new text
- open the Speed-test page
- enter Typing mode
- wait for the first typed character before starting the timer
```

Unsupported commands should show a small error message and return to Normal mode.

The following commands should not be implemented in `v0.1`:

```txt
:quit
:theme default
:set_theme
```

Quitting is handled by `q` from Normal mode.

Themes are not part of `v0.1`.

### 8.5 Result Page Controls

The Result page is always in Normal mode.

Controls:

```txt
r    restart same mode with new text and enter Typing mode
s    start a new test in the current/last mode and enter Typing mode
p    open History page
q    quit app
:    enter Command mode
ESC  return to Speed-test page in Normal mode
```

### 8.6 History Page Controls

The History page is always in Normal mode.

Controls:

```txt
s    go to Speed-test page and enter Typing mode
r    restart current/last mode with new text and enter Typing mode
q    quit app
:    enter Command mode
ESC  return to Speed-test page in Normal mode
```

For `v0.1`, advanced navigation inside History page is optional.

## 9. Test Modes

The app supports two categories of speed-test modes.

### 9.1 Word-count Modes

```txt
10 words
25 words
50 words
100 words
```

In word-count mode, the test ends when the user finishes typing the generated text.

### 9.2 Time-based Modes

```txt
15 seconds
30 seconds
60 seconds
120 seconds
```

In time-based mode, the test ends when the selected time runs out.

The app should generate enough text so that the user does not run out of words before time ends.

For `v0.1`, it is acceptable to generate a long enough sequence of words for time-based tests.

## 10. Typing Engine

The typing engine should be independent from the UI.

It should not depend on Ratatui rendering logic.

The typing engine should manage:

```txt
target_text
typed_input
current_index
started_at
finished_at
mode
status
mistakes
correct_chars
incorrect_chars
```

Possible session states:

```txt
Waiting
Running
Finished
Aborted
```

The timer starts only when the first character is typed.

### 10.1 Character Handling

For each typed character:

```txt
- compare it with the expected character at current_index
- update typed_input
- update correct/incorrect counters
- move current_index forward
```

Backspace behavior for `v0.1`:

```txt
Backspace removes the previous typed character if there is one.
```

The exact correction logic should be simple and predictable.

### 10.2 Test Completion

Word-count mode completes when:

```txt
current_index >= target_text length
```

Time-based mode completes when:

```txt
elapsed time >= selected duration
```

After completion:

```txt
- calculate result
- save result to SQLite
- open Result page
```

## 11. Scoring

### 11.1 WPM

WPM should be calculated using the standard character-based formula:

```txt
WPM = (correct characters / 5) / minutes
```

Where:

```txt
minutes = elapsed seconds / 60
```

### 11.2 Accuracy

Accuracy should be calculated as:

```txt
Accuracy = correct characters / total typed characters * 100
```

If no characters were typed, accuracy should be `0`.

### 11.3 Mistakes

Mistakes should represent the number of incorrect typed characters.

For `v0.1`, this can be a simple counter of incorrect character inputs.

## 12. Storage

The app should use:

```txt
TOML for config
SQLite for test results
```

### 12.1 Config

Config path:

```txt
~/.config/ttp/config.toml
```

Config should store:

```txt
last_selected_mode
```

Optional future-ready fields may exist, but should not complicate `v0.1`.

Example:

```toml
last_selected_mode = "30s"
```

### 12.2 SQLite Database

Database path:

```txt
~/.local/share/ttp/ttp.db
```

The database should store completed test results.

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

Example values:

```txt
mode_type: "time"
mode_value: 30

mode_type: "words"
mode_value: 25
```

## 13. UI Requirements

The UI should be minimal and clean.

### 13.1 General Style

The app should have:

```txt
dark background
soft gray inactive text
bright active text
simple accent color
minimal borders
```

No complex theming is required in `v0.1`.

### 13.2 Speed-test UI

The Speed-test page should show:

```txt
current mode
language: english
typing text
caret/current position
```

In Normal mode:

```txt
- typing text is pseudo-blurred or visually disabled
- center hint says: press s to start typing
```

In Typing mode:

```txt
- typed correct characters are visually distinct
- typed incorrect characters are visually distinct
- not-yet-typed characters are dim
- current character/caret is visible
```

### 13.3 Result UI

The Result page should show the latest result clearly.

Required values:

```txt
WPM
Accuracy
Mistakes
Mode
Time
```

### 13.4 History UI

The History page should show the latest 10–15 test results in a simple table.

Required columns:

```txt
Mode
WPM
Accuracy
Mistakes
Date/time
```

## 14. Suggested Project Structure

Recommended Rust project structure:

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

The exact structure can change during implementation, but the main principle must remain:

```txt
typing logic separate from UI
storage separate from UI
command parsing separate from input rendering
```

## 15. Architecture Principles

The codebase should follow these principles:

### 15.1 Separate Core Logic from UI

The typing engine must be testable without running the TUI.

Ratatui rendering code should only read app state and render it.

### 15.2 Separate Commands from Input Events

Raw key events should be converted into app actions.

Command parsing should be separate from rendering.

### 15.3 Keep v0.1 Small

Do not implement future features early.

The code should be ready for future features, but the user-facing app should remain focused.

### 15.4 Avoid Hardcoding UI State into Business Logic

Typing state, result state, and storage models should not depend on Ratatui types.

### 15.5 Make Future Features Easy

The architecture should make it possible to add later:

```txt
practice mode
themes
cursor trail
multiple languages
personal bests
daily stats
activity grid
graphs
keyboard heatmap
```

without rewriting the whole application.

## 16. v0.1 Success Criteria

`v0.1` is successful if:

```txt
- the user can install/run ttp
- app opens instantly
- app starts on Speed-test page in Normal mode
- text is visually disabled in Normal mode
- "press s to start typing" is shown
- pressing s enters Typing mode
- typing works without input lag
- WPM, accuracy, and mistakes are calculated correctly
- word modes work
- time modes work
- command mode works globally from Normal mode
- :25 and :30s-style commands switch mode and start a new test
- ESC aborts typing and returns to Normal mode
- finished tests show Result page
- finished tests are saved locally
- History page shows recent results
- q quits from Normal mode
```

## 17. Development Order

Recommended implementation order:

```txt
1. Project setup
2. Core data types
3. Typing engine
4. Scoring logic
5. Text generation
6. Basic Ratatui app loop
7. Speed-test page
8. Input mode system
9. Vim-inspired controls
10. Command mode
11. Result page
12. SQLite result saving
13. Config file with last selected mode
14. History page
15. UI polish
16. README
```

This order should keep the project stable and prevent UI complexity from blocking the core logic.

