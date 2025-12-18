import React, { useEffect, useState } from 'react';

export function LatestVersion(): JSX.Element {
  const [version, setVersion] = useState<string>('latest');
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    fetch('https://api.github.com/repos/thomas-mauran/chess-tui/releases/latest')
      .then((response) => response.json())
      .then((data) => {
        if (data.tag_name) {
          // Remove 'v' prefix if present
          const versionNumber = data.tag_name.replace(/^v/, '');
          setVersion(versionNumber);
        }
        setLoading(false);
      })
      .catch(() => {
        // Fallback to 'latest' if fetch fails
        setVersion('latest');
        setLoading(false);
      });
  }, []);

  if (loading) {
    return <code>latest</code>;
  }

  return <code>{version}</code>;
}

export function DockerCommands(): JSX.Element {
  const [version, setVersion] = useState<string>('latest');
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    fetch('https://api.github.com/repos/thomas-mauran/chess-tui/releases/latest')
      .then((response) => response.json())
      .then((data) => {
        if (data.tag_name) {
          // Remove 'v' prefix if present
          const versionNumber = data.tag_name.replace(/^v/, '');
          setVersion(versionNumber);
        }
        setLoading(false);
      })
      .catch(() => {
        // Fallback to 'latest' if fetch fails
        setVersion('latest');
        setLoading(false);
      });
  }, []);

  const tag = loading ? 'latest' : version;

  return (
    <>
      <p>Pull and run a specific version:</p>
      <pre>
        <code>{`docker pull ghcr.io/thomas-mauran/chess-tui:${tag}
docker run --rm -it ghcr.io/thomas-mauran/chess-tui:${tag}`}</code>
      </pre>
      <p>Or run directly without pulling first:</p>
      <pre>
        <code>{`docker run --rm -it ghcr.io/thomas-mauran/chess-tui:${tag}`}</code>
      </pre>
    </>
  );
}

