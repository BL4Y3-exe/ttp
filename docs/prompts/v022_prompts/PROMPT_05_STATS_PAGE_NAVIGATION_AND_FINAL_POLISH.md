# PROMPT_05_STATS_PAGE_NAVIGATION_AND_FINAL_POLISH.md

Now implement the final v0.2.2 step for `ttp`.

This prompt should be done after these parts are already implemented:

1. typing layout and text scroll;
2. timer/counter placement and word counter;
3. caret movement improvement;
4. expanded Profile / Stats page.

## Goal

Finish v0.2.2 by adding stats page navigation and doing final integration polish.

The main new feature in this prompt is:

```text
j -> scroll down
k -> scroll up
```

on the Profile / Stats page.

After that, verify that all v0.2.2 features work together and that existing behavior from v0.2.1 is not broken.

---

# 1. Stats page scrolling

The Profile / Stats page now contains more content than before:

```text
Today's statistics
Personal bests
Overall statistics
History
```

Because this content may not fit on smaller terminal screens, add vertical scrolling to this page.

The page should have an internal scroll offset.

Only the visible part of the stats page should be rendered based on:

```text
scroll_offset
terminal_height
available_viewport_height
```

If the full stats page content fits on the screen, scrolling should do nothing.

---

# 2. Keybindings

Add these keybindings on the Profile / Stats page:

```text
j -> scroll down
k -> scroll up
```

## Important behavior

These keys should work only when:

* the app is in normal mode;
* the current page/view is the Profile / Stats page.

They should not affect typing input mode.

Do not allow `j` and `k` to insert characters while typing.
Do not make them global scrolling keys everywhere unless the architecture already has that concept and it is safe.

---

# 3. Scroll behavior

Scrolling should be line-based.

Expected behavior:

```text
j
```

moves the stats page content down by one line.

```text
k
```

moves the stats page content up by one line.

Clamp the scroll offset:

```text
minimum: 0
maximum: max(0, total_content_lines - visible_content_height)
```

Do not allow negative scroll offset.

Do not allow scrolling past the bottom.

---

# 4. Scroll reset behavior

When the user opens the Profile / Stats page with `p`, the scroll offset should normally start at the top:

```text
scroll_offset = 0
```

If the app architecture prefers remembering page scroll while staying on the same page, that is acceptable, but opening/reopening the page should not create confusing behavior.

A simple and clean behavior is:

* open stats page -> scroll at top;
* press `j`/`k` -> scroll;
* leave page -> reset next time or keep only if already implemented cleanly.

Choose the simpler behavior that fits the current architecture.

---

# 5. Visual hints

Add a small hint if it fits the existing UI style.

For example:

```text
j/k scroll
```

or:

```text
j/k: scroll
```

This hint should not be too distracting.

If the current UI already has a footer/help line, integrate it there.

If there is no good place for it, it is acceptable to skip the hint, but the keybindings must work.

---

# 6. Rendering strategy

If the Profile / Stats page from the previous prompt was implemented as a list of renderable lines, use that list and slice it by scroll offset.

Suggested approach:

```text
all_lines = build_stats_page_lines(...)
visible_lines = all_lines[scroll_offset : scroll_offset + visible_height]
render(visible_lines)
```

If the project uses widgets, panels, or another rendering abstraction, follow the existing style.

Do not rewrite the whole rendering system just for this.

---

# 7. Integration checks

After adding stats page scrolling, verify the full v0.2.2 integration.

Check that:

* typing layout still shows maximum 3 visible lines;
* text scroll still works;
* timer/counter is still above text;
* word-count modes still show words, not characters;
* caret still appears at the correct position;
* Profile / Stats page opens with `p`;
* Profile / Stats page shows all required sections;
* `j` scrolls down on stats page;
* `k` scrolls up on stats page;
* `j`/`k` do not break typing mode;
* history still saves completed tests;
* stats update after completing tests.

---

# 8. Required v0.2.2 feature checklist

Before finishing, check the full required v0.2.2 scope.

## Typing text space

Required:

* maximum 3 visible text lines;
* text scroll;
* 9% left margin;
* 82% text area;
* 9% right margin;
* text is left-aligned;
* text area is vertically centered;
* middle visible line is near terminal center.

## Text scroll

Required:

* beginning shows first three lines;
* active third line becomes middle line;
* active middle lines scroll correctly;
* final line does not become middle if there are no more lines after it;
* last three lines stay visible at the end.

## Timer/counter

Required:

* timer/counter is above typing text;
* timer/counter aligned with the start of the text area;
* time modes still show timer;
* word-count modes show words, not characters.

## Caret

Required:

* block shape remains;
* movement/positioning is improved or at least stable and flicker-free;
* no caret trail;
* no new caret shapes;
* works with wrapping and text scroll.

## Profile / Stats page

Required sections:

```text
Today's statistics
Personal bests
Overall statistics
History
```

Today’s statistics:

```text
Tests completed
Highest WPM
Average WPM
```

Personal bests:

```text
Time modes: 15s, 30s, 60s, 120s
Word modes: 10 words, 25 words, 50 words, 100 words
```

Each personal best should show:

```text
Best WPM
Accuracy
Date
```

Overall statistics:

```text
Total completed tests
Highest WPM
Average WPM
Highest accuracy
Average accuracy
```

History:

```text
Last 15 completed tests
```

Stats navigation:

```text
j -> scroll down
k -> scroll up
```

Only in normal mode on the stats page.

---

# 9. What should NOT be added

Do not add v0.2.3 features.

Do not add:

* config file;
* themes;
* color customization;
* caret shape customization;
* caret trail;
* github-like activity grid;
* speed graph;
* accuracy graph;
* chart system;
* advanced stats filters;
* export system.

Only finish v0.2.2.

---

# 10. Manual testing checklist

Run the app and manually test:

## Normal typing flow

* start app;
* select/switch modes as usual;
* start typing;
* complete a test;
* restart a test;
* return to normal mode.

## Time modes

Test:

```text
15s
30s
60s
120s
```

Check:

* timer is above text;
* timer works correctly;
* test ends correctly;
* result is saved.

## Word modes

Test:

```text
10 words
25 words
50 words
100 words
```

Check:

* counter shows `0/target`;
* counter increments by completed words;
* counter reaches `target/target`;
* test ends correctly;
* result is saved.

## Text layout

Test in fullscreen and half-screen terminal.

Check:

* text is centered vertically;
* margins look correct;
* only 3 lines are visible;
* scroll behavior is correct.

## Stats page

Open with `p`.

Check:

* all sections render;
* empty stats do not crash;
* completed tests appear;
* personal bests update;
* overall stats update;
* history shows last 15 tests;
* `j` scrolls down;
* `k` scrolls up;
* scrolling is clamped.

## Input safety

Check:

* `j`/`k` scroll only on stats page in normal mode;
* `j`/`k` do not interfere with typing;
* existing keybindings from v0.2.1 still work.

---

# 11. Cleanup

After implementation and testing:

* remove unused code introduced during v0.2.2 work;
* keep naming consistent;
* avoid debug prints/logs unless the project already uses them intentionally;
* run formatter/linter if the project has one;
* run existing tests if any;
* make sure the app builds/runs successfully.

Do not do unrelated refactoring.

---

# 12. Expected output

After finishing, respond with:

1. Summary of changes in this prompt.
2. Full v0.2.2 completion summary.
3. Files modified.
4. Explanation of stats page scroll implementation.
5. Any known limitations.
6. Manual tests performed or recommended.
7. Confirmation that v0.2.3 features were not implemented.
