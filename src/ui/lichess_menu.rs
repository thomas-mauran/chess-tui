use crate::app::App;
use chrono::{DateTime, NaiveDate, Utc};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        Axis, Block, BorderType, Borders, Chart, Dataset, GraphType, LegendPosition, Paragraph,
    },
    Frame,
};

pub fn render_lichess_menu(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Create main layout: title, content (menu + stats), footer
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Content (menu + stats)
            Constraint::Length(3), // Footer
        ])
        .split(area);

    // Title
    let title = Paragraph::new("Lichess Menu")
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
    frame.render_widget(title, main_chunks[0]);

    // Split content area into menu (left) and stats (right)
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40), // Menu
            Constraint::Percentage(60), // Stats
        ])
        .split(main_chunks[1]);

    // Split stats area into stats text and graph
    let stats_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(2),     // Stats text
            Constraint::Length(24), // Graph
        ])
        .split(content_chunks[1]);

    // Menu options
    let menu_items = [
        ("Seek Game", "Find a random opponent"),
        ("Puzzle", "Play a puzzle"),
        ("My Ongoing Games", "View and join your current games"),
        ("Join by Code", "Enter a game code to join"),
        ("Disconnect", "Remove Lichess token and logout"),
    ];

    let mut menu_lines = vec![Line::from("")];

    for (idx, (option, description)) in menu_items.iter().enumerate() {
        let is_selected = app.menu_cursor == idx as u8;
        let is_disconnect = idx == 4; // Disconnect is the 5th option (index 4)

        let style = if is_selected {
            if is_disconnect {
                // Selected disconnect option - red background with white text
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            }
        } else if is_disconnect {
            // Unselected disconnect option - red text
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::White)
        };

        let prefix = if is_selected { "► " } else { "  " };

        menu_lines.push(Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(*option, style),
        ]));

        menu_lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(*description, Style::default().fg(Color::Gray)),
        ]));

        menu_lines.push(Line::from(""));
    }

    let menu = Paragraph::new(menu_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Select an option"),
        )
        .alignment(Alignment::Left);
    frame.render_widget(menu, content_chunks[0]);

    // User stats panel
    let mut stats_lines = vec![Line::from("")];

    if let Some(profile) = &app.lichess_user_profile {
        // Username and title
        let username_display = if let Some(title) = &profile.title {
            format!("{} {}", title, profile.username)
        } else {
            profile.username.clone()
        };
        stats_lines.push(Line::from(vec![Span::styled(
            "Username:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]));
        stats_lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(username_display, Style::default().fg(Color::White)),
        ]));

        // Online status
        if let Some(online) = profile.online {
            stats_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    if online { "● Online" } else { "○ Offline" },
                    Style::default().fg(if online { Color::Green } else { Color::Gray }),
                ),
            ]));
        }
        stats_lines.push(Line::from(""));

        // Location
        if let Some(profile_info) = &profile.profile {
            if let Some(country) = &profile_info.country {
                stats_lines.push(Line::from(vec![Span::styled(
                    "Country:",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )]));
                stats_lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(country, Style::default().fg(Color::White)),
                ]));
                stats_lines.push(Line::from(""));
            }
        }

        // Ratings with progress
        if let Some(perfs) = &profile.perfs {
            stats_lines.push(Line::from(vec![Span::styled(
                "Ratings:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));

            if let Some(blitz) = &perfs.blitz {
                let mut rating_text = format!("{}", blitz.rating);
                if let Some(prog) = blitz.prog {
                    if prog > 0 {
                        rating_text = format!("{} (+{})", rating_text, prog);
                    } else if prog < 0 {
                        rating_text = format!("{} ({})", rating_text, prog);
                    }
                }
                stats_lines.push(Line::from(vec![
                    Span::raw("  Blitz: "),
                    Span::styled(rating_text, Style::default().fg(Color::Yellow)),
                ]));
            }
            if let Some(rapid) = &perfs.rapid {
                let mut rating_text = format!("{}", rapid.rating);
                if let Some(prog) = rapid.prog {
                    if prog > 0 {
                        rating_text = format!("{} (+{})", rating_text, prog);
                    } else if prog < 0 {
                        rating_text = format!("{} ({})", rating_text, prog);
                    }
                }
                stats_lines.push(Line::from(vec![
                    Span::raw("  Rapid: "),
                    Span::styled(rating_text, Style::default().fg(Color::Yellow)),
                ]));
            }
            if let Some(classical) = &perfs.classical {
                let mut rating_text = format!("{}", classical.rating);
                if let Some(prog) = classical.prog {
                    if prog > 0 {
                        rating_text = format!("{} (+{})", rating_text, prog);
                    } else if prog < 0 {
                        rating_text = format!("{} ({})", rating_text, prog);
                    }
                }
                stats_lines.push(Line::from(vec![
                    Span::raw("  Classical: "),
                    Span::styled(rating_text, Style::default().fg(Color::Yellow)),
                ]));
            }
            if let Some(bullet) = &perfs.bullet {
                let mut rating_text = format!("{}", bullet.rating);
                if let Some(prog) = bullet.prog {
                    if prog > 0 {
                        rating_text = format!("{} (+{})", rating_text, prog);
                    } else if prog < 0 {
                        rating_text = format!("{} ({})", rating_text, prog);
                    }
                }
                stats_lines.push(Line::from(vec![
                    Span::raw("  Bullet: "),
                    Span::styled(rating_text, Style::default().fg(Color::Yellow)),
                ]));
            }
            if let Some(puzzle) = &perfs.puzzle {
                let mut rating_text = format!("{}", puzzle.rating);
                if let Some(prog) = puzzle.prog {
                    if prog > 0 {
                        rating_text = format!("{} (+{})", rating_text, prog);
                    } else if prog < 0 {
                        rating_text = format!("{} ({})", rating_text, prog);
                    }
                }
                stats_lines.push(Line::from(vec![
                    Span::raw("  Puzzle: "),
                    Span::styled(rating_text, Style::default().fg(Color::Yellow)),
                ]));
            }
        }

        // Game statistics
        if let Some(counts) = &profile.count {
            stats_lines.push(Line::from(""));
            stats_lines.push(Line::from(vec![Span::styled(
                "Games:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));

            if let Some(all) = counts.all {
                stats_lines.push(Line::from(vec![
                    Span::raw("  Total: "),
                    Span::styled(format!("{}", all), Style::default().fg(Color::White)),
                ]));
            }
            if let Some(win) = counts.win {
                stats_lines.push(Line::from(vec![
                    Span::raw("  Wins: "),
                    Span::styled(format!("{}", win), Style::default().fg(Color::Green)),
                ]));
            }
            if let Some(loss) = counts.loss {
                stats_lines.push(Line::from(vec![
                    Span::raw("  Losses: "),
                    Span::styled(format!("{}", loss), Style::default().fg(Color::Red)),
                ]));
            }
            if let Some(draw) = counts.draw {
                stats_lines.push(Line::from(vec![
                    Span::raw("  Draws: "),
                    Span::styled(format!("{}", draw), Style::default().fg(Color::Yellow)),
                ]));
            }
            if let Some(playing) = counts.playing {
                if playing > 0 {
                    stats_lines.push(Line::from(vec![
                        Span::raw("  Playing: "),
                        Span::styled(format!("{}", playing), Style::default().fg(Color::Magenta)),
                    ]));
                }
            }
        }
    } else {
        stats_lines.push(Line::from(vec![Span::styled(
            "Loading...",
            Style::default().fg(Color::Gray),
        )]));
    }

    let stats = Paragraph::new(stats_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("User Stats"),
        )
        .alignment(Alignment::Left);
    frame.render_widget(stats, stats_chunks[0]);

    // Statistics graph - show rating history line chart
    if let Some(history) = &app.lichess_rating_history {
        render_rating_history_chart(frame, history, stats_chunks[1]);
    } else {
        let loading_graph = Paragraph::new(if app.lichess_user_profile.is_some() {
            "Loading rating history..."
        } else {
            "Loading..."
        })
        .style(Style::default().fg(Color::Gray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Rating History"),
        )
        .alignment(Alignment::Center);
        frame.render_widget(loading_graph, stats_chunks[1]);
    }

    // Footer with controls
    let footer = Paragraph::new(vec![Line::from(vec![
        Span::styled("↑/↓", Style::default().fg(Color::Cyan)),
        Span::raw(" Navigate  "),
        Span::styled("Enter", Style::default().fg(Color::Cyan)),
        Span::raw(" Select  "),
        Span::styled("Esc", Style::default().fg(Color::Cyan)),
        Span::raw(" Back to Home"),
    ])])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    frame.render_widget(footer, main_chunks[2]);
}

fn render_rating_history_chart(
    frame: &mut Frame,
    history: &[crate::lichess::RatingHistoryEntry],
    area: Rect,
) {
    let cutoff_days = (Utc::now() - chrono::Duration::days(90)).timestamp() as f64 / 86400.0;
    let color_map = get_time_control_colors();

    // Process and filter rating data
    let (datasets_data, dataset_names, bounds) = match process_rating_data(history, cutoff_days) {
        Some(result) => result,
        None => {
            render_empty_chart(frame, area);
            return;
        }
    };

    let (min_date, max_date, min_rating, max_rating) = bounds;
    if datasets_data.is_empty() {
        render_empty_chart(frame, area);
        return;
    }

    // Sort and create datasets
    let (datasets, last_points) =
        create_sorted_datasets(&datasets_data, &dataset_names, &color_map);
    if datasets.is_empty() {
        render_empty_chart(frame, area);
        return;
    }

    // Calculate chart bounds and labels
    let (min_rating_display, max_rating_display) = calculate_rating_bounds(min_rating, max_rating);
    let x_labels = create_x_axis_labels(min_date, max_date);
    let y_labels = create_y_axis_labels(min_rating_display, max_rating_display);

    // Render chart
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Rating History (last 90 days");
    let inner_area = block.inner(area);
    let max_date_with_padding =
        calculate_x_padding(max_date, min_date, &last_points, inner_area.width);

    let chart = Chart::new(datasets)
        .block(block)
        .x_axis(
            Axis::default()
                .title("Time")
                .style(Style::default().fg(Color::Gray))
                .labels(x_labels)
                .bounds([min_date, max_date_with_padding]),
        )
        .y_axis(
            Axis::default()
                .title("Rating")
                .style(Style::default().fg(Color::Gray))
                .labels(y_labels)
                .bounds([min_rating_display, max_rating_display]),
        )
        .legend_position(Some(LegendPosition::BottomLeft))
        .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)));

    frame.render_widget(chart, area);

    // Draw ELO labels
    draw_elo_labels(
        frame.buffer_mut(),
        &last_points,
        inner_area,
        min_date,
        max_date_with_padding,
        min_rating_display,
        max_rating_display,
    );
}

/// Convert rating history point to days since epoch
fn point_to_days(year: i32, month: i32, day: i32) -> Option<f64> {
    NaiveDate::from_ymd_opt(year, (month + 1) as u32, day as u32)
        .and_then(|date| date.and_hms_opt(0, 0, 0))
        .map(|datetime| {
            DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc).timestamp() as f64 / 86400.0
        })
}

/// Process rating history data and filter to last 90 days
fn process_rating_data(
    history: &[crate::lichess::RatingHistoryEntry],
    cutoff_days: f64,
) -> Option<(Vec<Vec<(f64, f64)>>, Vec<String>, (f64, f64, f64, f64))> {
    let mut datasets_data = Vec::new();
    let mut dataset_names = Vec::new();
    let mut min_date = f64::MAX;
    let mut max_date = f64::MIN;
    let mut min_rating = f64::MAX;
    let mut max_rating = f64::MIN;

    for entry in history.iter() {
        if entry.points.is_empty() {
            continue;
        }

        let mut data_points: Vec<(f64, f64)> = entry
            .points
            .iter()
            .filter_map(|point| {
                if point.len() != 4 {
                    return None;
                }
                point_to_days(point[0], point[1], point[2])
                    .filter(|&days| days >= cutoff_days)
                    .map(|days| (days, point[3] as f64))
            })
            .collect();

        if data_points.is_empty() {
            continue;
        }

        data_points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        // Add cutoff point if first point is after cutoff
        if let Some(&(first_days, first_rating)) = data_points.first() {
            if first_days > cutoff_days {
                data_points.insert(0, (cutoff_days, first_rating));
            }
        }

        // Update bounds
        for (days, rating) in &data_points {
            min_date = min_date.min(*days);
            max_date = max_date.max(*days);
            min_rating = min_rating.min(*rating);
            max_rating = max_rating.max(*rating);
        }

        if data_points.len() >= 2 {
            datasets_data.push(data_points);
            dataset_names.push(entry.name.clone());
        }
    }

    if datasets_data.is_empty() || min_date == f64::MAX {
        return None;
    }

    Some((
        datasets_data,
        dataset_names,
        (min_date, max_date, min_rating, max_rating),
    ))
}

/// Get color mapping for time controls
fn get_time_control_colors() -> std::collections::HashMap<&'static str, Color> {
    [
        ("Bullet", Color::Cyan),
        ("Blitz", Color::LightBlue),
        ("Classical", Color::Magenta),
        ("Correspondence", Color::LightYellow),
        ("Rapid", Color::Green),
        ("Puzzles", Color::Red),
    ]
    .iter()
    .cloned()
    .collect()
}

/// Create sorted datasets with colors and extract last points for labels
fn create_sorted_datasets<'a>(
    datasets_data: &'a [Vec<(f64, f64)>],
    dataset_names: &'a [String],
    color_map: &std::collections::HashMap<&str, Color>,
) -> (Vec<Dataset<'a>>, Vec<(f64, f64, Color)>) {
    const PRIORITY_ORDER: &[&str] = &[
        "Bullet",
        "Blitz",
        "Classical",
        "Correspondence",
        "Rapid",
        "Puzzles",
        "Other",
    ];

    // Sort by priority, then alphabetically
    let mut indexed: Vec<_> = datasets_data
        .iter()
        .zip(dataset_names.iter())
        .enumerate()
        .collect();

    indexed.sort_by(|(_, (_, a_name)), (_, (_, b_name))| {
        let a_priority = PRIORITY_ORDER
            .iter()
            .position(|&x| x == a_name.as_str())
            .unwrap_or(999);
        let b_priority = PRIORITY_ORDER
            .iter()
            .position(|&x| x == b_name.as_str())
            .unwrap_or(999);

        a_priority.cmp(&b_priority).then_with(|| a_name.cmp(b_name))
    });

    // Create datasets and collect last points
    let mut datasets = Vec::new();
    let mut last_points = Vec::new();

    for (_, (data_points, name)) in indexed.iter() {
        let color = color_map
            .get(name.as_str())
            .copied()
            .unwrap_or(Color::White);

        if let Some(&(last_days, last_rating)) = data_points.last() {
            last_points.push((last_days, last_rating, color));
        }

        datasets.push(
            Dataset::default()
                .name(name.as_str())
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(color))
                .graph_type(GraphType::Line)
                .data(data_points),
        );
    }

    (datasets, last_points)
}

/// Calculate rating bounds with padding, rounded to multiples of 5
fn calculate_rating_bounds(min_rating: f64, max_rating: f64) -> (f64, f64) {
    let padding = (max_rating - min_rating) * 0.2;
    let mut min_display = (min_rating - padding).max(0.0);
    let mut max_display = max_rating + padding;

    min_display -= min_display % 5.0;
    max_display += max_display % 5.0;

    (min_display, max_display)
}

/// Create X-axis labels in month/year format
fn create_x_axis_labels(min_date: f64, max_date: f64) -> Vec<Span<'static>> {
    if max_date <= min_date {
        return vec![Span::raw("Time".to_string())];
    }

    let format_date = |days: f64| {
        let timestamp = (days * 86400.0) as i64;
        DateTime::<Utc>::from_timestamp(timestamp, 0)
            .map(|dt| dt.format("%b %Y").to_string())
            .unwrap_or_else(|| format!("{:.0}", days))
    };

    vec![
        Span::styled(
            format_date(min_date),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(format_date((min_date + max_date) / 2.0)),
        Span::styled(
            format_date(max_date),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ]
}

/// Create Y-axis labels with 5 evenly spaced values
fn create_y_axis_labels(min_rating: f64, max_rating: f64) -> Vec<Span<'static>> {
    const NUM_LABELS: usize = 5;
    let range = max_rating - min_rating;
    let mut labels = Vec::new();

    for i in 0..NUM_LABELS {
        let value = min_rating + (range * i as f64 / (NUM_LABELS - 1) as f64);
        let formatted = format!("{:.0}", value);
        let is_bold = i == 0 || i == NUM_LABELS / 2 || i == NUM_LABELS - 1;

        labels.push(if is_bold {
            Span::styled(formatted, Style::default().add_modifier(Modifier::BOLD))
        } else {
            Span::raw(formatted)
        });
    }

    labels
}

/// Calculate X-axis padding needed to fit ELO labels
fn calculate_x_padding(
    max_date: f64,
    min_date: f64,
    last_points: &[(f64, f64, Color)],
    chart_width: u16,
) -> f64 {
    let max_label_width = last_points
        .iter()
        .map(|(_, rating, _)| format!("{:.0}", rating).len())
        .max()
        .unwrap_or(4) as u16;

    let total_label_width = 1 + 1 + max_label_width + 2; // square + spacing + text + margin
    let date_range = max_date - min_date;
    let padding_ratio = (total_label_width as f64 / chart_width as f64).max(0.05);

    max_date + (date_range * padding_ratio)
}

/// Render empty chart message
fn render_empty_chart(frame: &mut Frame, area: Rect) {
    let empty_graph = Paragraph::new("No rating history available")
        .style(Style::default().fg(Color::Gray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Rating History"),
        )
        .alignment(Alignment::Center);
    frame.render_widget(empty_graph, area);
}

/// Calculate label positions and draw them
fn draw_elo_labels(
    buffer: &mut ratatui::buffer::Buffer,
    last_points: &[(f64, f64, Color)],
    inner_area: Rect,
    min_date: f64,
    max_date_with_padding: f64,
    min_rating_display: f64,
    max_rating_display: f64,
) {
    let chart_x = inner_area.x;
    let chart_y = inner_area.y;
    let chart_width = inner_area.width;
    let chart_height = inner_area.height;

    // Calculate label positions
    let mut label_positions: Vec<(u16, u16, f64, Color, String)> = last_points
        .iter()
        .map(|(days, rating, color)| {
            let line_x = if max_date_with_padding > min_date {
                chart_x
                    + (((days - min_date) / (max_date_with_padding - min_date))
                        * (chart_width - 1) as f64) as u16
            } else {
                chart_x
            };

            let y = if max_rating_display > min_rating_display {
                chart_y
                    + (((max_rating_display - rating) / (max_rating_display - min_rating_display))
                        * (chart_height - 1) as f64) as u16
            } else {
                chart_y + chart_height / 2
            };

            let x = line_x + 3; // label offset
            let rating_text = format!("{:.0}", rating);

            (x, y, *rating, *color, rating_text)
        })
        .collect();

    // Sort by Y position and group by proximity
    label_positions.sort_by(|a, b| a.1.cmp(&b.1));
    let groups = group_labels_by_proximity(&label_positions, 2);

    // Draw labels
    for group in groups.iter() {
        match group.len() {
            1 => draw_single_label(
                buffer,
                &label_positions[group[0]],
                chart_x,
                chart_y,
                chart_width,
                chart_height,
            ),
            _ => draw_grouped_labels(
                buffer,
                group,
                &label_positions,
                chart_x,
                chart_y,
                chart_width,
                chart_height,
            ),
        }
    }
}

/// Group labels that are too close together vertically
fn group_labels_by_proximity(
    label_positions: &[(u16, u16, f64, Color, String)],
    threshold: u16,
) -> Vec<Vec<usize>> {
    let mut groups: Vec<Vec<usize>> = Vec::new();

    for (idx, label) in label_positions.iter().enumerate() {
        let label_y = label.1;
        let mut added = false;

        for group in groups.iter_mut() {
            if group
                .iter()
                .any(|&group_idx| label_positions[group_idx].1.abs_diff(label_y) <= threshold)
            {
                group.push(idx);
                added = true;
                break;
            }
        }

        if !added {
            groups.push(vec![idx]);
        }
    }

    groups
}

/// Draw a single ELO label with its colored square and rating text
fn draw_single_label(
    buffer: &mut ratatui::buffer::Buffer,
    label: &(u16, u16, f64, Color, String),
    chart_x: u16,
    chart_y: u16,
    chart_width: u16,
    chart_height: u16,
) {
    let (x, y, _rating, color, rating_text) = label;

    // Check bounds
    if *x >= chart_x + chart_width || *y >= chart_y + chart_height {
        return;
    }

    // Draw colored square
    buffer[(*x, *y)]
        .set_symbol("█")
        .set_style(Style::default().fg(*color));

    // Draw rating number next to the square
    let text_x = *x + 1;
    let text_end = text_x + rating_text.len() as u16;

    if text_end <= chart_x + chart_width {
        for (i, ch) in rating_text.chars().enumerate() {
            let pos_x = text_x + i as u16;
            buffer[(pos_x, *y)]
                .set_char(ch)
                .set_style(Style::default().fg(*color));
        }
    }
}

/// Draw multiple labels stacked vertically when they're too close together
fn draw_grouped_labels(
    buffer: &mut ratatui::buffer::Buffer,
    group: &[usize],
    label_positions: &[(u16, u16, f64, Color, String)],
    chart_x: u16,
    chart_y: u16,
    chart_width: u16,
    chart_height: u16,
) {
    // Calculate average Y position of all labels in the group
    let avg_y = (group
        .iter()
        .map(|&idx| label_positions[idx].1 as u32)
        .sum::<u32>()
        / group.len() as u32) as u16;

    // Use X position of the first label
    let x = label_positions[group[0]].0;

    // Calculate starting Y position (centered around average)
    let half_group_size = group.len() as u16 / 2;
    let start_y = if avg_y >= half_group_size {
        avg_y.saturating_sub(half_group_size)
    } else {
        chart_y
    };

    // Check X bounds
    if x >= chart_x + chart_width {
        return;
    }

    // Draw each label on its own line, stacked vertically
    for (line_idx, &label_idx) in group.iter().enumerate() {
        let (_x, _y, _rating, color, rating_text) = &label_positions[label_idx];
        let y = start_y + line_idx as u16;

        // Check Y bounds
        if y >= chart_y + chart_height {
            continue;
        }

        // Draw colored square for this label
        buffer[(x, y)]
            .set_symbol("█")
            .set_style(Style::default().fg(*color));

        // Draw rating text in its color
        let text_x = x + 1;
        let text_end = text_x + rating_text.len() as u16;

        if text_end <= chart_x + chart_width {
            for (i, ch) in rating_text.chars().enumerate() {
                let pos_x = text_x + i as u16;
                buffer[(pos_x, y)]
                    .set_char(ch)
                    .set_style(Style::default().fg(*color));
            }
        }
    }
}
