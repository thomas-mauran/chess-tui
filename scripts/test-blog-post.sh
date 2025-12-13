#!/bin/bash

# Test script for generate-blog-post.js
# This script tests the blog post generation with sample release data

set -e

echo "🧪 Testing blog post generation script..."
echo ""

# Sample release data (similar to what GitHub provides)
VERSION="2.1.0"
DATE="2025-01-15"
TITLE="Release 2.1.0 - New Features and Improvements"

# Sample release body (similar to GitHub release notes format)
RELEASE_BODY=$(cat <<'EOF'
## What's Changed

* New feature: Custom themes by @user1 in #123
* Fix: Improved performance by @user2 in #124
* docs: Update README by @user3 in #125

## New Contributors

* @user1 made their first contribution in #123
* @user2 made their first contribution in #124

**Full Changelog**: 2.0.0...2.1.0
EOF
)

echo "📝 Test parameters:"
echo "  Version: $VERSION"
echo "  Date: $DATE"
echo "  Title: $TITLE"
echo ""

# Run the script
echo "🚀 Running generate-blog-post.js..."
node scripts/generate-blog-post.js "$VERSION" "$DATE" "$TITLE" "$RELEASE_BODY"

echo ""
echo "✅ Test complete! Check website/blog/${DATE}-release-${VERSION}.md"
echo ""
echo "To test with real GitHub release data, use:"
echo "  ./scripts/test-blog-post-github.sh <tag-name>"
