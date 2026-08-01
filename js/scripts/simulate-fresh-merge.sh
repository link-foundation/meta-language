#!/usr/bin/env bash
# Merge the latest base branch into the checked-out PR before running checks.
set -euo pipefail

if [ -z "${BASE_REF:-}" ]; then
  echo "::error::BASE_REF is required for fresh merge simulation"
  exit 1
fi

git config user.email "github-actions[bot]@users.noreply.github.com"
git config user.name "github-actions[bot]"
git fetch --no-tags origin "$BASE_REF"

if [ "$(git rev-list --count "HEAD..origin/$BASE_REF")" -eq 0 ]; then
  echo "Merge preview already contains origin/$BASE_REF"
  exit 0
fi

if ! git merge "origin/$BASE_REF" --no-edit; then
  echo "::error::PR does not merge cleanly with origin/$BASE_REF"
  exit 1
fi

echo "Fresh merge simulation succeeded"
