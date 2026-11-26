#!/bin/sh
set -eu

local_commit=$(git rev-parse --verify "$1^{commit}")
site_ref=refs/remotes/sites/main
if ! git remote get-url sites >/dev/null 2>&1; then
    echo "pre-push: configure the Sites source repository as the 'sites' remote (see README.md)." >&2
    exit 1
fi

# Accept a temporary Sites credential or the executor's existing Git authentication.
# Pass the header through the environment, so it stays out of Git arguments and config.
if [ -n "${SITES_PUSH_TOKEN:-}" ]; then
    SITES_GIT_HEADER="Authorization: Bearer $SITES_PUSH_TOKEN"
    export SITES_GIT_HEADER
    set -- --config-env=http.extraHeader=SITES_GIT_HEADER
else
    set --
fi
export GIT_TERMINAL_PROMPT=0

echo "pre-push: syncing Sites source ..."
git "$@" fetch --no-tags --no-write-fetch-head sites "refs/heads/main:$site_ref"
site_commit=$(git rev-parse "$site_ref")
local_tree=$(git rev-parse "$local_commit^{tree}")

if [ "$(git rev-parse "$site_commit^{tree}")" = "$local_tree" ] &&
    git merge-base --is-ancestor "$local_commit" "$site_commit"; then
    echo "pre-push: Sites already has $local_commit."
    exit 0
fi

if git merge-base --is-ancestor "$site_commit" "$local_commit"; then
    sync_commit=$local_commit
else
    if ! merged_tree=$(git merge-tree --write-tree "$site_commit" "$local_commit") ||
        [ "$merged_tree" != "$local_tree" ]; then
        echo "pre-push: Sites has divergent changes. Reconcile $site_ref before syncing." >&2
        exit 1
    fi
    sync_commit=$(git commit-tree "$local_tree" -p "$site_commit" -p "$local_commit" -m "Sync GitHub main to Sites")
fi

git "$@" push --no-verify sites "$sync_commit:refs/heads/main"
