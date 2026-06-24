Now focus specifically on responsive layout, terminal resizing, and preventing UI overlap for the `v0.2.3` redesign.

The main UI redesign has already been implemented or partially implemented. This step is not about changing the visual direction again. The goal now is to make the new panel-based UI robust and usable across different terminal sizes and font sizes.

Important context:

The app is a terminal UI app, so it cannot control actual font size per element. The UI must be stable in terminal cells. This means the layout must be built carefully with Ratatui layouts and constraints instead of fixed visual assumptions.

Main requirement:

Do not let text overlap with borders, panels, header, footer, or other content.

Keep the three-part screen structure:

1. header area
2. main content area
3. footer hint area

The header and footer must remain visually separated from the main page content. The profile page must remain scrollable without content entering the header/footer zones.

What to check and improve:

1. Global layout

* Make sure the screen is split through Ratatui `Layout`.
* The header should have a stable height.
* The footer should have a stable height.
* The main content should use the remaining space.
* Do not manually draw page content into the full terminal area if it can overlap with header/footer.
* Avoid random hardcoded `Rect` coordinates.
* Use safe layout helpers if needed.

2. Header

* The full-width header panel must not overflow horizontally.
* `ttp` on the left and `speed-test | profile` on the right must stay inside the header.
* If terminal width is too small, degrade gracefully:

  * keep `ttp`
  * shorten spacing
  * avoid panic/overflow
  * avoid text drawing over borders
* Active page highlight must still work.
* Result page must still highlight `speed-test`.

3. Footer

* Keep the footer as simple hint text without a bordered panel.
* Make sure footer text never overlaps with main content.
* Center it or position it consistently.
* If width is too small, it is acceptable for hints to be shorter or omitted.

4. Speed-test page

* The mode/language panel should stay centered and not overlap the typing text.
* The typing text area should remain unboxed.
* The tracker should remain visible and not collide with the text.
* During typing mode, current character highlighting must remain correct.
* Make sure the typing text wraps/clips safely inside the available main area.
* Do not allow mode/language panel, tracker, and text to occupy the same rows.
* If terminal height is too small, prioritize typing text and essential information.

5. Result page

* The result card should remain centered or upper-centered.
* The card should have enough internal space for:

  * WPM
  * accuracy
  * mistakes
  * mode
  * time
* Do not allow labels or values to touch/overwrite borders.
* If the terminal is too narrow, make the result card smaller or use a more compact layout.
* If the terminal is too short, still show the core result values clearly.

6. Profile page

This is the most important part.

* The dashboard panels must not overlap each other.
* Panel titles must not collide with borders.
* Text inside panels must not touch borders.
* History rows must not overflow outside the history panel.
* Show only as many history rows as fit inside the panel.
* Keep existing scroll behavior working.
* Scrolling should affect profile content/history safely, not header/footer.
* For smaller terminals, degrade gracefully:

  * reduce spacing
  * show fewer history rows
  * optionally stack panels vertically instead of side-by-side
  * avoid panic or broken layout
* Personal bests time modes and word modes should not overlap. If there is not enough width for side-by-side panels, stack them vertically.

7. Data display safety

* Use real app data, not fixed concept numbers.
* Handle missing stats safely.
* Handle empty history safely.
* Handle very long mode names or dates safely by clipping/truncating where appropriate.
* Do not let any text exceed its allocated Rect in a way that breaks borders.

8. Code quality

* Prefer `Layout`, `Constraint::Length`, `Constraint::Min`, `Constraint::Percentage`, `Constraint::Ratio`.
* Use helper functions for reusable layout behavior if useful.
* Use `Rect::inner` or equivalent safe inner areas before rendering text inside bordered blocks.
* Avoid duplicated magic numbers.
* Keep the rendering code readable and maintainable.
* Do not introduce unnecessary dependencies.

Manual testing checklist:

Test the app in at least these situations:

* normal terminal size
* wide fullscreen terminal
* narrower terminal
* shorter terminal
* profile page with multiple history rows
* profile page with little or no history
* speed-test normal mode
* speed-test typing mode
* result page after completing a test
* switching between speed-test and profile
* scrolling profile page

Commands to run:

* `cargo fmt`
* `cargo clippy` if available
* `cargo test` if tests exist
* `cargo run` for manual visual testing

At the end, report:

* what responsive layout issues were found
* what was fixed
* what files were changed
* how small terminal cases are handled
* whether profile scrolling still works
* what commands were run
* any remaining limitations
