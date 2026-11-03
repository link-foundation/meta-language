# Loop state selection and remaining region integration

This is an unexecuted source draft under the architecture in
[vision](../vision.md). No tests, generation, represented imports, validation,
formatting or compilation ran. No CI conclusions or logs were inspected.

## Drafted source path

`selectRustOwnedRecordLoopStateSlots` in
`js/src/translation/rust-owned-record-effects.js` selects from the actual outer
slot inventory. Its bounded traversal inspects real private reads, writes,
source state transitions and exclusive call argument places. Exclusive borrow
operands are consumed through the selected call's actual argument mapping,
rather than mistaken for unknown independent effects. Direct source state
names and nested loop operands retain their dependencies. Unknown private
operations conservatively return the original full inventory.

`rust-standard-collection-source-loops.js` applies the selection to the actual
body only. The complete head still evaluates once before the fold. A mutable
Vec consumed by that head is not inserted into the accumulator merely because
it existed in the outer scope. Required reads/writes and exclusive writeback
roots remain. The original completion packet, zero-iteration initializer,
break/continue/return transport and source loan admission remain in use.

`rust-owned-record-loops.js` selects from the complete guard and body after
the existing ownership/progress checks. Generated chunk signatures and their
continuations therefore retain actual dependencies without carrying an unrelated
moved root. Helper seals and checked acyclic call-graph checks remain required.
The outer effect selector recognizes the same loop state instead of repacking
every unrelated outer variable around the reduced loop packet.

Eight appended owned Vec originals cover unrelated moved/live roots, a consumed
mutable head, nested record writes, exclusive calls, empty traversal, break
writeback and head-before-body effects. Two appended shared BTree originals
cover unrelated moved roots and actual readonly outer state. Two appended finite
record originals cover moved and live unrelated records; their existing four
predecessors remain. Deferred structural assertions inspect actual checked fold
slots and generated helper parameters, alongside the independent stdout oracles.
All preexisting inputs, refusal controls and their indexes remain.

The complete-original CI driver now includes all six finite-record originals in
a separate group with its actual nonexclusive source profile. The inventory is
414 authored positives in nineteen groups: 350 literal originals and 64 actual
source scenarios. These are declared controls, not passing results or a count
of completed requirements. Existing CI performs native original/product checks,
default complete CST reconstruction, and typed Links text/binary rechecks. The
driver was edited but never invoked in this phase.

The completion-reader follow-up reuses the actual token boundary rule already
used by ordinary function/main parsing. Brace-delimited println is a statement;
parenthesis/bracket invocations need a separator before a following effect but
can be final Unit expressions. Direct Unit blocks may omit the separator;
parenthesized blocks may not. Six appended owned-loop originals and three
invalid-source refusals retain ordered stdout, empty/direct blocks and actual
function returns. These bring the owned-loop group to 38 positives and 19
refusals. The 414-positive total above includes this follow-up. None ran.

Source review found that owned vector loop/sort fixture ranges spell the same
bounds with a leading `Rust ` label, while the original driver compared the raw
string with an unlabeled range. It now removes only that exact optional label
before the unchanged exact-bounds assertion. The actual compiler profile still
checks the active Rust/Cargo versions and target. No supported interval, input,
runtime or assertion coverage is removed, and no outcome has been observed.

## Still required for signature-3 loop region checking

State selection is necessary infrastructure, not a lifetime proof. The readonly
source-region visitor still needs the following actual loop integration:

1. Retain a bounded source fold interface derived from the real admitted loop:
   iterable and element positions, the generated control declaration, actual
   ordered state binders, source spans and completion destination. Do not forge
   Rust field signatures for compiler control data or accept an arbitrary reduce.
2. Validate that interface against the checked reduce/lambda/initializer shapes,
   real declaration layout and every retained state position. Add the facet to
   ordinary checked projection, Links schema, validation and type-reference
   traversal so source-free text/binary reload has the same obligations.
3. Extend the region interpreter with explicit compiler control/accumulator roles.
   Start with the actual zero-iteration state; check one symbolic body iteration
   and its repeated-state invariant, all captures and item ownership. Require
   every continuing path to publish the exact current storage epochs.
4. Check break, continue, returned values and residuals against the enclosing
   source result and lifetime bindings. Merge zero and nonzero paths; expire
   item roots and reject escaping iteration-local loans or stale prior epochs.
5. Preserve mandatory exclusive formal/callback writebacks. Nested folds must
   derive authority from their own actual interfaces, not inherited names or a
   caller flag. Unknown loop/private operations remain explicit boundaries.
6. Append real mixed-profile original programs for empty/nested loops, moves,
   state publication, borrowed result parameters and early exits. Retain invalid
   lifetime, forged facet, wrong slot/owner, moved capture and stale epoch
   controls. Generate native policy implementations from the shared JavaScript
   source after full drafting; do not add handwritten Rust semantics.

The existing native-valid unknown-loop refusal in the mixed-region originals
remains until that path is drafted. Standard collection/signature-3 interfaces,
projected exclusive places, borrowed callbacks and full own-project module
translation are additional independent frontiers; this batch closes none by
declaration. The active goal retains their full scope.
