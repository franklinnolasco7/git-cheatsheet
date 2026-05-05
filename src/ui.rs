use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Mode, Pane};
use crate::data::CATEGORIES;

const GREEN: Color = Color::Green;
const BLUE: Color = Color::Cyan;
const ORANGE: Color = Color::Yellow;
const RUST: Color = Color::Magenta;
const RED: Color = Color::Red;
const YELLOW: Color = Color::LightYellow;
const GRAY: Color = Color::Gray;
const DIM: Color = Color::DarkGray;
const FG: Color = Color::White;

const SEARCH_HEIGHT: u16 = 2;
const SIDEBAR_WIDTH: u16 = 26;
const HELP_MODAL_HEIGHT: u16 = 26;
const HELP_MODAL_WIDTH: u16 = 54;
const DETAILS_HEIGHT: u16 = 9;
const MIN_WIDTH: u16 = 60;
const MIN_HEIGHT: u16 = 24;

pub fn render(f: &mut Frame, app: &App) {
    let size = f.size();

    if size.width < MIN_WIDTH || size.height < MIN_HEIGHT {
        render_too_small(f, size);
        return;
    }

    let root_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(GREEN))
        .title(Span::styled(
            " git-cheat ",
            Style::default().fg(GREEN).add_modifier(Modifier::BOLD),
        ));

    let inner_area = root_block.inner(size);
    f.render_widget(root_block, size);

    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(inner_area);

    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(SIDEBAR_WIDTH), Constraint::Min(0)])
        .split(root[0]);

    render_categories_sidebar(f, app, main[0]);
    render_commands_pane(f, app, main[1]);
    render_status_bar(f, app, root[1]);

    if app.mode == Mode::Help {
        render_help_modal(f, size);
    }
}

fn render_too_small(f: &mut Frame, area: Rect) {
    let bg = Block::default().style(Style::default().bg(Color::Rgb(30, 30, 46)));
    f.render_widget(bg, area);

    let lines = vec![
        Line::from(vec![Span::styled(
            " Terminal size too small: ",
            Style::default().fg(GRAY).add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled(" Width = ", Style::default().fg(GRAY)),
            Span::styled(
                format!("{}", area.width),
                Style::default().fg(if area.width < MIN_WIDTH { RED } else { GREEN }),
            ),
            Span::styled(" Height = ", Style::default().fg(GRAY)),
            Span::styled(
                format!("{}", area.height),
                Style::default().fg(if area.height < MIN_HEIGHT { RED } else { GREEN }),
            ),
        ]),
        Line::from(Span::raw("")),
        Line::from(vec![Span::styled(
            " Needed for current config: ",
            Style::default().fg(DIM),
        )]),
        Line::from(vec![
            Span::styled(" Width = ", Style::default().fg(DIM)),
            Span::styled(format!("{}", MIN_WIDTH), Style::default().fg(GRAY)),
            Span::styled(" Height = ", Style::default().fg(DIM)),
            Span::styled(format!("{}", MIN_HEIGHT), Style::default().fg(GRAY)),
        ]),
    ];

    let text_height = lines.len() as u16;
    let text_width = 36u16;
    let x = area.x + area.width.saturating_sub(text_width) / 2;
    let y = area.y + area.height.saturating_sub(text_height) / 2;
    let popup = Rect {
        x,
        y,
        width: text_width.min(area.width),
        height: text_height.min(area.height),
    };

    f.render_widget(Paragraph::new(Text::from(lines)), popup);
}

fn render_categories_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let is_active = app.active_pane == Pane::Sidebar;
    let block = Block::default()
        .borders(Borders::RIGHT)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(if is_active { GREEN } else { DIM }));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let header_area = Rect {
        x: inner.x,
        y: inner.y,
        width: inner.width,
        height: 1,
    };
    let content_area = Rect {
        x: inner.x,
        y: inner.y + 1,
        width: inner.width,
        height: inner.height.saturating_sub(1),
    };

    let total_commands = App::total_commands();
    let header = Paragraph::new(Line::from(vec![
        Span::styled(" \u{25a0} ", Style::default().fg(GREEN)),
        Span::styled(
            format!("{:<17}", "categories"),
            Style::default().fg(GRAY).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{:>3}", total_commands),
            Style::default().fg(DIM).add_modifier(Modifier::BOLD),
        ),
    ]));
    f.render_widget(header, header_area);

    let items: Vec<ListItem> = CATEGORIES
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            let is_active = i == app.category_index;
            let count_str = format!("{:>3}", cat.commands.len());
            let line = if is_active {
                Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(
                        format!("{:<18}", cat.name),
                        Style::default().fg(FG).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        count_str,
                        Style::default().fg(GREEN).add_modifier(Modifier::BOLD),
                    ),
                ])
            } else {
                Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(format!("{:<18}", cat.name), Style::default().fg(GRAY)),
                    Span::styled(count_str, Style::default().fg(DIM)),
                ])
            };
            let style = if is_active {
                Style::default().bg(DIM)
            } else {
                Style::default()
            };
            ListItem::new(line).style(style)
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.category_index));

    let list = List::new(items).highlight_style(Style::default().bg(DIM));
    f.render_stateful_widget(list, content_area, &mut state);
}

fn render_commands_pane(f: &mut Frame, app: &App, area: Rect) {
    let details_h = DETAILS_HEIGHT.min(area.height.saturating_sub(4));
    let list_h = area.height.saturating_sub(details_h);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(list_h), Constraint::Length(details_h)])
        .split(area);

    render_command_list(f, app, chunks[0]);
    if details_h > 0 {
        render_command_details(f, app, chunks[1]);
    }
}

fn render_command_list(f: &mut Frame, app: &App, area: Rect) {
    let search_height = SEARCH_HEIGHT;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(search_height), Constraint::Min(0)])
        .split(area);

    let search_text = if app.mode == Mode::Search {
        let spans = if app.search_query.is_empty() {
            vec![
                Span::styled(" type to search...", Style::default().fg(DIM)),
                Span::styled(
                    "\u{2588}",
                    Style::default()
                        .fg(GREEN)
                        .add_modifier(Modifier::SLOW_BLINK),
                ),
            ]
        } else {
            vec![
                Span::styled(" ", Style::default()),
                Span::styled(
                    app.search_query.as_str(),
                    Style::default().fg(FG).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "\u{2588}",
                    Style::default().fg(FG).add_modifier(Modifier::SLOW_BLINK),
                ),
            ]
        };
        Line::from(spans)
    } else {
        Line::from(vec![
            Span::styled(" press ", Style::default().fg(DIM)),
            Span::styled("/", Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
            Span::styled(" to search", Style::default().fg(DIM)),
        ])
    };

    let search_block = Block::default()
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(if app.mode == Mode::Search { GREEN } else { DIM }));
    let inner_area = search_block.inner(chunks[0]);
    f.render_widget(search_block, chunks[0]);

    let search_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(15)])
        .split(inner_area);

    f.render_widget(Paragraph::new(search_text), search_chunks[0]);

    let commands = app.visible_commands();
    let result_count = commands.len();

    let showing_text = Paragraph::new(Line::from(vec![Span::styled(
        format!("{} ", result_count),
        Style::default().fg(DIM),
    )]))
    .alignment(ratatui::layout::Alignment::Right);
    f.render_widget(showing_text, search_chunks[1]);

    let is_list_active = app.active_pane == Pane::List;
    let list_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(if is_list_active { GREEN } else { DIM }));
    let list_area = list_block.inner(chunks[1]);
    f.render_widget(list_block, chunks[1]);

    if commands.is_empty() {
        f.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled("  \u{25cb} ", Style::default().fg(DIM)),
                Span::styled("no results found", Style::default().fg(DIM)),
            ])),
            list_area,
        );
        return;
    }

    let items: Vec<ListItem> = commands
        .iter()
        .enumerate()
        .map(|(i, cmd)| {
            let is_selected = i == app.command_index;
            let danger_badge = if cmd.danger {
                Span::styled(" ! ", Style::default().fg(RED).add_modifier(Modifier::BOLD))
            } else {
                Span::raw("   ")
            };

            let cmd_span = colorize_command(cmd.cmd, is_selected);
            let cmd_span: Vec<Span> = if cmd.danger {
                cmd_span
                    .into_iter()
                    .map(|s| {
                        let mut style = s.style;
                        style.fg = Some(RED);
                        let _ = style.add_modifier(Modifier::BOLD);
                        Span::styled(s.content, style)
                    })
                    .collect()
            } else {
                cmd_span
            };
            let desc_span = Span::styled(
                format!("  {}", cmd.desc),
                Style::default().fg(if is_selected { GRAY } else { DIM }),
            );

            let line = Line::from(
                std::iter::once(danger_badge)
                    .chain(cmd_span)
                    .chain(std::iter::once(desc_span))
                    .collect::<Vec<_>>(),
            );

            let style = if is_selected {
                if app.copy_feedback_timer > 0 {
                    Style::default()
                        .fg(Color::Black)
                        .bg(GREEN)
                        .add_modifier(Modifier::BOLD)
                } else if cmd.danger {
                    Style::default()
                        .fg(Color::White)
                        .bg(RED)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().bg(DIM)
                }
            } else {
                Style::default()
            };

            ListItem::new(line).style(style)
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.command_index));

    let list = List::new(items).highlight_style(Style::default().bg(DIM));
    f.render_stateful_widget(list, list_area, &mut state);
}

fn colorize_command(cmd: &'static str, selected: bool) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let parts: Vec<&str> = cmd.splitn(2, ' ').collect();

    if parts.is_empty() {
        return spans;
    }

    spans.push(Span::styled(
        " git ",
        Style::default()
            .fg(if selected { GREEN } else { DIM })
            .add_modifier(Modifier::BOLD),
    ));

    if parts.len() < 2 {
        return spans;
    }

    for token in parts[1].split(' ') {
        if token.starts_with('-') {
            spans.push(Span::styled(
                format!("{} ", token),
                Style::default().fg(ORANGE),
            ));
        } else if (token.starts_with('<') && token.ends_with('>')) || token.starts_with('"') {
            spans.push(Span::styled(
                format!("{} ", token),
                Style::default().fg(RUST),
            ));
        } else {
            spans.push(Span::styled(
                format!("{} ", token),
                Style::default().fg(if selected { BLUE } else { Color::Cyan }),
            ));
        }
    }

    spans
}

fn render_command_details(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::TOP)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(GREEN));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let Some(cmd) = app.selected_command() else {
        return;
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(inner);

    f.render_widget(
        Paragraph::new(Line::from(colorize_command(cmd.cmd, true))),
        chunks[0],
    );

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::raw("  "),
            Span::styled(cmd.desc, Style::default().fg(GRAY)),
        ])),
        chunks[1],
    );

    if !cmd.note.is_empty() {
        f.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled("  # ", Style::default().fg(DIM)),
                Span::styled(cmd.note, Style::default().fg(DIM)),
            ])),
            chunks[2],
        );
    }

    let ex_lines: Vec<Line> = cmd
        .example
        .lines()
        .map(|l| {
            Line::from(vec![
                Span::styled("  $ ", Style::default().fg(GREEN)),
                Span::styled(l, Style::default().fg(FG)),
            ])
        })
        .collect();

    f.render_widget(
        Paragraph::new(Text::from(ex_lines)).wrap(Wrap { trim: false }),
        chunks[3],
    );
}

fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let keys: &[(&str, &str)] = if app.mode == Mode::Search {
        &[
            ("ESC", "exit search"),
            ("Enter", "confirm"),
            ("j/k", "navigate"),
            ("y", "yank"),
        ]
    } else {
        &[
            ("j/k", "navigate"),
            ("h/l", "category"),
            ("/", "search"),
            ("y", "yank cmd"),
            ("?", "help"),
            ("q", "quit"),
        ]
    };

    let mut spans = vec![Span::raw(" ")];

    for (i, (key, label)) in keys.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(" \u{2502} ", Style::default().fg(DIM)));
        }
        spans.push(Span::styled(
            format!(" {} ", key),
            Style::default()
                .fg(Color::Black)
                .bg(GREEN)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            format!(" {}", label),
            Style::default().fg(DIM),
        ));
    }

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(85), Constraint::Min(0)])
        .split(area);

    f.render_widget(Paragraph::new(Line::from(spans)), bottom_chunks[0]);

    if let Some(ref msg) = app.status_message {
        let msg_span = vec![
            Span::styled(msg.as_str(), Style::default().fg(YELLOW)),
            Span::raw(" "),
        ];
        f.render_widget(
            Paragraph::new(Line::from(msg_span)).alignment(ratatui::layout::Alignment::Right),
            bottom_chunks[1],
        );
    }
}

fn render_help_modal(f: &mut Frame, area: Rect) {
    let width = HELP_MODAL_WIDTH.min(area.width);
    let height = HELP_MODAL_HEIGHT.min(area.height);
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    let popup = Rect {
        x,
        y,
        width,
        height,
    };

    f.render_widget(Clear, popup);

    let block = Block::default()
        .title(Line::from(vec![
            Span::styled("\u{2501}\u{2501}\u{2501}", Style::default().fg(GREEN)),
            Span::styled(
                " keybindings ",
                Style::default().fg(FG).add_modifier(Modifier::BOLD),
            ),
            Span::styled("\u{2501}\u{2501}\u{2501}", Style::default().fg(GREEN)),
        ]))
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(GREEN))
        .style(Style::default().bg(Color::Reset));

    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let bindings: &[(&str, &str)] = &[
        ("Navigation", ""),
        ("  j / Down", "move selection down"),
        ("  k / Up", "move selection up"),
        ("  l / Right", "next category"),
        ("  h / Left", "prev category"),
        ("  g", "jump to first item"),
        ("  G", "jump to last item"),
        ("  Tab", "toggle sidebar / list focus"),
        ("", ""),
        ("Search", ""),
        ("  /", "enter search mode"),
        ("  ESC", "exit search / close help"),
        ("  Backspace", "delete last char"),
        ("", ""),
        ("Actions", ""),
        ("  y / Enter", "yank command to clipboard"),
        ("", ""),
        ("App", ""),
        ("  ?", "toggle this help"),
        ("  q / Ctrl-C", "quit"),
    ];

    let lines: Vec<Line> = bindings
        .iter()
        .map(|(key, desc)| {
            if desc.is_empty() && key.is_empty() {
                Line::from(Span::raw(""))
            } else if desc.is_empty() {
                Line::from(vec![
                    Span::styled("\u{25b8} ", Style::default().fg(GREEN)),
                    Span::styled(
                        *key,
                        Style::default().fg(GREEN).add_modifier(Modifier::BOLD),
                    ),
                ])
            } else {
                Line::from(vec![
                    Span::styled(format!("  {:<20}", key), Style::default().fg(FG)),
                    Span::styled(*desc, Style::default().fg(GRAY)),
                ])
            }
        })
        .collect();

    f.render_widget(Paragraph::new(Text::from(lines)), inner);
}
