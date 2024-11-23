# NixOS

**Chess-tui** can be directly installed from the NixOS package manager.


## Installation

A nix-shell will temporarily modify your $PATH environment variable. This can be used to try a piece of software before deciding to permanently install it.

```bash
nix-shell -p chess-tui
```

This package is available on the [official NixOS repositories](https://search.nixos.org/packages?channel=24.05&show=chess-tui&from=0&size=50&sort=relevance&type=packages&query=chess-tui).