You are working on my Rust terminal typing app project called `ttp`.

Before making any changes, first inspect and understand the current project structure, especially the main app architecture, rendering/layout code, page/state management, input handling, statistics/profile logic, and any existing project documentation or prompts.

Important context:

The project is a TUI typing application built with Rust and Ratatui. The current version already has working core functionality:

* speed-test page
* typing mode and normal mode
* result page after completing a test
* profile/statistics page
* scrolling on the profile page
* persistent statistics/history
* keyboard controls/hotkeys
* existing three-part screen structure:

  * top/header area
  * main content area
  * bottom/footer hint area

The next patch/version is `v0.2.3`.

Originally I planned to focus on customization for this version, but the plan has changed. For `v0.2.3`, the goal is now:

UI redesign / Ratatui visual polish.

The purpose is to make the app look more structured, pleasant, and professional by using Ratatui panels, blocks, borders, better spacing, alignment, and a more dashboard-like layout.

Do not change the core behavior of the app unless it is absolutely necessary for the UI work.

Do not rewrite the app architecture from scratch.

Do not break existing behavior:

* existing hotkeys must keep working
* speed-test flow must keep working
* typing logic must keep working
* result calculation must keep working
* profile/statistics logic must keep working
* profile scrolling must keep working
* saved data format should not be changed unless there is a very strong reason
* current mode/language/test settings must continue to work
* result page should remain part of the speed-test flow

For now, do not implement the full redesign yet.

Your task in this step:

1. Read the project files and understand how the current layout is built.
2. Identify which files/functions/components are responsible for rendering:

   * global layout
   * speed-test page
   * typing mode
   * result page
   * profile page
   * footer hints
3. Identify how page navigation/state is represented.
4. Identify where the existing three-part layout is implemented.
5. Identify any risks before redesigning the UI.
6. Give me a short implementation plan for the next step.

The next prompt will contain the actual UI redesign requirements and concept images, so prepare the codebase mentally for a layout/rendering-focused change.

Do not make large code changes in this step unless you need tiny harmless cleanup to understand or compile the project.

At the end, report:

* which files are relevant for the UI redesign
* how the current layout works
* what should be changed in the next step
* what should not be touched
* any risks or constraints you found
