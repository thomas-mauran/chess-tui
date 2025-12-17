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
const BLOG_DIR = path.join(__dirname, '..', '..', 'website', 'blog');
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

// Extract description from release body
function extractDescription(body) {
  if (!body) return '';
  
  const lines = body.split('\n');
  const features = [];
  
  // Look for PR titles (lines starting with * feat:, * fix:, etc.)
  for (const line of lines) {
    const trimmed = line.trim();
    // Match PR entries like "* feat: description" or "* feat(module): description"
    const match = trimmed.match(/^\*\s*(?:feat|fix|refactor|docs|style|test|chore)(?:\([^)]+\))?:\s*(.+?)(?:\s+by\s+@|\s+in\s+https?|$)/i);
    if (match) {
      let desc = match[1].trim();
      // Remove PR links if present
      desc = desc.replace(/\s+in\s+https?:\/\/[^\s]+/i, '');
      // Remove "by @username" if present
      desc = desc.replace(/\s+by\s+@[\w-]+/i, '');
      if (desc && desc.length > 0) {
        features.push(desc);
      }
    }
  }
  
  // If we found features, create a description from the first few
  if (features.length > 0) {
    // Take up to 3 features and create a description
    const mainFeatures = features.slice(0, 3);
    // Capitalize first letter and join with ", "
    const description = mainFeatures
      .map(f => f.charAt(0).toUpperCase() + f.slice(1))
      .join(', ');
    
    // Limit description length
    if (description.length > 100) {
      return description.substring(0, 97) + '...';
    }
    return description;
  }
  
  // Fallback: look for any meaningful text in the first few lines
  for (let i = 0; i < Math.min(10, lines.length); i++) {
    const trimmed = lines[i].trim();
    // Skip empty lines, headers, and common patterns
    if (trimmed && 
        !trimmed.match(/^(#+|What's|Contributors|Full Changelog)/i) &&
        trimmed.length > 10 &&
        !trimmed.match(/^[-*]\s*$/)) {
      // Use first meaningful line, but limit length
      if (trimmed.length > 100) {
        return trimmed.substring(0, 97) + '...';
      }
      return trimmed;
    }
  }
  
  return '';
}

// Generate blog post for a release
function generateBlogPost(release) {
  const version = release.tag_name.replace(/^v/, '');
  const date = release.published_at.split('T')[0];
  const body = release.body || '';
  
  // Extract description from release body
  let description = extractDescription(body);
  
  // Format title as "Release X.X.X - Description"
  let title;
  if (description) {
    title = `Release ${version} - ${description}`;
  } else {
    // If no description found, check if release.name has a description
    const releaseName = release.name || '';
    if (releaseName && 
        releaseName.trim() !== version && 
        releaseName.trim() !== `v${version}` &&
        !releaseName.match(/^Release\s+\d+\.\d+\.\d+$/i)) {
      // Use release name if it's not just the version
      title = `Release ${version} - ${releaseName.trim()}`;
    } else {
      // Final fallback: just the version
      title = `Release ${version}`;
    }
  }

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
