# Standard Option and Result source closure draft

This handoff describes source changes subordinate to [the vision](../vision.md).
The policy now has an actual JavaScript source adapter and parent-owned frontend
hooks. No represented program, fixture, translator, generator, test, build or
validation gate ran. No execution evidence or completion is claimed. The online
source audit, source registration and ledger reconciliation remain deferred.

## Source changes and actual representation

`js/src/translation/rust-standard-sum-source-rules.js` owns standard owner and
constructor selection, child counts, constructor field positions, narrow
question-mark context/branch decisions and finite source budgets. Its new consumer,
`js/src/translation/rust-standard-sum-source.js`, reads actual cursors, source
catalog entries, aliases, lexical bindings and complete child origins. The adapter
calls those shared decisions and constructs existing named datatype, constructor
and match representation; it introduces no new semantic IR operation.

Actual source type and expression paths are checked against the complete local
catalog. A user declaration or local namespace named `Option`, `Result`, `std`,
`core` or a selected constructor does not become the standard owner. Standard
imports retain actual std/core paths. Source instantiation requires explicit child
arguments and direct immutable Copy facts supplied by the existing origin engine.
Every child retains its complete source owner; absent ownership refuses instead of
inventing a Unit owner. Canonical named child types resolve against actual catalog
definitions before their use in root generated datatype fields.

Each concrete sum registers an actual `k: 'data'` origin catalog entry, plus exact
`k: 'ctor'` entries with complete field origins, owner and result. A private
`standardSumSource` origin projection retains canonical owner, child origins,
actual type/constructor paths and specialization identity. Expression projections
are re-resolved through their actual named catalog entry before use; they are not
serialized proof flags. `RustSourceOrigins.typed` and `typeOrigin` now recover the
complete actual sum origin for an existing instantiated datatype.

Specialization compares complete canonical source types and owners, including
pointer-sized owners, recursively retained fields, wrappers, text and float source
models. It does not identify types merely by width or source spelling. Generated
names avoid actual source identifiers and preexisting catalog entries. Existing
sum entries survive parser catalog passes, preserving their actual names.

Generated sum datatypes always retain existing `sourceFieldSignatures`, including
when ordinary function source-owner export is not selected. This supplies actual
constructor field types and child owners through the existing graph schema. No
new source facet or codec schema is introduced. Parent-owned signature checking
connects standard library positions to these actual declarations and fields;
source-owner declarations must never be accepted merely because a generated
name resembles a standard sum name.

The representation keeps `None`, `Some(None)` and `Some(Some(value))` distinct.
`types.option` is deliberately not used: its nested optional flattening serves a
different JavaScript absence domain. The closed-empty `Option<f64>` profile and
its existing independent proof checks remain unchanged.

## Source adapter contracts

| Export | Contract |
| --- | --- |
| `initializeRustStandardSumSource` | Initialize bounded instantiation rows from actual catalog data entries; reserve their generated names across parser passes. |
| `readRustStandardSumSourceType` | Consume explicit Option/Result child types, preserve actual child owners and install the complete instantiated type origin. |
| `readRustStandardSumSourceConstructor` | Read actual source constructor arguments under expected child contexts; emit the actual constructor path and mark its full sum origin. |
| `readRustStandardSumSourcePattern` | Read a private resolved-family constructor pattern before or after its scrutinee; actual origin extraction resolves its concrete datatype path before binding fields. |
| `resolveRustStandardSumSourcePattern` | Normalize the same private pattern object into an existing constructor node against its actual catalog sum and complete child origins; nested extraction follows actual fields. |
| `refuseRustStandardSumSourcePatterns` | Refuse any unresolved private pattern at final source admission before core checking/emission. |
| `readRustStandardSumSourceImport` | Preserve checked std/core import paths through the existing import resolver. |
| `withRustStandardSumSourceContext` | Restore the preceding expected-origin slot with `finally`, including nested constructor contexts. |
| `readRustStandardSumSourceOrigin` | Resolve the actual named catalog datatype before returning its complete sum origin. |
| `readRustStandardSumSourceIdentity` | Return complete canonical type/source-owner identity, or an empty string for an unresolved type. |
| `finishRustStandardSumSource` | Emit complete existing datatype declarations and field-signature facets for every instantiated sum. |

Parent-owned `rust.js` hooks supply expected contexts for explicit declarations,
callable results, actual call parameters and match scrutinees. An unrelated
unannotated initializer clears the enclosing result context. Constructor field
contexts are passed recursively. Source expression construction supports contextual
None/Some/Ok/Err; only Some can infer its complete family from a proven payload
when no expected sum exists. Bare None and incomplete Result construction refuse.
Explicit constructor turbofish forms supply complete child contexts:
`Some::<T>`, `None::<T>`, `Ok::<T,E>`, `Err::<T,E>` and
`Option/Result::<...>::Variant`, including qualified/imported owner spellings.
The existing type reader partitions nested generic closing tokens. Actual core
type unification checks explicit arguments against surrounding source contracts.

If-let and let-else read a pattern before its scrutinee. A private source pattern
therefore retains actual canonical family/variant syntax until
`RustSourceOrigins.pattern` sees the actual scrutinee. It rewrites the same node
into its concrete constructor path before extracting payload bindings, including
nested patterns and aliases. Only initial collection may provisionally extract
unknown bindings for a forward scrutinee; it sets a final-admission requirement,
and final unresolved patterns refuse instead of returning the provisional surface.

The sibling completion source draft owns postfix question-mark markers and their
actual lowering into ordinary callable completion trees. It uses the adapter's
actual constructor paths and payload/error origins. The parent sets and restores
an independent callable return-origin slot, including closure boundaries. This
handoff does not claim those sibling changes have passed any control.

## Matching and propagation obligations

Standard sum patterns continue through `RustParser.match`, existing origin scopes,
`Checker.compileMatch`, `compileRows` and `normalisePattern`. Their real complete
constructor inventories must drive exhaustiveness. Guarded matches keep the
existing independent coverage match built from unguarded source arms. A generated
packet, implementation wildcard or branch guard cannot manufacture source
coverage. Missing Some/None/Ok/Err and guarded-only coverage remain refusals.

The selected question-mark policy permits an ordinary callable returning the same
standard sum family and an actual direct Copy operand. Option may propagate
absence into a different successful result type. Result requires identical
complete error types; a valid Rust conversion such as u8 error to u16 remains
explicitly unsupported until its actual From/FromResidual model is integrated.

The operand must evaluate once. Success projects its actual field into the
continuation. Failure exits the nearest callable with its actual absence/error
payload. A local expression conditional cannot substitute for that return.
Initial direct let/tail/return integration must retain explicit refusals for
unimplemented contextual positions. Later arguments, branches and nonselected
callbacks must not run after propagation. Nested closure propagation must not
return from the outer function.

## Deferred literal controls and remaining source work

`js/tests/fixtures/rust-standard-sum-source-rules.mjs` contains independent literal
projection controls, complete original Rust programs with declared stdout/source
ranges and refusal inputs. It imports and invokes nothing. Checked-identity labels
in the decision controls are projection examples, not source certificates. The
sibling integration controls exercise the real parser/checker/typed graph paths
when a future authorized phase executes them.

The declared original range is `>=1.90.0 <1.100.0`; this authoring phase has not
independently checked it against compilers. Existing TryFrom Debug still has its
separate pre/post-1.97 interpretations. Source std must come from the preserved
source selector rather than the eventual target compiler. Uncovered versions
remain explicit.

Remaining source work includes complete generic aliases,
unconstrained whole-expression inference, non-Copy/borrow/drop behavior, custom Try/FromResidual,
error conversion, arbitrary contextual question-mark and full standard sum
methods. Stored TryFrom, checked-arithmetic and lookup results still need actual
typed producer integration without changing existing guarded arithmetic, immediate
consumer effects or versioned panic messages. Debug strings cannot substitute for
an actual error payload.

Future controls must cover all admitted constructors/children, duplicate and nested
instantiations, transparent aliases, actual namespace shadows, incompatible owners
and child arities, source payload effects, closure boundaries, all success/failure
branches, exact source order and original coverage refusal cases. Saved-graph edits
must change/delete actual constructor fields, owners and source facts and still
reach full semantic rechecking through both codecs and all four emitters. No
serialized claimed/proven field may bypass those checks.

After the complete source batch is drafted, translate complete shared JavaScript
rule closures automatically into native definitions and typed Links, and keep
native adapters as syntax/origin/term transport. Do not handwrite replacement Rust
semantics or edit generated output. Reconcile naming, parity, source registration
and ledger obligations after the mandated audit. This source draft alone supplies
no native equivalence, runtime observations, whole-language coverage or completion.
