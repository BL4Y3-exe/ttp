# PROMPT_01_V0.2.1_CONTROL_CLEANUP.md

You are working on an existing TUI typing application project called `ttp`.

This is not a new project. Do not recreate the project from scratch. Do not redesign the whole architecture unless it is necessary for this patch.

Your task is to implement the `v0.2.1` patch focused only on control behavior cleanup.

Before making any code changes, first inspect and understand the existing project context.

## 0. Required context reading

First, read the main project documents and prompts that already exist in the repository.

You must look for and review files such as:

* `PROJECT_SPEC_V0.1.md`
* the original master prompt / project prompt, if present
* previous setup prompts, if they were saved in the repo
* README files
* any architecture notes
* any keybinding/control documentation
* any files that describe modes, pages, commands, state, or navigation

The goal is to understand:

* the intended application behavior;
* the current architecture;
* how pages are represented;
* how modes are represented;
* how keyboard input is handled;
* how test restart/start logic works;
* how command mode works;
* how result/history/speed-test pages are switched.

After reading the context, inspect the source code and identify where keyboard events are handled.

Do not start editing until you understand the current control flow.

---

# Patch: v0.2.1 Control Cleanup

This patch has two main goals:

1. Remove the redundant `r` key behavior.
2. Fix `ESC` so it only controls mode switching and never changes the current page.

Do not include UI redesign in this patch.
Do not add new features in this patch.
Do not change unrelated behavior.

---

## 1. Remove `r` action

Currently, the `r` key is redundant.

It behaves like a limited version of `s`, and in practice `s` already covers the same use cases.

Examples of current redundant behavior:

1. After finishing a test and landing on the result page, both `r` and `s` restart/start a new test.
2. If the user wants to refresh the test with a new set of words, they can press `ESC` to enter normal mode and then press either `r` or `s`. Both produce the same result.

Because of this, `r` should be removed for now.

### Required behavior

* Pressing `r` should do nothing.
* `r` should not restart the test.
* `r` should not refresh the test.
* `r` should not navigate anywhere.
* `r` should not be reassigned to a new feature.
* Remove `r` from help text, keybinding hints, status bars, command descriptions, or documentation if it is currently shown as an active shortcut.

### Expected behavior after the patch

On the result page:

```text
s -> start a new test
r -> do nothing
```

On the speed-test page in normal mode:

```text
s -> start/refresh the test
r -> do nothing
```

On history page or any other non-test page:

```text
r -> do nothing
```

---

## 2. Fix `ESC` behavior

Currently, `ESC` has incorrect behavior.

The intended purpose of `ESC` is only to switch the app back to normal mode or close command input.

However, in the current app, pressing `ESC` sometimes also navigates to the speed-test page. This is incorrect.

### Current bug examples

Example 1:

```text
User finishes a test.
Current page: result page.
Current mode: normal mode.
User presses ESC.
Bug: app navigates to speed-test page.
```

Expected behavior:

```text
User finishes a test.
Current page: result page.
Current mode: normal mode.
User presses ESC.
App stays on result page.
App stays in normal mode.
```

Example 2:

```text
Current page: history page.
Current mode: normal mode.
User presses ESC.
Bug: app navigates to speed-test page.
```

Expected behavior:

```text
Current page: history page.
Current mode: normal mode.
User presses ESC.
App stays on history page.
App stays in normal mode.
```

Example 3:

```text
Current page: result page.
Current mode: command mode.
Command input is open.
User presses ESC.
```

This behavior is already correct and must be preserved:

```text
Command input closes.
App returns to normal mode.
Current page remains result page.
```

### Required behavior

`ESC` must only affect the current mode.

It may:

```text
ESC -> switch to normal mode
ESC -> close command input
ESC -> cancel unfinished command input
```

It must not:

```text
ESC -> go to speed-test page
ESC -> reset the current page
ESC -> navigate back
ESC -> restart the test
ESC -> refresh the test
```

If the app is already in normal mode, pressing `ESC` should be idempotent:

```text
current page: history
current mode: normal
press ESC many times
current page: history
current mode: normal
```

In other words, repeated `ESC` presses must not change the page.

---

# Acceptance criteria

The patch is complete only if all of these checks pass:

## `r` key checks

* On result page, pressing `s` starts a new test.
* On result page, pressing `r` does nothing.
* On speed-test page in normal mode, pressing `s` starts or refreshes the test.
* On speed-test page in normal mode, pressing `r` does nothing.
* On history page, pressing `r` does nothing.
* `r` is removed from visible shortcut/help hints if it was shown there.

## `ESC` checks

* From result page in normal mode, pressing `ESC` keeps the user on result page.
* From history page in normal mode, pressing `ESC` keeps the user on history page.
* From speed-test page in normal mode, pressing `ESC` keeps the user on speed-test page.
* From command mode on any page, pressing `ESC` closes command mode and keeps the current page unchanged.
* Pressing `ESC` repeatedly never changes the page.
* `ESC` does not restart, refresh, or reset the test.

---

# Testing requirement

After implementing the patch:

1. Run the existing test suite, if the project has tests.
2. If there are no tests, manually verify the acceptance criteria.
3. If the codebase has a place for regression tests, add tests for:

   * `ESC` does not change page;
   * repeated `ESC` is idempotent;
   * `r` does nothing;
   * command mode `ESC` closes command input without navigation.

Do not introduce large refactors unless they are necessary to fix the bug cleanly.

At the end, summarize:

* what files were changed;
* how `r` behavior was removed;
* how `ESC` behavior was fixed;
* what tests/checks were run;
* any remaining risks or follow-up items.
