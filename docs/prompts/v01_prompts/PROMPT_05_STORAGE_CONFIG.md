# PROMPT_05_STORAGE_CONFIG.md — Implement Config and SQLite Result Saving

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
PROMPT_04_COMMAND_MODE.md
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
- real Command mode for :10 / :25 / :30s etc.
```

Your task is to implement local configuration and SQLite result saving.

## Goal of This Step

Implement:

```txt
- config file creation/loading
- saving last selected test mode
- loading last selected test mode on startup
- SQLite database initialization
- test_results table
- saving completed test results to SQLite
- safe storage path creation
```

Do not implement full History page UI yet. That will be done in the next prompt.

## Important Scope Rule

Do not implement:

```txt
real History page table UI
personal bests
daily statistics
dashboard
activity grid
graphs
practice mode
themes
cursor trail
multiple languages
online accounts
leaderboards
sync
```

This step is only about local storage foundation and result persistence.

## Required Files to Work On

You will likely need to modify:

```txt
src/storage/mod.rs
src/storage/config.rs
src/storage/database.rs
src/storage/models.rs
src/app.rs
src/event.rs
src/command.rs
src/core/test_session.rs
```

You may add helper files if useful, but keep the structure simple.

## Storage Paths

Use platform-appropriate config/data directories where possible.

For Linux, preferred paths are:

```txt
Config:
~/.config/ttp/config.toml

Database:
~/.local/share/ttp/ttp.db
```

Use the `dirs` crate or a similar already configured dependency.

Create directories automatically if they do not exist.

If platform directories cannot be found, fall back to local project-relative paths:

```txt
./.ttp/config.toml
./.ttp/ttp.db
```

The app must not crash just because config/data directories do not already exist.

## Config Model

In `src/storage/config.rs`, create a config model.

Example:

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    pub last_selected_mode: String,
}
```

Default config:

```txt
last_selected_mode = "30s"
```

Implement methods/functions similar to:

```rust
impl AppConfig {
    pub fn default() -> Self;
}

pub fn load_config() -> anyhow::Result<AppConfig>;

pub fn save_config(config: &AppConfig) -> anyhow::Result<()>;
```

The exact API can differ, but it should be clean and simple.

## Config Behavior

On app startup:

```txt
- load config.toml if it exists
- if it does not exist, create default config
- parse last_selected_mode into TestMode
- if parsing fails, fall back to TestMode::default()
```

When a valid command changes the test mode:

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

the app should:

```txt
- update app.current_mode
- save last_selected_mode to config.toml
```

When `r` restarts the current mode, config does not need to change.

When `s` starts the current mode, config does not need to change.

## TestMode Serialization Helpers

Add helper methods to `TestMode` if they do not already exist:

```rust
impl TestMode {
    pub fn label(&self) -> String;
    pub fn from_label(input: &str) -> Option<Self>;
}
```

Expected labels:

```txt
Words(10)  -> "10w"
Words(25)  -> "25w"
Words(50)  -> "50w"
Words(100) -> "100w"

Time(15)   -> "15s"
Time(30)   -> "30s"
Time(60)   -> "60s"
Time(120)  -> "120s"
```

For config, it is acceptable to save word modes as either:

```txt
"25w"
```

or:

```txt
"25"
```

But prefer explicit labels:

```txt
"25w"
"30s"
```

Make sure command parsing still accepts:

```txt
:25
:30s
```

## SQLite Model

In `src/storage/models.rs`, create a model for saved results.

Example:

```rust
#[derive(Debug, Clone)]
pub struct SavedTestResult {
    pub id: Option<i64>,
    pub mode_type: String,
    pub mode_value: u16,
    pub wpm: f64,
    pub accuracy: f64,
    pub mistakes: usize,
    pub correct_chars: usize,
    pub incorrect_chars: usize,
    pub total_typed_chars: usize,
    pub elapsed_seconds: f64,
    pub created_at: chrono::DateTime<chrono::Local>,
}
```

You may adjust the exact type design if needed.

The storage model should be easy to create from the core `TestResult`.

Implement a conversion helper if useful:

```rust
impl SavedTestResult {
    pub fn from_test_result(result: &TestResult) -> Self;
}
```

## SQLite Database

In `src/storage/database.rs`, implement a simple database wrapper.

Example:

```rust
pub struct Database {
    conn: rusqlite::Connection,
}
```

Required functions:

```rust
impl Database {
    pub fn open() -> anyhow::Result<Self>;
    pub fn init(&self) -> anyhow::Result<()>;
    pub fn insert_test_result(&self, result: &SavedTestResult) -> anyhow::Result<()>;
}
```

The exact API can differ, but it must stay simple.

## Database Table

Create table:

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

Use ISO/RFC3339 string format for `created_at`.

## App Integration

Update `App` to hold storage if appropriate.

Example:

```rust
pub struct App {
    ...
    pub config: AppConfig,
    pub database: Database,
}
```

If ownership/lifetime makes this awkward, use a separate app initialization function.

Keep the code simple.

On startup:

```txt
- load config
- open database
- initialize database table
- set current_mode from config
- generate initial typing session for current_mode
- start on Speed-test page in Normal mode
```

## Saving Results

When a test finishes:

```txt
- calculate TestResult
- store it in app.last_result
- convert it to SavedTestResult
- insert it into SQLite
- open Result page
- set input mode to Normal
```

This should happen for both:

```txt
word-count tests
time-based tests
```

If saving fails:

```txt
- do not crash the app
- still show the Result page
- optionally store a small error message in App
```

For v0.1, it is acceptable to show a simple error message somewhere unobtrusive.

## Command Mode Integration

When a valid command changes the mode:

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

the app should:

```txt
- update current_mode
- save config.last_selected_mode
- create new session
- open Speed-test page
- enter Typing mode
```

If saving config fails:

```txt
- do not crash the app
- still change mode
- optionally store a warning/error message
```

## Tests

Add unit tests where reasonable.

Required tests:

```txt
AppConfig default mode is 30s
TestMode label returns expected values
TestMode from_label parses 30s
TestMode from_label parses 25w
TestMode from_label rejects invalid labels
SavedTestResult can be created from TestResult
```

SQLite integration tests are optional, but useful if simple.

Do not create tests that depend on the user's real home directory.

For config/database path tests, use temporary directories if needed, or keep tests focused on pure helper functions.

## Code Quality Requirements

After implementation, run:

```bash
cargo fmt
cargo check
cargo test
cargo run
```

All must pass.

Avoid panics in normal app flow.

Handle missing directories gracefully.

Keep storage logic separate from UI rendering.

Do not put SQL directly into UI modules.

## Expected Final Response

After completing this step, respond with:

```txt
Implemented config and SQLite result saving for ttp.

Files changed:
- src/storage/config.rs
- src/storage/database.rs
- src/storage/models.rs
- src/storage/mod.rs
- src/app.rs
- src/event.rs
- src/command.rs
- ...

What works now:
- config.toml is created/loaded
- last selected mode is saved
- app starts with last selected mode
- SQLite database is created
- test_results table is initialized
- completed tests are saved to SQLite
- result page still works if saving succeeds
- app does not crash on storage errors

Commands to verify:
cargo fmt
cargo check
cargo test
cargo run

Next recommended step:
PROMPT_06_HISTORY_PAGE.md
```

Do not claim that the full History page, dashboard, or full v0.1 is complete.
