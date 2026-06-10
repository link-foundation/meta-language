---
title: "Port competitor test corpora (wave 2), extend parity targets, and ratchet a coverage gate"
labels: enhancement, documentation
---

## Context

Issue #47: "We should have 100% tests coverage, which should copy most of test
cases from our competitors in each sector/scope" and "We should check that
nothing is deferred or left unimplemented in our vision and roadmap." Today 48
provenanced `PARITY_FIXTURES` exist (a small fraction of upstream suites), the
`cargo llvm-cov` job enforces no threshold, and `docs/parity-roadmap.md` still
defers SQL dialect and Delphi-specific coverage. See
[`requirements.md`](../requirements.md) **R-19**/**R-20** and
[`solution-plans.md`](../solution-plans.md) **S-15**.

**Blocked by:** `#02`, `#08`, `#12`, `#14` - this is the closing gate over the
features those issues add.

## Scope

- Port the five highest-value suites identified in
  [`competitors-code-tooling.md`](../competitors-code-tooling.md):
  Coccinelle `tests/` (`.c`/`.cocci`/`.res` transform triples), tree-sitter
  `test/corpus` + core `test/fixtures/error_corpus`, Semgrep
  `tests/patterns/<lang>/`, srcML `test/parser/testsuite` lossless round-trip
  cases, LibCST adversarial whitespace fixtures (runner-up: babel-parser
  fixtures). Sample per construct where suites are huge; document sampling so
  nothing reads as "covered everything" when it is not.
- Extend `PARITY_TARGETS` with the surveyed projects not yet tracked:
  ast-grep, Semgrep, Comby, GritQL, srcML, difftastic, Babel, SWC,
  OpenRewrite, Spoon, JavaParser, Rascal, Stratego/Spoofax, TXL, MPS,
  Coccinelle; GF, Universal Dependencies, LanguageTool from the
  natural-language survey; doublets-rs and links-notation storage gates.
- Ratchet coverage: record the current `cargo llvm-cov` line coverage, fail
  CI below the floor, raise the floor with each wave toward 100%.
- Roadmap audit (R-19): every deferral in `docs/parity-roadmap.md` is either
  implemented or tracked by an open issue with a link.

## Acceptance criteria

- [ ] Each new parity target has provenanced, license-recorded fixtures and
      capability gates, like existing targets.
- [ ] Ported corpora include both round-trip and transform-expectation cases.
- [ ] CI fails when coverage drops below the recorded floor.
- [ ] Roadmap contains no untracked deferral.
- [ ] Changelog fragment added (`bump: minor`).

## References

- Requirement: [`requirements.md`](../requirements.md) R-19, R-20
- Solution: [`solution-plans.md`](../solution-plans.md) S-15
- Part of #47; work lands on branch `issue-47-76af108c0f24` (PR #48).
