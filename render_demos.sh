#!/bin/sh

# Ensure the vhs command is available
command -v vhs >/dev/null 2>&1 || { echo >&2 "vhs command not found. Please install it."; exit 1; }

# Check if the examples directory exists
if [ ! -d "examples" ]; then
  echo "Examples directory not found."
  exit 1
fi

# Loop through all .tape files in the examples directory and run vhs for each file
for file in examples/*.tape; do
  vhs "$file"
done

cp ./examples/*.gif ./website/static/gif