#!/bin/bash
set -e

# Extract version from Cargo.toml
VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

# Generate changelog with explicit version tag
git cliff --config scripts/git_cliff_template.toml --tag "v$VERSION" -o CHANGELOG.md

echo "Generated CHANGELOG.md for version v$VERSION"

# Stage the changelog and amend it to the bump commit
git add CHANGELOG.md
git commit --amend --no-edit
