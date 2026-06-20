#!/usr/bin/env bash
# Phase 2 — attach every issue as a sub-issue of #93.
# Phase 3 — wire blocked_by dependencies per the canonical DAG.
# Idempotent: GETs current relationships and skips ones already present.
set -uo pipefail

REPO="link-foundation/meta-language"
PARENT=93
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

echo "=== Phase 2: sub-issues of #${PARENT} ==="
# numbers already attached as sub-issues of the parent
mapfile -t CURRENT_SUB < <(gh api --paginate "repos/${REPO}/issues/${PARENT}/sub_issues" --jq '.[].number')
declare -A HAVE_SUB; for n in "${CURRENT_SUB[@]:-}"; do HAVE_SUB["$n"]=1; done
for sid in "${ORDER[@]}"; do
  n="${NUM[$sid]}"
  if [ -n "${HAVE_SUB[$n]:-}" ]; then echo "= ${sid} (#${n}) already a sub-issue"; continue; fi
  out="$(gh api --method POST "repos/${REPO}/issues/${PARENT}/sub_issues" \
          -H "Accept: application/vnd.github+json" -F "sub_issue_id=${DBID[$sid]}" 2>&1)"
  if [ $? -eq 0 ]; then echo "+ ${sid} (#${n}) -> sub-issue of #${PARENT}";
  else echo "!! ${sid} (#${n}) sub-issue FAILED: $out"; fi
done

echo
echo "=== Phase 3: blocked_by dependencies ==="
for sid in "${ORDER[@]}"; do
  blk="${BLOCKERS[$sid]}"
  [ -z "$blk" ] && { echo "= ${sid} (#${NUM[$sid]}) root, no blockers"; continue; }
  child="${NUM[$sid]}"
  mapfile -t CUR < <(gh api --paginate "repos/${REPO}/issues/${child}/dependencies/blocked_by" --jq '.[].number' 2>/dev/null)
  declare -A HAVEB; HAVEB=(); for n in "${CUR[@]:-}"; do HAVEB["$n"]=1; done
  for b in $blk; do
    bnum="${NUM[$b]}"; bdb="${DBID[$b]}"
    if [ -n "${HAVEB[$bnum]:-}" ]; then echo "= #${child} already blocked by #${bnum} (${b})"; continue; fi
    out="$(gh api --method POST "repos/${REPO}/issues/${child}/dependencies/blocked_by" \
            -H "Accept: application/vnd.github+json" -F "issue_id=${bdb}" 2>&1)"
    if [ $? -eq 0 ]; then echo "+ #${child} (${sid}) blocked by #${bnum} (${b})";
    else echo "!! #${child} blocked_by #${bnum} FAILED: $out"; fi
  done
done
echo
echo "=== DONE wiring ==="