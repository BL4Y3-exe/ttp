Apply a small focused UI patch to the current `v0.2.3` redesign.

Do not start a new redesign.
Do not change core logic.
Do not change typing behavior, stats calculation, saved data, hotkeys, or navigation.
Only fix the specific visual/layout issues listed below.

I will attach the latest screenshots as reference.

Required fixes:

1. Speed-test typing text vertical position

On the speed-test page, the typing text space has moved too far down.

Move the typing text area back to the vertical middle of the main content area.

The middle line of the text space should be visually centered, like before.

Important:

* Do not add a border around the text space.
* Do not change the text rendering logic.
* Do not change character highlighting.
* Do not change tracker behavior.
* Do not change the mode/language panel.
* Only adjust the vertical layout so the typing area is centered again.

2. Header inner padding

The global header panel is good, but the text is too close to the panel borders.

Fix the inner spacing:

* Move `ttp` slightly away from the left border.
* Move `profile` / right-side navigation slightly away from the right border.
* Keep the header full-width.
* Keep active page highlighting.
* Keep result page highlighting `speed-test`.
* Do not make the header taller unless absolutely necessary.

This should be a subtle padding fix, not a redesign.

3. Profile history mode labels

In the profile page history table, the `Mode` column currently uses short labels like:

* `15s`
* `10w`

Change the history mode labels to the clearer format:

* `time 15`
* `time 30`
* `time 60`
* `time 120`
* `words 10`
* `words 25`
* `words 50`
* `words 100`

Use this format only where it makes sense for history/profile display.

Do not change the actual stored data format unless absolutely necessary.

Do not break existing stats loading.

This should be a display-formatting change.

4. Personal Bests panel height

The `Personal Bests` panel is currently too tall and has a lot of empty space below the content.

Reduce its height so it fits the content more naturally.

Important:

* Keep the section readable.
* Keep both nested panels:

  * Time Modes
  * Word Modes
* Keep human-readable mode labels:

  * `15 seconds`, `30 seconds`, etc.
  * `10 words`, `25 words`, etc.
* Keep WPM, accuracy, and date visible.
* Do not cut off dates.
* Do not make it cramped.
* Just remove the unnecessary empty vertical space.

5. Preserve current good parts

Do not change these unless required by the fixes above:

* rounded borders
* header style
* speed-test mode/language panel
* result card layout
* profile dashboard structure
* full profile-page scrolling
* footer hint behavior
* active page highlight colors
* history table width distribution

Validation checklist:

After the patch, verify:

* speed-test typing text is vertically centered again
* typing still works
* character highlighting still works
* header text no longer feels stuck to the borders
* profile history mode labels use `time 15` / `words 10` style
* personal bests panel no longer has excessive empty space
* profile scrolling still works
* no panel text overlaps borders
* result page still works
* navigation still works

Run:

* `cargo fmt`
* `cargo clippy` if available
* `cargo test` if tests exist
* `cargo run` for manual visual check

At the end, report:

* what files were changed
* which of the four issues were fixed
* whether any behavior changed
* commands run and results
* any remaining known limitations
