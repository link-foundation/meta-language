#!/usr/bin/env bash
# Phase 3 (standalone, robust) — wire blocked_by dependencies per the canonical DAG.
# Idempotent: GETs current blockers (as a newline string) and skips ones present.
set -uo pipefail

REPO="link-foundation/meta-language"
WORK="$(cd "$(dirname "$0")" && pwd)"
MAP="${WORK}/issue-map.tsv"

declare -A NUM DBID
while IFS=$'\t' read -r sid num dbid title; do
  [ -z "${sid:-}" ] && continue
  NUM["$sid"]="$num"; DBID["$sid"]="$dbid"
done < "$MAP"

declare -A BLOCKERS=(
  [A1]="" [A2]="A1" [A3]="A1" [F1]="A1"
  [B1]="A1" [B2]="A1" [B3]="A1" [B4]="A1" [B5]="A1" [B6]="A1" [B7]="A1"
  [C1]="A1" [C2]="A1" [C3]="A1" [C4]="A1" [C5]="A1" [C7]="A1" [C6]="A1 A3"
  [D1]="A1" [D2]="A1" [D4]="A1" [D6]="A1 A2" [D3]="A1 D1" [D5]="A1 D1 D6"
  [D7]="A1 D5" [D8]="A1 D5" [D9]="A3 D5" [E2]="A1" [D10]="A1 E2"
  [E1]="A1 B1 C1 D5" [E3]="D1 D5" [E4]="A1 A2" [F2]="B1 C1" [E5]="C4 C5 D5 E1"
)
ORDER=(A1 A2 A3 F1 B1 B2 B3 B4 B5 B6 B7 C1 C2 C3 C4 C5 C7 C6 D1 D2 D4 D6 D3 D5 D7 D8 D9 E2 D10 E1 E3 E4 F2 E5)

added=0 skipped=0 failed=0
for sid in "${ORDER[@]}"; do
  blk="${BLOCKERS[$sid]}"
  [ -z "$blk" ] && { echo "= ${sid} (#${NUM[$sid]}) root, no blockers"; continue; }
  child="${NUM[$sid]}"
  curb="$(gh api --paginate "repos/${REPO}/issues/${child}/dependencies/blocked_by" --jq '.[].number' 2>/dev/null)"
  for b in $blk; do
    bnum="${NUM[$b]}"; bdb="${DBID[$b]}"
    if printf '%s\n' "$curb" | grep -qx "$bnum"; then
      echo "= #${child} (${sid}) already blocked by #${bnum} (${b})"; skipped=$((skipped+1)); continue
    fi
    if gh api --method POST "repos/${REPO}/issues/${child}/dependencies/blocked_by" \
         -H "Accept: application/vnd.github+json" -F "issue_id=${bdb}" >/dev/null 2>"${WORK}/last-dep.err"; then
      echo "+ #${child} (${sid}) blocked by #${bnum} (${b})"; added=$((added+1))
    else
      echo "!! #${child} (${sid}) blocked_by #${bnum} (${b}) FAILED: $(cat "${WORK}/last-dep.err")"; failed=$((failed+1))
    fi
  done
done
echo
echo "=== Phase 3 summary: added=${added} skipped=${skipped} failed=${failed} ==="