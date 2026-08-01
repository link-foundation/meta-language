#!/usr/bin/env bash
# Simulate merging the pull request head into a fresh copy of the base branch and
# run the fast checks against that merge result.
#
# A pull request can be green in isolation and still break the base branch when a
# semantic (non-textual) conflict is introduced: both sides merge cleanly, but the
# combined tree no longer compiles or passes tests. GitHub only tests the PR head
# (or a merge commit computed at PR creation time), so this script recreates the
# merge locally against the current tip of the base branch.
#
# Environment:
#   GITHUB_BASE_REF - base branch name (set automatically for pull_request events)
set -euo pipefail

BASE_REF="${GITHUB_BASE_REF:-main}"

if [ -z "${GITHUB_BASE_REF:-}" ]; then
  echo "GITHUB_BASE_REF is not set; assuming base branch '${BASE_REF}'"
fi

echo "Fetching origin/${BASE_REF}..."
git fetch --no-tags origin "${BASE_REF}"

BASE_SHA="$(git rev-parse "origin/${BASE_REF}")"
HEAD_SHA="$(git rev-parse HEAD)"
echo "Base: ${BASE_REF} (${BASE_SHA})"
echo "Head: ${HEAD_SHA}"

if git merge-base --is-ancestor "${HEAD_SHA}" "${BASE_SHA}"; then
  echo "Head is already contained in origin/${BASE_REF}; nothing to simulate."
  exit 0
fi

# Merge into a detached checkout of the base tip so the working branch is untouched.
git config user.name "${GIT_AUTHOR_NAME:-github-actions[bot]}"
git config user.email "${GIT_AUTHOR_EMAIL:-github-actions[bot]@users.noreply.github.com}"
git checkout --detach "${BASE_SHA}"

if ! git merge --no-edit "${HEAD_SHA}"; then
  echo "::error::Textual merge conflict with origin/${BASE_REF}. Merge the base branch into this pull request and resolve the conflicts."
  git merge --abort || true
  git checkout --force -
  exit 1
fi

echo "Merge succeeded. Running checks on the merged tree..."

# This repository is a Rust/JavaScript monorepo. Cargo commands must run from
# the Rust package rather than the repository root.
REPO_ROOT="$(git rev-parse --show-toplevel)"
cd "${REPO_ROOT}/rust"

status=0
echo "::group::cargo fmt --all -- --check"
cargo fmt --all -- --check || status=1
echo "::endgroup::"

echo "::group::cargo clippy --all-targets --all-features"
cargo clippy --all-targets --all-features || status=1
echo "::endgroup::"

echo "::group::cargo test --all-features"
cargo test --all-features || status=1
echo "::endgroup::"

if [ "${status}" -ne 0 ]; then
  echo "::error::Checks failed on the simulated merge with origin/${BASE_REF} even though they pass on the pull request head. This is a semantic merge conflict."
fi

git checkout --force - >/dev/null 2>&1 || true
exit "${status}"
