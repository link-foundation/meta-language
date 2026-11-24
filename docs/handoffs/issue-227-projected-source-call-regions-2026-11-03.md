# Projected source call regions draft

This continues [source-region divergence](issue-227-source-region-divergence-2026-11-03.md)
under the sole architecture in [vision](../vision.md). Only source edits and
review were performed. No represented program, test, generator, formatter,
validation gate, compilation or CI-feedback review ran. The complete issue goal
remains active, with all obligations in vision and predecessor handoffs.

## Actual compiler path drafted

`rust-source-place-transforms.js` creates an ordinary executable extraction,
call and record-reconstruction expression. Its `sourcePlaceTransform` facet
retains original root binders and complete closed positions, actual current input
variables, ordered exclusive parameters, declaration-order field paths and the
original call span. It is a compiler expression interface, not a fabricated Rust
function signature or stored ownership epoch.

The validator checks the entire real expression: each nested single-constructor
field projection; the selected callee's actual closed result/writeback ABI;
each exact returned packet slot; every unchanged sibling; the ordered chain of
reconstructed roots; and the complete final result/root packet. It rejects
overlapping places, changed paths, omitted/duplicated roots, sibling rewiring,
hidden ordinary-operand effects, duplicate pattern binders, shadowed call
packets, competing qualifiers and arbitrary extra proof fields. Actual field
inspection is bounded independently. Canonical owner comparison is structural,
not object property ordering or source spelling.

`rust-owned-record-effects.js` evaluates ordinary arguments once in their actual
source order, retaining observations before the call. Exclusive acquisition
still follows the original parser's ordered place/overlap/loan checks. Plain
closed field extraction adds no observation and can use the current root after
a later disjoint argument call: that call's own checked reconstruction preserves
the earlier selected field. The new transformation wraps the actual call and
returns one reconstructed value per unique enclosing root, even if two disjoint
mutable arguments select different fields of that root. Alpha renaming retains
the corresponding actual input binding names in the facet.

`rust-source-place-transform-regions.js` independently validates the graph before
interpreting its storage effects. It requires each input's exact source binder,
current initialized storage revision and complete closed position, checks all
ordinary source operands and their selected parameter modes, acquires each
actual whole-root epoch once, and publishes the actual closed result/new root
packet. Reference operands remain checked against the consumed epochs. Existing
aliases keep their old revisions and cannot be revived by the writeback.
No field receives a Copy trait or an invented region.

The source checker, typed Links validation, source-free projection and datatype
reference walk retain the facet. The schema author and automatic source-author
consumer inventory include it. Generated assets and translated Rust policies
remain deferred; no replacement native semantics were written. Exact immutable
facet facts can be shared during finite expansion under its existing budgets.

## Independent source controls drafted

The exact earlier projected-field refusal is retained as a positive original,
with unchanged source, empty stdout and declared native acceptance. Its previous
revision `4e3af91c2`, fixture name and index 18 remain in explicit history. This
changes the draft support boundary; it claims no executed evidence. No source
input or unrelated refusal assertion was removed.

Eleven further originals cover sibling preservation, two disjoint fields of one
root, nested ancestors, owned Vec and String fields, readonly result helpers,
ordinary argument/result positions, two different roots, observable argument
order, nested disjoint argument mutation and mutable method receivers. Five
invalid originals preserve same-field/ancestor overlap, moved roots, old loans
and incompatible primitive owners.

Structural controls inspect actual binders/paths, corrupt sibling wiring,
replace packet edges, forge identities, omit interfaces, inject hidden operand
effects and require refusal. Full Links text/binary reload controls retain the
two nested call contracts and compare JavaScript, Rust, Lean and Rocq emissions.
All controls are unexecuted and retain the existing explicit version bounds.

The authored inventory is now 467 positives in nineteen groups: 403 literal
originals plus 64 actual-source scenarios. The mixed composition group has 102
positives and 34 refusals. These are input counts only, not observed passes,
drafted requirement counts or a requirement completion percentage.

## Remaining implementation paths

1. Add a complete projected replacement-expression contract for direct String
   and Vec intrinsic mutations that lower to a field read plus replacement,
   rather than a selected source callable. The guard now remains at that actual
   intrinsic method path; ordinary already-admitted scalar field writes retain
   their existing rules. Preserve read order and all unchanged fields; do not remove
   it merely because callable transformations are available.
   String push_str/clear and Vec push were subsequently drafted with that exact
   replacement contract in
   [projected intrinsic replacements](issue-227-projected-intrinsic-replacements-2026-11-03.md).
   Other intrinsic helper/body interfaces remain explicit remaining work there.
2. Extend the selected-call contract for actual callback-specialized ABIs. The
   current transformation explicitly requires a complete noncallback closed
   callee. Retain real capture binders, callback bodies, argument modes and all
   hidden state writebacks before permitting callbacks on projected places.
   This immediate closed callback path was subsequently drafted in
   [projected callback interfaces](issue-227-projected-callback-interfaces-2026-11-03.md).
   Its handoff distinguishes whole captured roots from remaining field capture
   and escaping environment obligations; no execution evidence is claimed.
3. Continue borrowed/non-Copy place references, partial moves and disjoint field
   loans. Closed field call reconstruction does not establish general field
   reference regions. Add actual field-region paths before relaxing whole-root
   loan approximations or admitting reference-bearing state fields.
4. Complete actual borrowed capture/result and residual ABIs in finite helpers,
   loop initialization-state carriers, borrowed iteration and shared collection
   composition. Continue full own-project JavaScript/Rust CST, emission and both
   translation directions, constructor gaps and all broader vision obligations.
5. After all requirements have full code drafts, reconcile sources online before
   ledger use, publish pending automatic schemas/receipts/native translations,
   and run the authorized JavaScript and CI-only gates. Record observations only
   afterward at the exact executed commit. No local Rust compilation is allowed.
