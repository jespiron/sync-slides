#!/bin/bash

# Generate slides with bespoke template
marp slides.md -o output.html --template bespoke

# Read template and escape special characters
template=$(cat template.html | awk '{printf "%s\\n", $0}')
sed -i '' "s|</body>|$template</body>|" output.html