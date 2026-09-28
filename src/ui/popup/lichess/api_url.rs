//! Lichess API base URL entry popup.

use crate::state::lichess_state::ApiUrlSuffixChoice;
use crate::ui::prompt::Prompt;
use crate::{
    app::App,
    constants::{
        BLACK, DEFAULT_LICHESS_API_URL, LICHESS_API_URL_ENV, LICHESS_API_URL_SUFFIX, WHITE,
        append_lichess_api_suffix, is_valid_lichess_api_url, lichess_api_url_has_suffix,
        lichess_api_url_is_env_pinned, lichess_token_create_url,
    },
    ui::components::centered_rect::centered_rect,
};
use ratatui::{
    Frame,
    layout::{Alignment, Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
};

/// Renders a text input for changing the Lichess API base URL.
///
/// The prompt is pre-filled with the URL currently in effect, so submitting an
/// unchanged input is a no-op. Clearing the input restores the default endpoint.
/// Submitting a URL without the `/api` suffix swaps the input for a confirmation
/// offering to append it.
pub fn render_enter_lichess_api_url_popup(frame: &mut Frame, app: &App) {
    let prompt = &app.game.ui.prompt;
    let block = Block::default()
        .title("Lichess API URL")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(70, 60, frame.area());

    let text = match app.lichess_state.api_url_suffix_choice {
        Some(choice) => missing_suffix_lines(prompt, choice),
        None => {
            place_cursor(frame, area, prompt);
            editing_lines(prompt)
        }
    };

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}

/// Draws the cursor at the current position in the input field.
fn place_cursor(frame: &mut Frame, area: Rect, prompt: &Prompt) {
    frame.set_cursor_position(Position::new(
        // This position is can be controlled via the left and right arrow key
        area.x + prompt.character_index as u16 + 2,
        // Move one line down, from the border to the input line
        area.y + 3,
    ));
}

/// Body shown while the user is typing a URL.
fn editing_lines(prompt: &Prompt) -> Vec<Line<'static>> {
    let mut text = vec![
        Line::from("Enter the Lichess API base URL:").alignment(Alignment::Center),
        Line::from(""),
        Line::from(prompt.input.clone()),
        Line::from(""),
    ];

    if let Some(error) = &prompt.error {
        text.push(Line::from(Span::styled(
            error.clone(),
            Style::default().fg(Color::Red),
        )));
        text.push(Line::from(""));
    }

    text.push(Line::from(format!("Default: {}", DEFAULT_LICHESS_API_URL)));
    text.push(Line::from("Leave empty to restore the default."));

    // Once the URL looks usable, offer the matching token page so the user does not
    // have to assemble the scope query string by hand.
    if is_valid_lichess_api_url(&prompt.input) && lichess_api_url_has_suffix(&prompt.input) {
        text.push(Line::from(""));
        text.push(Line::from("Create a token with the right scopes:"));
        text.push(Line::from(Span::styled(
            lichess_token_create_url(&prompt.input),
            Style::default().fg(Color::Cyan),
        )));
    }

    if lichess_api_url_is_env_pinned() {
        text.push(Line::from(""));
        text.push(Line::from(Span::styled(
            format!(
                "Note: {} is set and wins on next start.",
                LICHESS_API_URL_ENV
            ),
            Style::default().fg(Color::Yellow),
        )));
    }

    text.push(Line::from(""));
    text.push(Line::from("Press `Enter` to save, `Esc` to cancel.").alignment(Alignment::Center));
    text
}

/// Body shown when the submitted URL has no `/api` suffix.
fn missing_suffix_lines(prompt: &Prompt, choice: ApiUrlSuffixChoice) -> Vec<Line<'static>> {
    let typed = prompt.input.trim().to_string();
    let appended = append_lichess_api_suffix(&typed);

    vec![
        Line::from(Span::styled(
            format!("This URL does not end in {}", LICHESS_API_URL_SUFFIX),
            Style::default().fg(Color::Yellow),
        ))
        .alignment(Alignment::Center),
        Line::from(""),
        Line::from(format!("You typed: {}", typed)),
        Line::from(""),
        Line::from("Lichess serves its API under /api, so requests will very likely"),
        Line::from("fail without it. You can still save the URL as typed."),
        Line::from(""),
        Line::from(vec![
            button(
                format!("Append {}", LICHESS_API_URL_SUFFIX),
                choice == ApiUrlSuffixChoice::Append,
            ),
            Span::raw("  "),
            button(
                "Keep as typed".to_string(),
                choice == ApiUrlSuffixChoice::KeepAsTyped,
            ),
        ])
        .alignment(Alignment::Center),
        Line::from(""),
        Line::from(format!(
            "Will save: {}",
            match choice {
                ApiUrlSuffixChoice::Append => appended,
                ApiUrlSuffixChoice::KeepAsTyped => typed,
            }
        )),
        Line::from(""),
        Line::from("`←`/`→` to choose, `Enter` to confirm, `Esc` to keep editing.")
            .alignment(Alignment::Center),
    ]
}

/// Renders one confirmation button, inverted when it is the highlighted one.
fn button(label: String, highlighted: bool) -> Span<'static> {
    let style = if highlighted {
        Style::default()
            .fg(BLACK)
            .bg(WHITE)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(WHITE)
    };
    Span::styled(format!(" {} ", label), style)
}
