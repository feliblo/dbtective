#!/bin/bash
# Update version snippets in README.md before committing the bump

NEW_VERSION=$CZ_PRE_NEW_VERSION


# Update GitHub Actions version in docs
sed -i "s/uses: feliblo\/dbtective@v[0-9]\+\.[0-9]\+\.[0-9]\+/uses: feliblo\/dbtective@v$NEW_VERSION/" docs/content/docs/running/github-actions.md
