We are working on `ttp` version `v0.2.4`.

Before making changes, first read and understand the current project structure, especially the existing technical/master documentation and the current typing-test implementation. This update is focused only on typing logic. Do not redesign UI, do not change layouts, do not change pages, do not change visual style, do not change profile/history features unless it is strictly necessary for the typing logic to work correctly.

# Goal

Implement `v0.2.4` as a focused typing logic improvement.

The current typing logic behaves too much like absolute character-by-character input. It treats the whole text as one continuous sequence, including spaces, so pressing `Space` inside a word creates a blank/missing position inside that same word.

We need to make typing behavior word-aware, closer to how Monkeytype behaves.

# Scope

Only change:

* typing input logic;
* word transition logic;
* `Space` behavior;
* extra character behavior;
* last-word completion behavior;
* accuracy calculation;
* rendering of missed characters after skipping the rest of a word.

Do not change:

* general UI layout;
* page layout;
* panel sizes;
* colors/theme;
* profile page design;
* history page design;
* keyboard shortcuts unrelated to typing;
* app architecture unless required by this logic change.

# New typing logic

## 1. Word-aware input

Typing should be handled as word-based input, not as one absolute text string.

The test text should be understood as a list of words. The user types inside the current word. Pressing `Space` is not just a normal typed character; it is an action that can finish the current word and move to the next one.

## 2. Pressing Space inside an unfinished word

If the user has typed at least one character in the current word and presses `Space`, the current word should be considered finished immediately, even if it was not fully typed.

Example:

Target word:

```text
through
```

User typed:

```text
thr
```

Then user presses `Space`.

Expected behavior:

* current word becomes finished;
* remaining characters `ough` are treated as missed characters;
* cursor moves to the next word;
* the skipped letters remain visible on screen and are shown as missed/wrong, not replaced by blank spaces.

Important: do not render this as a blank gap inside the word. The original target letters must stay visible.

## 3. Pressing Space at the beginning of a word

If the user is at the beginning of a word and has not typed any character in that word yet, pressing `Space` should do nothing.

This prevents the user from skipping whole words by spamming `Space`.

Rule:

```text
if current_word_input is empty:
    ignore Space
else:
    finish current word and move to next word
```

## 4. Extra characters

If the user types more characters than the current word has, these characters must stay attached to the current word as extra characters.

They must not transfer to the next word.

Example:

Target:

```text
form
```

User typed:

```text
formm
```

The last `m` is an extra character in the current word and should be treated as an error. The next word must not receive this extra character.

The user should move to the next word only by pressing `Space`, except for the special last-word auto-finish case described below.

## 5. Last word completion

When the user is typing the last word of the test:

If the last word is typed fully and correctly, the test should finish automatically.

Example:

```text
target: say
input:  say
```

Expected behavior:

* finish the test automatically as soon as the last correct character is typed.

But if the last word contains mistakes, the test must not auto-finish.

Examples that must not auto-finish:

```text
target: say
input:  sa
```

```text
target: say
input:  sat
```

```text
target: say
input:  sayy
```

In these cases, the test should finish only if:

* the user presses `Space`; or
* the user corrects the word with Backspace/retyping until the word becomes exactly correct.

So the final word auto-finish condition should be:

```text
is_last_word && current_word_input == target_last_word
```

Do not auto-finish if there are wrong letters, missing letters, or extra letters.

# Accuracy logic

## Current problem

Currently, accuracy is calculated based mostly on the final state of the test. This means a user can make a mistake, fix it with Backspace, and still finish with 100% accuracy.

That is not correct for a typing test.

Accuracy should represent how correctly the user pressed keys during the test, not only the final text state.

## New accuracy formula

Use this concept:

```text
Accuracy = (1 - Total Errors / Total Keystrokes) * 100
```

Clamp the result safely between `0` and `100`.

If there are no keystrokes yet, accuracy should be `100%`.

## What counts as Total Keystrokes

Count typing-related input actions that produce or commit text progress:

* normal typed characters;
* extra typed characters;
* `Space` when it successfully finishes a word and moves to the next word.

Do not count ignored `Space` presses at the beginning of a word.

Do not count `Backspace` as a keystroke for accuracy.

## What counts as Total Errors

Every wrong typed character should immediately increase `Total Errors`.

Examples:

If target word is:

```text
through
```

and the user types:

```text
thrp
```

The `p` is an error because the expected character at that position was `o`.

Extra letters after the end of a word also count as errors.

Example:

```text
target: form
input:  formm
```

The extra `m` counts as an error.

## Backspace behavior and accuracy

Backspace can visually correct the current input, but it must not remove previous error count.

Example:

1. User types a wrong character:

   * `Total Keystrokes +1`
   * `Total Errors +1`

2. User presses Backspace:

   * visual input is corrected;
   * `Total Keystrokes` does not increase;
   * `Total Errors` does not decrease.

3. User types the correct character:

   * `Total Keystrokes +1`
   * `Total Errors` does not increase.

This means corrected mistakes still affect accuracy, which is the intended behavior.

## Missed characters from pressing Space

If the user presses `Space` inside an unfinished word, all remaining untyped characters in that word should count as missed errors.

Example:

```text
target: through
typed:  thr
then Space
```

Missed part:

```text
ough
```

This means:

* `o`, `u`, `g`, `h` are 4 missed errors;
* `Total Errors +4`;
* the successful `Space` that moves to the next word should count as a typing keystroke.

So early word skipping should strongly affect accuracy.

# Missed character rendering

When the user presses `Space` before finishing a word, the remaining letters should stay visible and be displayed as missed/wrong characters.

Current bad behavior:

```text
thr  next
```

Expected behavior:

```text
through next
```

Where the typed part and missed part are visually distinguishable according to the existing wrong/missed character styling.

Important:

* do not insert blank spaces inside the word;
* do not remove the skipped letters;
* keep the target letters visible;
* mark the untyped skipped letters as missed/wrong;
* preserve current UI style as much as possible.

# Required edge cases to test manually

Please test these cases before finishing:

## Case 1: Space at beginning of word

Target:

```text
form those
```

Action:

```text
press Space before typing anything
```

Expected:

* nothing happens;
* still on `form`;
* no accuracy penalty;
* no keystroke counted.

## Case 2: Space after partial word

Target:

```text
through say
```

Action:

```text
type thr
press Space
```

Expected:

* `ough` stays visible and marked as missed/wrong;
* cursor moves to `say`;
* missed letters count as errors;
* successful Space counts as a keystroke.

## Case 3: Extra letters stay in current word

Target:

```text
form those
```

Action:

```text
type formm
```

Expected:

* extra `m` is attached to `form`;
* extra `m` is marked as wrong;
* next word `those` does not receive that character.

## Case 4: Correct last word auto-finishes

Target:

```text
form say
```

Action:

```text
type form
press Space
type say
```

Expected:

* test finishes automatically immediately after `say` is typed correctly.

## Case 5: Wrong last word does not auto-finish

Target:

```text
form say
```

Action:

```text
type form
press Space
type sat
```

Expected:

* test does not finish automatically;
* user can fix it with Backspace and type `say`;
* or user can press Space to finish with error.

## Case 6: Corrected mistake still affects accuracy

Target:

```text
form
```

Action:

```text
type f
type x
press Backspace
type o
type r
type m
```

Expected:

* final visible word is correct;
* test can finish successfully;
* accuracy is less than 100% because `x` was a wrong keypress.

# Implementation notes

Try to keep the implementation clean and minimal.

Prefer improving the existing typing state model rather than rewriting unrelated systems.

If the current code stores only absolute character indices, introduce whatever small state is needed to support word-aware behavior, for example:

* current word index;
* current input for each word;
* whether a word is finished;
* missed character information for finished incomplete words;
* total keystrokes;
* total errors.

But do not over-engineer it.

Make sure existing features still work:

* speed test starts correctly;
* restart works;
* profile/history still work;
* final stats still save;
* mistakes count still displays correctly;
* WPM and other metrics still work;
* navigation shortcuts still work.

At the end, update the project version/reference to `v0.2.4` wherever the project currently tracks version changes, if such a place already exists. Do not invent a large changelog system if there is none.
