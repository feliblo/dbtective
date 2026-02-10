#!/bin/bash
# Update version snippets in docs before committing the bump

NEW_VERSION=$CZ_PRE_NEW_VERSION

# Cross-platform sed in-place edit (macOS vs Linux)
sedi() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "$@"
    else
        sed -i "$@"
    fi
}

# Update GitHub Actions version in docs
sedi -E "s/uses: feliblo\/dbtective@v[0-9]+\.[0-9]+\.[0-9]+/uses: feliblo\/dbtective@v$NEW_VERSION/" docs/content/docs/running/github-actions.md

# Update pre-commit version in precommit docs
sedi -E "s/rev: v[0-9]+\.[0-9]+\.[0-9]+/rev: v$NEW_VERSION/" docs/content/docs/running/precommit.md
sedi -E "s/uses: feliblo\/dbtective@v[0-9]+\.[0-9]+\.[0-9]+/uses: feliblo\/dbtective@v$NEW_VERSION/" docs/content/docs/running/precommit.md
