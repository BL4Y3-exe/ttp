# PROMPT_01_V0.2.2_CONTEXT_AND_PLAN.md

We are starting the main work on `ttp` v0.2.2.

Before implementing anything, first carefully inspect the existing project and understand the current architecture.

## Context

This project is a terminal-based typing trainer app.

The previous versions are:

* `v0.1` — initial working version.
* `v0.2.1` — already completed. It focused on cleanup in the control system.

Now we are starting `v0.2.2`.

`v0.2.2` should focus on improving the main app experience itself:

* better typing text layout;
* text scrolling;
* better timer/counter placement;
* word counter in word-count modes;
* smoother caret movement;
* expanded profile/stats page;
* navigation inside the stats page.

`v0.2.3` will be focused on customization, config file, themes, changing UI elements, caret styles, etc. Do not implement those customization features in `v0.2.2`.

## Important existing context

There is a `PROJECT_SPEC_V0.1.md` file in the project root. Read it first.

Also inspect the current source code and understand:

* project structure;
* app state model;
* render/layout system;
* input/control system;
* typing test logic;
* timer logic;
* word-count mode logic;
* stats/history storage;
* current profile/history page;
* current caret rendering;
* current keybindings and mode switching.

Do not assume the architecture. Read the files and understand how the app currently works.

## Important rule

Do not rewrite the whole app from scratch.

Work with the existing architecture. Make focused, incremental changes.
Avoid large unnecessary refactors unless they are clearly required for v0.2.2.

The control system was already cleaned up in `v0.2.1`, so be careful not to break it.

Some keys can have context-dependent behavior. For example, the same key can be used differently depending on the current page/mode. Do not simplify the control system into completely separate isolated parts if that would break existing behavior.

## v0.2.2 feature scope

The full v0.2.2 work will be split into several implementation steps. For now, only analyze the project and prepare the implementation plan.

The planned v0.2.2 areas are:

1. Typing text space and layout

   * maximum 3 visible text lines;
   * text scroll;
   * 9% left margin, 82% text area, 9% right margin;
   * left-aligned text;
   * vertically centered text area;
   * middle visible line should be near the vertical center of the terminal.

2. Text scroll logic

   * active line should become the middle line after the user moves past the second line;
   * at the beginning, keep the first three lines visible;
   * at the end, keep the last three lines visible;
   * the last line should not become the middle line if there are no more lines after it.

3. Timer/counter

   * move timer/counter from the bottom to above the typing text space;
   * align it horizontally with the beginning of the text;
   * in word-count modes, show completed words instead of typed characters.

4. Caret

   * keep the current block shape for now;
   * improve movement smoothness if possible within the current TUI rendering model;
   * do not add caret trail;
   * do not add new caret shapes.

5. Profile/stats page

   * expand the current history page into a full stats/profile page;
   * include today’s statistics;
   * include personal bests by mode;
   * include overall statistics;
   * keep history of last 15 completed tests.

6. Stats page navigation

   * add vertical scrolling on the stats/profile page;
   * `j` scrolls down;
   * `k` scrolls up;
   * this navigation should work only in normal mode and only where appropriate;
   * do not break typing input behavior.

## What NOT to implement in v0.2.2

Do not implement:

* config file;
* themes;
* color customization;
* changing caret shape;
* caret trail;
* github-like activity grid;
* graphs for speed and accuracy;
* full UI customization system.

Those belong to `v0.2.3` or later.

## Your task now

Do not start implementing features yet.

First:

1. Read `PROJECT_SPEC_V0.1.md`.
2. Inspect the project structure.
3. Identify the files/modules responsible for:

   * app state;
   * rendering;
   * input handling;
   * typing logic;
   * timer/counter;
   * stats/history storage;
   * profile/history page;
   * caret rendering.
4. Summarize the current architecture.
5. Propose a clean implementation plan for v0.2.2.
6. Split the work into safe steps.
7. Mention any risks or parts of the code that need extra care.

## Expected output

After analysis, respond with:

1. A short summary of the current architecture.
2. A list of files that will likely need changes.
3. A step-by-step implementation plan for v0.2.2.
4. Any important risks or constraints.
5. A clear confirmation that no implementation has been done yet.

Do not modify files in this prompt unless absolutely necessary for inspection.
This prompt is for understanding and planning only.
