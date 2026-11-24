# Box payload assignment draft

Continues [Box compound places](issue-227-box-compound-places-2026-11-04.md)
under the sole architecture in [vision](../vision.md). This batch uses source
edits, textual review, Git metadata and primary specification reads only. No
test, represented program, generator, formatter, validation gate, compilation or
CI feedback ran or was reviewed. All controls below are authored and unexecuted.
The complete drafting goal remains active; no audited drafted/total count or
completion percentage is available.

## Actual code paths

`rust-exclusive-source-places.js` shares original token selection, canonical
partial Box projection and actual mutable place resolution between ordinary and
compound assignments. The pure ordered policy remains in
`rust-exclusive-source-compound-rules.js`; its preceding compound API delegates
to the same exported general plan. Prefix/postfix/grouping, bounds and actual
root/field identities are retained. The new adapter is registered as an automatic
source consumer; no generated file or handwritten replacement Rust was added.

`rust-exclusive-source-calls.js` now admits complete explicit dereference places
for ordinary assignment and retains the preceding plain-field path. It parses
the real RHS against the selected payload position, checks actual owning layers
after RHS effects, transfers owning input values, and restores the original
complete Box field around the replacement expression. The replacement expression
and actual payload binder retain their own positions. Shared/immutable or moved
roots, nonexistent layers and actual surviving loans grant no mutation authority.
Bare assignment to an exclusive reference binding does not assign its referent.

Reinitialization of explicit selected fields now retains every initialized
containing field, consumes owning replacements, clears only the selected target's
descendants and publishes that target. Complete whole-root replacement retains
its prior moved-root reinitialization route. Payload dereference cannot borrow
that exception: its actual owning layers must remain initialized. General partial
moves and possibly initialized indirection remain separate required work.

`rust-borrowed-source-regions.js` establishes actual vector/element positions
before canonical Box array construction. The final existing layer adjustment
retains the complete Box position; the vector element cannot be inferred from a
Box owner argument as though it were an ordinary Vec element. Existing noncomposed
operation paths and stored-reference restrictions remain separate. Compound
writeback uses the same complete payload restoration helper and original strict
primitive/ordering controls. Textual review also repaired the missing comma at
the preceding fixture append boundary without changing an input or assertion.

Sixteen appended positive originals cover scalar/exclusive/nested/inner Box
replacement, owning String transfer, record/vector payloads, parenthesized and
field places, latest RHS root/sibling effects, early completion and Unit results.
Ten appended invalid originals cover immutable/shared/moved storage, excess or
missing indirection, exact widths, surviving payload/field loans, consumed String
inputs and forbidden owning moves through shared references. Independent controls
retain scalar-versus-storage qualifiers, forged root/element owners, complete
Box depth, source-free Links text/binary reload and four target emissions.
All previous source inputs and assertions remain, under their explicit version
ranges. These are source input counts, not audited requirements or observed passes.

Primary contract: [Rust basic assignment](https://doc.rust-lang.org/reference/expressions/operator-expr.html#basic-assignments)
evaluates the replacement before selecting its mutable place, copies/moves the
replacement and returns Unit. The current closed/destructor-free source profile
does not establish generalized Drop, unsafe memory or custom Deref behavior.

## Remaining actual code, in implementation order

1. Complete explicit payload call borrows in `readRustExclusiveSourceBorrow`.
   Reuse ordered place resolution without replacing the original root/field.
   Keep a separate actual referent position for the selected exclusive formal;
   preserve complete original Box positions for reconstruction and overlap tests.
   Update `rust-source-place-transforms.js` and
   `rust-owned-record-effects.js` together: root payload calls also need projected
   calls even with an empty field path. Independent validation must prove exact
   canonical Box projection on the designated argument and retain the actual
   checked callee, complete packet order, latest roots and unchanged siblings.
   A payload qualifier alone must not permit arbitrary empty-path transformations.
2. Complete Box String/Vec receiver autoderef with actual peeled payload and
   original mutable slot. Extend `rust-exclusive-source-strings.js`,
   `rust-exclusive-source-vectors.js`, call acquisition, intrinsic replacements
   and region operations together. Preserve actual receiver/argument ordering;
   pop/truncate/clear reuse actual checked Vec helper bodies, complete Option/
   result positions and full caller Box reconstruction. Shared receiver spellings
   and erased core types cannot confer mutable authority. Explicit owning payload
   expressions used as readonly receivers also need actual place/autoref selection
   before ownership transfer; observing a separately moved payload does not supply
   that general receiver protocol.
3. Finish recursive state with constructive finite witnesses, reference-bearing
   fields/payloads, per-layer lifetime substitution, temporary lifetime extension,
   general partial moves and disjoint surviving field revisions. Complete
   Vec<Box>, actual generalized iterators and Drop/allocator/unsafe observations.
   Existing acyclic/destructor-free restrictions have not been removed.
4. Finish borrowed, escaping and field callback environments/aliases, Box callback
   captures/results/state, finite helper borrowed captures/results/residuals,
   zero-iteration state and actual iterator short-circuit/drop. The preceding
   handoffs retain their concrete environment/signature/interpreter entry points.
5. Finish own-project default CST parsing, emission and both JavaScript/Rust
   translation directions, including modules, generics, traits and macros.
   Preserve all JavaScript dynamic Function, callback/async, derived/super,
   Proxy/private-field, mutable-cell/effect and live-root obligations in vision.
6. Finish remaining full grammar/global concept/deep formal definition/equivalence,
   recursive any/all parsing, ontology/taxonomy, RDF/OWL import/conversion/merge,
   dynamic text/binary/link-cli reconstruction, grammar inference, formal-system
   and distribution requirements. Translator priority removes none of this scope.
7. Only after every actual code/control draft: reconcile sources online before
   ledger use, publish automatic source/schema/naming/native assets, run authorized
   JavaScript and CI-only native work, repair actual failures and record observed
   assertions at their executed commits. Never compile Rust locally.
