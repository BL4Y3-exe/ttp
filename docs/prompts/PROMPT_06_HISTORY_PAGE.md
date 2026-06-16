# PROMPT_06_HISTORY_PAGE.md — Implement Real History Page

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
PROMPT_05_STORAGE_CONFIG.md
```

The project should already have:

```txt
- initial Rust project structure
- minimal Ratatui app loop
- core typing engine
- real Speed-test page
- real Result page
- real Command mode
- config.toml loading/saving
- SQLite database initialization
- completed test result saving
```

Your task is to implement the real History page for `ttp`.

## Goal of This Step

Implement a History page that reads saved test results from SQLite and displays recent completed tests.

For `v0.1`, this is not a full statistics dashboard.

It should only show recent results.

## Important Scope Rule

Do not implement:

```txt
personal bests
daily statistics
average WPM
average accuracy
dashboard
activity grid
line graphs
practice mode
themes
cursor trail
multiple languages
leaderboards
sync
```

This prompt is only about reading saved results from SQLite and rendering a simple recent history table.

## Required Files to Work On

You will likely need to modify:

```txt
src/storage/database.rs
src/storage/models.rs
src/app.rs
src/event.rs
src/ui/history.rs
src/ui/mod.rs
```

You may add helper functions if needed, but keep the implementation simple.

## Database Query

In `src/storage/database.rs`, add a method to fetch recent test results.

Example:

```rust
impl Database {
    pub fn recent_test_results(&self, limit: usize) -> anyhow::Result<Vec<SavedTestResult>>;
}
```

The query should return the newest results first.

Suggested SQL:

```sql
SELECT
    id,
    mode_type,
    mode_value,
    wpm,
    accuracy,
    mistakes,
    correct_chars,
    incorrect_chars,
    total_typed_chars,
    elapsed_seconds,
    created_at
FROM test_results
ORDER BY datetime(created_at) DESC
LIMIT ?1;
```

If `created_at` is stored as RFC3339 text, make sure sorting works reliably. If needed, order by `id DESC` instead.

For v0.1, ordering by `id DESC` is acceptable and simple.

## Saved Result Model

Make sure `SavedTestResult` can represent rows loaded from SQLite.

It should include at least:

```rust
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

The exact timestamp type may differ, but the UI must be able to display date/time.

## App State

Update `App` to hold recent history results.

Example:

```rust
pub struct App {
    ...
    pub recent_results: Vec<SavedTestResult>,
    pub storage_error: Option<String>,
}
```

If a similar error field already exists, reuse it.

## Loading History

When the user opens History page with:

```txt
p
```

the app should:

```txt
- load the latest 10–15 results from SQLite
- store them in app.recent_results
- set page to History
- set input mode to Normal
```

Recommended limit:

```txt
15
```

If loading fails:

```txt
- do not crash
- open History page anyway
- show a small error message
```

## Updating History After Test Completion

When a test finishes and result is saved:

```txt
- save result to SQLite as already implemented
- update last_result
- open Result page
```

You do not need to automatically refresh history at that moment.

But when user later presses `p`, History page should load fresh data from SQLite.

## History Page UI

Replace the placeholder History page with a real table.

The page should show:

```txt
ttp
history
```

Then show recent results with columns:

```txt
Mode
WPM
Accuracy
Mistakes
Date/time
```

Example layout:

```txt
ttp
history

Mode     WPM     Accuracy   Mistakes   Date
30s      82      97%        4          2026-06-16 14:32
25w      79      98%        2          2026-06-16 14:25
60s      75      96%        9          2026-06-15 22:10
10w      91      100%       0          2026-06-15 21:55
```

For mode formatting:

```txt
mode_type = "time", mode_value = 30   -> 30s
mode_type = "words", mode_value = 25  -> 25w
```

WPM can be rounded to the nearest integer or shown with one decimal.

Accuracy can be rounded to one decimal or nearest integer.

Keep it clean.

## Empty History State

If there are no saved results yet, show:

```txt
No results yet.
Complete a test first.
```

Do not crash on empty database.

## History Page Controls

History page is always in Normal mode.

Controls:

```txt
s    go to Speed-test page and enter Typing mode
r    restart current/last mode with new text and enter Typing mode
q    quit app
:    enter Command mode
ESC  return to Speed-test page in Normal mode
```

These controls should already mostly exist from previous prompts. Make sure they still work.

## Command Mode From History Page

Command mode must still be available from History page.

Example flow:

```txt
History page → : → type 30s → Enter
```

Expected behavior:

```txt
- set mode to 30s
- save last selected mode
- generate new session
- go to Speed-test page
- enter Typing mode
- timer waits for first typed character
```

## Error Handling

The app should not crash if:

```txt
- database file does not exist yet
- database table is empty
- loading recent results fails
- timestamp parsing fails
```

For timestamp parsing failure, either:

```txt
- fall back to current local time
```

or:

```txt
- store created_at as a display string in the model
```

Choose the simpler robust option.

## Optional Scrolling

For v0.1, scrolling is optional.

Since only 10–15 results are displayed, no complex navigation is required.

Do not implement advanced table selection unless it is simple and does not complicate the code.

## Tests

Add tests where reasonable.

Required pure/helper tests:

```txt
format saved result mode as 30s
format saved result mode as 25w
empty history formatting does not panic
```

Database integration tests are optional.

If adding database tests, do not use the user's real database path. Use temporary files or in-memory SQLite.

## Code Quality Requirements

After implementation, run:

```bash
cargo fmt
cargo check
cargo test
cargo run
```

All must pass.

Keep SQL in storage modules.

Keep History page UI focused only on rendering data.

Do not put database query logic inside UI modules.

## Expected Final Response

After completing this step, respond with:

```txt
Implemented real History page for ttp.

Files changed:
- src/storage/database.rs
- src/storage/models.rs
- src/app.rs
- src/event.rs
- src/ui/history.rs
- ...

What works now:
- p opens History page
- History page loads recent results from SQLite
- latest 10–15 completed tests are displayed
- empty history state works
- command mode still works from History page
- s/r/q/ESC controls still work from History page

Commands to verify:
cargo fmt
cargo check
cargo test
cargo run

Next recommended step:
PROMPT_07_V01_POLISH_AND_README.md
```

Do not claim that personal bests, dashboard, or full advanced statistics are complete.
