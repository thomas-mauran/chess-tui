#!/usr/bin/env node

/**
 * Script to generate a Docusaurus blog post from GitHub release information
 * Usage: node generate-blog-post.js <version> <date> <title> <release-body>
 */

const fs = require('fs');
const path = require('path');

const [version, date, title, releaseBody] = process.argv.slice(2);

if (!version || !date || !title) {
  console.error('Usage: node generate-blog-post.js <version> <date> <title> <release-body>');
  process.exit(1);
}

// Format date for display
const dateObj = new Date(date);
const formattedDate = dateObj.toLocaleDateString('en-US', { 
  year: 'numeric', 
  month: 'long', 
  day: 'numeric' 
});

// Generate filename using absolute path
const blogDir = path.join(__dirname, '..', '..', 'website', 'blog');
const filename = path.join(blogDir, `${date}-release-${version}.md`);

// Parse release body and convert to blog format
function parseReleaseBody(body) {
  if (!body) return '';
  
  const lines = body.split('\n');
  const sections = [];
  let currentSection = [];
  let inWhatChanged = false;
  let inContributors = false;
  let inChangelog = false;
  let skipNextEmpty = false;
  
  for (let i = 0; i < lines.length; i++) {
    let line = lines[i];
    const trimmed = line.trim();
    
    // Skip empty lines at the very start
    if (sections.length === 0 && !trimmed) continue;
    
    // Skip "Full Changelog" section (we'll add our own)
    if (trimmed.includes('Full Changelog') || trimmed.includes('**Full Changelog**')) {
      inChangelog = true;
      continue;
    }
    
    if (inChangelog && (trimmed.startsWith('---') || trimmed === '')) {
      inChangelog = false;
      continue;
    }
    if (inChangelog) continue;
    
    // Convert "What's Changed" to "What's New"
    if (trimmed.includes("What's Changed")) {
      if (currentSection.length > 0) {
        sections.push(currentSection.join('\n'));
        currentSection = [];
      }
      sections.push('## What\'s New\n');
      inWhatChanged = true;
      skipNextEmpty = true;
      continue;
    }
    
    // Handle contributors section
    if (trimmed.includes('Contributors') && !trimmed.includes('New Contributors')) {
      if (currentSection.length > 0) {
        sections.push(currentSection.join('\n'));
        currentSection = [];
      }
      sections.push('## Contributors\n');
      inContributors = true;
      inWhatChanged = false;
      skipNextEmpty = true;
      continue;
    }
    
    // Skip "New Contributors" header but keep the content
    if (trimmed.includes('New Contributors')) {
      skipNextEmpty = true;
      continue;
    }
    
    // Skip empty line after section headers
    if (skipNextEmpty && !trimmed) {
      skipNextEmpty = false;
      continue;
    }
    skipNextEmpty = false;
    
    // Convert PR references to markdown links (but preserve existing links)
    let processedLine = line.replace(/#(\d+)/g, (match, prNum) => {
      // Check if it's already a link by looking at surrounding context
      const before = line.substring(0, line.indexOf(match));
      const after = line.substring(line.indexOf(match) + match.length);
      if (before.includes('[') || after.includes(']')) {
        return match;
      }
      return `[#${prNum}](https://github.com/thomas-mauran/chess-tui/pull/${prNum})`;
    });
    
    // Convert @username to GitHub links (but not if already a link)
    // Match @ followed by word characters and hyphens, but not if inside a markdown link
    processedLine = processedLine.replace(/@([a-zA-Z0-9](?:[a-zA-Z0-9-]*[a-zA-Z0-9])?)/g, (match, username, offset) => {
      const before = processedLine.substring(0, offset);
      const after = processedLine.substring(offset + match.length);
      // Don't convert if already inside a markdown link
      if (before.includes('[') && !before.includes(']') || after.includes(']') && !after.includes('[')) {
        return match;
      }
      // Don't convert if it's part of a URL
      if (before.match(/https?:\/\/[^\s]*$/) || after.match(/^[^\s]*github\.com/)) {
        return match;
      }
      return `[@${username}](https://github.com/${username})`;
    });
    
    // Add line to current section
    currentSection.push(processedLine);
  }
  
  // Add remaining section
  if (currentSection.length > 0) {
    sections.push(currentSection.join('\n'));
  }
  
  return sections.join('\n\n');
}

// Extract H1 heading (just "Release X.X.X" without description)
const h1Heading = title.includes(' - ') 
  ? title.substring(0, title.indexOf(' - '))
  : title;

// Generate blog post content
const blogContent = `---
title: ${title}
authors:
  - name: Thomas Mauran
    url: https://github.com/thomas-mauran
tags:
  - release
---

# ${h1Heading}

**Released:** ${formattedDate}

${parseReleaseBody(releaseBody)}

## Full Changelog

For the complete list of changes, see the [full changelog](https://github.com/thomas-mauran/chess-tui/releases/tag/${version}).

---

[View on GitHub](https://github.com/thomas-mauran/chess-tui/releases/tag/${version})
`;

// Ensure blog directory exists
if (!fs.existsSync(blogDir)) {
  fs.mkdirSync(blogDir, { recursive: true });
}

// Write file
fs.writeFileSync(filename, blogContent, 'utf8');
console.log(`✅ Created blog post: ${filename}`);
