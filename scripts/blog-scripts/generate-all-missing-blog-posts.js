#!/usr/bin/env node

/**
 * Script to fetch all GitHub releases and generate blog posts for any missing ones
 * Usage: node generate-all-missing-blog-posts.js
 */

const https = require('https');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const REPO = 'thomas-mauran/chess-tui';
const API_URL = `https://api.github.com/repos/${REPO}/releases`;
const BLOG_DIR = path.join(__dirname, '..', 'website', 'blog');
const GENERATE_SCRIPT = path.join(__dirname, 'generate-blog-post.js');

// Get existing blog posts
function getExistingBlogPosts() {
  const files = fs.readdirSync(BLOG_DIR);
  const existing = new Set();
  
  files.forEach(file => {
    const match = file.match(/(\d{4}-\d{2}-\d{2})-release-(.+)\.md/);
    if (match) {
      existing.add(match[2]); // version number
    }
  });
  
  return existing;
}

// Fetch all releases
function fetchAllReleases() {
  return new Promise((resolve, reject) => {
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
          const releases = JSON.parse(data);
          resolve(releases);
        } catch (error) {
          reject(error);
        }
      });
    }).on('error', (error) => {
      reject(error);
    });
  });
}

// Generate blog post for a release
function generateBlogPost(release) {
  const version = release.tag_name.replace(/^v/, '');
  const date = release.published_at.split('T')[0];
  const title = release.name || `Release ${version}`;
  const body = release.body || '';

  // Properly escape the body for shell execution
  const escapedBody = body.replace(/'/g, "'\"'\"'");
  
  const command = `node "${GENERATE_SCRIPT}" "${version}" "${date}" "${title}" '${escapedBody}'`;
  
  try {
    console.log(`Generating blog post for release ${version}...`);
    execSync(command, { stdio: 'inherit' });
    return true;
  } catch (error) {
    console.error(`Error generating blog post for ${version}:`, error.message);
    return false;
  }
}

// Main function
async function main() {
  console.log('Fetching all releases from GitHub...');
  const releases = await fetchAllReleases();
  
  console.log(`Found ${releases.length} releases`);
  
  console.log('Checking existing blog posts...');
  const existing = getExistingBlogPosts();
  console.log(`Found ${existing.size} existing blog posts`);
  
  // Filter out releases that already have blog posts
  const missing = releases.filter(release => {
    const version = release.tag_name.replace(/^v/, '');
    return !existing.has(version);
  });
  
  console.log(`\nFound ${missing.length} releases without blog posts:`);
  missing.forEach(release => {
    const version = release.tag_name.replace(/^v/, '');
    console.log(`  - ${version} (${release.published_at.split('T')[0]})`);
  });
  
  if (missing.length === 0) {
    console.log('\n✅ All releases have blog posts!');
    return;
  }
  
  console.log(`\nGenerating blog posts for ${missing.length} releases...\n`);
  
  let successCount = 0;
  for (const release of missing) {
    if (generateBlogPost(release)) {
      successCount++;
    }
    console.log(''); // Empty line for readability
  }
  
  console.log(`\n✅ Generated ${successCount}/${missing.length} blog posts successfully!`);
}

main().catch(error => {
  console.error('Error:', error.message);
  process.exit(1);
});
