# Canonical Box record field source draft

Continues [direct record places](issue-227-record-field-place-regions-2026-11-03.md)
and [Function source text](issue-227-function-source-text-2026-11-03.md), under the
sole architecture in [vision](../vision.md). Source edits, textual review and
primary specification reads only. No tests, represented programs, generators,
formatters, validation gates, compilation or CI feedback ran or was reviewed.
The full requirements drafting goal remains active and unfinished.

Continuation: [owned Box helper calls](issue-227-owned-box-helper-calls-2026-11-03.md)
connects value/shared Box arguments and owning results to ordinary/projected
calls. Mutable Box roots, stored references and all other scope below remain.

## Actual source changes

The parser already derives field projections through actual canonical Box
parents. The source-region place attachment previously refused those parents
and owned Box fields, leaving this path without the corresponding containing
storage proof. The existing `Rust source record place 1` facet now retains those
complete original positions. Its serialized fields and preceding interpretation
remain; no hand-authored Rust semantic implementation or generated asset changes.

`rust-record-place-owner-rules.js` selects an underlying owner only through an
actual bounded shared-reference prefix followed by canonical `std::boxed::Box`
or `alloc::boxed::Box` layers. Every Box has its real library owner and one type
argument, and the number of reference layers agrees with its lifetime column.
Reference layers after a Box, unknown wrappers, missing arguments, homographs
and leftover owner layers refuse. The complete original position stays on the
facet; the peeled view never replaces source qualifiers or executable authority.

`rust-record-place-regions.js` reuses the same view during actual source
attachment and independent checked-AST validation. The actual nominal record,
single constructor, field inventory/index, complete selected field source
position and exact projection body remain mandatory. Box fields keep their own
Box owners/wrappers, checked against the declared field; stored references remain
outside this direct owned-field profile. Ordinary fields retain their existing
closed-output check. An actual owned Box field uses the closed referent graph
check after canonical layer validation, without broadly loosening closed call,
mutable state or callback interfaces.

Runtime-independent source regions still evaluate the actual owner once and
derive place loans from its containing storage or actual shared referent regions.
Nested projections reuse the original regions, including a Box field inside a
shared record. An owning transfer consumes its real owner under the existing
conservative protocol; the new binding has fresh storage. A shared parent cannot
supply an owning String or Box field. Copying a scalar carries no allocation loan.
No native dereference/body dispatcher or alternate emitter path was introduced.

The primary [field expression contract](https://doc.rust-lang.org/reference/expressions/field-expr.html)
permits automatic dereferencing and specifies separate field places. The
[dereference contract](https://doc.rust-lang.org/reference/expressions/operator-expr.html#dereference-operator)
permits owning transfers from Box places. This draft supplies one needed proof
component; it does not yet prove all partial-field move or disjoint-loan behavior.

## Independent controls authored, not executed

Fourteen appended native-original programs cover scalar/String field loans through
owned/shared/nested Box, copying before moving the allocation, owned String and
Box fields transferred into fresh bindings, nested Box fields inside records,
shared Box field results, copied reference aliases, repeated shared layers with
their complete common lifetime column and loans ending before moves.
Six invalid originals cover local escaping field references, owner moves with
surviving loans, owning transfer through shared parents and fabricated static
lifetimes. Previous inputs, assertions and explicit Rust/Node/dependency version
ranges remain unchanged. These are authored input counts, not requirement counts
or observed passes.

Independent structural controls inspect exact wrapper chains and nominal owners,
forge library ownership/arguments/order/lifetimes, reject stored reference layers
after Box, and attempt to turn a borrowed Box field into an owning result. Links
text/binary reload controls retain Box field positions and all four target
emissions through the existing transport. The automatic source recipe includes
the new pure library; receipt/native/naming publication and execution remain
deferred until the complete requirements code draft.

## Remaining actual code and resumption order

1. Add precise per-layer referent regions and lifetime substitution for repeated
   references, rather than using the current conservative containing-root union.
   Model stored-reference fields and temporary lifetime extension at their actual
   Rust scopes. Retain exact source positions and independently reject forged
   per-layer loans; neither physical indirection nor a source annotation grants
   a longer lifetime.
2. Complete partial field moves, independently revised field paths and disjoint
   surviving loans. Current field moves consume the containing root and current
   loans cover it. Retain field names/indexes, whole-owner move restrictions,
   initialization state, Drop obligations and reconstruction of untouched fields.
   Add custom Deref through actual trait/body/effect authority separately from
   canonical Box ownership; do not classify homographs as library intrinsics.
3. Finish owned Box/state/call composition interfaces where they still require
   direct referents. In particular `rust-exclusive-source-signatures.js` and
   `rust-owned-record-state.js` must preserve wrapper/lifetime ownership instead
   of erasing Box layers to enter an old direct-root interface. Complete projected
   Box mutations and readonly calls with owning Box result/state carriers.
4. Complete borrowed callback interiors and forwarding environments through the
   actual region interpreter, not by removing the closed-body source-position
   guard. `rust-fnmut-source-environments.js`, `rust-closed-callback-regions.js`
   and `rust-borrowed-source-regions.js` are the next shared entry points. Keep
   creation-time capture transfers, actual explicit/hidden formal positions,
   callback-local scope expiry and result/state publication. Add field-granular,
   nested, escaping and alias callback environments; all remain outstanding.
5. Continue iterator stages/terminals and real short-circuit/drop effects,
   finite-helper borrowed captures/results/residuals and zero-iteration state;
   whole own-project default CST parsing, emission and JavaScript/Rust translation
   in both directions; mutable methods, modules, generics, traits and macros.
   Retain JavaScript dynamic Function, callbacks/async, derived/super/Proxy/private
   state and live-handle root/retention work from the preceding handoffs.
6. Draft every broader grammar/global concept/deep formal definition/equivalence,
   recursive any/all parsing, ontology/taxonomy, RDF/OWL import/conversion/merge,
   dynamic text/binary/link-cli reconstruction, grammar inference, formal systems
   and delivery requirement in vision. The immediate translator priority removes
   none of them.
7. Only after every actual code/control draft is complete, reconcile sources
   online before ledger access, publish automatic schema/source/naming/native
   assets, run authorized JavaScript and CI-only gates, repair actual failures and
   record observations at the executed commits. Never compile Rust locally.
