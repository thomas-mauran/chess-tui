//! Turns HTTP failures into messages that name the likely cause.
//!
//! Every Lichess call goes through a configurable base URL, so a failure can just
//! as easily be a bad host or a missing `/api` suffix as a bad token. These helpers
//! keep that context — the URL, the status, and what to try next — in the message
//! the user actually sees.

use crate::constants::{
    LICHESS_API_URL_SUFFIX, LICHESS_TOKEN_SCOPES, lichess_api_url, lichess_api_url_has_suffix,
    lichess_token_create_url,
};
use reqwest::StatusCode;
use std::error::Error;

/// Describes a request that never got a response: DNS, TCP, TLS, or a timeout.
pub fn transport_error(action: &str, url: &str, err: &reqwest::Error) -> String {
    let cause = cause_chain(err);
    let hint = if err.is_timeout() {
        format!(
            "The server did not answer in time. Check that {} is reachable from this machine.",
            base_host()
        )
    } else if err.is_connect() {
        format!(
            "Could not open a connection to {}. Check the host name, the port, and its TLS certificate.",
            base_host()
        )
    } else if err.is_builder() || err.is_request() {
        format!("The request could not be built for {}.", lichess_api_url())
    } else if err.is_decode() {
        format!(
            "The response from {} was not the JSON the API returns. {}",
            url,
            wrong_base_url_hint()
        )
    } else {
        format!("Request to {} failed.", url)
    };

    format!("Could not {}.\n\n{}\n\nDetails: {}", action, hint, cause)
}

/// Describes a response whose status was not a success.
///
/// `body` is the response body, if it was read; it is trimmed and truncated since
/// a wrong base URL usually returns a whole HTML page.
pub fn status_error(action: &str, url: &str, status: StatusCode, body: &str) -> String {
    let hint = match status {
        StatusCode::UNAUTHORIZED => format!(
            "The server rejected the token. Generate a new one for this instance:\n{}",
            lichess_token_create_url(&lichess_api_url())
        ),
        StatusCode::FORBIDDEN => format!(
            "The token is missing a scope. chess-tui needs: {}.\nGenerate one with them ticked:\n{}",
            LICHESS_TOKEN_SCOPES.join(", "),
            lichess_token_create_url(&lichess_api_url())
        ),
        StatusCode::NOT_FOUND => {
            format!("The server has no such endpoint. {}", wrong_base_url_hint())
        }
        StatusCode::TOO_MANY_REQUESTS => {
            "Rate limit exceeded. Wait a minute before trying again.".to_string()
        }
        status if status.is_server_error() => format!(
            "The server at {} reported an internal error, so this is very likely not a problem on your side.",
            base_host()
        ),
        _ => format!("Unexpected response from {}.", url),
    };

    let mut message = format!(
        "Could not {}.\n\n{}\n\nHTTP {} from {}",
        action, hint, status, url
    );
    if let Some(snippet) = body_snippet(body) {
        message.push_str(&format!("\nResponse: {}", snippet));
    }
    message
}

/// Points at the base URL when the response does not look like the API at all.
///
/// A base URL without `/api` is the common cause: the web root answers, so the
/// host looks healthy while every API call lands on a page that is not one.
fn wrong_base_url_hint() -> String {
    let base = lichess_api_url();
    if lichess_api_url_has_suffix(&base) {
        format!(
            "Check that {} is the API base URL of a Lichess server.",
            base
        )
    } else {
        format!(
            "The configured base URL {} does not end in {}, which is where Lichess serves its API. Try {}{} instead.",
            base, LICHESS_API_URL_SUFFIX, base, LICHESS_API_URL_SUFFIX
        )
    }
}

/// Host part of the configured base URL, for messages about reachability.
fn base_host() -> String {
    let base = lichess_api_url();
    base.split_once("://")
        .map(|(scheme, rest)| {
            let host = rest.split('/').next().unwrap_or(rest);
            format!("{}://{}", scheme, host)
        })
        .unwrap_or(base)
}

/// Flattens an error and its sources, which is where the real cause usually is.
fn cause_chain(err: &dyn Error) -> String {
    let mut parts = vec![err.to_string()];
    let mut source = err.source();
    while let Some(cause) = source {
        let text = cause.to_string();
        // Reqwest repeats its own message in the first source; skip the duplicate.
        if !parts.contains(&text) {
            parts.push(text);
        }
        source = cause.source();
    }
    parts.join(": ")
}

/// Trims a response body down to one short, single-line snippet.
fn body_snippet(body: &str) -> Option<String> {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return None;
    }
    let single_line = trimmed.split_whitespace().collect::<Vec<_>>().join(" ");
    let snippet: String = single_line.chars().take(160).collect();
    Some(if single_line.chars().count() > 160 {
        format!("{}…", snippet)
    } else {
        snippet
    })
}

#[cfg(test)]
mod tests {
    use super::{body_snippet, status_error, wrong_base_url_hint};
    use crate::constants::{DEFAULT_LICHESS_API_URL, set_lichess_api_url};
    use reqwest::StatusCode;

    /// The base URL is a process-wide global, so one test drives every case that
    /// depends on it rather than racing a sibling test that also writes it.
    #[test]
    fn messages_name_the_likely_cause() {
        set_lichess_api_url("https://lichess.verde.zoe");

        // A base URL without /api is the reason every endpoint 404s, so say so.
        let hint = wrong_base_url_hint();
        assert!(hint.contains("does not end in /api"), "{hint}");
        assert!(hint.contains("https://lichess.verde.zoe/api"), "{hint}");

        let not_found = status_error(
            "fetch your Lichess profile",
            "https://lichess.verde.zoe/account",
            StatusCode::NOT_FOUND,
            "<html>404</html>",
        );
        assert!(not_found.contains("does not end in /api"), "{not_found}");
        assert!(not_found.contains("404"), "{not_found}");

        set_lichess_api_url("https://lichess.verde.zoe/api");
        let forbidden = status_error(
            "seek a game",
            "https://lichess.verde.zoe/api/board/seek",
            StatusCode::FORBIDDEN,
            "",
        );
        assert!(forbidden.contains("board:play"), "{forbidden}");
        assert!(
            forbidden.contains("account/oauth/token/create"),
            "{forbidden}"
        );

        let unauthorized = status_error(
            "fetch your Lichess profile",
            "https://lichess.verde.zoe/api/account",
            StatusCode::UNAUTHORIZED,
            r#"{"error":"Missing authorization header"}"#,
        );
        assert!(
            unauthorized.contains("rejected the token"),
            "{unauthorized}"
        );
        assert!(
            unauthorized.contains("Missing authorization header"),
            "{unauthorized}"
        );

        set_lichess_api_url(DEFAULT_LICHESS_API_URL);
    }

    #[test]
    fn body_snippets_are_single_line_and_bounded() {
        assert_eq!(body_snippet("   "), None);
        assert_eq!(
            body_snippet("  line one\n  line two  "),
            Some("line one line two".to_string())
        );

        let long = "x".repeat(400);
        let snippet = body_snippet(&long).unwrap_or_default();
        assert_eq!(snippet.chars().count(), 161, "160 chars plus the ellipsis");
        assert!(snippet.ends_with('…'));
    }
}
