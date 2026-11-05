# Loop-aware overwrite liveness source draft

This extends the [owned-loop region draft](issue-227-owned-loop-region-interface-2026-11-03.md)
under [vision](../vision.md). No tests, represented imports, source generation,
formatters, validation gates or compilation ran. No CI feedback was inspected.

The actual source continuation scan in `rust-owned-record-effects.js` now
descends into real admitted ordered/finite loop heads and bodies instead of
treating every loop marker as unknown. The original expressions, output and
evaluation order remain. Actual private state reads/writes and exclusive call
arguments still identify their roots. A loop's whole writes are accesses for
this analysis: even a body that assigns a root cannot initialize the root on
the zero-iteration path. Only a genuine later complete write before any access,
or an actually complete unused scope, establishes the existing omission rule.

Borrowed iteration retains its actual lease as a dependency even if the item
pattern copies a primitive or ignores all items. Exact storage/field identities
map those loan roots to outer state slots. The comparison work shares the
bounded inspection budget. Unknown operations, missing lease facts, returning
loops, aborts and unresolved completion packets remain barriers. Nested admitted
loops establish their own dependencies; no body execution is used to guess
whether they will run.

The ordered loop origin reader now requires its real private WeakSet admission
before returning a descriptor. Finite loop admission exposes only a membership
query, keeping the existing throwing assertion and seals. Liveness uses that
query; a provisional or copied finite descriptor grants no authority. An added
control places a structurally matching copied ordered descriptor in the expected
reader inventory and retains rejection before its consumer callbacks.

The exact source formerly named `unknown loop effects remain a barrier to source
overwrite projection` moves from a translator-frontier refusal to the positive
originals. Its historical name, source, previous fixture/index and revision
`53e8d86c6` remain in `rustSourceRegionCompositionLoopHistory`. The old row already
declared native acceptance; its throw assertion described an unsupported source
path rather than invalid Rust. The new positive keeps the same complete input
and adds an stdout/packet obligation. No compiler observation or passing outcome
is asserted by this authoring change.

Four further positives retain loop output, unrelated scalar state, nested loop
operands and a readonly callable's outer continuation. Two new invalid-Rust
controls retain moved-root access in the head and in an empty iterator's body.
Deferred graph assertions require omitted unrelated Vec slots while preserving
the actual `count` slot. All previous positive inputs and other refusal controls
remain. Version ranges and the independent native original/product CI driver
are unchanged.

This overwrite checkpoint declared 428 positive inputs in nineteen groups, 364 literal
originals plus 64 actual-source scenarios. The mixed-region group has 63 positives
and 22 refusals. Counts are source declarations, not completed requirements or
passing evidence. The unified CI driver includes the new positives through its
existing mixed inventory; it has not run for this batch.

Remaining work includes uninitialized/moved loop entry carriers when a loop
itself writes a root before any use, especially its zero-iteration case. That
needs explicit initialization-state transport and repeated/exit invariants,
rather than pretending the old root is initialized. Returning/residual loops
need their full enclosing completion and result-region interface before the
continuation scan may cross them. Finite helper region interfaces, shared
collection/signature-3 composition, projected exclusive places, borrowed
callbacks and full project translation remain under the same active goal.
Generated schema/policy/naming/source-register publication remains deferred;
automatic translation must supply native semantics after full drafting.

The [finite-helper successor](issue-227-finite-helper-state-regions-2026-11-03.md)
drafts compiler state/body region interfaces and primitive record projection
semantics. Its ten further positives bring the inventory to 438; the mixed
group has 73 positives and 25 refusals. These remain source declarations only.
