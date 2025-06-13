# Binary

**Chess-tui** is just a binary, so you can just download it through `wget` or `curl`.

## Installation

You can get the latest release with:

```bash
LATEST=$(curl -s "https://api.github.com/repos/thomas-mauran/chess-tui/releases" | jq -r '.[0].name')

curl -LO https://github.com/thomas-mauran/chess-tui/releases/download/$LATEST/chess-tui-$LATEST-x86_64-unknown-linux-gnu.tar.gz
```

Then, extract the binary:

```bash
tar xvf chess-tui-$LATEST-x86_64-unknown-linux-gnu.tar.gz
```

**And finally, run the game with:**

```bash
./chess-tui
```

You can find the latest release here [github.com/thomas-mauran/chess-tui/releases/latest](https://github.com/thomas-mauran/chess-tui/releases/latest) :tada:
