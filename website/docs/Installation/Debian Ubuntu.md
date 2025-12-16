# Debian/Ubuntu

**Chess-tui** can be installed on Debian-based systems using the `.deb` package from GitHub releases.

## Installation

### One-liner (recommended)

```bash
DEB_URL=$(curl -s "https://api.github.com/repos/thomas-mauran/chess-tui/releases/latest" | jq -r '.assets[] | select(.name | endswith(".deb")) | .browser_download_url') && curl -LO "$DEB_URL" && sudo dpkg -i "$(basename "$DEB_URL")" && sudo apt-get install -f
```

### Step-by-step

```bash
# Get the latest release and find the .deb file
DEB_URL=$(curl -s "https://api.github.com/repos/thomas-mauran/chess-tui/releases/latest" | jq -r '.assets[] | select(.name | endswith(".deb")) | .browser_download_url')

# Download the .deb package
curl -LO "$DEB_URL"

# Get the filename from the URL
DEB_FILE=$(basename "$DEB_URL")

# Install the package
sudo dpkg -i "$DEB_FILE"

# Fix any missing dependencies (if needed)
sudo apt-get install -f
```

**Then run the game with:**
```bash
chess-tui
```

You can find the latest release here [github.com/thomas-mauran/chess-tui/releases/latest](https://github.com/thomas-mauran/chess-tui/releases/latest) :tada:
