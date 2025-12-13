#!/bin/bash

# Test script for generate-blog-post.js using real GitHub release data
# Usage: ./scripts/test-blog-post-github.sh <tag-name>
# Example: ./scripts/test-blog-post-github.sh 2.0.0

set -e

if [ -z "$1" ]; then
  echo "❌ Error: Tag name required"
  echo "Usage: $0 <tag-name>"
  echo "Example: $0 2.0.0"
  exit 1
fi

TAG_NAME="$1"
REPO="thomas-mauran/chess-tui"

echo "🔍 Fetching release data for tag: $TAG_NAME"
echo ""

# Check if GitHub CLI is available
if ! command -v gh &> /dev/null; then
  echo "⚠️  GitHub CLI (gh) not found. Installing or using API..."
  echo ""
  
  # Try to get release data using curl (requires GITHUB_TOKEN or public repo)
  if [ -z "$GITHUB_TOKEN" ]; then
    echo "ℹ️  Using public API (rate limited)"
    API_URL="https://api.github.com/repos/$REPO/releases/tags/$TAG_NAME"
    RELEASE_DATA=$(curl -s "$API_URL")
  else
    API_URL="https://api.github.com/repos/$REPO/releases/tags/$TAG_NAME"
    RELEASE_DATA=$(curl -s -H "Authorization: token $GITHUB_TOKEN" "$API_URL")
  fi
  
  # Extract data using jq if available, otherwise use grep/sed
  if command -v jq &> /dev/null; then
    VERSION=$(echo "$RELEASE_DATA" | jq -r '.tag_name // empty' | sed 's/^v//')
    DATE=$(echo "$RELEASE_DATA" | jq -r '.published_at // .created_at // empty' | cut -d'T' -f1)
    TITLE=$(echo "$RELEASE_DATA" | jq -r '.name // .tag_name // empty')
    BODY=$(echo "$RELEASE_DATA" | jq -r '.body // empty')
  else
    echo "❌ Error: jq is required for parsing JSON. Install it or use GitHub CLI (gh)"
    exit 1
  fi
else
  # Use GitHub CLI
  echo "✅ Using GitHub CLI"
  RELEASE_DATA=$(gh release view "$TAG_NAME" --repo "$REPO" --json tagName,publishedAt,name,body)
  
  VERSION=$(echo "$RELEASE_DATA" | jq -r '.tagName // empty' | sed 's/^v//')
  DATE=$(echo "$RELEASE_DATA" | jq -r '.publishedAt // empty' | cut -d'T' -f1)
  TITLE=$(echo "$RELEASE_DATA" | jq -r '.name // .tagName // empty')
  BODY=$(echo "$RELEASE_DATA" | jq -r '.body // empty')
fi

if [ -z "$VERSION" ] || [ "$VERSION" = "null" ]; then
  echo "❌ Error: Could not find release for tag: $TAG_NAME"
  exit 1
fi

echo "📋 Release information:"
echo "  Version: $VERSION"
echo "  Date: $DATE"
echo "  Title: $TITLE"
echo ""

# Run the script
echo "🚀 Generating blog post..."
node scripts/generate-blog-post.js "$VERSION" "$DATE" "$TITLE" "$BODY"

echo ""
echo "✅ Blog post generated! Check website/blog/${DATE}-release-${VERSION}.md"
echo ""
echo "💡 Tip: Review the file and if it looks good, you can commit it manually"
