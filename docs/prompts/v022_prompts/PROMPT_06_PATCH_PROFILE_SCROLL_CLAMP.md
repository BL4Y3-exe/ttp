# PROMPT_06_PATCH_PROFILE_SCROLL_CLAMP.md

We need to fix a bug in the Profile / Stats page scrolling for `ttp` v0.2.2.

The page already has vertical scrolling with:

```text
j -> scroll down
k -> scroll up
```

Technically the scrolling works, but there is a bug when holding `j` at the bottom of the page.

## Problem description

When the user scrolls down with `j`, everything works correctly.

But if the user reaches the bottom of the Profile / Stats page and keeps holding `j` for a few extra seconds, the page visually stays at the bottom, as expected.

After that, when the user presses or holds `k` to scroll up, there is a delay before the page starts moving up.

The delay duration feels proportional to how long the user kept holding `j` after reaching the bottom.

This feels like the internal scroll offset keeps increasing beyond the real maximum scroll value, even though visually the rendered page is clamped at the bottom.

So when pressing `k`, the app first decreases that oversized hidden offset until it reaches the real maximum, and only after that the page visually starts scrolling up.

## Expected behavior

The scroll offset must never go below `0` or above `max_scroll`.

When the page is already at the bottom:

```text
j
```

should do nothing.

It should not continue increasing the internal scroll offset.

When the page is already at the top:

```text
k
```

should do nothing.

It should not make the scroll offset negative.

## Required fix

Find the Profile / Stats page scroll state and update logic.

Make sure the scroll offset is clamped immediately when handling input, not only during rendering.

The logic should be equivalent to:

```text
max_scroll = max(0, total_content_lines - visible_content_height)

if key == 'j':
    scroll_offset = min(scroll_offset + 1, max_scroll)

if key == 'k':
    scroll_offset = max(scroll_offset - 1, 0)
```

Do not only clamp the offset when slicing/rendering visible lines.

The actual stored scroll offset in app state must always remain valid:

```text
0 <= scroll_offset <= max_scroll
```

## Important details

`max_scroll` should be recalculated based on:

```text
total_content_lines
visible_content_height
```

If the terminal is resized, the current scroll offset should also be clamped again because `max_scroll` may change.

For example:

```text
scroll_offset = min(scroll_offset, max_scroll)
```

If the page content fits entirely on screen, then:

```text
max_scroll = 0
scroll_offset = 0
```

In that case, both `j` and `k` should do nothing.

## Scope

Fix only the Profile / Stats page scrolling bug.

Do not change:

* typing layout;
* text scroll on the speed-test page;
* timer/counter;
* caret;
* stats calculations;
* history storage;
* visual design of the Profile page;
* keybindings unrelated to Profile page scrolling.

## Keybinding constraints

The scroll keys must still work only when:

* current page/view is Profile / Stats page;
* app is in normal mode.

`j` and `k` must not affect typing input mode.

## Testing checklist

After the fix, test:

1. Open Profile / Stats page.
2. Hold `j` until the bottom is reached.
3. Keep holding `j` for several extra seconds.
4. Release `j`.
5. Press or hold `k`.

Expected result:

* page should start scrolling up immediately;
* there should be no hidden delay;
* scroll offset should not have grown past the bottom.

Also test:

1. Hold `k` at the top for several seconds.
2. Then press `j`.

Expected result:

* page should start scrolling down immediately;
* scroll offset should not go below zero.

Also test resizing terminal:

* open Profile page;
* scroll somewhere down;
* resize terminal smaller/larger;
* make sure scroll offset remains valid;
* app should not crash.

## Expected output

After implementing the fix, respond with:

1. Summary of the bug cause.
2. Files modified.
3. Explanation of how scroll offset is now clamped.
4. Confirmation that the stored scroll offset can no longer go below `0` or above `max_scroll`.
5. Manual tests performed or recommended.
