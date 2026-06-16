# PROMPT_07_V01_POLISH_AND_README.md — v0.1 Polish, Cleanup, and README

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
PROMPT_06_HISTORY_PAGE.md
```

The project should already have:

```txt
- Rust project structure
- Ratatui app loop
- core typing engine
- Speed-test page
- Result page
- History page
- command mode
- config.toml loading/saving
- SQLite result saving
- recent results loaded from SQLite
```

Your task is to polish the current `v0.1` implementation, clean up rough edges, improve UX consistency, update README, and make sure the project is stable.

## Goal of This Step

Bring the app closer to a clean `v0.1` state.

Focus on:

```txt
- fixing bugs
- cleaning code
- improving small UX details
- improving error handling
- improving README
- checking that controls match PROJECT_SPEC_V0.1.md
- making sure cargo fmt / check / test pass
```

Do not add new major features.

## Important Scope Rule

Do not implement features outside `v0.1`.

Do not implement:

```txt
practice mode
themes
theme command
cursor trail
advanced animations
personal bests
dashboard
activity grid
line graphs
keyboard heatmap
multiple languages
punctuation mode
numbers mode
online accounts
leaderboards
sync
```

This step is for polishing the existing `v0.1`, not expanding scope.

## Required Files to Review

Review and update as needed:

```txt
src/main.rs
src/app.rs
src/event.rs
src/command.rs

src/ui/mod.rs
src/ui/speed_test.rs
src/ui/result.rs
src/ui/history.rs
src/ui/components/typing_area.rs
src/ui/components/top_panel.rs

src/core/mod.rs
src/core/test_session.rs
src/core/typing_engine.rs
src/core/text_generator.rs
src/core/scoring.rs

src/storage/mod.rs
src/storage/config.rs
src/storage/database.rs
src/storage/models.rs

src/theme/mod.rs
src/theme/default.rs

README.md
Cargo.toml
```

Do not rewrite everything from scratch. Improve what already exists.

## v0.1 Behavior Checklist

Make sure the app behavior matches this checklist.

### Startup

On app startup:

```txt
- app opens on Speed-test page
- input mode is Normal
- current mode is loaded from config.toml
- if config does not exist, default mode is 30s
- generated text is available
- text is visually disabled / pseudo-blurred
- center hint says exactly:
  press s to start typing
```

Do not add a bottom help bar.

### Speed-test Page / Normal Mode

Controls:

```txt
s    enter Typing mode with current generated test
r    generate new test in current mode and enter Typing mode
p    open History page
q    quit app
:    enter Command mode
ESC  stay on Speed-test page in Normal mode
```

### Speed-test Page / Typing Mode

Controls:

```txt
text keys    type characters
Backspace    delete previous typed character
ESC          abort current attempt and return to Normal mode
```

Expected behavior:

```txt
- timer starts only after first typed character
- correct chars are styled clearly
- incorrect chars are styled clearly
- current/caret position is visible
- remaining chars are dim
- word-count tests finish when target text is completed
- time-based tests finish when time runs out
- finished test opens Result page
- finished test is saved to SQLite
```

### Command Mode

Command mode is available globally from Normal mode.

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

Expected behavior after valid command:

```txt
- current mode is updated
- config last_selected_mode is saved
- new session is generated
- Speed-test page opens
- input mode becomes Typing
- timer waits for first typed character
```

Invalid commands:

```txt
- must not crash the app
- should show a small error message
- should return to Normal mode
```

Do not implement:

```txt
:quit
:theme
:set_theme
```

### Result Page

Result page must show:

```txt
WPM
Accuracy
Mistakes
Mode
Time
```

Controls:

```txt
r    restart same/current mode with new text and enter Typing mode
s    start new test in current/last mode and enter Typing mode
p    open History page
q    quit app
:    enter Command mode
ESC  return to Speed-test page in Normal mode
```

### History Page

History page must show latest 10–15 completed tests from SQLite.

Columns:

```txt
Mode
WPM
Accuracy
Mistakes
Date/time
```

Controls:

```txt
s    go to Speed-test page and enter Typing mode
r    restart current/last mode with new text and enter Typing mode
q    quit app
:    enter Command mode
ESC  return to Speed-test page in Normal mode
```

If there are no results:

```txt
No results yet.
Complete a test first.
```

## UI Polish Requirements

Keep the UI minimal.

Improve only small details:

```txt
- make spacing cleaner
- make centered text more readable
- avoid overflowing on small terminals
- format WPM and accuracy consistently
- format dates consistently
- avoid visual clutter
- keep "press s to start typing" as the only main hint in Normal mode
```

Do not add large help panels.

Do not add complex menus.

Do not add theme selection.

## Performance Requirements

The app should feel fast.

Check that:

```txt
- typing does not feel laggy
- event loop does not busy-loop at 100% CPU
- tick rate is reasonable, for example 50ms or 100ms
- rendering does not do unnecessary heavy work
```

Do not over-optimize prematurely, but remove obvious inefficiencies.

## Error Handling Requirements

The app should not crash during normal use.

Handle gracefully:

```txt
- missing config directory
- missing data directory
- missing database
- empty database
- invalid command
- storage save failure
- storage load failure
- tiny terminal size
- ESC during typing
- Backspace on empty input
```

Storage errors can be shown as small messages.

Do not panic in normal app flow.

## Code Cleanup Requirements

Clean up:

```txt
- unused imports
- duplicated logic
- unclear function names
- overly large functions where easy to split
- TODO comments that are no longer needed
- dead placeholder code from earlier prompts
```

Keep useful TODO comments for future versions only if they are clear and not noisy.

Examples:

```rust
// TODO(v0.2): add personal bests
// TODO(v0.3): add theme support
```

Do not leave misleading comments saying something is unimplemented if it is now implemented.

## README.md Requirements

Update `README.md` so it accurately describes current `v0.1`.

It should include:

```txt
# ttp

Short description:
A terminal typing practice app inspired by Monkeytype.

Status:
Early v0.1 development / usable local MVP.

Features:
- terminal-based speed typing
- word modes: 10, 25, 50, 100
- time modes: 15s, 30s, 60s, 120s
- WPM / accuracy / mistakes
- Vim-inspired controls
- command mode
- local SQLite result saving
- recent history page
- config with last selected mode
- English-only word list for now

Install/run:
cargo run

Controls:
Normal mode:
s, r, p, q, :, ESC

Typing mode:
text keys, Backspace, ESC

Command mode:
:10, :25, :50, :100, :15s, :30s, :60s, :120s

Storage paths:
~/.config/ttp/config.toml
~/.local/share/ttp/ttp.db

Roadmap:
v0.2 ideas may include themes, personal bests, better stats, cursor trail, practice mode later.
```

Keep README clear and not too long.

Do not claim the app has features that are not implemented yet.

## Cargo.toml Review

Check `Cargo.toml`.

Make sure:

```txt
- package name is ttp
- binary name is ttp, if explicitly configured
- dependencies are reasonable
- unused dependencies are removed only if definitely unused
```

Do not remove dependencies that are planned and already used by storage/config/UI.

## Tests

Make sure all existing tests still pass.

Add small tests only if useful and not time-consuming.

Useful test areas:

```txt
- TestMode label/from_label
- command parser
- scoring
- typing session backspace behavior
- history formatting helpers
```

Do not add brittle UI snapshot tests.

## Manual Test Checklist

After implementation, manually test with:

```bash
cargo run
```

Check:

```txt
1. App opens.
2. Speed-test page appears.
3. Normal mode shows pseudo-blurred text.
4. "press s to start typing" is shown.
5. Press s.
6. Typing mode starts.
7. Timer does not start until first character.
8. Typing works.
9. Backspace works.
10. ESC aborts and returns to Normal mode.
11. r restarts and enters Typing mode.
12. :30s changes mode and enters Typing mode.
13. :25 changes mode and enters Typing mode.
14. Invalid command shows error and does not crash.
15. Finished test opens Result page.
16. Result is saved.
17. p opens History page.
18. History page shows recent results.
19. q quits from Normal mode.
```

## Verification Commands

Run:

```bash
cargo fmt
cargo check
cargo test
cargo run
```

All should work.

If any command fails, fix the issue before finishing.

## Expected Final Response

After completing this step, respond with:

```txt
Polished ttp v0.1 implementation.

Files changed:
- README.md
- src/app.rs
- src/event.rs
- src/ui/speed_test.rs
- src/ui/result.rs
- src/ui/history.rs
- ...

What improved:
- cleaned up v0.1 UX
- verified controls against PROJECT_SPEC_V0.1.md
- improved README
- improved error handling
- removed rough placeholder code where appropriate
- ensured cargo fmt/check/test pass

Commands to verify:
cargo fmt
cargo check
cargo test
cargo run

Current status:
ttp is now a usable local v0.1 MVP.

Next recommended step:
manual testing and bug fixing based on real terminal usage
```

Do not claim that future features such as practice mode, themes, cursor trail, or dashboard are complete.
