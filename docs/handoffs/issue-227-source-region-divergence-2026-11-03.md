# Source-region divergence draft

This follows [iteration-local scopes](issue-227-finite-iteration-local-scopes-2026-11-03.md)
and implements the structured abort part of its remaining completion work under
the sole architecture in [vision](../vision.md). No represented code, tests,
generation, formatting, validation gates, compilation or CI-feedback review ran.
The full drafting goal remains active, including all other handoff obligations.

## Actual source paths drafted

`rust-source-region-divergence.js` defines private proof control flow raised only
when the visitor reaches an actual checked abort expression. It has no value,
region, ownership state or serialized witness. It cannot substitute for a source
signature or justify an otherwise unsupported operation. Other checker errors
propagate unchanged.

`rust-borrowed-source-regions.js` visits all preceding operands normally. An
actual abort then ends the current path. Conditional and nominal-match joins
restore the real entry state before checking each alternative and merge only
normal outcomes. If every arm aborts, divergence propagates to its enclosing
expression/callable. Diverging function bodies need no fabricated result or
return-region check. Separate functions are still checked, and cumulative work,
loan, position and branch budgets retain charges for visited aborting paths.

Boolean and/or are interpreted through their actual lazy branch order. The right
operand is checked as a possible path, and its abort does not remove the normal
path which skips it. A preceding invalid read still fails even on a syntactically
possible branch whose particular fixture input short-circuits. Ordinary if is
now an explicit source operation in the shared rule; it does not grant mutable
state or packet authority in a historical readonly profile.

Compiler-control matches follow the same normal-only join. An owned fold checks
its actual symbolic body. When that body wholly aborts, only the original
zero-iteration state can reach the loop continuation: the visitor restores that
exact initial snapshot, expires symbolic item/accumulator bindings, and returns
the actual initial packet. Partly aborting bodies retain the existing repeated
capture and continuing-state invariants on all normal paths.

Finite-loop inspection recognizes the lexer’s actual supported panic macro
tokens and routes their bodies through the existing completion parser. A checked
abort is a source leaf for capture/progress inspection. An aborting path needs
no decrement; every continuing path still needs its authentic exact decrement.
There is no fuel, invented initialized state, erased source input or host panic
implementation. Existing finite admission and helper witness gates remain.

The new module is registered with the automatic source-author consumer inventory.
The shared rule decision fixture adds ordinary if without changing the separate
match-operation negative assertion. Native policies remain deferred to automatic
translation; no Rust replacement semantics were written.

## Independent controls drafted

Nine added positive originals cover readonly panic alternatives, ownership moves
on abort-only conditional and match paths, a zero-element owned fold whose body
always aborts, an abort-only finite body, finite continuing progress, and both
Boolean short-circuit operators and an actual nominal borrowed match. Their
actual executions avoid the panic path;
the structural region proof must still check every possible source alternative.
Three invalid originals preserve use-after-move before a panic, a moved normal
path beside an aborting sibling, and an invalid read before an abort in an empty
fold body. Existing inputs, assertions and ranges are retained.

Further controls require the real abort nodes to survive normal checking and
compare emitted JavaScript output. They inject an unsupported operation and a
missing actual binder before aborts to ensure divergence cannot conceal earlier
region errors. An additional finite-loop variant changes the zero counter to
one and requires the emitted program to abort, preserving the actual failure
rather than a fabricated normal continuation. These controls are declarations
only and were not executed.

Latest authored inventory: 455 positives in nineteen groups, 391 literal
originals and 64 actual-source scenarios. The mixed group has 90 positives and
30 refusals. These count inputs, not observed passes, drafted requirements or a
requirement completion percentage. Original/native and translated-product checks
remain CI-only under the existing explicit version ranges.

## Remaining full-scope work

- Complete actual borrowed capture/return ABIs for finite helpers. Structured
  abort does not establish nonlocal result loans or question-mark residual
  payload ownership. Preserve the closed-state parameter restriction until real
  reference arguments and output provenance are represented and checked.
- Complete record-place region paths, non-Copy projections, disjoint field
  loans and projected exclusive calls. The source review confirms that the
  existing guard protects a missing field-storage contract: removing it alone
  would lose the actual root revision between field extraction, call and root
  reconstruction. Draft that complete contract and independent old-alias checks.
- Complete loop initialization-state carriers, borrowed iterator/callback
  interfaces, shared collection/signature-3 composition and full own-project
  JavaScript/Rust translation. Continue constructor and all grammar/concept,
  ontology/taxonomy, RDF/OWL and dynamic inference obligations in vision.
- After all requirements have full code drafts, reconcile sources online before
  ledger access, generate pending schemas/receipts/native translations, and run
  the required JavaScript and CI-only gates. Record observations only afterward
  at the actual executed commit. No test result or completion claim exists here.
