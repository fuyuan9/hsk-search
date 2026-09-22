use crate::app::{App, AppMode};
use crate::cjk::{display_width, truncate_to_width};
use crate::models::EntryKind;
use crate::search::LevelFilter;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Cell, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table,
    },
    Frame,
};

pub fn render(frame: &mut Frame, app: &mut App) {
    let size = frame.area();

    // Vertical layout: Header (3), Search (3), Main Body (Min 0), Footer (1)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(1),
        ])
        .split(size);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
        .split(chunks[2]);

    app.layout_areas.search_bar = chunks[1];
    app.layout_areas.results_table = body_chunks[0];
    app.layout_areas.inspector_card = body_chunks[1];

    render_header(frame, app, chunks[0]);
    render_search_bar(frame, app, chunks[1]);
    render_main_body(frame, app, chunks[2]);
    render_footer(frame, app, chunks[3]);

    if app.show_help {
        render_help_modal(frame, size);
    }
}

fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(area);

    // Left: Title
    let title_line = Line::from(vec![
        Span::styled(
            " HSK 3.0 ",
            Style::default()
                .bg(Color::Cyan)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            " Chinese Hanzi & Word Search Client ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ]);
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .border_type(BorderType::Rounded);
    frame.render_widget(
        Paragraph::new(title_line).block(title_block),
        header_chunks[0],
    );

    // Right: Level and Kind Filter Tabs
    let levels = [
        LevelFilter::All,
        LevelFilter::Level(1),
        LevelFilter::Level(2),
        LevelFilter::Level(3),
        LevelFilter::Level(4),
        LevelFilter::Level(5),
        LevelFilter::Level(6),
        LevelFilter::Level(7),
    ];

    let mut tab_spans = Vec::new();
    tab_spans.push(Span::raw(" Level: "));

    for lvl in levels {
        let is_selected = app.level_filter == lvl;
        let label = lvl.short_label();

        if is_selected {
            tab_spans.push(Span::styled(
                format!(" [{label}] "),
                Style::default()
                    .bg(Color::Yellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            tab_spans.push(Span::styled(
                format!(" {label} "),
                Style::default().fg(Color::Gray),
            ));
        }
    }

    tab_spans.push(Span::raw(" | Kind: "));
    let kind_label = app.kind_filter.short_label();
    tab_spans.push(Span::styled(
        format!(" [{kind_label}] "),
        Style::default()
            .bg(Color::Magenta)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    ));

    let filter_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .border_type(BorderType::Rounded);
    frame.render_widget(
        Paragraph::new(Line::from(tab_spans)).block(filter_block),
        header_chunks[1],
    );
}

fn render_search_bar(frame: &mut Frame, app: &App, area: Rect) {
    let result_count_text = format!(" {} matching items ", app.filtered_items.len());

    let mode_badge = match app.mode {
        AppMode::Insert => Span::styled(
            " [INSERT] ",
            Style::default()
                .bg(Color::Green)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        ),
        AppMode::Normal => Span::styled(
            " [NORMAL] ",
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    };

    let prompt_prefix = " 🔍 /: ";
    let display_query = &app.search_query;
    let query_span = Span::styled(
        display_query,
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    let search_line = Line::from(vec![
        mode_badge,
        Span::styled(
            prompt_prefix,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        query_span,
    ]);

    let border_color = match app.mode {
        AppMode::Insert => Color::Cyan,
        AppMode::Normal => Color::Blue,
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
        .title(Span::styled(
            result_count_text,
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Right);

    frame.render_widget(Paragraph::new(search_line).block(block), area);

    // Only render cursor when in Insert mode
    if app.mode == AppMode::Insert {
        let mode_badge_width = 10u16; // " [INSERT] " width
        let cursor_screen_x = area.x
            + mode_badge_width
            + display_width(prompt_prefix) as u16
            + display_width(&app.search_query[..app.cursor_position]) as u16
            + 1;
        let cursor_screen_y = area.y + 1;
        if cursor_screen_x < area.x + area.width - 1 {
            frame.set_cursor_position((cursor_screen_x, cursor_screen_y));
        }
    }
}

fn render_main_body(frame: &mut Frame, app: &mut App, area: Rect) {
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
        .split(area);

    render_results_table(frame, app, body_chunks[0]);
    render_inspector_card(frame, app, body_chunks[1]);
}

fn level_color(level: u8) -> Color {
    match level {
        1 => Color::Green,
        2 => Color::LightGreen,
        3 => Color::Yellow,
        4 => Color::LightYellow,
        5 => Color::Rgb(255, 165, 0), // Orange
        6 => Color::LightRed,
        7.. => Color::Magenta,
        _ => Color::White,
    }
}

fn render_results_table(frame: &mut Frame, app: &mut App, area: Rect) {
    let header_cells = [
        "Type",
        "Level",
        "Simplified",
        "Traditional",
        "Pinyin",
        "English Meaning",
    ]
    .into_iter()
    .map(|h| {
        Cell::from(h).style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
    });
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows: Vec<Row> = app
        .filtered_items
        .iter()
        .map(|item| {
            let kind_badge = item.kind.badge();
            let kind_style = match item.kind {
                EntryKind::Word => Style::default().fg(Color::LightBlue),
                EntryKind::Hanzi => Style::default().fg(Color::LightMagenta),
            };

            let lvl_label = item.level_label();
            let lvl_style = Style::default()
                .fg(level_color(item.level))
                .add_modifier(Modifier::BOLD);

            let simp_style = Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD);
            let trad_style = Style::default().fg(Color::DarkGray);
            let pinyin_style = Style::default().fg(Color::Yellow);

            let truncated_meaning = truncate_to_width(&item.meaning, 40, "..");

            Row::new(vec![
                Cell::from(Span::styled(format!("[{kind_badge}]"), kind_style)),
                Cell::from(Span::styled(lvl_label, lvl_style)),
                Cell::from(Span::styled(&item.simplified, simp_style)),
                Cell::from(Span::styled(&item.traditional, trad_style)),
                Cell::from(Span::styled(&item.pinyin_display, pinyin_style)),
                Cell::from(Span::raw(truncated_meaning)),
            ])
        })
        .collect();

    let table_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Gray))
        .title(" Results (结果) ");

    let widths = [
        Constraint::Length(5),
        Constraint::Length(9),
        Constraint::Length(12),
        Constraint::Length(12),
        Constraint::Length(16),
        Constraint::Min(20),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(table_block)
        .row_highlight_style(
            Style::default()
                .bg(Color::Rgb(30, 60, 120))
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(table, area, &mut app.table_state);

    // Scrollbar
    let total_items = app.filtered_items.len();
    if total_items > 0 {
        let mut scrollbar_state =
            ScrollbarState::new(total_items).position(app.table_state.selected().unwrap_or(0));
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼")),
            area,
            &mut scrollbar_state,
        );
    }
}

fn render_inspector_card(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Green))
        .title(" Inspector / 详细信息 ");

    if let Some(item) = app.selected_item() {
        let mut lines = Vec::new();

        // 1. Large Header: Simplified & Level Badge
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {}  ", item.simplified),
                Style::default()
                    .fg(Color::Yellow)
                    .bg(Color::Rgb(40, 40, 40))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                format!(" [{}] ", item.level_label()),
                Style::default()
                    .fg(level_color(item.level))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(
                format!("({})", item.kind.label()),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
        lines.push(Line::raw(""));

        // 2. Traditional Form
        lines.push(Line::from(vec![
            Span::styled("Traditional (繁体): ", Style::default().fg(Color::Cyan)),
            Span::styled(
                &item.traditional,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::raw(""));

        // 3. Pinyin Breakdown
        lines.push(Line::from(Span::styled(
            "Phonetics (拼音解析):",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::UNDERLINED),
        )));
        lines.push(Line::from(vec![
            Span::styled(" • With Tone:   ", Style::default().fg(Color::Gray)),
            Span::styled(
                &item.pinyin_display,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled(" • Numbered:    ", Style::default().fg(Color::Gray)),
            Span::styled(&item.pinyin_numbered, Style::default().fg(Color::White)),
        ]));
        lines.push(Line::from(vec![
            Span::styled(" • Toneless:    ", Style::default().fg(Color::Gray)),
            Span::styled(&item.pinyin_clean, Style::default().fg(Color::White)),
        ]));
        lines.push(Line::from(vec![
            Span::styled(" • Initials/首字母: ", Style::default().fg(Color::Gray)),
            Span::styled(
                &item.pinyin_initials,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

        // 4. Polyphone Info (if available)
        if !item.polyphones.is_empty() {
            lines.push(Line::raw(""));
            lines.push(Line::from(Span::styled(
                "Polyphonic Readings (多音字异读):",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::UNDERLINED),
            )));
            let poly_str = item.polyphones.join(" / ");
            lines.push(Line::from(vec![
                Span::styled(" • Readings:    ", Style::default().fg(Color::Gray)),
                Span::styled(
                    poly_str,
                    Style::default()
                        .fg(Color::LightMagenta)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        }

        // 5. English Meaning
        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(
            "English Meaning (英文释义):",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::UNDERLINED),
        )));
        if item.meaning.is_empty() {
            lines.push(Line::from(Span::styled(
                "  (No definition available)",
                Style::default().fg(Color::DarkGray),
            )));
        } else {
            // Format definition lines
            for part in item.meaning.split(';') {
                let trimmed = part.trim();
                if !trimmed.is_empty() {
                    lines.push(Line::from(vec![
                        Span::styled(" • ", Style::default().fg(Color::Green)),
                        Span::raw(trimmed),
                    ]));
                }
            }
        }

        // 6. Dataset Attribution badge at bottom
        lines.push(Line::raw(""));
        lines.push(Line::from(vec![Span::styled(
            "Data: HSK 3.0 (CC BY-SA 4.0) | Verified by PinyinPro",
            Style::default().fg(Color::DarkGray),
        )]));

        frame.render_widget(Paragraph::new(lines).block(block), area);
    } else {
        let placeholder = Paragraph::new("No item selected\n没有选中的项目")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center)
            .block(block);
        frame.render_widget(placeholder, area);
    }
}

fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let key_hints = match app.mode {
        AppMode::Insert => vec![
            Span::styled(
                "-- INSERT --",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                "[Esc/Enter]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Normal  "),
            Span::styled(
                "[Ctrl+j/k]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Select  "),
            Span::styled(
                "[Ctrl+w]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Del Word  "),
            Span::styled(
                "[Tab]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Level  "),
            Span::styled(
                "[F1]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Word/Hanzi  "),
            Span::styled(
                "[F2 / ?]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Help"),
        ],
        AppMode::Normal => vec![
            Span::styled(
                "-- NORMAL --",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                "[i/a//]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Search  "),
            Span::styled(
                "[j/k]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Select  "),
            Span::styled(
                "[h/l]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Level  "),
            Span::styled(
                "[gg/G]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Top/End  "),
            Span::styled(
                "[Ctrl+d/u]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" HalfPg  "),
            Span::styled(
                "[K]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Kind  "),
            Span::styled(
                "[?]",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Help  "),
            Span::styled(
                "[q]",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Quit"),
        ],
    };

    let footer = Paragraph::new(Line::from(key_hints)).alignment(Alignment::Center);
    frame.render_widget(footer, area);
}

fn clear_modal_area(frame: &mut Frame, area: Rect) {
    let buf = frame.buffer_mut();
    for y in area.top()..area.bottom() {
        if area.left() > 0 {
            let left_x = area.left() - 1;
            if crate::cjk::display_width(buf[(left_x, y)].symbol()) > 1 {
                buf[(left_x, y)].reset();
            }
        }
        if area.right() < buf.area().right() {
            let right_x = area.right();
            if crate::cjk::display_width(buf[(right_x, y)].symbol()) > 1 {
                buf[(right_x, y)].reset();
            }
        }
        for x in area.left()..area.right() {
            buf[(x, y)].reset();
        }
    }
}

fn render_help_modal(frame: &mut Frame, screen_area: Rect) {
    let popup_width = 82u16.min(screen_area.width.saturating_sub(4));
    let popup_height = 25u16.min(screen_area.height.saturating_sub(2));

    let area = Rect {
        x: screen_area.x + (screen_area.width.saturating_sub(popup_width)) / 2,
        y: screen_area.y + (screen_area.height.saturating_sub(popup_height)) / 2,
        width: popup_width,
        height: popup_height,
    };

    clear_modal_area(frame, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .title(" About & Help (帮助与关于) ");

    let help_text = vec![
        Line::from(Span::styled(
            " HSK 3.0 Chinese Hanzi & Word Search Client ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::raw(""),
        Line::from(Span::styled(
            "> Normal Mode (浏览模式 / Vim Keybindings):",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  - j / k:              Select next / previous item (下/上 移动)"),
        Line::from("  - h / l:              Previous / next HSK level filter (切换等级)"),
        Line::from("  - gg / G:             Jump to top / bottom of results (跳至首项/末项)"),
        Line::from("  - Ctrl+d / Ctrl+u:    Scroll half-page down / up (半屏翻页)"),
        Line::from("  - i / a / /:          Enter Insert mode to search (进入输入模式)"),
        Line::from("  - c / C:              Clear search and enter Insert mode (清空并输入)"),
        Line::from("  - K / F1:             Cycle Kind filter: All / Words / Hanzi (切换分类)"),
        Line::from("  - q / Ctrl+c:         Quit application (退出程序)"),
        Line::raw(""),
        Line::from(Span::styled(
            "> Insert Mode (输入模式 / Instant Search):",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  - Typing:             Search pinyin (\x27nihao\x27), initials (\x27yh\x27), hanzi, english"),
        Line::from("  - Esc / Enter:        Return to Normal mode (返回浏览模式)"),
        Line::from("  - Ctrl+j / Ctrl+n:    Select next item without leaving Insert mode"),
        Line::from("  - Ctrl+k / Ctrl+p:    Select previous item without leaving Insert mode"),
        Line::from("  - Ctrl+w / Ctrl+u:    Delete word backward / clear search line"),
        Line::raw(""),
        Line::from(Span::styled(
            "> License & Credits (许可与致谢):",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  - Code: MIT (fuyuan9) | Data: CC BY-SA 4.0 (HSK 3.0, CC-CEDICT, Pleco)"),
        Line::from("  - Phonetics: Assisted by PinyinPro (99.85% polyphone precision)"),
        Line::raw(""),
        Line::from(Span::styled(
            " Press [Esc], [F2], or [?] to close this help window ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
    ];

    frame.render_widget(Paragraph::new(help_text).block(block), area);
}
