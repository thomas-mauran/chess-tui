import type { SidebarsConfig } from "@docusaurus/plugin-content-docs";

const sidebars: SidebarsConfig = {
  tutorialSidebar: [
    "intro",
    {
      type: "category",
      label: "Installation",
      items: ["Installation/Packaging status", "Installation/Binary", "Installation/Debian Ubuntu", "Installation/Cargo", "Installation/Build from source", "Installation/NetBSD", "Installation/Arch Linux", "Installation/NixOS", "Installation/Docker"],
    },
    {
      type: "category",
      label: "Configuration",
      items: ["Configuration/configuration-intro", "Configuration/display", "Configuration/skins", "Configuration/logging", "Configuration/bot"],
    },
    {
      type: "category",
      label: "Multiplayer",
      items: ["Multiplayer/Local multiplayer", "Multiplayer/Online multiplayer"],
    },
    {
      type: "category",
      label: "Lichess",
      items: ["Lichess/features", "Lichess/setup"],
    },
    {
      type: "category",
      label: "Code Architecture",
      items: [
        "Code Architecture/Intro",
        "Code Architecture/Game",
        "Code Architecture/Pieces",
        "Code Architecture/App",
        "Code Architecture/GameLogic",
        "Code Architecture/GameBoard",
        "Code Architecture/UI",
        "Code Architecture/Bot",
        "Code Architecture/Opponent",
      ],
    },
  ],
};

export default sidebars;
