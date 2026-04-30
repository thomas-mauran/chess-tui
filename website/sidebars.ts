import type { SidebarsConfig } from "@docusaurus/plugin-content-docs";

const sidebars: SidebarsConfig = {
  tutorialSidebar: [
    "intro",
    {
      type: "category",
      label: "Installation",
      items: [
        "Installation/Packaging status",
        "Installation/Arch Linux",
        "Installation/Binary",
        "Installation/Build from source",
        "Installation/Cargo",
        "Installation/Debian Ubuntu",
        "Installation/Docker",
        "Installation/Homebrew",
        "Installation/NetBSD",
        "Installation/NixOS",
      ],
    },
    {
      type: "category",
      label: "Configuration",
      items: ["Configuration/configuration-intro", "Configuration/skins", "Configuration/logging", "Configuration/sound"],
    },
    {
      type: "category",
      label: "Bot",
      items: ["Bot/bot-intro", "Bot/bot-engines", "Bot/bot-settings"],
    },
    {
      type: "category",
      label: "Multiplayer",
      items: ["Multiplayer/multiplayer-intro", "Multiplayer/Local multiplayer", "Multiplayer/Online multiplayer"],
    },
    {
      type: "category",
      label: "Lichess",
      items: ["Lichess/lichess-intro", "Lichess/features", "Lichess/setup"],
    },
    "Usage/keyboard-shortcuts",
    "faq",
    "credits",
    {
      type: "link",
      label: "Contributing",
      href: "https://github.com/thomas-mauran/chess-tui/blob/main/CONTRIBUTING.md",
    },
    {
      type: "link",
      label: "API Documentation",
      href: "https://docs.rs/chess-tui/latest/chess_tui/",
    },
  ],
};

export default sidebars;
