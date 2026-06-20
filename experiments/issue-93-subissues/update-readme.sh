#!/usr/bin/env bash
# Replace the "_pending_" Filed-as cell on each table row with a live issue link.
set -uo pipefail
WORK="$(cd "$(dirname "$0")" && pwd)"
MAP="${WORK}/issue-map.tsv"
README="$(cd "${WORK}/../../docs/case-studies/issue-93/proposed-issues" && pwd)/README.md"
URLBASE="https://github.com/link-foundation/meta-language/issues"

while IFS=$'\t' read -r sid num dbid title; do
  [ -z "${sid:-}" ] && continue
  # anchor the row by its leading "| [<ID>]" cell; the "\]" disambiguates D1 from D10.
  sed -i -E "/^\| \[${sid}\]\(/ s#_pending_#[#${num}](${URLBASE}/${num})#" "$README"
done < "$MAP"

echo "=== remaining _pending_ (expect 0) ==="
grep -c '_pending_' "$README" || true
echo "=== Filed-as column sample ==="
grep -nE '^\| \[(A1|D5|E5|F2)\]\(' "$README"