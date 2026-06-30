# PROMPT_02_TYPING_LAYOUT_AND_SCROLL.md

Now implement the main typing layout and text scroll system for `ttp` v0.2.2.

Before changing code, use the architecture analysis from the previous step. Work inside the existing structure and avoid unnecessary rewrites.

## Goal

Improve the typing screen layout.

Currently the typing text is too small, too wide, too high on the screen, and the app shows too much text at once.

The new layout should look like a focused typing trainer:

* compact centered text area;
* maximum 3 visible text lines;
* left-aligned text;
* proper horizontal margins;
* vertical centering;
* line-based scrolling while typing.

## Scope of this prompt

Implement only:

1. typing text area layout;
2. word wrapping for the visible typing text;
3. 3-line viewport;
4. text scroll logic;
5. correct positioning of the text block.

Do not implement stats page changes in this prompt.
Do not implement timer/counter changes in this prompt.
Do not implement caret smoothing in this prompt.
Do not implement customization/config/themes.

Those will be handled in later prompts.

---

# 1. Text area width

The typing text should not use the full terminal width.

Use this horizontal layout:

```text
9% left margin | 82% text area | 9% right margin
```

This rule should be based on the current terminal width.

For example, if terminal width is 100 columns:

```text
left margin: 9 columns
text area: 82 columns
right margin: 9 columns
```

If the terminal width changes, recalculate the layout.

The text should be rendered only inside the text area.

The text inside the text area should be left-aligned.

## Important

The exact amount of words per line depends on word length and terminal size, but in fullscreen the result should feel like approximately 10–12 words per line.

Do not hardcode “10–12 words per line”.
Instead, use the 9% / 82% / 9% width rule and normal word wrapping.

---

# 2. Text area height

The visible typing text area should always be maximum 3 lines.

The three visual positions are:

```text
upper line
middle line
bottom line
```

If there is less text than 3 lines, render only the existing lines.

Do not show all generated text at once.

---

# 3. Vertical positioning

The text area should be vertically centered.

The middle visible line should be approximately at the vertical center of the terminal.

Example:

```text
          terminal vertical center
                    ↓
upper line
middle line  <- near the vertical center
bottom line
```

So the starting Y position should be calculated from terminal height.

If the terminal is too small, clamp positions safely and avoid panics/out-of-bounds rendering.

---

# 4. Word wrapping

Wrap the target text into visual lines according to the calculated text area width.

The wrapping should be word-based, not random character splitting, unless a single word is longer than the available line width.

Expected behavior:

* words should stay intact when possible;
* spaces should be handled cleanly;
* the app should still correctly map typed characters to visible characters;
* typed/correct/incorrect styling should still work after wrapping;
* cursor/caret position should still correspond to the current typed character.

Be careful not to break typing logic while changing visual wrapping.

---

# 5. Scroll logic

The visible text viewport should show at most 3 wrapped lines.

The current active line is the wrapped visual line where the user is currently typing.

Scrolling rules:

## Beginning of text

When the user is typing the first line:

```text
line 1 -> upper line
line 2 -> middle line
line 3 -> bottom line
```

When the user is typing the second line:

```text
line 1 -> upper line
line 2 -> middle line
line 3 -> bottom line
```

So at the beginning, do not scroll too early.

## Middle of text

When the user moves to the third line, the third line becomes the middle line:

```text
line 2 -> upper line
line 3 -> middle line
line 4 -> bottom line
```

When the user moves to the fourth line:

```text
line 3 -> upper line
line 4 -> middle line
line 5 -> bottom line
```

And so on.

## End of text

If the active line is the last line and there are no more lines after it, do not force the active line into the middle.

At the end, keep the last three lines visible.

Example with 5 total lines:

When typing line 5:

```text
line 3 -> upper line
line 4 -> middle line
line 5 -> bottom line
```

Not:

```text
line 4 -> upper line
line 5 -> middle line
empty  -> bottom line
```

## Suggested logic

Use logic equivalent to this:

```text
if active_line <= 1:
    first_visible_line = 0
elif active_line >= total_lines - 1:
    first_visible_line = max(0, total_lines - 3)
else:
    first_visible_line = active_line - 1
```

Adjust for zero-based indexing if needed.

---

# 6. Active line detection

You need to determine which wrapped visual line contains the current typing position.

The active line should be based on the current character/word position in the target text.

Make sure this works for:

* time modes;
* word-count modes;
* short texts;
* long texts;
* 15s, 30s, 60s, 120s modes;
* 10 words, 25 words, 50 words, 100 words modes.

---

# 7. Resizing behavior

If the terminal size changes:

* recalculate margins;
* recalculate text area width;
* re-wrap text;
* recalculate active visual line;
* recalculate visible viewport;
* keep the current typing progress valid.

The app should not crash on small terminal sizes.

If the terminal is too narrow or too short, render a reasonable fallback or minimal layout.

---

# 8. Do not break existing behavior

After this change, these existing behaviors should still work:

* starting a test;
* typing characters;
* handling correct and incorrect characters;
* backspace behavior, if currently supported;
* finishing time-based tests;
* finishing word-count tests;
* switching modes/pages;
* restart behavior;
* normal mode/input mode behavior;
* existing keybindings from v0.2.1.

Do not simplify the control system in a way that breaks context-dependent keys.

---

# 9. Testing checklist

After implementation, manually test:

## Fullscreen terminal

* text is not too wide;
* text has visible margins;
* text appears centered vertically;
* only 3 lines are visible;
* scrolling starts when reaching the third line;
* at the end, last line stays on the bottom if there is no next line.

## Half-screen terminal

* text wraps into fewer words per line;
* margins still follow the 9% / 82% / 9% rule;
* scroll still works correctly;
* no crash.

## Small terminal

* app should not panic;
* layout should degrade safely.

## Modes

Test all currently supported modes:

```text
15s
30s
60s
120s
10 words
25 words
50 words
100 words
```

---

# 10. Expected output

After implementation, respond with:

1. Summary of what changed.
2. Files modified.
3. Explanation of the wrapping/scroll logic used.
4. Any limitations or edge cases.
5. Confirmation that stats page, timer/counter, and caret smoothing were not implemented in this prompt.
6. Suggested tests I should run manually.
