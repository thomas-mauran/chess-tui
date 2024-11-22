import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  tutorialSidebar: [
    'intro',
    {
      type: 'category',
      label: 'Installation', 
      items: ['Installation/cargo'], 
    },

    {
      type: 'category',
      label: 'Code Architecture', 
      items: ['Code Architecture/Intro', 'Code Architecture/Pieces', 'Code Architecture/Game'], 
    },
  ],
};

export default sidebars;
