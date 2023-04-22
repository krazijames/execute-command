#!/bin/bash

set -e

# Get the version from the first argument or use "Unreleased"
VERSION=${1:-Unreleased}

# Get the release note for the given version from CHANGELOG.md
RELEASE_NOTE=$(sed -n "/## \[\{0,1\}$VERSION\]\{0,1\}/, /## \[/p" CHANGELOG.md | awk '!/^## /' | sed '/./,$!d' | sed -e :a -e '/^\n*$/{$d;N;ba' -e '}')

# Trim
RELEASE_NOTE=$(echo "$RELEASE_NOTE" | sed '/./,$!d' | sed -e :a -e '/^\n*$/{$d;N;ba' -e '}')

# Print the release note
echo "$RELEASE_NOTE"
