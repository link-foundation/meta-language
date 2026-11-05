# Completed match and loop residual source handoff

This describes an unexecuted implementation draft. The architecture remains
[the vision](../vision.md). No test result, compiler observation, parity result,
or completion claim is recorded here.

The JavaScript ordinary completion reader now selects direct match expressions
containing actual completion tokens. It retains actual patterns, bindings,
per-arm ownership snapshots and completion trees. Its native transport is
`rust/ordinary_completions/matches.rs`. Lowering uses the existing checked Core
match operation, which still must establish coverage and exhaustiveness.
Selected unguarded arms can return, break or continue, including owned results;
the ordinary and finite-loop traversals retain their distinct exit paths.

Question-mark source attempts retain their actual post-operand ownership and
temporary frame before creating the successful payload. These frames stay in
the parser arena, outside copied expression trees. The existing standard-sum
and primitive-From residual constructor lowerer restores that failure frame,
records the genuinely produced residual origin and exit state, and restores
its caller. JavaScript loop tracing reads those actual records after lowering.
Native tracing first creates explicitly pending ordinal/count descriptors,
then resolves them only after lowering has produced the corresponding records.
Pending descriptors do not certify a producer, Copy trait or successful source
admission. Ordered operands and short circuit paths keep their own progress
counts, so an early residual can return without a successful iteration's
decrement. Continuing paths still require exactly one authentic decrement.

The new independent deferred inventories are:

| Fixture | Decisions | Original programs | Refusals |
| --- | ---: | ---: | ---: |
| `rust-ordinary-match-completions.mjs` | 14 | 5 | 3 |
| `rust-owned-record-loop-residuals.mjs` | 4 | 7 | 3 |

Their JavaScript consumers are `translation-rust-ordinary-match-completions.test.js`
and `translation-rust-owned-record-loop-residuals.test.js`. They declare future
frontend, checker, exact concrete syntax transport, both typed codecs, four
emissions and literal stdout assertions. Original compiler expectations remain
independent future endpoint oracles for Rust 1.90.0 and 1.99.0; the source range
is `>=1.90.0 <1.100.0`. No assertion was executed for this handoff.

Remaining source boundaries:

- Guarded completed match arms explicitly refuse until selected guard-failure
  continuations and their ordered effects have a complete path derivation.
  Ordinary guarded matches without these selected completions keep their
  previous reader. The corresponding valid Rust refusal input remains intact.
- Match selection scans the first balanced brace body. Block-valued scrutinees
  need a proper expression-boundary selector before claiming that source form.
- Loop tracing still refuses conditional-chain packets, callable effects and
  quantified/proposition operand exits requiring their separate derivations.
- The existing finite-loop profile remains exact direct `u8` record counters,
  pure nonzero guards, actual continuing decrements, bounded depth and static
  helper graphs. General scalar/range/iterator loops, labelled targets, break
  values, mutable captures and custom Drop remain separate work.
- Actual behavioral validation, automatic native policy generation, native
  compilation and endpoint compiler/runtime comparisons remain deferred.

All earlier source inputs and assertions were retained. The new files are
source contracts and consumers, not replacements for evidence at a recorded
commit.
