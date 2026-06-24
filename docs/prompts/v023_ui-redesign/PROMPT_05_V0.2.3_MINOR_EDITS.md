Now apply a focused visual/layout patch to the `v0.2.3` UI redesign.

The main redesign is already implemented, but several details need to be corrected based on manual testing and screenshots.

Do not start another full redesign.
Do not rewrite the app architecture.
Do not change core app logic, stats logic, typing logic, result calculation, saved data format, or hotkeys.

This patch is only about visual/layout fixes.

I will attach current screenshots and the previous concept images as reference.

Required changes:

1. Rounded panel corners

All bordered UI panels should use slightly rounded corners instead of sharp square corners.

Use Ratatui’s rounded border style if available, for example:

* `BorderType::Rounded`

Apply this consistently to:

* global header panel
* speed-test mode/language panel
* result card
* profile page panels
* nested personal best panels
* history panel

Keep the visual style minimal and clean.

2. Move the header closer to the top edge

The top header panel currently has too much empty space above it.

Adjust the global layout so the header appears closer to the top of the terminal, similar to the concept images.

Do not remove the global three-part layout:

* header area
* main content area
* footer hint area

Just reduce unnecessary top padding/margin.

The header should still stay visually separated from main content.

3. Fix profile page scrolling logic

This is the most important change.

Currently the profile page seems to force all panels to fit into one screen, and only the history panel scrolls. This is not the intended behavior.

The entire profile page content should be scrollable as a page/dashboard.

The profile page should be allowed to be taller than the visible main area.

The scroll should move the profile dashboard content vertically inside the main content area.

The header and footer must stay fixed and must not scroll.

Do not shrink every panel just to fit everything into one screen.

Instead:

* keep panels readable
* allow vertical scrolling
* show the top part of the dashboard first
* allow the user to scroll down to see lower sections/history
* preserve existing `j/k` profile scrolling controls
* prevent content from overlapping header/footer

In other words:

* header = fixed
* footer hint = fixed
* profile main content = scrollable

4. Today's statistics panel

The `Today's Statistics` panel should not stretch across the full terminal width.

Make it a compact panel similar to the concept image.

It should be aligned toward the left side of the main content, with reasonable width based on content and terminal size.

It should show:

* tests completed
* highest WPM
* average WPM

Keep the values centered inside the panel.

Do not hardcode example values.

Use real app data.

5. Personal bests panel

The `Personal Bests` section should be taller and more readable.

It should contain two nested panels:

* Time Modes
* Word Modes

Each personal best item should show three lines of information:

For time modes:

* mode label, for example:

  * `15 seconds`
  * `30 seconds`
  * `60 seconds`
  * `120 seconds`
* WPM value, for example `82 WPM`
* accuracy, for example `100%`
* date at the bottom, for example `16 Jun 2026`

For word modes:

* mode label, for example:

  * `10 words`
  * `25 words`
  * `50 words`
  * `100 words`
* WPM value
* accuracy
* date at the bottom, for example `22 Jun 2026`

Important:

* Do not use shortened labels like `15s`, `30s`, `10w` inside the personal best cards.
* Use human-readable labels like in the concept image.
* Show the date if the data exists.
* Format dates as `16 Jun 2026` if possible.
* If no date exists, show a clean placeholder like `--`.
* Make the section tall enough so the date does not get cut off.
* Do not compress the panel just to fit the whole profile page on one screen.

6. Overall statistics panel

The `Overall Statistics` panel should not stretch across the full terminal width.

Make it closer to the concept image:

* compact but wider than today's statistics
* aligned toward the left side
* enough width for five stats
* not full screen width unless the terminal is too narrow and it needs to adapt

It should show:

* tests completed
* highest WPM
* average WPM
* highest accuracy
* average accuracy

Use real app data.

7. History panel and table

The history panel itself can be wide, but the table inside should use the available width better.

Currently the history data appears cramped on the left.

Update the table layout so columns are distributed across the full panel width.

Columns:

* Mode
* WPM
* Accuracy
* Mistakes
* Date

Requirements:

* The Date column should have enough space and appear toward the right side.
* Column spacing should feel balanced.
* The table should look like a real dashboard table, not a left-packed text dump.
* Keep history rows clipped safely inside the panel.
* If there are more rows than fit, scrolling should work through the profile page/dashboard scroll.
* Do not allow rows to overflow outside the history panel.

8. Preserve page behavior

After this patch, verify:

* speed-test normal mode still works
* speed-test typing mode still works
* result page still works
* profile page opens correctly
* profile scrolling works
* header active page highlight works
* result page still highlights `speed-test`
* footer hints still work
* no text overlaps borders
* no data is accidentally removed or hardcoded

9. Technical notes

Prefer Ratatui layout primitives:

* `Layout`
* `Constraint::Length`
* `Constraint::Min`
* `Constraint::Percentage`
* `Constraint::Ratio`
* `Block`
* `Borders`
* `BorderType::Rounded`
* `Paragraph`
* `Table`
* `Row`
* `Cell`

Avoid random fixed coordinates.

Using fixed heights for individual profile sections is acceptable if they are intentional and readable, because the entire profile dashboard should now scroll.

Do not force all profile sections to fit in the visible area at once.

10. Commands

Run:

* `cargo fmt`
* `cargo clippy` if available
* `cargo test` if tests exist
* `cargo run` for manual visual check

At the end, report:

* what visual/layout issues were fixed
* how profile scrolling now works
* which files were changed
* whether any behavior changed
* commands run and results
* any remaining limitations
