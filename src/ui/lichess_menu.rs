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
    // Calculate the date 3 months ago
    let now = Utc::now();
    let three_months_ago = now - chrono::Duration::days(90);
    let cutoff_days = three_months_ago.timestamp() as f64 / 86400.0;

    // Collect and convert rating history data (filtered to last 90 days)
    let mut all_datasets_data: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut dataset_names = Vec::new();
    let mut min_date = f64::MAX;
    let mut max_date = f64::MIN;
    let mut min_rating = f64::MAX;
    let mut max_rating = f64::MIN;

    // Color mapping for different time controls
    let color_map: std::collections::HashMap<&str, Color> = [
        ("Bullet", Color::Cyan),
        ("Blitz", Color::LightBlue),
        ("Classical", Color::Magenta),
        ("Correspondence", Color::LightYellow),
        ("Rapid", Color::Green),
        ("Puzzles", Color::Red),
    ]
    .iter()
    .cloned()
    .collect();

    for entry in history.iter() {
        if entry.points.is_empty() {
            continue;
        }

        let mut data_points = Vec::new();
        for point in &entry.points {
            if point.len() != 4 {
                continue;
            }
            let year = point[0];
            let month = point[1] + 1; // JavaScript months are 0-indexed
            let day = point[2];
            let rating = point[3] as f64;

            // Convert to days since epoch for easier handling
            if let Some(date) = NaiveDate::from_ymd_opt(year, month as u32, day as u32) {
                if let Some(datetime) = date.and_hms_opt(0, 0, 0) {
                    // Convert NaiveDateTime to DateTime<Utc> and get timestamp
                    let utc_datetime = DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc);
                    let days = utc_datetime.timestamp() as f64 / 86400.0;

                    // Only include data points from the last 90 days
                    if days >= cutoff_days {
                        data_points.push((days, rating));
                    }
                }
            }
        }

        // Only add dataset if we have at least 1 point
        if !data_points.is_empty() {
            // Sort by date to ensure proper line drawing
            data_points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

            // Add a point at the cutoff date with the same rating as the first point
            // This ensures the line starts from the left edge of the chart
            if let Some(&(first_days, first_rating)) = data_points.first() {
                // Only add the cutoff point if the first point is after the cutoff
                if first_days > cutoff_days {
                    data_points.insert(0, (cutoff_days, first_rating));
                }
            }

            // Update min/max for filtered data (including the cutoff point)
            for (days, rating) in &data_points {
                min_date = min_date.min(*days);
                max_date = max_date.max(*days);
                min_rating = min_rating.min(*rating);
                max_rating = max_rating.max(*rating);
            }

            // Only add dataset if we have at least 2 points (needed for a line)
            if data_points.len() >= 2 {
                all_datasets_data.push(data_points);
                dataset_names.push(entry.name.clone());
            }
        }
    }

    if all_datasets_data.is_empty() || min_date == f64::MAX {
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
        return;
    }

    // Sort datasets by predefined priority order, then alphabetically
    let priority_order = vec![
        "Bullet",
        "Blitz",
        "Classical",
        "Correspondence",
        "Rapid",
        "Puzzles",
        "Other",
    ];
    let mut indexed_datasets: Vec<(usize, &Vec<(f64, f64)>, &String)> = all_datasets_data
        .iter()
        .zip(dataset_names.iter())
        .enumerate()
        .map(|(idx, (data, name))| (idx, data, name))
        .collect();

    indexed_datasets.sort_by(|a, b| {
        let a_name = a.2;
        let b_name = b.2;

        // Get priority indices (lower = higher priority)
        let a_priority = priority_order
            .iter()
            .position(|&x| x == a_name)
            .unwrap_or(999);
        let b_priority = priority_order
            .iter()
            .position(|&x| x == b_name)
            .unwrap_or(999);

        // First sort by priority, then alphabetically
        match a_priority.cmp(&b_priority) {
            std::cmp::Ordering::Equal => a_name.cmp(b_name),
            other => other,
        }
    });

    // Create datasets from sorted data and store last points for labels
    let mut datasets = Vec::new();
    let mut last_points: Vec<(f64, f64, Color)> = Vec::new(); // (days, rating, color)

    for (_original_idx, data_points, name) in indexed_datasets.iter() {
        // Get the color for this time control from the color map
        let color = color_map
            .get(name.as_str())
            .copied()
            .unwrap_or(Color::White);

        // Store the last point for drawing the label
        if let Some(&(last_days, last_rating)) = data_points.last() {
            last_points.push((last_days, last_rating, color));
        }

        // Create a dataset (line) for this time control
        let dataset = Dataset::default()
            .name(name.as_str())
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(color))
            .graph_type(GraphType::Line)
            .data(data_points);

        datasets.push(dataset);
    }

    if datasets.is_empty() || min_date == f64::MAX {
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
        return;
    }

    // Add padding to rating range
    let rating_range = max_rating - min_rating;
    let padding = rating_range * 0.2;
    let min_rating_display = (min_rating - padding).max(0.0);
    let max_rating_display = max_rating + padding;

    // Create X-axis labels - convert days since epoch to actual dates (month/year format)
    let x_labels = if max_date > min_date {
        let min_timestamp = (min_date * 86400.0) as i64;
        let max_timestamp = (max_date * 86400.0) as i64;
        let mid_timestamp = ((min_date + max_date) / 2.0 * 86400.0) as i64;

        let min_date_str = DateTime::<Utc>::from_timestamp(min_timestamp, 0)
            .map(|dt| dt.format("%b %Y").to_string())
            .unwrap_or_else(|| format!("{:.0}", min_date));
        let mid_date_str = DateTime::<Utc>::from_timestamp(mid_timestamp, 0)
            .map(|dt| dt.format("%b %Y").to_string())
            .unwrap_or_else(|| format!("{:.0}", (min_date + max_date) / 2.0));
        let max_date_str = DateTime::<Utc>::from_timestamp(max_timestamp, 0)
            .map(|dt| dt.format("%b %Y").to_string())
            .unwrap_or_else(|| format!("{:.0}", max_date));

        vec![
            Span::styled(min_date_str, Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(mid_date_str),
            Span::styled(max_date_str, Style::default().add_modifier(Modifier::BOLD)),
        ]
    } else {
        vec![Span::raw("Time".to_string())]
    };

    // Create Y-axis labels with better formatting and more granularity
    let rating_range = max_rating_display - min_rating_display;
    let num_labels = 5; // Show 5 labels for better readability
    let mut y_labels = Vec::new();

    for i in 0..num_labels {
        let rating_value = min_rating_display + (rating_range * i as f64 / (num_labels - 1) as f64);
        let formatted_rating = format!("{:.0}", rating_value);

        // Make first, middle, and last labels bold
        if i == 0 || i == num_labels / 2 || i == num_labels - 1 {
            y_labels.push(Span::styled(
                formatted_rating,
                Style::default().add_modifier(Modifier::BOLD),
            ));
        } else {
            y_labels.push(Span::raw(formatted_rating));
        }
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Rating History (last 90 days");
    let inner_area = block.inner(area);

    // Calculate maximum label width needed (for padding calculation)
    let max_label_width = last_points
        .iter()
        .map(|(_, rating, _)| format!("{:.0}", rating).len())
        .max()
        .unwrap_or(4) as u16;
    // Add space for square (1) + spacing (1) + label text + extra margin
    let total_label_width = 1 + 1 + max_label_width + 2;

    // Calculate padding needed on X-axis to fit labels
    let date_range = max_date - min_date;
    let chart_width = inner_area.width;
    let padding_ratio = (total_label_width as f64) / chart_width as f64;
    let padding = date_range * padding_ratio.max(0.05);
    let max_date_with_padding = max_date + padding;

    let chart = Chart::new(datasets.clone())
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

    // Draw current ELO labels at the end of each line
    let buffer = frame.buffer_mut();
    let chart_x = inner_area.x;
    let chart_y = inner_area.y;
    let chart_width = inner_area.width;
    let chart_height = inner_area.height;

    // First, calculate all label positions
    let mut label_positions: Vec<(u16, u16, f64, Color, String)> = Vec::new();
    for (last_days, last_rating, color) in last_points.iter() {
        // Convert data coordinates to screen coordinates for the line endpoint
        let line_x = if max_date_with_padding > min_date {
            chart_x
                + (((last_days - min_date) / (max_date_with_padding - min_date))
                    * (chart_width - 1) as f64) as u16
        } else {
            chart_x
        };

        // Keep Y position aligned with the line
        let y = if max_rating_display > min_rating_display {
            chart_y
                + (((max_rating_display - last_rating) / (max_rating_display - min_rating_display))
                    * (chart_height - 1) as f64) as u16
        } else {
            chart_y + chart_height / 2
        };

        let label_offset = 3;
        let x = line_x + label_offset;
        let rating_text = format!("{:.0}", last_rating);

        label_positions.push((x, y, *last_rating, *color, rating_text));
    }

    // Sort by Y position (top to bottom)
    label_positions.sort_by(|a, b| a.1.cmp(&b.1));

    // Group labels that are too close together (within 2 lines)
    let mut groups: Vec<Vec<usize>> = Vec::new(); // Store indices instead
    let proximity_threshold = 2;

    for (idx, label) in label_positions.iter().enumerate() {
        let label_y = label.1;
        let mut added_to_group = false;

        // Try to add to an existing group
        for group in groups.iter_mut() {
            // Check if this label is close to any label in the group
            if group.iter().any(|&group_idx| {
                let group_y = label_positions[group_idx].1;
                let y_diff = if label_y > group_y {
                    label_y - group_y
                } else {
                    group_y - label_y
                };
                y_diff <= proximity_threshold
            }) {
                group.push(idx);
                added_to_group = true;
                break;
            }
        }

        // If not added to any group, create a new group
        if !added_to_group {
            groups.push(vec![idx]);
        }
    }

    // Draw grouped labels
    for group in groups.iter() {
        if group.len() == 1 {
            // Single label - draw normally
            let (x, y, _rating, color, rating_text) = &label_positions[group[0]];
            if *x < chart_x + chart_width && *y < chart_y + chart_height {
                buffer[(*x, *y)]
                    .set_symbol("█")
                    .set_style(Style::default().fg(*color));

                let text_x = *x + 1;
                if text_x + (rating_text.len() as u16) < chart_x + chart_width {
                    for (i, ch) in rating_text.chars().enumerate() {
                        let pos_x = text_x + i as u16;
                        if pos_x < chart_x + chart_width {
                            buffer[(pos_x, *y)]
                                .set_char(ch)
                                .set_style(Style::default().fg(*color));
                        }
                    }
                }
            }
        } else {
            // Multiple labels - draw in a combined box
            // Use the average Y position of the group
            let avg_y = (group
                .iter()
                .map(|&idx| label_positions[idx].1 as u32)
                .sum::<u32>()
                / group.len() as u32) as u16;
            let x = label_positions[group[0]].0; // Use the X position of the first label

            // Calculate starting Y position (centered around avg_y)
            let start_y = if avg_y >= (group.len() as u16 / 2) {
                avg_y.saturating_sub(group.len() as u16 / 2)
            } else {
                chart_y
            };

            if x < chart_x + chart_width {
                // Draw each label on its own line, stacked vertically
                for (line_idx, &idx) in group.iter().enumerate() {
                    let (_x, _y, _rating, color, rating_text) = &label_positions[idx];
                    let y = start_y + line_idx as u16;

                    if y < chart_y + chart_height {
                        // Draw square
                        buffer[(x, y)]
                            .set_symbol("█")
                            .set_style(Style::default().fg(*color));

                        // Draw rating text in its color
                        let text_x = x + 1;
                        if text_x + (rating_text.len() as u16) < chart_x + chart_width {
                            for (i, ch) in rating_text.chars().enumerate() {
                                let pos_x = text_x + i as u16;
                                if pos_x < chart_x + chart_width {
                                    buffer[(pos_x, y)]
                                        .set_char(ch)
                                        .set_style(Style::default().fg(*color));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
