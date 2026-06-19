# E3 — Competitor benchmark suite

> **Epic:** E — Tooling, integration, benchmarking · **Blocked by:** [`D1`](./D1-inference-evaluation-harness.md), [`D5`](./D5-blackbox-cfg-inference.md)
> **Requirements:** P-12 · **Milestone:** M4
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic E,
> [`competitive-analysis.md`](../competitive-analysis.md) (the bar),
> [`literature-review.md`](../literature-review.md) §4 (the ladder).

## Context

P-12 — "beats all the competitors in all metrics" — is the one requirement that
cannot be satisfied by code alone; it must be *demonstrated on the same corpora
the competitors published*, with the same metrics, and gated so the claim cannot
silently regress. [`competitive-analysis.md`](../competitive-analysis.md)
operationalises P-12: it names the competitor ladder
(**GLADE → Arvada → TreeVada → Kedavra → NatGI**, §1), pins the metrics
(precision, recall, F1, wall-clock, §2), and states the per-tool bar (§3). D1
provides the metric engine (precision/recall by sampling, F1, MDL/grammar-size,
golden-corpus runner — [`D1`](./D1-inference-evaluation-harness.md)) and D5
provides the inference engine under test ([`D5`](./D5-blackbox-cfg-inference.md)).
This issue is the **harness that wires D5 outputs through D1 metrics on the
vendored competitor corpora and gates the P-12 claims in CI**.

The repo already has the fixture discipline this builds on: `src/parity.rs`
exposes executable corpora (`PROGRAMMING_LANGUAGE_TARGETS` at
`src/parity.rs:619`, plus `LANGUAGE_FIXTURES`/`PARITY_FIXTURES`, see
[`existing-capabilities.md`](../existing-capabilities.md) §1), and the ~30 wired
tree-sitter grammars are golden CST oracles. E3 mirrors that discipline for the
*competitor* corpora rather than the parity corpora.

The integrity risk is explicit. [`competitive-analysis.md`](../competitive-analysis.md)
§5 names two: **corpus cherry-picking** (mitigation: vendor the *published*
corpora unchanged and **log any subset explicitly — no silent truncation**), and
**"all metrics" being unbounded** (mitigation: scope to the published primary
metrics plus the named secondary metrics; new metrics are added by amending D1,
not by moving goalposts). E3 must encode both mitigations as hard rules.

## Goal

Vendor the published **TreeVada / Arvada / GLADE** benchmark corpora unchanged,
run the D1 metrics (precision, recall, F1, wall-clock) of the D5 inference engine
against them, compare to the pinned per-competitor baselines, and **gate the P-12
claims in CI**: the build fails if D5 does not meet-or-exceed the bar, or if any
corpus is silently subset. Additionally report the **secondary metrics no
competitor reports** (format coverage, round-trip fidelity, GBNF emit,
cross-language translation) so P-12's "all metrics" is covered, not just F1.

## Scope

**In scope**
- A vendored corpus tree (`benches/corpora/`) holding the published
  TreeVada/Arvada/GLADE example sets and their golden grammars/oracles, each with
  a `LICENSE`/`PROVENANCE` note (all three are MIT/Apache — see manifest below).
- A `corpus-manifest.toml` (or `.json`) enumerating every corpus: source repo +
  commit, licence, file count, total bytes, and **which subset (if any) is run**,
  with a machine-checkable `included`/`excluded` split and a reason per exclusion.
- A benchmark runner (`benches/competitor_bench.rs` + a `tests/` gate) that, per
  corpus: runs D5, computes D1 metrics, prints a per-tool comparison table, and
  asserts the bar.
- The **baseline table** (pinned numbers below) the runner compares against.
- The **secondary-metric** report (format coverage, round-trip, GBNF emit,
  cross-language translation), computed from the B*/C*/E pipelines.
- A CI gate that fails on (a) any primary metric below the bar, or (b) a manifest
  whose `included` set is smaller than the vendored set without a logged reason.

**Out of scope** (owned elsewhere)
- The metric definitions/engine → **D1** (E3 *calls* D1; it does not redefine
  precision/recall/F1/MDL).
- The inference engine → **D5** (E3 *runs* it).
- Re-running the competitors' own code (Python/Java) is **not** required —
  baselines are taken from the *published papers* (cited below). E3 may
  optionally shell out to a competitor (e.g. flexfringe as a GPL **external** CLI,
  [`library-survey.md`](../library-survey.md) §C.3) but must not vendor GPL code.
- The LLM-assisted path's numbers → **D9** (E3 reports the deterministic D5 result
  as the headline; a D9 LLM-assisted column is added when D9 lands).

## Design / specification

### Corpus manifest

Vendor the corpora from the three permissive competitor repos, unchanged, each
pinned to a commit. Layout:

```
benches/
  corpora/
    treevada/        # github.com/rifatarefin/treevada (MIT) — ICSE 2024
      PROVENANCE      #   repo URL + commit SHA + licence (MIT)
      LICENSE
      <grammar>/...   #   example sets + golden grammar per subject
    arvada/          # github.com/neil-kulkarni/arvada   (MIT) — ASE 2021
      PROVENANCE; LICENSE; <grammar>/...
    glade/           # github.com/obastani/glade        (Apache-2.0) — PLDI 2017
      PROVENANCE; LICENSE; <grammar>/...
  corpus-manifest.toml
  competitor_bench.rs
```

`corpus-manifest.toml` — one entry per subject grammar:

```toml
[[corpus]]
tool      = "treevada"          # treevada | arvada | glade
subject   = "json"              # the subject grammar/language
source     = "github.com/rifatarefin/treevada"
commit     = "<sha>"            # pinned
license    = "MIT"
files      = 0                  # vendored example count
bytes      = 0                  # vendored total size
included   = true               # is this subject RUN by the gate?
exclude_reason = ""             # REQUIRED non-empty string when included = false
```

**Hard rule (anti-cherry-pick, [`competitive-analysis.md`](../competitive-analysis.md) §5):**
the gate enumerates `benches/corpora/**` and asserts that **every vendored
subject is either `included = true` or has a non-empty `exclude_reason`** — there
is no silent cap. Any `included = false` is printed in the run log as
`SKIPPED <tool>/<subject>: <reason>`. The CI gate fails if a vendored subject is
absent from the manifest, or `included = false` with an empty reason. The crate
`include` allowlist (`Cargo.toml:16-24`) already keeps `benches/` out of the
published `.crate`, so vendored corpora do not inflate the release archive.

### Harness layout

`benches/competitor_bench.rs` (and a thin `tests/` gate that reuses the same
library functions) does, per `included` corpus subject:

1. Load the example set and the golden grammar/oracle from `benches/corpora/...`.
2. Run D5 inference ([`D5`](./D5-blackbox-cfg-inference.md)) → an A1 `Grammar`.
3. Compute D1 metrics ([`D1`](./D1-inference-evaluation-harness.md)) — precision
   (sample from inferred, check against golden), recall (sample from golden,
   check against inferred), F1, wall-clock, MDL/grammar-size.
4. Emit a per-subject row and a per-tool aggregate (mean F1, etc.).
5. Assert the bar (below); collect failures and report them all before failing.

Keep the metric math in D1; E3 only orchestrates and asserts. Determinism is a
competitive property (TreeVada's claim, §2) — the runner must be reproducible:
no random seeds in D5's deterministic path, fixed sampling seed for D1's
precision/recall sampling, recorded in the run log.

### Baseline table (the bar to beat — pinned)

Primary F1 baselines from the published papers
([`competitive-analysis.md`](../competitive-analysis.md) §1–§3,
[`literature-review.md`](../literature-review.md) §4):

| Competitor | Venue | Pinned headline | Bar for D5 (this project) |
|---|---|---|---|
| **GLADE** | PLDI 2017 | baseline precision/recall (lower bound) | meet-or-exceed precision **and** recall on every corpus |
| **Arvada** | ASE 2021 | ~4.98× GLADE recall on recursive grammars | meet-or-exceed recall on every corpus |
| **TreeVada** | ICSE 2024 | **avg F1 ≈ 0.32**; ~2.4× faster than Arvada; deterministic | meet-or-exceed F1 on every corpus **and** match/beat determinism + wall-clock |
| **Kedavra** | ASE 2024 | better all-round under limited examples (incremental) | meet-or-exceed precision/recall/runtime under the incremental protocol |
| **NatGI** | 2025 (arXiv 2509.26616) | **avg F1 ≈ 0.57** (SOTA, +25 pts over TreeVada) | meet-or-exceed **avg F1 > 0.57** while *not requiring* an LLM at runtime (P-7) |

These exact numbers (TreeVada ≈ 0.32, NatGI ≈ 0.57) are encoded as constants in
the harness with a citation comment, so the gate is self-documenting. NatGI is
the **top bar**; the central, honestly-tracked risk is that matching its F1
*deterministically* is hard ([`competitive-analysis.md`](../competitive-analysis.md)
§5) — hence the harness reports D5's deterministic F1 as the headline and leaves a
labelled column for the D9 LLM-assisted result.

### Secondary metrics (no competitor reports these — free P-12 wins)

Per [`competitive-analysis.md`](../competitive-analysis.md) §2 ("Secondary"), the
report also tabulates, with the competitors scored 0 by construction:

| Secondary metric | How E3 measures it | Owning issue |
|---|---|---|
| **Format coverage** | count of notations importable (B1–B7) × emittable (C1–C3, C7), exercised on a sample grammar | B*/C* |
| **Round-trip fidelity** | import format X → emit format Y → re-import → structural equality (the A1 round-trip invariant), per the fidelity matrix pattern | [`C1`](./C1-bnf-ebnf-abnf-emitters.md), F2 |
| **GBNF emit** | inferred grammar → GBNF; assert it is non-empty and well-formed for llama.cpp constraint use | C3 |
| **Cross-language translation** | translate a grammar's human-facing surface en↔ru and assert structure preserved | [`C6`](./C6-concept-aligned-translation.md) |

These need not gate the build at M4 (some of their owning issues land at M5), but
the report must *list* them so "all metrics" (P-12) is visibly covered, not just
F1. Where an owning issue is not yet merged, the row prints `n/a (pending <issue>)`
rather than a fabricated number.

### CI gate

A `tests/` integration test (so it runs under `cargo test --all-features`, not
only `cargo bench`) named e.g. `competitor_bar`:
- runs the harness on every `included` subject,
- asserts each primary metric ≥ its pinned bar,
- asserts the manifest integrity rule (no silent subset),
- on failure, prints the full comparison table + every `SKIPPED` line, then fails.

Wire it into the existing CI workflow alongside `cargo test`. Long-running full
corpora may use a `#[ignore]`d "full" variant plus a fast representative subset
in the always-on gate — but the fast subset's `included`/`excluded` split is
itself logged through the same manifest rule (no silent cap).

## File-level plan

| File | Change |
|---|---|
| `benches/corpora/{treevada,arvada,glade}/` | New. Vendored published example sets + golden grammars, each with `PROVENANCE` (repo + commit + licence) and `LICENSE`. |
| `benches/corpus-manifest.toml` | New. One entry per subject: tool, source, commit, licence, files, bytes, `included`, `exclude_reason`. |
| `benches/competitor_bench.rs` | New. The runner: load → D5 → D1 metrics → table → assert bar. Registered under `[[bench]]` in `Cargo.toml`. |
| `tests/integration/competitor_bar.rs` (+ `tests/integration/mod.rs`) | New. The CI gate: primary-metric bar + manifest-integrity assertions, reusing the runner's library functions. |
| `src/benchmark/mod.rs` (optional) | If shared between bench + test, a small library module holding the baseline constants + manifest parser + comparison logic; re-export from `src/lib.rs`. Keep metric math in D1. |
| `Cargo.toml` | Add `[[bench]]` for `competitor_bench`; add a `toml` (or reuse `serde_json`) dev-dependency for the manifest if needed. `benches/` is already excluded from the published archive (`Cargo.toml:16-24`). |
| `changelog.d/` | Add a fragment (see CONTRIBUTING / `README.md` changelog section). |

## Reuse

- D1 metric engine (precision/recall by sampling, F1, MDL, golden-corpus runner) — [`D1`](./D1-inference-evaluation-harness.md). **Do not re-implement metrics here.**
- D5 inference engine under test — [`D5`](./D5-blackbox-cfg-inference.md).
- `src/parity.rs` executable-corpus discipline (`PARITY_TARGETS` at `src/parity.rs:107`, `PROGRAMMING_LANGUAGE_TARGETS` at `src/parity.rs:619`, `GRAMMAR_EMBEDDING_TARGETS` at `src/parity.rs:795`) — the pattern for provenance-stamped, machine-enumerable corpora.
- The `include` allowlist (`Cargo.toml:16-24`) already keeps docs/benches out of the `.crate` — vendored corpora are safe to add under `benches/`.
- Secondary metrics come from the B*/C*/E pipelines: importers [`B1`](./B1-bnf-importer.md), emitters [`C1`](./C1-bnf-ebnf-abnf-emitters.md), translation [`C6`](./C6-concept-aligned-translation.md).
- The bar + provenance: [`competitive-analysis.md`](../competitive-analysis.md) §1–§5; the ladder: [`literature-review.md`](../literature-review.md) §4.

## Acceptance criteria

- [ ] The published **TreeVada, Arvada, and GLADE** corpora are vendored under
      `benches/corpora/` unchanged, each with a `PROVENANCE` (repo + pinned
      commit + licence) and `LICENSE` file (all MIT/Apache — no GPL vendored).
- [ ] `benches/corpus-manifest.toml` enumerates every vendored subject with
      `tool`, `source`, `commit`, `license`, `files`, `bytes`, `included`, and a
      non-empty `exclude_reason` whenever `included = false`.
- [ ] The runner computes precision, recall, F1, and wall-clock **via D1** for
      every `included` subject and prints a per-tool comparison table against the
      pinned baselines (TreeVada ≈ 0.32, NatGI ≈ 0.57, encoded with a citation).
- [ ] The CI gate (`tests/integration/competitor_bar.rs`) fails the build if any
      primary metric is below its pinned bar
      ([`competitive-analysis.md`](../competitive-analysis.md) §3), and runs under
      `cargo test --all-features`.
- [ ] **No silent truncation:** the gate enumerates `benches/corpora/**` and fails
      if a vendored subject is missing from the manifest or has `included = false`
      with an empty reason; every skip prints `SKIPPED <tool>/<subject>: <reason>`
      ([`competitive-analysis.md`](../competitive-analysis.md) §5).
- [ ] The report tabulates the secondary metrics (format coverage, round-trip,
      GBNF emit, cross-language translation), printing `n/a (pending <issue>)`
      where the owning issue has not yet merged — never a fabricated number.
- [ ] The deterministic D5 result is the headline (no LLM required, P-7); a
      labelled column is reserved for the D9 LLM-assisted result.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (clippy
      `pedantic`/`nursery` are `warn` per `Cargo.toml:103-106`), and
      `cargo test --all-features` all pass; `rust-script scripts/check-no-src-tests.rs`
      passes (the gate and runner live under `tests/` and `benches/`, not `src/`).

## Tests

- `tests/integration/competitor_bar.rs`:
  - manifest-integrity: a doctored manifest that drops a vendored subject, or sets
    `included = false` with an empty reason, makes the gate **fail** (test the
    failure path with a fixture manifest).
  - bar assertion: on a small `included` subset, D5 → D1 metrics meet-or-exceed
    the pinned per-tool bar; a deliberately crippled grammar fails the bar (proves
    the gate has teeth).
  - secondary report: the table includes a row per secondary metric, with pending
    rows rendered as `n/a (pending <issue>)`.
  - determinism: two runs on the same corpus produce identical primary metrics
    (fixed sampling seed; D5 deterministic path).
- `benches/competitor_bench.rs` runs the full corpora under `cargo bench` (and an
  `#[ignore]`d full variant under test) for the published comparison table.
- No network access at test time — corpora are vendored, not downloaded.

## References

- The bar + integrity rules: [`competitive-analysis.md`](../competitive-analysis.md) §1 (landscape), §2 (metrics), §3 (per-competitor bar), §5 (cherry-picking + unbounded-metrics mitigations).
- The ladder + pinned numbers: [`literature-review.md`](../literature-review.md) §4 (GLADE→Arvada→TreeVada→Kedavra→NatGI; TreeVada F1 ≈ 0.32, NatGI F1 ≈ 0.57).
- Corpora: TreeVada <https://github.com/rifatarefin/treevada> (MIT, ICSE 2024, arXiv 2308.06163) · Arvada <https://github.com/neil-kulkarni/arvada> (MIT, ASE 2021, arXiv 2108.13340) · GLADE <https://github.com/obastani/glade> (Apache-2.0, PLDI 2017). Licence/permissiveness vetted in [`library-survey.md`](../library-survey.md) §C.1, §C.6.
- Metric engine + inference engine: [`D1`](./D1-inference-evaluation-harness.md), [`D5`](./D5-blackbox-cfg-inference.md).
- [`solution-plans.md`](../solution-plans.md) §Epic E (E3), [`existing-capabilities.md`](../existing-capabilities.md) §1, §3 (E3 row).
