import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  tutorialSidebar: [
    'Intro',
    {
      type: 'category',
      label: 'Installation', 
      items: ['Installation/Packaging status', 'Installation/Cargo', 'Installation/Build from source', 'Installation/NetBSD', 'Installation/Arch Linux', 'Installation/NixOS', 'Installation/Docker'], 
    },

    {
      type: 'category',
      label: 'Code Architecture', 
      items: ['Code Architecture/Intro', 'Code Architecture/Pieces', 'Code Architecture/Game'], 
    },
  ],
};

export default sidebars;
