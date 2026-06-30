Now do the final `v0.2.3` UI redesign polish and QA pass.

The main UI redesign and responsive layout pass should already be implemented. This final step is about reviewing the whole result, fixing small visual issues, cleaning up code, and making sure the app still behaves correctly.

Do not start another major redesign.
Do not rewrite the app architecture.
Do not introduce new features.
Do not change core behavior unless it is required to fix a bug introduced by the UI work.

Main goal:

Make the current `v0.2.3` UI redesign feel finished, consistent, and stable.

Review all pages:

1. Speed-test page, normal mode

Check that:

* global header is visible and clean
* `speed-test` is highlighted in the header
* mode/language panel looks centered and readable
* typing text area is still unboxed
* text spacing feels natural
* footer hint is visible and not inside a panel
* pressing `s` starts typing as before
* navigation to profile still works

2. Speed-test page, typing mode

Check that:

* typing still works exactly as before
* typed/current/untyped character styling still works
* tracker/time/word information still works
* mode/language panel does not interfere with typing
* no text overlaps with the header, footer, tracker, or panel
* completing a test still opens the result page correctly

3. Result page

Check that:

* result card is centered or visually balanced
* `speed-test` remains highlighted in the header
* WPM, accuracy, and mistakes are easy to read
* mode and time are displayed as secondary info
* footer hints still work
* restarting or returning from result page works as before
* the page does not feel empty or awkward on normal terminal sizes
* card content does not touch or overwrite borders

4. Profile page

Check that:

* `profile` is highlighted in the header
* dashboard panels are visually consistent
* each panel has a clear title
* today's statistics are shown correctly
* personal bests are shown correctly
* overall statistics are shown correctly
* history is shown as a clean table/list
* empty or missing stats are handled nicely
* profile scrolling still works
* scrolling never moves content into the header/footer
* history rows are clipped or limited safely
* panels do not overlap on normal, wide, narrow, or short terminals

Global polish requirements:

* Make spacing consistent across pages.
* Make border styles consistent.
* Make title casing consistent.
* Make labels and values visually distinct.
* Keep the UI minimal and clean.
* Avoid too many colors.
* Use bold/dim/accent styling only where it improves readability.
* Do not rely on different font sizes.
* Make sure there are no random leftover debug labels or temporary strings.
* Make sure there are no hardcoded concept-image numbers.
* Use real application data everywhere.

Code quality cleanup:

* Remove duplicated layout logic if it became messy.
* Extract small helper functions only if it improves readability.
* Keep helper names clear.
* Keep rendering code easy to understand.
* Avoid large unnecessary abstractions.
* Avoid unnecessary dependencies.
* Make sure formatting is clean.
* Fix clippy warnings if they are reasonable to fix.
* Do not silence warnings without a good reason.

Regression checklist:

Make sure all existing behavior still works:

* quitting the app
* switching pages
* starting a test
* typing a test
* finishing a test
* seeing results
* restarting from result page if supported
* changing modes/language if supported
* opening profile page
* scrolling profile page
* loading saved stats
* saving new stats after completed tests

Manual terminal-size checks:

Test visually in at least:

* normal terminal size
* wide/fullscreen terminal
* narrow terminal
* short terminal

The app should not panic or visually collapse in smaller sizes. If the terminal is too small to show everything, degrade gracefully by showing the most important content and clipping/omitting less important rows.

Documentation:

If the project has a README, changelog, or version notes, update only the relevant part to mention `v0.2.3` UI redesign / Ratatui polish.

Do not over-document internal implementation details.

Required commands:

Run:

* `cargo fmt`
* `cargo clippy` if available
* `cargo test` if tests exist
* `cargo run` for manual visual check

At the end, report:

* final files changed
* final UI polish fixes made
* whether any behavior changed
* whether profile scrolling still works
* commands run and their results
* remaining limitations, if any
* short summary of what is ready for `v0.2.3`
