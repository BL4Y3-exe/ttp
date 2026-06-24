Now implement the main `v0.2.3` UI redesign.

I will attach concept images for the new UI direction. Use them as visual guidance, but do not copy them blindly if the terminal constraints require small adjustments.

The goal of this patch is to make the app look more structured, pleasant, and professional using Ratatui layout/panels/blocks/borders/spacing/alignment.

This is a UI/layout/rendering-focused patch.

Do not rewrite the app architecture from scratch.
Do not change the core app logic unless absolutely necessary.
Do not break existing behavior.

Keep working:

* existing hotkeys
* page navigation
* speed-test normal mode
* speed-test typing mode
* result page flow
* profile/statistics page
* profile scrolling
* result calculation
* statistics saving/loading
* current test modes
* language display/settings
* footer hints

Important layout concept:

The screen should still be divided into 3 main areas:

1. top/header area
2. main content area
3. bottom/footer hint area

This separation is important because it prevents content from overlapping when pages scroll or when the terminal size changes.

Global UI changes:

Create a persistent top header panel that appears on all pages.

Header requirements:

* The header should be a full-width bordered panel near the top.
* Left side: app name `ttp`.
* Right side: page names `speed-test | profile`.
* Highlight the currently active page.
* When the user is on the result page, keep `speed-test` highlighted because result is part of the speed-test flow.
* The header content should not change except for the active page highlight.
* Keep the style minimal and clean.
* Use Ratatui styling such as bold/dim/accent colors where useful.
* Do not add too many colors. The UI should stay clean.

Footer requirements:

* Keep the bottom/footer area simple.
* Do not add a separate footer panel.
* Continue showing the small hint text as before.
* The hint should stay visually separated from main content by spacing/alignment, not by a bordered box.

Speed-test page redesign:

Normal mode:

* Move the mode/language information out of the top area and into the main content area.
* Add a small centered bordered panel in the main area showing:

  * `mode: ...`
  * separator `|`
  * `language: ...`
* The text area itself should remain without a separate bordered panel.
* Do not put a border around the typing text space.
* Keep the current text rendering logic as close as possible.
* Keep the current “press s to start typing” behavior/hint.
* Keep the current time/word tracker behavior.

Typing mode:

* Keep the same header and footer structure.
* Keep the small mode/language panel in the main area if it does not interfere with the typing experience.
* Keep the text area unboxed.
* Keep the tracker unchanged.
* Do not make typing worse visually or functionally.
* Current character/typed text highlighting must continue to work.

Result page redesign:

* Add one centered bordered result card in the main area.
* The card should show the result in a clean layout, similar to the concept image.
* It should include:

  * WPM
  * accuracy
  * mistakes
  * mode
  * time
* Make WPM / accuracy / mistakes feel like the main values.
* Mode and time can be smaller secondary values below.
* The result card should be centered horizontally and placed in the upper-middle/center of the main area.
* Result page should still behave as part of the speed-test flow.
* Header active page should still be `speed-test`.

Profile page redesign:

Convert the profile page into a dashboard-style layout with multiple bordered panels.

The profile page should contain these sections:

1. Today's statistics

   * tests completed
   * highest WPM
   * average WPM

2. Personal bests

   * should contain time modes and word count modes
   * time modes examples:

     * 15 seconds
     * 30 seconds
     * 60 seconds
     * 120 seconds
   * word count modes examples:

     * 10 words
     * 25 words
     * 50 words
     * 100 words
   * each item should show:

     * WPM
     * accuracy
     * date

3. Overall statistics

   * tests completed
   * highest WPM
   * average WPM
   * highest accuracy
   * average accuracy

4. History

   * table-like layout with columns:

     * mode
     * WPM
     * accuracy
     * mistakes
     * date

Profile page behavior:

* Keep profile scrolling working.
* Make sure the content does not overlap with header or footer.
* The history section can show only as many rows as fit.
* Do not let history rows overflow outside the panel.
* If there is more history than fits, keep existing scroll behavior or adapt it safely.
* The dashboard should remain readable on normal terminal sizes.
* Do not hardcode the exact example numbers from the concept images; use real app data.

Visual style:

* Minimal dark terminal UI.
* Use borders/panels to create structure.
* Use consistent spacing.
* Use consistent block titles.
* Use active page highlight in the header.
* Avoid visual noise.
* Use colors/styles carefully:

  * active page can use the app accent color
  * labels can be dim
  * important values can be bold or brighter
* Do not rely on different font sizes because terminal apps cannot control font size per element.

Technical requirements:

* Prefer Ratatui `Layout`, `Block`, `Borders`, `Paragraph`, `Table`, `Row`, `Cell`, `Span`, `Line`, etc.
* Avoid random hardcoded coordinates.
* Use layout splitting instead of manually placing everything with fixed `Rect` values where possible.
* Keep rendering code maintainable.
* If useful, create helper functions for common UI pieces:

  * header rendering
  * footer hint rendering
  * centered rect/panel helpers
  * stat card rendering
  * profile table/panel rendering
* Do not introduce large unnecessary dependencies.

Implementation steps:

1. Implement the global header panel.
2. Keep footer hint simple.
3. Update speed-test page layout with the mode/language panel.
4. Update result page with centered result card.
5. Update profile page into dashboard panels.
6. Make sure all current interactions still work.
7. Run formatting and checks:

   * `cargo fmt`
   * `cargo clippy` if available
   * `cargo test` if tests exist
   * `cargo run` for manual check

At the end, report:

* which files were changed
* what UI changes were implemented
* whether any behavior was intentionally changed
* what commands were run
* any remaining known issues
