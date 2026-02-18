import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import Heading from '@theme/Heading';
import { useRef, useEffect } from 'react';

import styles from './index.module.css';

const DEMO_GIF_SRC = require('@site/static/gif/demo.gif').default;

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
      <div className="container">
          {/* <Heading as="h1" className={styles.hero__title}>
            {siteConfig.title}
          </Heading> */}
        <img
          ref={gifRef}
          className={styles.featureGif}
          src={DEMO_GIF_SRC}
          alt="Demo"
          role="img"
        />
        <p className={styles.hero__subtitle}>{siteConfig.tagline}</p>
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/docs/intro">
            Play now ♟️
          </Link>
        </div>
      </div>
    </header>
  );
}



export default function Home(): JSX.Element {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title={`chess-tui`}
      description="Play chess in your terminal">
      <HomepageHeader />
      <main>
        <HomepageFeatures />
      </main>
    </Layout>
  );
}
