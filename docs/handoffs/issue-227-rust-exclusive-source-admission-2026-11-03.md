# Exclusive Rust source text admission draft

This batch extends the preceding [loan/packet kernel draft](issue-227-rust-exclusive-source-loans-2026-11-02.md)
through actual Rust source reading and ordinary checked bodies. It remains an
unexecuted source draft. No tests, represented imports, generators, validation
gates, formatting or native compilation ran. No Rust implementation was
handwritten. The source register and requirement ledger remain unread and
unchanged; none of these declarations establishes execution or completion.

## Public selection and original interfaces

The additive entry is
`parseRust(source,{sourceVersion:'1.90.0',exclusiveSourceModel:'Rust exclusive source loans 1'})`.
`parseRustTokenStream` receives the same explicit option. Coverage is Rust
>=1.90.0 <1.100.0. Existing options and refusals retain their behavior when the
new model is absent. Compositions with previous signature, loop, runtime-shell,
shared-handle and other independent source profiles explicitly refuse.

An exclusive parameter type is admitted only while reading a real formal.
Stored and returned mutable references, mutable slices/str, nested reference
layers, indexed place aliases, named mutable regions and Box storage retain
explicit boundaries. Ordinary `&mut self` belongs to its actual declared
inherent owner. `mut self` and arbitrary explicit-self interpretations were not
added. The original mutable type/expression/receiver guards remain outside the
new profile.

Each source function retains a separate `sourceExclusiveSignature`:

`{model,language,sourceVersion,parameters:[{mode,position}],result,packet}`.

Modes are `value`, `shared` and `exclusive`. The packet contains exactly the
exclusive formal indices in source order. With exclusive formals, the actual
lowered return type is a tuple containing the original result at zero and each
updated referent afterward. Without exclusive formals it retains its original
result type. Referent positions are owned positions, while canonical shared
`&str` retains its genuine reference wrapper/null lifetime and primitive str
owner. This narrow text case does not admit stored or escaping references.

`createRustExclusiveSourceSignature(parser,parameters,result)` authors that
interface from actual parsed origins. `checkRustExclusiveSourceSignature`
accepts `(signature,parameters,result,language,resolveType,resolveData)`, checks
the explicit canonical selected version, validates complete type/owner positions
using existing signature-position checking, verifies exact packet slots and
traverses complete closed referent storage. It returns normalized checked
positions; it never relabels `sourceSignature`. `check.js` retains the new facet
separately and refuses mixed old/new interfaces.

## Actual source slots, calls and writeback

`rust-exclusive-source-parameters.js` selects admitted formal modes and creates
an actual function state frame. `registerRustExclusiveSourceStateRoot` in the
existing state module registers actual source lexical/root identities and
referent origins. Collecting sites are private discovery state; the model forces
the complete final pass before publication. Mutable reference bindings cannot
become owned mutable referents. Exclusive header mode is removed from copied
carrier values, so a detached copied scalar cannot masquerade as an exclusive
formal. Actual shared source loans still prevent conflicting mutation.

`rust-exclusive-source-calls.js` reads explicit `&mut` places and nonescaping
reborrows only within an actual selected exclusive Call. Its source-place
records retain real slots, constructor/field layouts and exact field indices.
Argument acquisition checks overlap and source availability before later
operands. Earlier exclusive operands prevent overlapping later reads/writes;
distinct fields may acquire separate exclusive loans. Implicit reborrow is
limited to actual exclusive parameter bindings. Receiver acquisition happens
before the actual method arguments. Temporary roots, stored aliases and
two-phase receiver borrowing retain conservative separate boundaries.

Private source Call markers hold the actual callee and full ordered argument
list. `rust-owned-record-effects.js` consumes those markers through its existing
ordered state algebra. It holds each actual argument, Calls the adapted function
once, and reconstructs caller roots from the returned packet before the next
expression. Multiple disjoint fields of one root reconstruct against the latest
state, preserving every completed write. Whole scalar dereference writes and
whole Vec append use the same actual state slots. Vec append is ordinary array
accumulation over held values, with no host Vec body execution.

`lowerRustExclusiveSourceParsedCallable(parser,frame,declaration)` in the existing
exclusive call adapter invokes the actual exported
`lowerRustOwnedRecordEffectBody` over the original parsed body, real declaration
map and real initial parameter slots. The established state/completion lowerer
owns branch order, early completion and ordinary field reconstruction. The
adapter packs each actual surviving exclusive parameter on completion. Empty
mutators also use a genuine initialized state arena. Source reading performs its
original move/borrow checks before this value-state adaptation.

Every collecting and final reader consumes or explicitly refuses private
exclusive operations. Header discovery retains actual mode/origin information;
the syntax-check fallback cannot publish an unproved collecting exclusive
program. The final ordinary checker validates real packet declarations and
their callers. No new native cell runtime, handwritten Rust counterpart or
native fallback implements these source bodies.

## Standard collection source hook coordination

The separate collection agent owns the actual collection kernel, projected
library interface and source helper modules. This batch wires the parser options
`standardCollectionSourceModel`, `standardCollectionSourceLibrary`, and
`standardCollectionPointerWidth` into all three source passes. The current source
profile is `Rust closed standard collections 1` with an actual supplied checked
library graph and explicit 64-bit target; the standalone kernel separately
retains its 32/64-bit domains. Existing source profiles, including exclusive
source text, do not silently compose with it.

Hooks cover actual standard imports, concrete type arguments, selected
constructors, eager method dispatch before nominal clone handling and complete
library/wrapper publication. Type registries derive from the actual complete
header pass and remain available to forward calls. Source argument, structural
comparison and branch join hooks preserve exact collection domain identities
even when different domains share one semantic carrier type.

Actual library implementation names are private to the projection. Token paths,
imports and source value-name reads reject references to those implementation
entries; helper-authored AST calls retain their independently validated actual
definitions. Real bound local expression names remain ordinary lexical names.
Whole collection slots require the helper's actual registered type/library
storage proof; no nominal/Vec alias or marker Boolean grants mutation authority.

`recordRustExclusiveSourcePacketEffect(parser,callee,args,parameters,result,borrows,span)`
is the shared source-state seam for real projected helper bodies. `borrows`
retains actual slots, field layouts and referent origins; whole carriers use
empty layouts. The collection helper validates its actual packet declaration,
body and underlying carrier before using that seam. Source &mut collection
composition still requires the combined source interface/storage proof and
remains explicitly refused by these initial profile selectors.

## Deferred controls and delivery boundaries

`rust-exclusive-source-admission.mjs` retains twelve complete positive Rust
programs and twelve compiler-classified refusal originals. The original bare
reference-field input is preserved and classified compiler rejection because
a struct field needs an actual source lifetime; an additional valid static
reference-field original independently covers the excluded storage family.
Neither classification is a compiler observation. Positive declarations
cover a real closed-record mutator call, scalar dereference mutation, whole Vec
accumulation, Scope::fresh-shaped methods, explicit/implicit reborrow, disjoint
fields, source evaluation order, nested argument effects, early completion and
canonical shared str. These are not the full actual lifetime-parameterized
`Scope` declaration: its stored callback, generic owner and standard collection
composition remain larger own-source obligations.

The deferred admission test declares actual parse/check, literal stdout,
complete physical CST/source text/binary reconstruction, retained packet facets,
typed Links text/binary reload, four-target emission and forged-interface
controls. The exact original mutable-reference refusal is asserted again without
selecting the new model. Rust versions 1.90.0 and 1.99.0 and Node
>=22.15 <23 || >=24 <25 are explicit expected control scopes. No original or
assertion has executed; compiler acceptance metadata is an independent planned
oracle, not an observation.

The existing `rust-source-project.js` already composes actual supplied out-of-line
modules and #[path] files. Its single-file parser's out-of-line refusal is not
absence of a project linker. The root batch forwards these new semantic options
through that existing composer and retains actual source/CST ownership; this
batch adds no competing linker.

Root-owned delivery work remains: automatic complete frontend/library source
authoring and registrations, additive schema/codec products, the generated
frontend's real native public parser bridge/ParseOptions, native controls and
all deferred execution. Registering a policy library alone does not update the
existing native Rust frontend. Exact source-header ABI emission, escaping
references, general indexed aliases, custom destruction/unwinding, arbitrary
trait dispatch, generic callable environments and unproved profile composition
retain independent obligations. No closure directive or parity claim follows
from this source draft.
