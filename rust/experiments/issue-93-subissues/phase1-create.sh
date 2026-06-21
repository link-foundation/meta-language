#!/usr/bin/env bash
# Phase 1 — create the 34 sub-issues of #93 (one per proposed-issue spec).
# Idempotent: dedupes by exact issue title, resumes from the map file.
set -uo pipefail

REPO="link-foundation/meta-language"
SHA="a5964ec5bc5846edf0dcf51549f66b2c9be3aa2e"
BASE="https://github.com/${REPO}/blob/${SHA}"
SPECDIR_URL="${BASE}/docs/case-studies/issue-93/proposed-issues"
DOCDIR_URL="${BASE}/docs/case-studies/issue-93"
SPECS_LOCAL="$(cd "$(dirname "$0")/../../docs/case-studies/issue-93/proposed-issues" && pwd)"
WORK="$(cd "$(dirname "$0")" && pwd)"
BODIES="${WORK}/bodies"
MAP="${WORK}/issue-map.tsv"      # specid \t number \t dbid \t title
mkdir -p "$BODIES"
touch "$MAP"

# Topological order: every spec's blockers appear earlier in this list.
ORDER=(A1 A2 A3 F1 B1 B2 B3 B4 B5 B6 B7 C1 C2 C3 C4 C5 C7 C6 D1 D2 D4 D6 D3 D5 D7 D8 D9 E2 D10 E1 E3 E4 F2 E5)

declare -A FILE=(
  [A1]=A1-grammar-ir.md [A2]=A2-grammar-surface-syntax.md [A3]=A3-grammar-concept-ontology.md
  [B1]=B1-bnf-importer.md [B2]=B2-ebnf-importer.md [B3]=B3-abnf-importer.md [B4]=B4-peg-importer.md
  [B5]=B5-tree-sitter-json-importer.md [B6]=B6-antlr-importer.md [B7]=B7-lark-gbnf-importer.md
  [C1]=C1-bnf-ebnf-abnf-emitters.md [C2]=C2-peg-emitter.md [C3]=C3-gbnf-emitter.md
  [C4]=C4-rust-parser-codegen.md [C5]=C5-javascript-parser-codegen.md [C6]=C6-concept-aligned-translation.md
  [C7]=C7-tree-sitter-grammar-js-emitter.md
  [D1]=D1-inference-evaluation-harness.md [D2]=D2-lexical-class-inference.md
  [D3]=D3-state-merging-regular-inference.md [D4]=D4-sequitur-compression.md
  [D5]=D5-blackbox-cfg-inference.md [D6]=D6-delimiter-structural-prior.md
  [D7]=D7-generalization-mdl-minimization.md [D8]=D8-semantic-constraint-inference.md
  [D9]=D9-llm-assisted-naming-merge.md [D10]=D10-active-learning-oracle.md
  [E1]=E1-cli-grammar-subcommands.md [E2]=E2-inferred-grammar-runtime-parser.md
  [E3]=E3-competitor-benchmark-suite.md [E4]=E4-grammar-authoring-ergonomics.md
  [E5]=E5-end-to-end-integration-examples.md
  [F1]=F1-grammar-subsystem-docs.md [F2]=F2-grammar-format-fidelity-matrix.md
)

declare -A TITLE=(
  [A1]="Grammar IR / expression algebra"
  [A2]="Grammar surface syntax (meta-notation-derived)"
  [A3]="Grammar-construct concept ontology"
  [B1]="BNF importer" [B2]="EBNF importer" [B3]="ABNF importer" [B4]="PEG (.pest) importer"
  [B5]="tree-sitter grammar.json importer" [B6]="ANTLR v4 .g4 importer" [B7]="Lark + GBNF importer"
  [C1]="BNF/EBNF/ABNF emitters" [C2]="PEG (.pest) emitter" [C3]="GBNF emitter (LLM interop)"
  [C4]="Rust parser codegen" [C5]="JavaScript parser codegen"
  [C6]="Concept-aligned cross-language translation" [C7]="tree-sitter grammar.js emitter"
  [D1]="Inference evaluation harness" [D2]="Tokenisation / lexical-class inference"
  [D3]="State-merging regular inference (RPNI/EDSM)" [D4]="Sequitur structural-compression pass"
  [D5]="Black-box CFG inference engine (TreeVada port)" [D6]="Delimiter-skeleton structural prior"
  [D7]="Generalization & MDL/Occam minimization" [D8]="Semantic-constraint inference (ISLearn-style)"
  [D9]="LLM-assisted naming & merge selection (optional)" [D10]="Optional active learning (L*/TTT oracle path)"
  [E1]="CLI: infer/import/emit/translate-grammar" [E2]="Inferred-grammar runtime parser (registry)"
  [E3]="Competitor benchmark suite" [E4]="Grammar authoring ergonomics"
  [E5]="End-to-end integration tests + examples/"
  [F1]="Grammar-subsystem user & architecture docs" [F2]="Grammar-format fidelity matrix"
)

declare -A BLOCKERS=(
  [A1]="" [A2]="A1" [A3]="A1" [F1]="A1"
  [B1]="A1" [B2]="A1" [B3]="A1" [B4]="A1" [B5]="A1" [B6]="A1" [B7]="A1"
  [C1]="A1" [C2]="A1" [C3]="A1" [C4]="A1" [C5]="A1" [C7]="A1" [C6]="A1 A3"
  [D1]="A1" [D2]="A1" [D4]="A1" [D6]="A1 A2" [D3]="A1 D1" [D5]="A1 D1 D6"
  [D7]="A1 D5" [D8]="A1 D5" [D9]="A3 D5" [E2]="A1" [D10]="A1 E2"
  [E1]="A1 B1 C1 D5" [E3]="D1 D5" [E4]="A1 A2" [F2]="B1 C1" [E5]="C4 C5 D5 E1"
)

# Load any existing repo issues (exact-title -> number) for dedupe / crash-recovery.
echo "Loading existing repo issues for dedupe..."
declare -A EXISTING_NUM
while IFS=$'\t' read -r n t; do
  [ -n "$n" ] && EXISTING_NUM["$t"]="$n"
done < <(gh issue list --repo "$REPO" --state all --limit 400 --json number,title \
           --jq '.[] | "\(.number)\t\(.title)"')

# Load already-recorded specs from the map (resume).
declare -A DONE_NUM
while IFS=$'\t' read -r sid num dbid title; do
  [ -n "${sid:-}" ] && DONE_NUM["$sid"]="$num"
done < "$MAP"

rewrite_links() { # stdin spec markdown -> stdout with absolute blob URLs
  sed \
    -e "s#](\./\.\./existing-capabilities\.md)#](${DOCDIR_URL}/existing-capabilities.md)#g" \
    -e "s#](\.\./\.\./\.\./\.\./#](${BASE}/#g" \
    -e "s#](\.\./#](${DOCDIR_URL}/#g" \
    -e "s#](\./#](${SPECDIR_URL}/#g"
}

for sid in "${ORDER[@]}"; do
  title="[${sid}] ${TITLE[$sid]}"
  file="${FILE[$sid]}"

  # already created (recorded in map or present in repo)?
  num="${DONE_NUM[$sid]:-}"
  [ -z "$num" ] && num="${EXISTING_NUM[$title]:-}"
  if [ -n "$num" ]; then
    if ! grep -qP "^${sid}\t" "$MAP"; then
      dbid="$(gh api "repos/${REPO}/issues/${num}" --jq '.id')"
      printf '%s\t%s\t%s\t%s\n' "$sid" "$num" "$dbid" "$title" >> "$MAP"
    fi
    echo "= ${sid}: exists as #${num} (skip create)"
    continue
  fi

  # blocked-by header, resolved to real #numbers (blockers created earlier in ORDER)
  blk="${BLOCKERS[$sid]}"
  if [ -z "$blk" ]; then
    blkline="_none — this is a root issue; it unblocks the rest of the initiative._"
  else
    refs=""
    for b in $blk; do
      bnum="${DONE_NUM[$b]:-}"
      [ -z "$bnum" ] && bnum="$(grep -P "^${b}\t" "$MAP" | head -1 | cut -f2)"
      refs="${refs:+$refs, }#${bnum} (${b})"
    done
    blkline="$refs"
  fi

  body="${BODIES}/${sid}.md"
  {
    echo "> **Part of #93** — _Easy grammar extensibility and grammar inference_. Filed from the"
    echo "> maximally-detailed case-study spec [\`${file}\`](${SPECDIR_URL}/${file})."
    echo ">"
    echo "> **Blocked by:** ${blkline}"
    echo ">"
    echo "> _Dependencies are also wired natively (GitHub sub-issue of #93 + \`blocked_by\`)._"
    echo
    echo "---"
    echo
    rewrite_links < "${SPECS_LOCAL}/${file}"
  } > "$body"

  url="$(gh issue create --repo "$REPO" --title "$title" --body-file "$body" 2>"${WORK}/last-create.err")"
  rc=$?
  if [ $rc -ne 0 ] || [ -z "$url" ]; then
    echo "!! ${sid}: create FAILED (rc=$rc)"; cat "${WORK}/last-create.err"; exit 1
  fi
  num="${url##*/}"
  dbid="$(gh api "repos/${REPO}/issues/${num}" --jq '.id')"
  DONE_NUM["$sid"]="$num"
  printf '%s\t%s\t%s\t%s\n' "$sid" "$num" "$dbid" "$title" >> "$MAP"
  echo "+ ${sid}: created #${num} (dbid ${dbid}) — ${title}"
done

echo
echo "=== issue-map.tsv (specid  number  dbid  title) ==="
sort -t$'\t' -k2 -n "$MAP"
echo "Total recorded: $(wc -l < "$MAP")"
