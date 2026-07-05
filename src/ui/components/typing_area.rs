use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::core::test_session::{RenderChar, TypingSession};

const VISIBLE_LINES: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VisualLine {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone)]
struct RenderCell {
    span: Span<'static>,
    target_index: usize,
    is_space: bool,
    is_caret: bool,
    is_active_word: bool,
}

pub fn render(frame: &mut Frame<'_>, area: Rect, session: &TypingSession, active: bool) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    let text = if active {
        active_text(session, area.width)
    } else {
        disabled_text(session, area.width)
    };

    let paragraph = Paragraph::new(text).alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}

fn active_text(session: &TypingSession, width: u16) -> Text<'static> {
    let cells = active_render_cells(session);
    let wrapped_lines = wrap_render_cells(&cells, width);
    let active_line =
        active_render_line_index(&cells, &wrapped_lines, session.active_target_index());
    let first_visible = first_visible_line(active_line, wrapped_lines.len());
    let lines = wrapped_lines
        .iter()
        .skip(first_visible)
        .take(VISIBLE_LINES)
        .map(|line| {
            Line::from(
                cells[line.start..line.end]
                    .iter()
                    .map(|cell| cell.span.clone())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    Text::from(lines)
}

fn active_render_cells(session: &TypingSession) -> Vec<RenderCell> {
    let target_chars: Vec<char> = session.target_text.chars().collect();
    let mut cells = Vec::new();

    for (index, expected) in target_chars.iter().copied().enumerate() {
        if let Some(word_index) = session.word_index_at_target_index(index) {
            cells.extend(session.render_chars_at_target_index(index).into_iter().map(
                |character| {
                    let is_caret = matches!(character, RenderChar::Caret(_));
                    RenderCell {
                        span: render_char_span(character),
                        target_index: index,
                        is_space: false,
                        is_caret,
                        is_active_word: word_index == session.active_word_index(),
                    }
                },
            ));
        } else {
            let is_caret = index == session.active_target_index();
            let span = if is_caret {
                Span::styled(
                    expected.to_string(),
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled(expected.to_string(), Style::default().fg(Color::DarkGray))
            };

            cells.push(RenderCell {
                span,
                target_index: index,
                is_space: expected.is_whitespace(),
                is_caret,
                is_active_word: false,
            });
        }
    }

    if cells.is_empty() {
        cells.push(RenderCell {
            span: Span::raw(""),
            target_index: 0,
            is_space: false,
            is_caret: true,
            is_active_word: false,
        });
    }

    cells
}

fn wrap_render_cells(cells: &[RenderCell], width: u16) -> Vec<VisualLine> {
    if cells.is_empty() {
        return vec![VisualLine { start: 0, end: 0 }];
    }

    let max_width = usize::from(width).max(1);
    let mut lines = Vec::new();
    let mut start = 0;

    while start < cells.len() {
        let mut end = start;
        let mut last_space_after = None;

        while end < cells.len() && end - start < max_width {
            if cells[end].is_space {
                last_space_after = Some(end + 1);
            }

            end += 1;
        }

        if end >= cells.len() {
            lines.push(VisualLine {
                start,
                end: cells.len(),
            });
            break;
        }

        let line_end = last_space_after
            .filter(|break_after| *break_after > start && *break_after < end)
            .unwrap_or(end);

        lines.push(VisualLine {
            start,
            end: line_end,
        });

        start = line_end;

        while start < cells.len() && cells[start].is_space {
            start += 1;
        }
    }

    lines
}

fn active_render_line_index(
    cells: &[RenderCell],
    lines: &[VisualLine],
    active_target_index: usize,
) -> usize {
    if lines.is_empty() {
        return 0;
    }

    if let Some(cell_index) = cells.iter().position(|cell| cell.is_caret) {
        return render_line_index_for_cell(lines, cell_index);
    }

    if let Some(cell_index) = cells.iter().rposition(|cell| cell.is_active_word) {
        return render_line_index_for_cell(lines, cell_index);
    }

    if let Some(cell_index) = cells
        .iter()
        .rposition(|cell| cell.target_index <= active_target_index)
    {
        return render_line_index_for_cell(lines, cell_index);
    }

    0
}

fn render_line_index_for_cell(lines: &[VisualLine], cell_index: usize) -> usize {
    lines
        .iter()
        .position(|line| cell_index >= line.start && cell_index < line.end)
        .unwrap_or_else(|| lines.len().saturating_sub(1))
}

fn render_char_span(character: RenderChar) -> Span<'static> {
    match character {
        RenderChar::Correct(ch) => Span::styled(
            ch.to_string(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        RenderChar::Wrong(ch) | RenderChar::Extra(ch) => {
            Span::styled(ch.to_string(), Style::default().fg(Color::Red))
        }
        RenderChar::Missed(ch) => Span::styled(
            ch.to_string(),
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::UNDERLINED),
        ),
        RenderChar::Caret(ch) => Span::styled(
            ch.to_string(),
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        RenderChar::Pending(ch) => {
            Span::styled(ch.to_string(), Style::default().fg(Color::DarkGray))
        }
    }
}

fn disabled_text(session: &TypingSession, width: u16) -> Text<'static> {
    let target_chars: Vec<char> = session.target_text.chars().collect();
    let visible_lines = visible_lines(&target_chars, width, session.current_index);
    let lines = visible_lines
        .iter()
        .map(|visual_line| {
            let spans = target_chars
                .iter()
                .copied()
                .enumerate()
                .take(visual_line.end)
                .skip(visual_line.start)
                .map(|(index, ch)| {
                    let rendered = if ch.is_whitespace() {
                        ch
                    } else {
                        match index % 3 {
                            0 => '░',
                            1 => '▒',
                            _ => '▓',
                        }
                    };

                    Span::styled(rendered.to_string(), Style::default().fg(Color::DarkGray))
                })
                .collect::<Vec<_>>();

            Line::from(spans)
        })
        .collect::<Vec<_>>();

    Text::from(lines)
}

fn visible_lines(target_chars: &[char], width: u16, current_index: usize) -> Vec<VisualLine> {
    let wrapped_lines = wrap_lines(target_chars, width);
    let active_line = active_line_index(&wrapped_lines, current_index);
    let first_visible = first_visible_line(active_line, wrapped_lines.len());

    wrapped_lines
        .into_iter()
        .skip(first_visible)
        .take(VISIBLE_LINES)
        .collect()
}

fn wrap_lines(target_chars: &[char], width: u16) -> Vec<VisualLine> {
    if target_chars.is_empty() {
        return vec![VisualLine { start: 0, end: 0 }];
    }

    let max_width = usize::from(width).max(1);
    let mut lines = Vec::new();
    let mut start = 0;

    while start < target_chars.len() {
        let mut end = start;
        let mut last_space_after = None;

        while end < target_chars.len() && end - start < max_width {
            if target_chars[end].is_whitespace() {
                last_space_after = Some(end + 1);
            }

            end += 1;
        }

        if end >= target_chars.len() {
            lines.push(VisualLine {
                start,
                end: target_chars.len(),
            });
            break;
        }

        let line_end = last_space_after
            .filter(|break_after| *break_after > start && *break_after < end)
            .unwrap_or(end);

        lines.push(VisualLine {
            start,
            end: line_end,
        });

        start = line_end;

        while start < target_chars.len() && target_chars[start].is_whitespace() {
            start += 1;
        }
    }

    lines
}

fn active_line_index(lines: &[VisualLine], current_index: usize) -> usize {
    if lines.is_empty() {
        return 0;
    }

    for (index, line) in lines.iter().enumerate() {
        if current_index < line.start {
            return index.saturating_sub(1);
        }

        if current_index >= line.start && current_index < line.end {
            return index;
        }
    }

    lines.len().saturating_sub(1)
}

fn first_visible_line(active_line: usize, total_lines: usize) -> usize {
    if total_lines <= VISIBLE_LINES || active_line <= 1 {
        0
    } else if active_line >= total_lines.saturating_sub(1) {
        total_lines.saturating_sub(VISIBLE_LINES)
    } else {
        active_line.saturating_sub(1)
    }
}

#[cfg(test)]
fn hidden_boundary_caret(target_chars: &[char], line: VisualLine, current_index: usize) -> bool {
    current_index == line.end
        && target_chars
            .get(current_index)
            .is_some_and(|ch| ch.is_whitespace())
}

#[cfg(test)]
mod tests {
    use ratatui::style::{Color, Modifier};

    use crate::core::test_session::{TestMode, TypingSession};

    use super::{
        active_line_index, active_render_cells, first_visible_line, hidden_boundary_caret,
        wrap_lines, wrap_render_cells, VisualLine,
    };

    fn chars(input: &str) -> Vec<char> {
        input.chars().collect()
    }

    #[test]
    fn wraps_on_word_boundaries_when_possible() {
        let lines = wrap_lines(&chars("one two three four"), 8);

        assert_eq!(
            lines,
            vec![
                VisualLine { start: 0, end: 8 },
                VisualLine { start: 8, end: 14 },
                VisualLine { start: 14, end: 18 },
            ]
        );
    }

    #[test]
    fn splits_long_words_when_they_exceed_width() {
        let lines = wrap_lines(&chars("abcdefghij"), 4);

        assert_eq!(
            lines,
            vec![
                VisualLine { start: 0, end: 4 },
                VisualLine { start: 4, end: 8 },
                VisualLine { start: 8, end: 10 },
            ]
        );
    }

    #[test]
    fn does_not_start_next_line_with_a_break_space() {
        let lines = wrap_lines(&chars("hello world"), 5);

        assert_eq!(
            lines,
            vec![
                VisualLine { start: 0, end: 5 },
                VisualLine { start: 6, end: 11 },
            ]
        );
    }

    #[test]
    fn finds_active_line_from_current_index() {
        let lines = wrap_lines(&chars("one two three four"), 8);

        assert_eq!(active_line_index(&lines, 0), 0);
        assert_eq!(active_line_index(&lines, 8), 1);
        assert_eq!(active_line_index(&lines, 16), 2);
        assert_eq!(active_line_index(&lines, 99), 2);
    }

    #[test]
    fn break_space_between_lines_stays_on_previous_active_line() {
        let lines = wrap_lines(&chars("hello world"), 5);

        assert_eq!(active_line_index(&lines, 5), 0);
        assert_eq!(active_line_index(&lines, 6), 1);
    }

    #[test]
    fn detects_caret_on_hidden_boundary_space() {
        let target_chars = chars("hello world");
        let lines = wrap_lines(&target_chars, 5);

        assert!(hidden_boundary_caret(&target_chars, lines[0], 5));
        assert!(!hidden_boundary_caret(&target_chars, lines[1], 6));
    }

    #[test]
    fn keeps_first_three_lines_at_the_beginning() {
        assert_eq!(first_visible_line(0, 5), 0);
        assert_eq!(first_visible_line(1, 5), 0);
    }

    #[test]
    fn centers_active_line_in_the_middle() {
        assert_eq!(first_visible_line(2, 5), 1);
        assert_eq!(first_visible_line(3, 6), 2);
    }

    #[test]
    fn keeps_last_three_lines_visible_at_the_end() {
        assert_eq!(first_visible_line(4, 5), 2);
        assert_eq!(first_visible_line(5, 6), 3);
    }

    #[test]
    fn missed_letters_are_underlined_without_red_glyphs() {
        let mut session = TypingSession::new(TestMode::Words(2), "people know".to_owned());

        for character in "peop ".chars() {
            session.input_char(character);
        }

        let cells = active_render_cells(&session);
        let missed_l = cells
            .iter()
            .find(|cell| cell.span.content.as_ref() == "l")
            .expect("missed l cell");

        assert_eq!(missed_l.span.style.fg, Some(Color::DarkGray));
        assert!(missed_l
            .span
            .style
            .add_modifier
            .contains(Modifier::UNDERLINED));
    }

    #[test]
    fn rendered_cells_wrap_long_extra_input() {
        let mut session = TypingSession::new(TestMode::Words(2), "form those".to_owned());

        for character in "formmmmmmmmm".chars() {
            session.input_char(character);
        }

        let cells = active_render_cells(&session);
        let lines = wrap_render_cells(&cells, 6);

        assert!(lines.len() > 1);
        assert!(lines.iter().all(|line| line.end - line.start <= 6));
        assert!(cells.iter().any(|cell| cell.span.content.as_ref() == "t"));
    }
}
