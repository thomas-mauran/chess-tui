import { themes as prismThemes } from 'prism-react-renderer';
import type { Config } from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

const config: Config = {
  title: 'Chess TUI',
  tagline: 'Play chess in your terminal',
  favicon: 'img/favicon-small.png',

  // Set the production url of your site here
  url: 'https://thomas-mauran.github.io',
  baseUrl: '/chess-tui/',

  organizationName: 'thomas-mauran',
  projectName: 'chess-tui',
  deploymentBranch: 'gh-pages',
  trailingSlash: false,

  onBrokenLinks: 'throw',

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          editUrl:
            'https://github.com/thomas-mauran/chess-tui/',
          sidebarCollapsible: true,
        },
        blog: {
          showReadingTime: true,
          blogSidebarCount: 'ALL',
          blogSidebarTitle: 'Recent posts',
          feedOptions: {
            type: ['rss', 'atom'],
            xslt: true,
          },
          editUrl:
            'https://github.com/thomas-mauran/chess-tui/',
          onInlineTags: 'warn',
          onInlineAuthors: 'warn',
          onUntruncatedBlogPosts: 'warn',
        },
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    image: 'img/social-card.png',
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: ['bash', 'toml'],
    },
    navbar: {
      title: 'Chess TUI',
      logo: {
        alt: 'Chess tui logo',
        src: 'img/logo.png',
      },
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'tutorialSidebar',
          position: 'left',
          label: 'Documentation',
        },
        { to: '/blog', label: 'Blog', position: 'left' },
        {
          href: 'https://docs.rs/chess-tui/latest/chess_tui/',
          label: 'docs.rs',
          position: 'left',
        },
        {
          type: 'custom-github-stars',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Docs',
          items: [
            {
              label: 'Getting Started',
              to: '/docs/intro',
            },
            {
              label: 'Installation',
              to: '/docs/Installation/Packaging status',
            },
            {
              label: 'Keyboard Shortcuts',
              to: '/docs/Usage/keyboard-shortcuts',
            },
            {
              label: 'FAQ',
              to: '/docs/faq',
            },
          ],
        },
        {
          title: 'Community',
          items: [
            {
              label: 'GitHub Discussions',
              href: 'https://github.com/thomas-mauran/chess-tui/discussions',
            },
            {
              label: 'GitHub Issues',
              href: 'https://github.com/thomas-mauran/chess-tui/issues',
            },
            {
              label: 'Releases',
              href: 'https://github.com/thomas-mauran/chess-tui/releases',
            },
          ],
        },
        {
          title: 'More',
          items: [
            {
              label: 'Blog',
              to: '/blog',
            },
            {
              label: 'GitHub',
              href: 'https://github.com/thomas-mauran/chess-tui/',
            },
            {
              label: 'docs.rs',
              href: 'https://docs.rs/chess-tui/latest/chess_tui/',
            },
            {
              label: 'Credits',
              to: '/docs/credits',
            },
          ],
        },
      ],
      copyright: `Free and Opensource Forever ♥`,
    },
  } satisfies Preset.ThemeConfig,

  headTags: [
    {
      tagName: 'script',
      attributes: { type: 'application/ld+json' },
      innerHTML: JSON.stringify({
        '@context': 'https://schema.org',
        '@type': 'SoftwareApplication',
        name: 'Chess TUI',
        applicationCategory: 'GameApplication',
        operatingSystem: 'Linux, macOS, Windows',
        description: 'Open-source chess game for the terminal. Supports UCI engines, Lichess online play, multiplayer, skins, pgn viewer, and more.',
        url: 'https://thomas-mauran.github.io/chess-tui/',
        downloadUrl: 'https://github.com/thomas-mauran/chess-tui/releases',
        author: {
          '@type': 'Person',
          name: 'Thomas Mauran',
          url: 'https://github.com/thomas-mauran',
        },
        offers: {
          '@type': 'Offer',
          price: '0',
          priceCurrency: 'USD',
        },
        codeRepository: 'https://github.com/thomas-mauran/chess-tui',
        license: 'https://opensource.org/licenses/MIT',
      }),
    },
  ],

  markdown: {
    mermaid: true,
    hooks: {
      onBrokenMarkdownLinks: 'warn',
    },
  },
  themes: ['@docusaurus/theme-mermaid'],
};

export default config;
