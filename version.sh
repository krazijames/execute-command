#!/bin/bash

set -e

# Validates the version number
function validate_version {
    # Reference: https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
    SEMVER_REGEX="^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"

    if [ $(uname -s) = Darwin ]; then
        echo $1 | grep -E $SEMVER_REGEX
    else
        echo $1 | grep -P $SEMVER_REGEX
    fi
}

# Updates Cargo package version
function update_cargo_package_version {
    SED_SCRIPT="/\[package\]/,/^ *version *= *".*" *$/ s/^ *version *= *".*" *$/version = \"$1\"/"

    if [ $(uname -s) = Darwin ]; then
        sed -i "" -e "$SED_SCRIPT" Cargo.toml
    else
        sed -i "$SED_SCRIPT" Cargo.toml
    fi

    cargo update
}

# Current version
CURRENT_VERSION=$(grep "^\[package\]" Cargo.toml -A 3 | grep "^version" | awk -F ' = ' '{print $2}' | tr -d '"')

# If no argument is passed, print the current version
if [ -z "$1" ]; then
    echo $CURRENT_VERSION
    exit 0
fi

# New version
NEW_VERSION=$1

# Validate new version
if [ ! $(validate_version $NEW_VERSION) ]; then
    echo "Invalid version: $NEW_VERSION" >&2
    exit 1
else
    BUILD_METADATA=$(echo $NEW_VERSION | grep -o "\+.*" || true)

    if [ -n "$BUILD_METADATA" ]; then
        echo "Build metadata in version is not supported: $BUILD_METADATA" >&2
        exit 1
    fi
fi

# Update version in Cargo.toml
echo Updating \'Cargo.toml\' ...
update_cargo_package_version $NEW_VERSION
echo

# Commit changes and create tag
echo Committing changes ...
git add \
    Cargo.toml \
    CHANGELOG.md
git commit --message v$NEW_VERSION
git tag v$NEW_VERSION
