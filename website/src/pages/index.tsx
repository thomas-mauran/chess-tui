import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import Heading from '@theme/Heading';
import { useRef, useEffect } from 'react';

import styles from './index.module.css';

const DEMO_GIF_SRC = require('@site/static/gif/ratatui.gif').default;
const LOGO_SRC = require('@site/static/img/logo.png').default;

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();
  const gifRef = useRef<HTMLImageElement>(null);

  useEffect(() => {
    const el = gifRef.current;
    if (!el) return;
    const durationMs = 12000;
    const id = setInterval(() => {
      if (el.src) {
        el.src = el.src.split('?')[0] + '?t=' + Date.now();
      }
    }, durationMs);
    return () => clearInterval(id);
  }, []);

  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className={clsx('container', styles.heroContainer)}>
        <div className={styles.heroText}>
          <Heading as="h1" className={styles.hero__title}>
            {siteConfig.title}
            <img src={LOGO_SRC} alt="Chess TUI logo" className={styles.heroLogo} />
          </Heading>
          <div className={styles.badges}>
            <span className={styles.badge}>🦀 Rust</span>
            <span className={styles.badge}>MIT License</span>
            <span className={styles.badge}>Open Source</span>
          </div>
          <p className={styles.hero__subtitle}>{siteConfig.tagline}</p>
          <div className={styles.buttons}>
            <Link className={clsx('button button--lg', styles.btnPrimary)} to="/docs/intro">
              Play now ♟️
            </Link>
            <Link
              className="button button--outline button--secondary button--lg"
              href="https://github.com/thomas-mauran/chess-tui"
              style={{display: 'flex', alignItems: 'center', gap: '0.5rem'}}>
              <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 0C5.37 0 0 5.37 0 12c0 5.3 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61-.546-1.385-1.335-1.755-1.335-1.755-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 21.795 24 17.295 24 12c0-6.63-5.37-12-12-12z"/>
              </svg>
              GitHub
            </Link>
          </div>
        </div>
        <div className={styles.heroMedia}>
          <img
            ref={gifRef}
            className={styles.featureGif}
            src={DEMO_GIF_SRC}
            alt="Chess TUI: playing a game of chess in the terminal with keyboard controls"
            width={1807}
            height={1091}
          />
        </div>
      </div>
      <div className={styles.scrollArrow} onClick={() => window.scrollBy({ top: window.innerHeight, behavior: 'smooth' })}>
        <span />
      </div>
    </header>
  );
}

export default function Home(): JSX.Element {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title="Play Chess in Your Terminal"
      description="Chess TUI is an open-source chess game for the terminal. Supports UCI engines, Lichess online play, multiplayer, custom skins, and more. Written in Rust.">
      <HomepageHeader />
      <main>
        <HomepageFeatures />
      </main>
    </Layout>
  );
}
