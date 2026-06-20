# PROMPT_03_TIMER_COUNTER_AND_CARET.md

Now implement the timer/counter placement changes and caret movement improvements for `ttp` v0.2.2.

This prompt should be done after the typing layout and text scroll system from the previous step is already implemented.

## Goal

Improve two parts of the typing screen:

1. Move the timer/counter from the bottom of the terminal to the top area above the typing text.
2. Improve caret movement smoothness while keeping the current block caret shape.

## Scope of this prompt

Implement only:

* timer/counter positioning;
* word counter behavior in word-count modes;
* caret movement smoothness.

Do not implement stats/profile page changes in this prompt.
Do not implement stats page navigation in this prompt.
Do not implement config, themes, customization, or new caret styles.

---

# 1. Timer/counter placement

Currently the timer/counter is rendered at the bottom of the terminal.

Change it so that it appears above the typing text space.

The timer/counter should be positioned relative to the new typing text layout from the previous prompt.

Expected placement:

* vertically: above the first visible typing line;
* horizontally: aligned with the beginning of the text area;
* visually: it should feel like it belongs to the typing block, not like a footer.

Example layout:

```text
timer/counter

first visible text line
second visible text line
third visible text line
```

More specifically:

```text
        30s

        word word word word
        word word word word
        word word word word
```

The left edge of the timer/counter should be aligned with the left edge of the text area.

If the text area starts at `text_x`, the timer/counter should also start at `text_x`.

Use the layout calculations from the new typing layout if they already exist.

Do not duplicate layout math in multiple places if the project has or can reasonably have a shared layout helper.

---

# 2. Timer behavior in time modes

In time-based modes, keep the existing timer logic.

Supported time modes:

```text
15s
30s
60s
120s
```

The timer may show remaining time or the current existing timer format, depending on how the app already works.

Do not change the core timer semantics unless necessary.

Only change where it is rendered.

Make sure the timer still:

* starts when the test starts;
* updates while typing;
* ends the test correctly;
* resets correctly when restarting the test;
* behaves correctly when switching modes.

---

# 3. Counter behavior in word-count modes

Currently, in word-count modes, the counter shows typed characters, for example:

```text
45/137
```

This is wrong for v0.2.2.

In word-count modes, the counter should show completed/target words.

Examples:

For `10 words` mode:

```text
0/10
1/10
2/10
...
10/10
```

For `25 words` mode:

```text
0/25
1/25
2/25
...
25/25
```

For `50 words` mode:

```text
0/50
```

For `100 words` mode:

```text
0/100
```

## Word counting rule

The counter should represent typing progress by words, not by characters.

Use the app’s existing target word list or target text generation logic if available.

The counter should be based on how many words are completed by the user.

A word should normally count as completed when the user has passed the word boundary, for example after typing the space after that word, or after completing the final word in the test.

Be careful with the final word: the user should be able to reach the target count when the last word is completed, even if there is no trailing space.

Examples:

* before typing anything in `25 words` mode: `0/25`;
* after finishing the first word and moving to the second: `1/25`;
* after completing the last word: `25/25`.

If the app already has a reliable word progress function, use it.
If not, add a small focused helper for this.

Do not break character-level typing validation.

---

# 4. Caret movement

The current caret shape is a block. Keep it as a block.

For v0.2.2, improve movement smoothness if possible within the current TUI rendering system.

## Requirements

* keep the current block caret shape;
* do not add caret trail;
* do not add new caret shapes;
* do not add customization options;
* do not implement config-based caret settings;
* do not make the caret visually distracting.

The improvement should be focused on making caret movement feel less jumpy.

Depending on the current architecture, this can mean:

* improving render timing;
* avoiding unnecessary full redraw artifacts;
* interpolating the caret position if the current rendering model supports it;
* ensuring the caret is consistently rendered at the correct visual position after wrapping/scrolling;
* preventing flicker;
* making sure caret updates match the typed character position.

If true smooth interpolation is not practical in the current TUI rendering architecture, prioritize stable, flicker-free, correct caret positioning over complicated animation.

Do not perform a large rendering rewrite only for caret smoothness.

---

# 5. Caret and new text layout compatibility

The caret must work correctly with the new 3-line text viewport.

Make sure the caret position is correct when:

* the active line is the upper line;
* the active line is the middle line;
* the active line is the bottom line;
* the text scrolls;
* the user reaches the last visible line;
* the user reaches the final word;
* the terminal is resized.

The caret must correspond to the current typed character position in the wrapped visual text, not just the raw text index.

---

# 6. Do not break existing behavior

After this change, these behaviors should still work:

* all time modes;
* all word-count modes;
* typing correct characters;
* typing incorrect characters;
* backspace behavior, if currently supported;
* restarting a test;
* switching modes;
* finishing a test;
* returning to normal mode;
* opening profile/stats/history page;
* all existing keybindings from v0.2.1.

Be especially careful not to break the new typing layout and scroll logic from the previous prompt.

---

# 7. Testing checklist

After implementation, manually test:

## Timer modes

Test:

```text
15s
30s
60s
120s
```

Check that:

* timer appears above the text area;
* timer is aligned with the beginning of the text;
* timer updates correctly;
* timer ending still finishes the test;
* restart resets timer correctly.

## Word-count modes

Test:

```text
10 words
25 words
50 words
100 words
```

Check that:

* counter appears above the text area;
* counter is aligned with the beginning of the text;
* counter shows words, not characters;
* counter starts from `0/target`;
* counter reaches `target/target` at the end;
* final word is counted correctly.

## Caret

Check that:

* caret is still block-shaped;
* caret appears at the correct character;
* caret works with wrapped lines;
* caret works after text scroll;
* caret does not flicker badly;
* caret does not leave visual artifacts;
* caret works in fullscreen and half-screen terminal.

---

# 8. Expected output

After implementation, respond with:

1. Summary of what changed.
2. Files modified.
3. Explanation of how timer/counter positioning now works.
4. Explanation of how word progress is calculated in word-count modes.
5. Explanation of what was done for caret smoothness.
6. Any limitations, especially if true smooth animation is not practical in the current TUI architecture.
7. Suggested manual tests.
