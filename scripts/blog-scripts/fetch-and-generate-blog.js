#!/usr/bin/env node

/**
 * Script to fetch the latest GitHub release and generate a blog post
 * Usage: node fetch-and-generate-blog.js
 */

const https = require('https');
const fs = require('fs');
const path = require('path');

const REPO = 'thomas-mauran/chess-tui';
const API_URL = `https://api.github.com/repos/${REPO}/releases/latest`;

// Fetch latest release
https.get(API_URL, {
  headers: {
    'User-Agent': 'Node.js'
  }
}, (res) => {
  let data = '';

  res.on('data', (chunk) => {
    data += chunk;
  });

  res.on('end', () => {
    try {
      const release = JSON.parse(data);
      
      // Extract information
      const version = release.tag_name.replace(/^v/, '');
      const date = release.published_at.split('T')[0];
      const title = release.name || `Release ${version}`;
      const body = release.body || '';

      // Call the generate-blog-post script
      const generateScript = path.join(__dirname, 'generate-blog-post.js');
      const { execSync } = require('child_process');
      
      // Properly escape the body for shell execution
      const escapedBody = body.replace(/'/g, "'\"'\"'");
      
      const command = `node "${generateScript}" "${version}" "${date}" "${title}" '${escapedBody}'`;
      
      console.log(`Generating blog post for release ${version}...`);
      execSync(command, { stdio: 'inherit' });
      
    } catch (error) {
      console.error('Error processing release:', error.message);
      process.exit(1);
    }
  });
}).on('error', (error) => {
  console.error('Error fetching release:', error.message);
  process.exit(1);
});
