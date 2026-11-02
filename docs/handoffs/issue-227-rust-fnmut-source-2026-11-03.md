# Rust closed source FnMut specialization draft — 2026-11-03

This is an unexecuted JavaScript source draft, not runtime evidence or a complete Rust generic frontend. No tests, source generation, imports, validation gates or native compilation were run for this batch. Source-register reconciliation, automatic source authoring, native frontend parity and observations remain separate work.

The explicit public parser option is `fnMutSourceModel: 'Rust closed source FnMut specializations 1'`, together with `exclusiveSourceModel: 'Rust exclusive source loans 1'` and an actual canonical `sourceVersion` in Rust >=1.90.0 <1.100.0. Typed source capture forwards `rustFnMutSourceModel`; project parsing forwards the unprefixed option through the existing real project composer. No module-composition replacement was introduced.

## Actual source admission

`rust-fnmut-source-declarations.js` captures original fn/header/body token ranges and concrete type-position bindings. Only the normal type reader interprets a generic type binder; source value identifiers are never rewritten. A selected call previews the actual original header, reads the actual immediate callback, then reparses the complete original function body through the existing parser, ownership checking and source-state interpreter. The three source passes retain actual template inventories and force final admission. An unused/uninstantiated generic definition is refused, including an ill-typed unused body; it is never silently dropped as an accepted Rust definition.

Initial callbacks use one final source operand, `mut map: impl FnMut(...) -> ...` or `map: &mut impl FnMut(...) -> ...`, with immediate non-move closures. Whole immutable `&self` methods and direct source functions are selected from real declaration ownership. Actual concrete turbofish arguments or a genuine expected standard Result error position select concrete arguments. Arbitrary generic forwarding, stored/dynamic callbacks, move environments, callback alias escape, mutable-reference callback operands, generic where predicates and non-final callback operands remain refused.

`rust-fnmut-source-environments.js` uses actual mutable caller slots and binder token identities, actual state read/write/call inventories and the established effect lowerer. Capture state is appended as explicit callback formals; each completion returns `(original_result, updated_capture_1, ...)`. Each selected outer function likewise returns `(original_result, every_exclusive_state...)`, including a one-element tuple when no state is captured. Existing exclusive call packets acquire real lexical loans, preserve left-to-right call order and install caller writeback before continuation. Nested state-block slot snapshots retain capture identity by lexical/root/callable equality. Bare aliases of a callback formal cannot expose the private expanded ABI.

`rust-fnmut-source-iterators.js` admits a contiguous actual standard Vec `.iter().map(&mut map).collect::<Result<_, E>>()` chain, or a mapped closure with its real admitted result interface. Its ordinary checked fold carries the accumulated Result plus all callback states. Once an Err appears, later fold elements produce no callback calls or writes. A failure publishes preceding writes and the first actual error. No floating bounds, host callback execution, native dispatch table or source-name replacement body is used. The mapper owns its computation.

Readonly helpers with no exclusive formals retain the actual `sourceSignature` model 2 after the same real body/state lowering. Existing immutable and exclusive profiles are not relabelled. Whole closed Vec state is admitted only under this explicit FnMut profile using actual container ownership facts; recursive whole values do not grant indexed, Box-field or arbitrary alias mutation authority. Genuine derived Clone on a borrowed nominal value supports `other => other.clone()` without manufacturing Clone evidence.

## Opaque error boundary

An unconstrained original generic error binder remains opaque while its selected body is checked, even when the caller selects a concrete Copy/Clone integer. The initial universal scope permits that binder only as a bare standard Result error argument. Argument, binding, return and residual transport preserve its original declaration/binder identity; source ownership is affine independently of the selected concrete type. Concrete callback bodies are checked with concrete positions, while the original generic body retains opacity. Canonical sum caches store clean concrete origins; opacity is reapplied on actual source positions and erased when the completed specialization returns to its concrete caller.

Result discriminant queries are permitted. Other generic observations, Clone, Display, equality, concrete coercions, duplicated opaque Result use, explicit generic Err extraction/construction and inferred vec! aggregation remain refused until their universal source interpretation is drafted. These restrictions are deliberate bounded admission, not claims of general Rust trait checking. `rust/src/translation/types.rs` still includes independent serde/trait generic helpers; the whole real file is not claimed supported by this batch. The recursive Type tuple method original is a complete standalone subset of the actual `map_children` body, not proof for every arm or every own-code module.

## Checked interface

`checkRustFnMutSourceSpecialization(facet, parameters, result, language, resolveType, resolveData)` is exported from `rust-fnmut-source-signatures.js` and called by the ordinary checker. It normalizes actual source positions with the existing source-signature checker. The independent `sourceFnMutSpecialization` facet retains:

- `model`, `language`, `sourceVersion`;
- `declaration: {path, range, bodyRange, generics, formalCount, callback}` naming the original generic source definition, not a fictitious emitted declaration link;
- ordered `typeArguments: [{name, position}]`;
- `callback: {parameter, parameters, result, captures: [{parameter, binder: {name, identity, start, end}, position}]}`;
- expanded `parameters: [{mode: 'value'|'shared'|'exclusive'|'callback', position}]`, original `result`, and ordered exclusive parameter `packet` indices.

The declaration callback retains `{index,name,mode,mutable,binder,typeRange,parameterRanges,resultRange}`. Original declaration names/ranges are provenance; copying a facet is not independent body authentication. The actual source parser/controller owns body correspondence. Checked projection, graph traversal, schema and public option forwarding are root-owned integrations; no generated schema/native parser edits belong to this source batch.

## Companion source hooks and controls

This parser/checker batch includes the ready root-owned stable UTF16 field-sort and literal String character push/insert readers, the companion primitive Vec.sort reader/checker, and the real ordered-loop reader/private-marker gate. The actual no-escape owned Vec `.into_iter()` callback is used only in the authenticated sole loop head. No general iterator object is admitted.

Whole assignment after an owned Vec move now validates the actual RHS and ownership transfer, actual current callable and absence of live overlapping loans, then restores the whole root and retires descendant moved facts. Field writes and method mutations still require an available owner. This does not revive a moved parent through a field assignment.

`rust-fnmut-source.mjs` exports `rustFnMutSourceVersions`, `rustFnMutSourceOriginals` (18 complete originals, each expected stdout `true\n`) and `rustFnMutSourceRefusals` (16 independent retained inputs with native compiler expectation). Native source bounds are Rust >=1.90.0 <1.100.0, edition 2024; deferred JavaScript behavior bounds are Node >=22.15 <23 || >=24 <25. The controls draft actual parser/checker/emitter behavior, public typed Links transport, textual/binary storage, checked facet corruption, source owner identity, source versions, first-error ordering, canonical sum-cache separation and whole moved-root reinitialization. They have not been run.
