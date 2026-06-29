We are continuing work on `ttp` `v0.2.4`.

Before changing anything, first read the current project context and the latest implementation of the `v0.2.4` typing logic. This is a focused follow-up fix pass. Do not redesign the app. Do not change unrelated UI or features.

We already implemented the main `v0.2.4` logic, but there are a few problems that must now be corrected.

# Scope of this fix pass

Only fix these specific issues:

1. missed-letter appearance after pressing `Space` inside an unfinished word;
2. `mistakes` metric logic;
3. word wrapping when there are many extra characters;
4. ability to return to the previous word with `Backspace` after pressing `Space`, but only when that previous word contains an error.

Do not change unrelated pages, layout systems, profile/history structure, keyboard shortcuts unrelated to typing, or app styling beyond what is explicitly described below.

---

# 1. Missed-letter appearance after early Space

## Current behavior

When the user presses `Space` in the middle of an unfinished word, the remaining letters now stay visible, which is correct. But currently those letters are rendered like normal wrong characters: red foreground + red underline.

## Required behavior

Keep the underline to show that these letters were missed, but do **not** render the letters themselves in red.

In other words:

* missed letters caused by early `Space` should remain visible;
* they should still be visually marked as missed;
* the underline should stay;
* but the glyph color itself should remain the normal target-text color / normal untyped letter color, not the red wrong-character color.

So the visual distinction should be:

* **wrong typed characters** → keep current wrong-character styling;
* **missed letters due to skipping the rest of a word with Space** → normal letter color + underline to indicate missed status.

This is a small but important visual distinction.

Please preserve the existing visual style as much as possible and only change the specific styling logic for this missed-letter case.

---

# 2. Mistakes metric must show final mistakes only

## Current behavior

After the recent accuracy update, `accuracy` now correctly counts all mistakes ever made, including corrected ones.

But the separate `mistakes` metric currently also shows all historical mistakes, including corrected ones. That is not what we want.

## Required behavior

We now need to clearly separate these two concepts:

### Accuracy

Keep the new logic exactly as it is now:

* accuracy is based on total historical keypress correctness;
* corrected mistakes still affect accuracy.

### Mistakes

`mistakes` must show only the number of mistakes that exist in the **final visible version of the finished test**.

That means `mistakes` should represent the amount of incorrectness remaining in the final submitted text state, not the number of all wrong keypresses during typing.

Examples:

### Example A

Target:

```text
form
```

User types:

```text
fx
Backspace
o
r
m
```

Final visible word is correct.

Expected:

* `accuracy < 100`
* `mistakes = 0`

### Example B

Target:

```text
form
```

User finishes with:

```text
forx
```

Expected:

* `mistakes = 1`

### Example C

Target:

```text
through
```

User types:

```text
thr
Space
```

Remaining `ough` are missed in final state.

Expected:

* `mistakes = 4`

### Example D

Target:

```text
form
```

User types:

```text
formm
Space
```

Expected:

* extra final `m` counts toward `mistakes`.

Implementation-wise, `mistakes` should be derived from the final completed test state, while `accuracy` remains derived from historical keystroke behavior.

Please make sure these two metrics are fully decoupled.

---

# 3. Long extra input must wrap instead of pushing text outside visible area

## Current behavior

Extra characters no longer spill into the next word logically, which is good. But visually, if the user types many extra characters, the rest of the words get pushed farther and farther to the right. Eventually they move outside the visible boundary and become impossible to see.

If the user then continues typing and moves forward through words, some words may remain off-screen / beyond the visible text area.

## Required behavior

The typing text rendering must wrap to the next line when needed, instead of letting words continue indefinitely beyond the visible width.

Important:

* words and extra characters should remain visible within the text area;
* if the current rendered content exceeds the available width, it should wrap to the next line;
* this must continue for additional lines as needed;
* the rest of the text should remain readable and usable;
* do not let long extra input permanently push upcoming words off-screen horizontally.

This fix is about the rendering/layout behavior of the test text area, not about changing the general page layout.

Please make sure the wrapping still works correctly with:

* normal words;
* words containing extra characters;
* skipped words with missed letters;
* cursor placement;
* active word highlighting / active position handling.

If there is already a text layout/rendering helper, improve it rather than building an entirely separate rendering system.

---

# 4. Allow returning to previous word with Backspace only if that word has an error

## Current behavior

After the new word-based logic, once the user presses `Space` and moves to the next word, they currently cannot use `Backspace` to return to the previous word and fix it.

## Required behavior

The user must be able to return to the **previous word** with `Backspace`, but only under a specific condition:

* if the previous word contains an error, returning should be allowed;
* if the previous word is fully correct, returning should not be allowed.

This should work naturally with the new word-based model.

## Expected behavior details

### Normal Backspace inside the current word

If the current word already has typed characters, `Backspace` should behave normally and delete within the current word.

### Returning to the previous word

If the current word is at its beginning / empty, and the user presses `Backspace`, then:

* check the immediately previous word;
* if that previous word has any final-state error, move the cursor/input focus back into that previous word so the user can fix it;
* if that previous word is correct, do nothing.

## What counts as “previous word has an error”

A previous word should be considered erroneous if its current final state does **not** exactly match the target word, including cases such as:

* wrong typed letters;
* missing letters;
* extra letters;
* skipped remainder after early `Space`.

So if the previous word is not exactly correct, user may go back and edit it.

If the previous word is correct, user may not go back into it.

## Example A

Target:

```text
form those
```

User types:

```text
forx
Space
```

Now they are at `those`.

If current word is still empty and user presses `Backspace`, they should return to `forx` because previous word has an error.

## Example B

Target:

```text
form those
```

User types:

```text
form
Space
```

Now they are at `those`.

If current word is empty and user presses `Backspace`, they should **not** return to `form` because previous word is already correct.

## Example C

Target:

```text
through say
```

User types:

```text
thr
Space
```

Now they are at `say`.

If current word is empty and user presses `Backspace`, they should be able to return to `through`, because that word has missed letters and is therefore incorrect.

Please implement this in a clean way that fits the current word-state model.

---

# Manual test cases to verify before finishing

Please test these exact scenarios manually.

## Case 1 — missed letters style

Target:

```text
people know
```

Action:

```text
type peop
press Space
```

Expected:

* `le` remains visible;
* `le` is marked as missed;
* underline remains;
* letters themselves are not red.

## Case 2 — corrected mistake should not count in final mistakes

Target:

```text
form
```

Action:

```text
type f
type x
Backspace
type o
type r
type m
finish test
```

Expected:

* final word is correct;
* `accuracy < 100`;
* `mistakes = 0`.

## Case 3 — final visible mistake count

Target:

```text
through
```

Action:

```text
type thr
press Space
finish test
```

Expected:

* final missed letters count toward `mistakes`;
* `mistakes = 4`.

## Case 4 — extra letters wrapping

Target:
Use a normal test line and type many extra characters into one word.

Expected:

* upcoming words do not disappear permanently off-screen to the right;
* rendered text wraps within the visible text area.

## Case 5 — return to previous incorrect word

Target:

```text
form those
```

Action:

```text
type forx
press Space
while current word is empty, press Backspace
```

Expected:

* cursor returns to `forx`;
* user can correct it.

## Case 6 — do not return to previous correct word

Target:

```text
form those
```

Action:

```text
type form
press Space
while current word is empty, press Backspace
```

Expected:

* no return to previous word;
* `form` remains locked because it is correct.

## Case 7 — return to previous skipped word

Target:

```text
through say
```

Action:

```text
type thr
press Space
while current word is empty, press Backspace
```

Expected:

* cursor returns to `through`;
* user can fix the skipped/missed part.

---

# Implementation guidance

Keep the fix minimal and clean.

Prefer adapting the current typing state and rendering logic rather than rewriting unrelated systems.

Pay special attention to keeping these concepts separate:

* **accuracy** = historical typing correctness;
* **mistakes** = final incorrect characters in completed test state.

Also make sure the new wrapping behavior does not break:

* cursor rendering;
* active word handling;
* word transitions;
* history saving;
* final stats calculations;
* speed test flow and restart behavior.

If there is any existing version marker or release note area, keep it consistent with `v0.2.4`. Do not create a large new versioning system if none exists.
