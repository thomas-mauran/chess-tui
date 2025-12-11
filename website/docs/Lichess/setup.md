# Setup & Authentication

To use the Lichess integration features in `chess-tui`, you need to authenticate with a Lichess API token.

## Generating a Token

1.  Go to your [Lichess Account Security](https://lichess.org/account/oauth/token) page.
2.  Click on **Generate a personal access token**.
3.  Give it a description (e.g., "chess-tui").
4.  **Important:** Make sure to check all the scopes (permissions) to ensure full functionality (playing games, puzzles, reading profile). At a minimum, you need:
    *   `Read preferences`
    *   `Create, read, update, delete games`
    *   `Read puzzle activity`
5.  Click **Submit** and copy your new token.

## Configuring the Token

There are two ways to provide your token to `chess-tui`.

### 1. Command Line Argument

You can pass the token directly when running the application using the `-l` or `--lichess-token` flag. This will also automatically save the token to your configuration file for future use.

```bash
chess-tui -l YOUR_LICHESS_TOKEN_HERE
```

### 2. Configuration File

You can manually add the token to your configuration file located at `~/.config/chess-tui/config.toml`.

```toml
lichess_token = "YOUR_LICHESS_TOKEN_HERE"
```

Once configured, the "Lichess" menu option in the main menu will become accessible.
