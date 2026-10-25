# Contributor instructions

Applies to all contributors; setup: [CONTRIBUTING.md](CONTRIBUTING.md).

## Contract

- Read [docs/vision.md](docs/vision.md) first. It alone defines architecture;
  update it with architectural changes, without competing specifications.
- Before using the ledger, run
  `cd js && npm run check:issue-195:sources -- --online`. Re-read edited/new
  sources, reconcile obligations, then refresh the source register with `--update`.
- Register hashed sources in `parity/issue-195-sources.lino`; generate atomic
  rows through `js/scripts/issue-195-requirements.mjs`. Preserve rows,
  source/behavior/scope, fixtures, runtimes and aliases; append obligations.
- Correct wrong assertions with documented evidence, retaining inputs and
  coverage. Never weaken requirements, oracles or gates to conceal failures.
- Bound version-sensitive assertions by explicit language/dependency ranges;
  select using active versions and report uncovered versions.
- Evidence requires every declared assertion on every fixture, executed at
  the recorded commit. Record observations only afterward
  (`recordIssue195Observations`/Rust `record`). No status/completion fields,
  skipped cells or plans/declarations presented as evidence.
- Every PR check must pass; implement obligations and fix failures.
  PR checks use pre-merge inputs. Check live rules/registries/comments/packages
  on `main` as non-blocking reports; repair failures afterward.
- Claim completion or add closing directives only when every ledger row passes.

## Development

- JavaScript first: draft/debug complete batches; self-translate to Rust together,
  then verify lossless round trips and parity. Unsupported translation requires improving the translator,
  never handwritten replacement semantics. Follow
  [self-translation](docs/self-translation.md).
- Generate grammars automatically across the catalog; adjustments are recorded
  rules, never generated-file edits. Tree-sitter serves only as a test oracle.
- Extend behavior through decorators stored as shared Links data.
- Keep runtimes observably equivalent; update `parity/language-features.lino`.
- Use unabbreviated English noun phrases for canonical concepts, verbs for
  operations. Register names in `parity/naming/canonical-concepts.lino`;
  `npm run check:naming` validates WordNet 3.1, registered terms/abbreviations,
  source names and positive/negative fixtures.
- Put experiments in `experiments/`, demonstrations in `examples/`, user-visible
  changelog fragments in `rust/changelog.d/`.
- Bulk work: commit steps, review diffs, push once per batch after previous CI finishes
  and its errors are addressed. Keep working during CI. Rust jobs run only after JavaScript passes;
  jobs report every failure (`cargo test --no-fail-fast`, checks unless cancelled).

## Local checks and resources

Current task phase: draft every requirement before tests or validation gates.
Source generation may parse, type-check and format; defer behavioral execution.
Commits/pushes are authorized; defer CI feedback until the full code draft.

Follow [resource limits](docs/vision.md#resource-limits); use the shared checkout
and release completed task-owned worktrees. Only targeted JavaScript tests
(`cd js && node --test tests/<area>*.test.js`) and relevant `check:<name>` scripts
run locally. Rust: `rustfmt --edition 2024 --check`, with
`--config skip_children=true` for module roots. **Never compile Rust locally.**

The [CI workflow](.github/workflows/ci.yml) runs native compilation, tests and
runtime parity, full suites, `npm run acceptance:issue-195`,
`js/scripts/run-issue-195-evidence.mjs`, native Lean, Rocq and Rust translation
matrices, clean-consumer installs, downstream workload clones, `cargo llvm-cov`,
and upstream-corpus/whole grammar registry experiments. Never run these locally.
Read CI logs/artifacts instead.

Wrap long checks with `node scripts/with-cache-cleanup.mjs --event test -- <command>`.
After batches run `node scripts/clean-caches.mjs`. For local cleanup/wrapped
checks, unset inherited `CARGO_TARGET_DIR`/`CARGO_BUILD_TARGET_DIR` unless their
resolved paths are verified task-owned; cache markers alone prove nothing.
Pass `--project-owned-only --no-preflight --no-docker`; retain receipts.

Remove only project-owned regenerable scratch/stopped workload clones.
Preserve logs, verification records, `issue-195-artifacts/`, and
`issue-195-results/` outside disposable `work/`. Never remove dirty worktrees,
uncommitted work, active tasks or unrelated data. Stop only leftover processes
owned by the failed task.
