import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  tutorialSidebar: [
    'intro',
    {
      type: 'category',
      label: 'Installation', 
      items: ['Installation/Packaging status', 'Installation/Cargo', 'Installation/Build from source', 'Installation/NetBSD', 'Installation/Arch Linux', 'Installation/NixOS', 'Installation/Docker'], 
    },
    {
      type: 'category',
      label: 'Multiplayer', 
      items: ['Multiplayer/Local multiplayer', 'Multiplayer/Online multiplayer'], 
    },
    {
      type: 'category',
      label: 'Code Architecture', 
      items: ['Code Architecture/Intro', 'Code Architecture/Pieces', 'Code Architecture/Game'], 
    },
  ],
};

export default sidebars;
