#!/usr/bin/env bash
#
# create-issues.sh — file the proposed implementation issues for issue #47.
#
# These issues decompose link-foundation/meta-language#47 into actionable work.
# Each NN-slug.md file in this directory carries YAML frontmatter (title, labels)
# followed by the issue body.
#
# SAFETY: this script does NOT create anything by default. It prints a dry-run.
# Pass --create to actually file the issues. It is idempotent: any issue whose
# exact title already exists (open or closed) is skipped.
#
# Usage:
#   ./create-issues.sh            # preview (creates nothing)
#   ./create-issues.sh --create   # actually file the issues
#
# Requires: gh (authenticated), run from anywhere.
#
# Note: relative links in the bodies (../requirements.md, ...) are rewritten to
# absolute GitHub blob URLs on the `main` branch, so they resolve once this PR is
# merged. Cross-references between specs are written as `#NN` (code-styled) so they
# do not mislink before the real GitHub issue numbers are known — wire up the real
# "blocked by" relationships after filing (see the table in README.md).

set -euo pipefail

REPO="link-foundation/meta-language"
BLOB_BASE="https://github.com/${REPO}/blob/main/docs/case-studies/issue-47"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

CREATE=0
if [ "${1:-}" = "--create" ]; then
  CREATE=1
elif [ -n "${1:-}" ]; then
  echo "Unknown argument: ${1}. Use --create to file issues, or no argument to preview." >&2
  exit 2
fi

if ! command -v gh >/dev/null 2>&1; then
  echo "error: gh CLI not found." >&2
  exit 1
fi

# Fetch existing titles once for idempotency (open + closed).
existing_titles="$(gh issue list --repo "$REPO" --state all --limit 500 --json title --jq '.[].title' 2>/dev/null || true)"

created=0
skipped=0
planned=0

for f in "$SCRIPT_DIR"/[0-9][0-9]-*.md; do
  [ -e "$f" ] || continue

  # --- parse frontmatter ---
  title="$(sed -n 's/^title:[[:space:]]*"\(.*\)"[[:space:]]*$/\1/p' "$f" | head -n1)"
  labels="$(sed -n 's/^labels:[[:space:]]*\(.*\)$/\1/p' "$f" | head -n1)"
  if [ -z "$title" ]; then
    echo "SKIP (no title frontmatter): $(basename "$f")" >&2
    continue
  fi

  # --- body = everything after the 2nd '---', with relative links absolutized ---
  fm_end="$(grep -n '^---[[:space:]]*$' "$f" | sed -n '2p' | cut -d: -f1)"
  if [ -z "$fm_end" ]; then
    echo "SKIP (malformed frontmatter): $(basename "$f")" >&2
    continue
  fi
  body="$(tail -n +"$((fm_end + 1))" "$f" | sed "s#](\.\./#](${BLOB_BASE}/#g")"
  body="${body}"$'\n\n'"---"$'\n'"_Filed from \`docs/case-studies/issue-47/proposed-issues/$(basename "$f")\`. Part of the implementation plan for #47._"

  # --- build label args (comma-separated -> repeated --label) ---
  label_args=()
  IFS=',' read -ra _labels <<< "$labels"
  for l in "${_labels[@]}"; do
    l_trim="$(printf '%s' "$l" | xargs)"
    [ -n "$l_trim" ] && label_args+=(--label "$l_trim")
  done

  # --- idempotency ---
  if printf '%s\n' "$existing_titles" | grep -Fxq "$title"; then
    echo "SKIP (exists): $title"
    skipped=$((skipped + 1))
    continue
  fi

  if [ "$CREATE" -eq 1 ]; then
    tmp="$(mktemp)"
    printf '%s\n' "$body" > "$tmp"
    url="$(gh issue create --repo "$REPO" --title "$title" "${label_args[@]}" --body-file "$tmp")"
    rm -f "$tmp"
    echo "CREATED: $url"
    created=$((created + 1))
  else
    echo "DRY-RUN would create: [${labels}] ${title}"
    planned=$((planned + 1))
  fi
done

echo
if [ "$CREATE" -eq 1 ]; then
  echo "Done. Created: ${created}, skipped (already existed): ${skipped}."
else
  echo "Dry run. Would create: ${planned}, would skip (already exist): ${skipped}."
  echo "Re-run with --create to file them."
fi
