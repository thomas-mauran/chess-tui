import clsx from 'clsx';
import Link from '@docusaurus/Link';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  img: any;
  description: JSX.Element;
  id: string;
  link: string;
  large?: boolean;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Plug any Chess Engine 🤖',
    img: require('@site/static/img/ferris-engine.webp').default,
    id: 'engine',
    link: '/docs/Bot/bot-intro',
    description: (
      <>Play locally against any UCI-compatible chess engine, including Stockfish.</>
    ),
  },
  {
    title: 'Challenge a Friend 🤼',
    img: require('@site/static/img/ferris-challenge.webp').default,
    id: 'challenge',
    large: true,
    link: '/docs/Multiplayer/multiplayer-intro',
    description: (
      <>Pass-and-play on the same machine, or play over the network with a friend.</>
    ),
  },
  {
    title: 'Lichess Integration 🌐',
    img: require('@site/static/img/ferris-lichess.webp').default,
    id: 'lichess',
    link: '/docs/Lichess/lichess-intro',
    description: (
      <>Seek games, solve puzzles, and play rated matches on Lichess from your terminal.</>
    ),
  },
];

function Feature({ title, img, id, link, description, large }: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <Link to={link} className={styles.featureLink}>
        <img src={img} alt={`${title} illustration`} className={clsx(styles.cardImg, large && styles.cardImgLarge)} id={id} />
        <h3 className={styles.cardTitle}>{title}</h3>
        <p className={styles.cardDesc}>{description}</p>
      </Link>
    </div>
  );
}

export default function HomepageFeatures(): JSX.Element {
  return (
    <section className={styles.features}>
      <div className="container">
        <h2 className={styles.sectionTitle}>Why Chess TUI?</h2>
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
