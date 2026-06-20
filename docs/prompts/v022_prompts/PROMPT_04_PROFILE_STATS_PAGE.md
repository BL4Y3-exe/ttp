# PROMPT_04_PROFILE_STATS_PAGE.md

Now implement the expanded Profile / Stats page for `ttp` v0.2.2.

This prompt should be done after the typing layout, scroll system, timer/counter placement, and caret improvements are already implemented.

## Goal

The current page opened by pressing `p` is mostly a history page because it only shows the last completed tests.

In v0.2.2, this page should become a full Profile / Stats page.

It should show:

1. Today’s statistics.
2. Personal bests.
3. Overall statistics.
4. History of the last 15 completed tests.

Do not implement stats page scrolling/navigation in this prompt yet.
Navigation with `j` and `k` will be implemented in the next prompt.

---

# 1. Page identity

The page can still be opened with the existing `p` keybinding.

The internal name can remain whatever currently exists if renaming would cause unnecessary refactoring.

But visually and functionally, this page should now behave as a stats/profile page, not just a history page.

Use a title like:

```text id="krsq2m"
Profile
```

or:

```text id="3lw34u"
Stats
```

Choose the one that fits the existing UI style better.

---

# 2. Page structure

Render the page from top to bottom in this order:

```text id="0zif4m"
Today's statistics
Personal bests
Overall statistics
History
```

The layout should be clean and readable in a terminal.

Use simple TUI-friendly panels/sections.
Do not overcomplicate the design.

---

# 3. Today’s statistics

Add a section for today’s stats.

Required values:

```text id="1k6zbx"
Tests completed
Highest WPM
Average WPM
```

These stats should be calculated only from tests completed today.

Use the local date based on the date stored in the test result data.
If the app currently stores timestamps differently, adapt carefully.

If there are no completed tests today, show sensible empty values, for example:

```text id="oium8d"
Tests completed: 0
Highest WPM: -
Average WPM: -
```

Do not crash when history is empty.

---

# 4. Personal bests

Add a Personal Bests section.

This section should have two panels:

1. Time modes.
2. Word count modes.

## Time modes panel

The time modes panel should include these columns:

```text id="vkfq5w"
15 seconds | 30 seconds | 60 seconds | 120 seconds
```

For each mode, show the best result for that exact mode.

Each column should include:

```text id="87s0q8"
Best WPM
Accuracy
Date
```

Example:

```text id="a9xfqi"
15s
92 WPM
98% acc
2026-06-20
```

If there is no result for a mode yet, show a clean placeholder:

```text id="t3x1bs"
15s
-
-
-
```

## Word count modes panel

The word count modes panel should include these columns:

```text id="halxr2"
10 words | 25 words | 50 words | 100 words
```

For each mode, show the best result for that exact mode.

Each column should include:

```text id="jjwb3w"
Best WPM
Accuracy
Date
```

Example:

```text id="3anlrk"
25 words
85 WPM
100% acc
2026-06-20
```

If there is no result for a mode yet, show a clean placeholder:

```text id="fz180s"
25 words
-
-
-
```

---

# 5. Personal best selection rule

For each mode, personal best should be selected primarily by highest WPM.

If two results have the same WPM, prefer the one with higher accuracy.

If WPM and accuracy are both the same, prefer the newer result.

Suggested sorting priority:

```text id="bad5wc"
1. higher WPM
2. higher accuracy
3. newer date
```

Keep this logic small and testable if possible.

---

# 6. Overall statistics

Add an Overall Statistics section.

Required values:

```text id="4k6si9"
Total completed tests
Highest WPM
Average WPM
Highest accuracy
Average accuracy
```

These values should be calculated from all completed tests in history/storage.

If there are no completed tests, show sensible empty values:

```text id="62v2as"
Total completed tests: 0
Highest WPM: -
Average WPM: -
Highest accuracy: -
Average accuracy: -
```

Do not crash when history is empty.

---

# 7. History section

Keep the history section with the last 15 completed tests.

The history should still show recent completed tests as it does now, but it can be visually adjusted to fit the new page.

Required behavior:

* show only the last 15 completed tests;
* newest tests should appear first if that is already the current behavior;
* each history item should include the same important data currently shown, such as:

  * mode;
  * WPM;
  * accuracy;
  * date/time if available.

Do not remove existing useful history information.

---

# 8. Data/storage compatibility

Be careful with the existing stats/history storage model.

If the current stored result model already includes all needed fields, use it.

If new fields are needed, add them carefully and preserve backward compatibility as much as possible.

Important fields needed for the new stats page:

```text id="d5er9l"
mode type
mode value
WPM
accuracy
completion timestamp/date
```

For example:

```text id="qyqwnt"
time mode, 30 seconds, 92 WPM, 98% accuracy, 2026-06-20
word mode, 25 words, 85 WPM, 100% accuracy, 2026-06-20
```

If older saved results do not have all fields, handle missing fields gracefully with placeholders.

Do not make the app fail to start because of old history data.

---

# 9. Layout expectations

The page will probably become taller than the terminal screen.

For this prompt, focus on rendering the full content cleanly.

Do not implement scrolling with `j`/`k` yet.

However, structure the rendering in a way that the next prompt can easily add a scroll offset or viewport.

A good approach is to build the stats page content as a list of renderable lines, then later render only the visible part based on scroll offset.

If the app already has a better pattern for scrollable pages, follow the existing architecture.

---

# 10. Do not implement future stats features

Do not implement:

* github-like activity grid;
* speed graph;
* accuracy graph;
* charts;
* advanced filtering;
* exporting stats;
* config options for stats;
* themes.

These are future features.

---

# 11. Do not break existing behavior

After this change, these should still work:

* opening the page with `p`;
* leaving the page using the existing keybinding;
* normal mode behavior;
* typing tests;
* saving completed test results;
* existing history data;
* all modes:

  * `15s`;
  * `30s`;
  * `60s`;
  * `120s`;
  * `10 words`;
  * `25 words`;
  * `50 words`;
  * `100 words`.

Do not break the typing screen changes from the previous prompts.

---

# 12. Testing checklist

After implementation, manually test these cases:

## Empty history

Start with no saved results or clear test data if the app supports it.

Check that:

* Today’s stats show empty values correctly;
* Personal bests show placeholders;
* Overall stats show empty values correctly;
* History does not crash.

## One completed test

Complete one test and check that:

* it appears in History;
* Today’s statistics update;
* Overall statistics update;
* Personal best for that exact mode updates.

## Multiple modes

Complete tests in several modes:

```text id="66r71l"
15s
30s
10 words
25 words
```

Check that:

* personal bests are grouped by exact mode;
* time mode results do not appear in word mode panel;
* word mode results do not appear in time mode panel.

## Personal best tie cases

If easy to test, check that:

* higher WPM wins;
* if WPM is equal, higher accuracy wins;
* if both are equal, newer result wins.

## Backward compatibility

If there is existing saved history from before this prompt, check that the app still starts and renders the stats page without crashing.

---

# 13. Expected output

After implementation, respond with:

1. Summary of what changed.
2. Files modified.
3. Explanation of today’s stats calculation.
4. Explanation of personal best calculation.
5. Explanation of overall stats calculation.
6. Notes about storage/backward compatibility.
7. Confirmation that `j`/`k` stats page navigation was not implemented yet.
8. Suggested manual tests.
