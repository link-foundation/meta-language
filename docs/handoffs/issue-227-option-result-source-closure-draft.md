# Standard Option and Result source closure draft

This handoff records an unintegrated JavaScript source policy and independent
literal controls. It is subordinate to [the vision](../vision.md), not an
architecture specification or a completion report. No represented program,
fixture, translator, generator, test, build or validation gate ran. No ledger,
source register, generated native module or generated Links asset was read or
changed. The source audit and subsequent reconciliation remain required before
using the ledger or registering this batch.

## Drafted source

`js/src/translation/rust-standard-sum-source-rules.js` defines source decisions
for proved standard owned Option and Result definitions, explicit type-argument
counts, exact constructor inventories/field positions and narrow question-mark
completion branch roles. It uses scalar/array operations in the style of existing
shared JavaScript rule modules. It has not been checked for self-translation.

`js/tests/fixtures/rust-standard-sum-source-rules.mjs` contains independent literal
decision inputs/expectations, complete original Rust sources with expected stdout,
and refusal inputs. It imports and invokes nothing. A future fixture author must
store these controls as shared Links data without executing represented sources.
The literal checked-identity labels in decision controls are projection examples,
not certificates that any real source type has that identity.

The module is deliberately not imported by a production adapter yet. Its callers
must derive standard-definition membership, full payload type identity and Copy
facts from the actual complete source catalog/origins. Passing `true`, comparing
names or comparing widths cannot establish those facts. The current API supplies
no proof objects; integration must preserve and recheck the underlying structures
in semantic Links rather than trusting those projected parameters on reload.

## Existing seams and blockers

| Source | Actual seam | Required continuation |
| --- | --- | --- |
| `js/src/translation/rust.js` | `RustParser.type` recognizes only the separate closed-empty Option profile before generic-type refusal. `RustParser.primary` has no ordinary standard sum construction path. | Read the actual resolved library type and explicit children; instantiate the existing named datatype representation, with constructor ownership and source ranges. Pass checked children through declarations, arguments and results. |
| `js/src/translation/types.js` | `option` recursively collapses an optional element into its absence domain. | Preserve ordinary Rust `None`, `Some(None)` and `Some(Some(value))` as distinct named datatype values. Reusing the JavaScript optional helper would lose a Rust distinction. Preserve the existing closed-empty interpretation unchanged. |
| `js/src/translation/rust-primitive-try-from.js` | `readRustPrimitiveTryFrom` consumes `.` and an admitted consumer immediately after conversion; `planRustPrimitiveTryFromConsumer` supplies existing guarded common terms. | Retain every existing immediate consumer. A stored Result must capture the input once, keep successful casts behind valid range guards and construct an actual typed error value instead of only its formatted Debug text. |
| `js/src/translation/rust.js` | `RustParser.postfix` unconditionally refuses `?`. | Attach propagation to the enclosing callable's actual result origin and completion tree before interpreting the continuation; an expression-only local `if` cannot represent early return. |
| `js/src/translation/rust-ordinary-completions.js` | `readRustOrdinaryFunctionCompletion`, private `sequence` and `lower` build private completion trees and lower return/continuation packets. | Extend the shared construction policy for question-mark inside admitted expression contexts; preserve the nearest callable boundary, evaluation order and single continuation occurrence. Do not add a handwritten native return implementation. |
| `js/src/translation/rust-source-origins.js` | `RustSourceOrigins.origin`, `cases`, `pattern`, `join`, `bind`, `argument`, owned-binding operations. | Carry complete instantiated child origins, constructor owner and structural source identity; preserve local owners and consumption on each branch. Copy-only admission does not prove arbitrary owned/borrowed payload flow. |
| `js/src/translation/rust.js` | `RustParser.match`, `pattern` and the guarded-arm route. | Resolve actual standard constructors through the same origin catalog, including qualified imports/aliases, and reuse existing arm scopes. Same-spelled user enums retain their own owners. |
| `js/src/translation/rust-conditional-chains.js` | `lowerRustGuardedMatch` preserves an independent coverage match built from unguarded arms. | Keep that match. A newly inserted question/continuation packet or an implementation wildcard must never make a guarded-only source constructor exhaustive. |
| `js/src/translation/check.js` | `Checker.compileMatch`, `compileRows`, `normalisePattern` already compile named datatype patterns and enumerate all constructors. | Supply complete actual standard sum declarations; let missing `Some`, `None`, `Ok` or `Err` reach existing exhaustiveness checks. Do not replace them with a bespoke Boolean presence check. |

The closed-empty profile in `rust-empty-options.js` and its proof checker remains
an independent source interpretation. It currently admits complete flat,
zero-argument `Option<f64>` leaf functions with an exact `None` body. Its proof
does not authorize ordinary Some values, generics, parameters or propagation.

## Coherent initial integration boundary

The initial source policy admits actual canonical std/core Option or Result
owners with explicit, direct immutable Copy payload origins. Nested admitted
sums must remain child datatypes and retain independently checked Copy facts.
Do not infer Copy merely because a user datatype has scalar-looking fields.
Constructor fields use actual argument zero for Some/Ok, argument one for Err,
and no field for None. Existing source identities must keep different
instantiations distinct; this draft does not invent a name mangling scheme.

For question-mark, the initial boundary is an ordinary enclosing callable of
the same standard sum family. Option may propagate absence into a different
payload result type. Result requires identical complete error types; its
successful payload and enclosing successful result may differ. Error conversion,
custom Try/FromResidual, borrow adjustment, non-Copy moves and destructors need
their full independent models. The valid Rust u8-to-u16 error conversion input
is retained explicitly as a future unsupported-profile control, not mislabeled
as an invalid Rust source.

Question-mark must capture its operand once, match the complete constructor
inventory and continue with the successful field only in the success branch.
Failure returns the original absence or error payload through the enclosing
callable's return packet. Nested call/argument/conditional expressions require
completion-aware ordered captures: later arguments and surrounding continuation
must not run after propagation. Begin with complete let-initializer and tail
contexts, and preserve explicit refusal for unimplemented contexts until their
shared expression-completion rules exist. In particular, a closure's question
mark must not return from its outer function.

This boundary is a source draft selection only. It does not narrow any wider
Option/Result, ownership, generic trait, matching, project or own-source obligation.

## Next source-authoring steps

1. Add a bounded shared source-type instantiation policy and real catalog/origin
   adapters. Register actual standard sum declarations with existing named
   datatype concepts and preserve owner/child/source-coordinate provenance.
   Reject missing, extra, inferred or unresolved children in this initial profile.
2. Integrate constructor expressions and patterns together. Cover explicit
   annotations, parameters, return positions, local bindings, nested instantiations,
   aliases/imports and same-spelled local owners before exposing escaping sums.
3. Extend actual ordinary completion source rules and JavaScript adapters for
   selected question-mark contexts. Bind actual operand values and payloads;
   consume the new branch dispositions only after the complete context check.
4. Extend the shared TryFrom/checked-integer/lookup producers to construct typed
   stored sums without changing existing immediate-consumer effects, range
   guards, lazy arithmetic or panic message/version behavior. Integer conversion
   error types need a source-version-aware value model; strings used for Debug
   cannot stand in for those values.
5. Author deferred controls for every projection admission/refusal, every
   constructor, both success/failure branches and every original fixture. Preserve
   all previous originals/oracles and add an explicit input for each uncovered
   source context rather than assuming the existing controls imply coverage.
6. After the complete batch is drafted, automatically self-translate complete
   rule closures into native rules and shared typed Links with the existing
   authoring workflow. Extend native adapters only as actual syntax/origin/term
   transport. Never handwrite replacement Rust semantics or edit generated output.
7. Reconcile the source register, naming inventory, parity feature inventory and
   ledger after the mandated online source audit. Generation, tests and runtime
   observations remain deferred to the parent task's authorized later phase.

## Deferred independent controls still needed

The literal original sources already retain nested absence, success/error payload
values, before/after propagation traces and guarded coverage with a real fallback.
The following controls must be authored before an integrated batch is reviewable:

- Both families with every admitted primitive/Unit child, two distinct Result
  child types, nested Option/Result combinations and complete concrete source
  reconstruction. None cannot be encoded as Some(Unit) or JavaScript undefined.
- Wrong constructor family/field count, missing/extra type arguments, local
  `Option`/`Result`/`Some`/`Ok` owners, imported standard aliases, source namespace
  shadows and forged catalog/child provenance. Projection booleans alone must
  never admit a forged saved graph.
- A counted operand function proving one evaluation; a failing operand before a
  later argument proving that argument is skipped; left-to-right ordering of
  nested propagation, all nonselected callbacks and panic-versus-return order.
- Actual callable boundary controls, including closures, conditional and match
  branches, distinct successful result types, direct return and tail positions.
  Unsupported source syntax remains an explicit refusal until integrated.
- Guarded-only Some/Ok branches with remaining constructor arms must remain
  non-exhaustive; a wildcard fallback must be checked against the original source
  datatype. Failed guards continue in original arm/alternative order.
- Source std selections from the existing canonical 1.90–1.99 range. The general
  original fixture range is declared as `>=1.90.0 <1.100.0`; it has not been
  independently verified in this authoring phase. TryFrom Debug remains bounded
  by its separate pre/post-1.97 policies. Never infer source std from the target
  compiler, and report uncovered versions explicitly.
- Both Links codecs, editable graph rechecking, all four emissions and matching
  actual native/JavaScript semantics at the recorded commit. Mutate constructor
  owner, child identity, field arity and callable return context in saved graphs;
  preserve refusal without trusting a serialized checked/proven flag.

No deferred control above is an observed result. The added policy alone does
not admit any new Rust program through the public parser.
