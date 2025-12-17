# Packaging status

Thanks to a few awesome people, `chess-tui` is available in a few package managers. Here is a list of the package managers and the current status of the packaging.

[![Packaging status](https://repology.org/badge/vertical-allrepos/chess-tui.svg)](https://repology.org/project/chess-tui/versions)

## Direct Downloads

**Debian/Ubuntu (.deb package):**
A `.deb` package is available for each release. You can download and install it with:

```bash
DEB_URL=$(curl -s "https://api.github.com/repos/thomas-mauran/chess-tui/releases/latest" | jq -r '.assets[] | select(.name | endswith(".deb")) | .browser_download_url') && curl -LO "$DEB_URL" && sudo dpkg -i "$(basename "$DEB_URL")" && sudo apt-get install -f
```

**Homebrew (unofficial tap):**
An unofficial Homebrew tap is available:

```bash
brew install thomas-mauran/tap/chess-tui
```
