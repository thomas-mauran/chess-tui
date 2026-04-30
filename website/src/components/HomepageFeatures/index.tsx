import clsx from 'clsx';
import Link from '@docusaurus/Link';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
  title: string,
  img: any,
  description: JSX.Element,
  id: string,
  link: string,
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Plug any Chess Engine 🤖',
    img: require('@site/static/img/ferris-engine.webp').default,
    id: 'engine',
    link: '/docs/Bot/bot-intro',
    description: (
      <>
        You can play locally against any UCI-compatible chess engine.
      </>
    ),
  },
  {
    title: 'Challenge a Friend 🤼',
    img: require('@site/static/img/ferris-challenge.webp').default,
    id: 'challenge',
    link: '/docs/Multiplayer/multiplayer-intro',
    description: (
      <>
        Chess TUI allows you to play chess with a friend on the same computer.
        Play against your friends over the network.
      </>
    ),
  },
  {
    title: 'Lichess Integration 🌐',
    img: require('@site/static/img/ferris-lichess.webp').default,
    id: 'lichess',
    link: '/docs/Lichess/lichess-intro',
    description: (
      <>
        Play against lichess players directly from your terminal.
      </>
    ),
  },
  // Additional features can go here
];

function Feature({title, img, id, link, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
      </div>
      <div className="text--center padding-horiz--md">
        <Link to={link}>
          <img src={img} alt={`${title} - art by everwinter`} className="feature-img" id={id}/>
        </Link>
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): JSX.Element {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
