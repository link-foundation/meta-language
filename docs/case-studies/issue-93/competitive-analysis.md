# Competitive Analysis (issue #93, requirement P-12)

> "…beats all the competitors in all metrics." — issue #93

This file operationalises P-12: it names the competitors, fixes the metrics, and
states the concrete bar each must be beaten on. The pinned metric definitions
live in proposed issue **D1**; the reproducible benchmark harness + vendored
corpora live in **E3**. Researched/verified 2026-06-19; see
[`literature-review.md`](./literature-review.md) for full citations.

## 1. Competitor landscape

| Tool | Year / venue | Setting | Licence | Status as competitor |
|---|---|---|---|---|
| **GLADE** | PLDI 2017 | Black-box, oracle-guided | Apache-2.0 | Baseline (lower bound) |
| **REINAM** | FSE 2019 | Black-box + symbolic exec + RL | none published | Study-only; not reproducible |
| **Mimid** | FSE 2020 | **White-box** (instruments parser) | CC-BY-NC-SA | Different setting; out of P-3 scope |
| **Arvada** | ASE 2021 | Black-box, oracle, bubble-merge | MIT | Recall reference (~5× GLADE) |
| **TreeVada** | ICSE 2024 | Black-box, deterministic, bracket prior | MIT | F1 + speed reference; **primary port** |
| **Kedavra** | ASE 2024 | Black-box, incremental | (per repo) | All-round reference |
| **NatGI** | 2025 (arXiv 2509.26616) | Black-box + LLM + brackets + HDD | (per repo) | **SOTA — top bar (F1 ≈ 0.57)** |
| flexfringe | maintained | Regular state-merging (DFA/PDFA) | GPL | Regular-only; external oracle for benchmarks |
| LearnLib | maintained | Active automata learning (L\*/TTT) | Apache-2.0 | Active-learning (oracle) reference for D10 |
| ISLa / ISLearn | FSE 2022 | Constraint mining (beyond CFG) | GPL | Semantic-layer reference for D8 |

The black-box, positive-leaning CFG-inference line — **GLADE → Arvada →
TreeVada → Kedavra → NatGI** — is the direct competition for P-3. The published
ordering on F1/recall makes the ladder unambiguous: **NatGI is the bar to beat.**

## 2. Metrics (pinned in D1)

**Primary (the competitors report these):**
- **Precision** — sample strings from the inferred grammar; fraction accepted by
  the golden grammar/oracle.
- **Recall** — sample strings from the golden grammar; fraction accepted by the
  inferred grammar.
- **F1** — harmonic mean. NatGI's headline ≈ **0.57 avg**; TreeVada ≈ 0.32 on the
  same suite (NatGI reports +25 pts).
- **Wall-clock** — TreeVada's claim is ~2.4× faster than Arvada; determinism
  (no random seeds) is itself a competitive property.

**Secondary — metrics the competitors largely do *not* report (free P-12 wins):**
- **Readability / naturalness** of the grammar (meaningful non-terminal names) —
  NatGI is the only one to push on this, via an LLM; the **concept ontology
  (P-11)** gives it deterministically.
- **Round-trip fidelity** — import a grammar in format X, emit format Y, re-import:
  measure exact/again-equivalent recovery (no competitor does cross-format).
- **Format coverage** — count of grammar notations importable (B1–B7) and
  emittable (C1–C3). Competitors infer; they do not interoperate.
- **Cross-language translation accuracy** — emit a working parser in Rust *and*
  JS from one inferred grammar (P-9); none of the competitors do codegen.
- **LLM-constraint readiness** — emit GBNF (C3) usable directly by
  llama.cpp/vLLM/XGrammar. Unique to this project.

## 3. The bar, per competitor

To satisfy "beats all competitors in all metrics," the E3 harness must show, on
the **shared TreeVada/Arvada/GLADE corpora** (D1 fixes the exact set):

1. **vs GLADE / Arvada** — meet-or-exceed precision *and* recall on every corpus.
2. **vs TreeVada** — meet-or-exceed F1 on every corpus *and* match/beat its
   determinism + wall-clock.
3. **vs Kedavra** — meet-or-exceed precision/recall/runtime under the
   incremental (limited-example) protocol.
4. **vs NatGI (SOTA)** — meet-or-exceed average F1 (**> 0.57**) *and* match its
   readability, while *not requiring* an LLM at runtime (P-7 determinism).
5. **On all secondary metrics** — win by construction (the competitors score 0
   on format coverage, round-trip, codegen, and GBNF emit).

## 4. Why this project can clear the bar — structural advantages

The SOTA had to *add* the things this project *starts with*:

| NatGI / SOTA technique | This project's native equivalent | Issue |
|---|---|---|
| Bracket-guided bubble exploration (the #1 driver of NatGI's gains) | meta-notation delimiter skeleton — parsed losslessly by construction (P-4) | D6 |
| LLM-generated meaningful non-terminal names | Shared concept ontology, 351-concept lexicon, exact-match interning (P-11) | A3, D9 |
| Hierarchical delta debugging to simplify trees | MDL/Occam minimization over the links IR | D7 |
| (none — competitors stop at one grammar) | Cross-format import/emit + parser codegen + GBNF | B*, C*, E |

So the strategy is not "invent a better bubble-merge"; it is **"reproduce the
SOTA pipeline on top of a substrate that already supplies its two strongest
priors (delimiters + concepts), then win the secondary metrics no competitor
contests."** That is a defensible, measurable route to P-12 — and every claim is
gated by the E3 harness, not asserted.

## 5. Risks to the P-12 claim (tracked honestly)

- **F1 parity with NatGI is hard.** NatGI uses an LLM in-loop; matching its F1
  deterministically is the central research risk. Mitigation: D9 keeps the LLM
  as an *optional accelerator* with a deterministic fallback, so the project can
  report *both* "best deterministic F1" and "best LLM-assisted F1."
- **Corpus cherry-picking.** Mitigation: E3 vendors the *published* corpora
  unchanged and logs any subset explicitly (no silent truncation).
- **"All metrics" is unbounded.** Mitigation: requirements.md §"Reading of the
  ambiguous points" scopes P-12 to the published primary metrics plus the named
  secondary metrics; new metrics are added by amending D1, not by moving goalposts.
</content>
