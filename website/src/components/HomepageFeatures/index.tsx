import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Plug any Chess Engine 🤖',
    description: (
      <>
        You can play locally against any UCI-compatible chess engine.
      </>
    ),
  },
  {
    title: 'Challenge a Friend 🤼',
    description: (
      <>
        Chess TUI allows you to play chess with a friend on the same computer.
        Games over the network are a work in progress.
      </>
    ),
  },
  {
    title: 'Lichess Integration 🌐',
    description: (
      <>
        This is a work in progress. You will soon be able to play online on Lichess.
      </>
    ),
  },
  // Additional features can go here
];

function Feature({title, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
      </div>
      <div className="text--center padding-horiz--md">
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
