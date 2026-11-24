# Vec mutation helper interfaces source draft

This continues [projected intrinsic replacements](issue-227-projected-intrinsic-replacements-2026-11-03.md)
under the sole architecture in [vision](../vision.md). No represented program,
test, generation, formatter, validation gate, compilation or CI-feedback review
ran. This is source drafting and review; the complete issue goal remains active.

## Actual compiler/body contract drafted

`rust-vector-mutation-interfaces.js` defines `Rust vector source mutation helper 1`
on generated helper declarations as `sourceVectorMutation`. The interface retains
the actual pop/truncate/clear operation, ordered standard Vec/usize parameters,
complete closed element and result positions, and original call span. Vec pop
requires the actual complete standard Option declaration for the same element;
clear/truncate require actual Unit. Truncate retains canonical usize rather than
an equal-width u64 under the existing supported 64-bit source target profile.
The compiler interface rejects competing source/closure/finite-state headers.

The body validator checks every real syntax node under an independent bounded
inspection graph. It rejects competing source qualifiers/certificates and derives
the whole actual algorithm rather than trusting its declared operation. Clear
must return Unit plus the real empty Vec. Pop must use the input Vec's initialized
reduce, None plus empty Vec initially, distinct actual accumulator/current/prior
item binders, both real Option alternatives, Some(current item), and exactly one
ordered append of the prior item to the remaining vector. Truncate must use the
actual usize input, empty initial prefix, positive-counter guard, one exact
decrement, one current-item append and the unchanged exhausted accumulator. It
then projects the actual vector from the fold and returns Unit. Arbitrary calls,
captures, rewired constructors/items, altered guards or fabricated bodies fail.

The normal checker validates the header before call bodies and validates the
actual helper body after normal type checking. The region interpreter repeats
header/body validation and checks each closed input/element/result domain,
including helpers with no remaining callers. No generic reduce body is accepted
merely because the declaration carries a marker. Proof work still has the
existing source-region limits; helper inspection has its own 100,000-node cap.

`rust-exclusive-source-vectors.js` retains the existing shared algorithms. The
pop template now directly emits its single-scrutinee match so the checked body
retains the actual accumulator projection, with the same original branches,
constructors, values and order. Only final admitted source passes attach the
interface; collecting passes retain their existing provisional declarations.
No source function header or replacement Rust behavior is handwritten.

The existing closed-call interface acquires the actual mutable vector and
returns its real result/state packet. Projected field calls reuse their complete
extraction and root-reconstruction proof, including ordinary argument evaluation
and disjoint nested mutations. Source-free declaration projection, datatype
edges, typed Links validation and the schema author preserve the full interface
and independently checked bodies. Automatic source-author inputs include the new
module. Generated assets/native policies remain deferred with all execution.

## Independent controls drafted

Thirteen new positive originals cover whole/projected pop, empty and singleton
vectors, owning String/nominal Option payloads, ordered truncate prefixes,
zero and exact usize bounds above binary64 precision, clear followed by push,
readonly result regions, disjoint bound mutations and observable bound order.
Four invalid originals retain transferred owning results, stale element loans,
u64-versus-usize and immutable receiver errors. No prior input or assertion was
removed. Native/reference drivers retain their existing explicit version and
target ranges.

Structural controls require each actual ABI and body and corrupt Option initial
state, current/prior items, input/capture binders, operation/model/result owner,
usize guard/decrement/exhausted path and interface presence. An explicitly unused
clear helper with false recursion flags must still reject a body that returns
the original Vec. Full Links text/binary controls retain all three actual helper
algorithms and compare JavaScript, Rust, Lean and Rocq emission. All controls are
unexecuted declarations, not observed evidence.

Latest authored inventory: 490 positives in nineteen groups, 426 literal
originals plus 64 actual-source scenarios. The mixed composition group has 125
positives and 42 refusals. These count inputs, not observed passes, drafted
requirement counts or a completion percentage.

## Remaining full-scope paths

1. Complete the other actual Vec/String mutation operations, including character
   and index operations, inserts/removals, iterator-derived operations and their
   real ownership/loan invalidation. Reuse actual shared source bodies with
   complete interfaces and independent body checks; do not bypass unsupported
   bodies or synthesize source headers for compiler helpers.
2. Add projected callback-specialized call contracts with actual callback bodies,
   capture binders, parameter modes and hidden state results. The projected
   selected-call contract still requires a complete noncallback closed callee.
   Immediate closed callback calls were subsequently drafted in
   [projected callback interfaces](issue-227-projected-callback-interfaces-2026-11-03.md)
   with actual explicit/hidden acquisitions and lexical capture derivation.
   General field-granular/escaping callback protocols remain open there.
3. Complete actual field-reference regions, partial moves, disjoint loans and
   reference-bearing/recursive storage. The current helpers admit only complete
   closed element domains already established by original source admission;
   this batch adds no borrowed item, Box indirection or custom destructor proof.
4. Complete finite-helper borrowed capture/result/residual interfaces, loop
   initialization-state carriers, borrowed iteration, shared collection
   composition and complete own-project JavaScript/Rust CST, emission and both
   translation directions. Preserve all constructor, grammar/concept,
   ontology/taxonomy, RDF/OWL and dynamic inference obligations in vision.
5. After every requirement has a full code draft, reconcile sources online before
   ledger access; automatically publish pending source receipts, schemas, naming
   changes and translated native policies; execute authorized JavaScript and
   CI-only gates; then record observations at the actual executed commit. Never
   compile Rust locally or count authored fixtures as verified requirements.
