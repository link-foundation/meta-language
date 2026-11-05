# Delivery time analysis — 2026-11-06

This report measures elapsed time between committed checkpoints and explains the
work those checkpoints contain. It provides a conditional budget for finishing
the remaining scope. **Git does not record active working time for each step,
and the current history does not establish a verified requirement-delivery
velocity.** A statement such as “this feature took 14 minutes” would go beyond
the evidence; “its checkpoint followed the previous one by 14 minutes” is valid.

The latest implementation examined is `2ae7378627a95233d9fed45879d10037f61a2ecf`, committed
2026-11-04 at 04:20:55 UTC+07:00. This report is prepared November 6. It does not
charge the subsequent period without commits to development. The
[vision](../../vision.md) remains the architecture; this is documentary analysis
and a forecast, not a replacement specification or a completion claim.

## What was measured

The [complete appendix](commit-timing-2026-11-06.md) records **2,140 distinct
commits across three task history ranges**, plus the separate PR #201 merge
checkpoint. Each commit has its exact identity, author and committer dates,
interval since its phase predecessor, exact subject and a clearly marked
subject-based work classification. Thus every included checkpoint has a timing
row; the table is not a selection of convenient fast commits. The
[calculation notes](commit-timing-calculations-2026-11-06.md) add daily spans,
the twenty largest gaps, threshold sensitivity and the latest 32 intervals.
The small [standard-library generator](build-commit-timing-report.py) pins the
ranges and arithmetic so later commits do not silently change this capture.

| Phase | Distinct commits | First → last checkpoint, UTC+07:00 | Span | Median adjacent gap | Mean adjacent gap | P90 adjacent gap |
| --- | ---: | --- | ---: | ---: | ---: | ---: |
| PR #201 branch | 1,474 | Oct 7 20:04:33 → Oct 19 15:18:00 | 283.22 h | 2m03s | 11m32s | 17m10s |
| PR #226 continuation | 130 | Oct 19 17:10:30 → Oct 22 00:28:24 | 55.30 h | 16m27s | 25m43s | 53m49s |
| Issue #227 continuation | 536 | Oct 22 01:42:34 → Nov 4 04:20:55 | 314.64 h | 13m36s | 35m17s | 1h04m29s |
| November subset of the continuation, already included above | 200 | Nov 1 05:38:37 → Nov 4 04:20:55 | 70.70 h | about 7m03s | 21m19s | See reproducible full rows |

The phase spans omit explicitly listed cross-phase gaps. There are no negative
committer-time gaps in these ranges. In PR #201, **241 author dates differ from
committer dates**; in the two later ranges, all agree. This is why committer
rather than author time defines checkpoint intervals. The discrepancy alone
does not prove rebasing, active work, waiting or a particular cause.

The first-parent ranges follow the task branch paths. They do not enumerate
all hidden side-branch commits or every unrelated repository commit. PR #201's
merge is listed separately, avoiding both omission and double-counting. The
current phase excludes the PR #226 head already counted in the preceding phase.
The earlier November 1 appendix included that boundary head in its own window;
its counts therefore must not simply be added to this capture.

No project code, test, represented fixture, compiler, dependency install,
generator or validation gate was executed for this report. Only the documentary
Python generator ran, using read-only Git metadata and Markdown output. Current
ledger files and generated delivery reports were not read or reconciled: their
online source-audit prerequisite remains subject to the existing phase policy.
The earlier case study's historical classifications remain historical, not a
new drafted/delivered denominator. CI logs, jobs and artifacts were not read.

## Latest 32 steps: interval and why the work was necessary

These explanations describe the actual source problem named by the commit and,
for the latest ownership chain, the linked handoff records. They explain the
complexity addressed; they **do not partition the measured interval into time
spent coding, reasoning, documentation, context recovery or waiting**. Those
substep timers do not exist. Several earlier commits can arrive from overlapping
agent work, so a short interval need not mean a feature was implemented that fast.

| Checkpoint | Committed (+07:00) | Since preceding checkpoint | Work and reason |
| --- | --- | ---: | --- |
| [`53e8d86c6`](https://github.com/link-foundation/meta-language/commit/53e8d86c65fae0d72c1f3bca7a6baa2c38783bcf) | 11-03 16:39:14 | 00:20:51 | **Bulk draft owned Rust loop region interfaces and Links transport.** Loops need actual owned state packets and region interfaces that survive Links reload; admitting loop syntax alone does not preserve ownership. |
| [`13f0a12af`](https://github.com/link-foundation/meta-language/commit/13f0a12afede7b7a0f63f8e342ef0d80100bbdc8) | 11-03 16:51:28 | 00:12:14 | **Bulk draft overwrite liveness across admitted Rust loops.** Overwrite liveness must identify the real last access and source slot before killing a value; otherwise a loop can hide an invalid read or retain dead state. |
| [`2198094cc`](https://github.com/link-foundation/meta-language/commit/2198094cc96a24da4ce20fd2916efedf19df9197) | 11-03 17:39:04 | 00:47:36 | **Bulk draft finite Rust helper state regions and Copy value transport.** Helper effects need complete caller/callee state interfaces and independent Copy transport, rather than assuming every returned tuple is ordinary source data. |
| [`581582376`](https://github.com/link-foundation/meta-language/commit/581582376ec16ede97b3cfe0396d32fc254fd061) | 11-03 18:07:40 | 00:28:36 | **Draft finite iteration local scope boundaries and shared source facts.** Iteration-local binders must expire without destroying outer state or leaking aliases into the next iteration. |
| [`4e3af91c2`](https://github.com/link-foundation/meta-language/commit/4e3af91c24ca71256c5b6309ba24db99d67090ec) | 11-03 18:17:39 | 00:09:59 | **Draft source-region divergence and normal-only ownership joins.** Only returning branches may contribute initialized state. Abort/return paths need explicit divergence treatment to avoid false ownership joins. |
| [`771725c00`](https://github.com/link-foundation/meta-language/commit/771725c00cc427da9bef75caa97d21462f498890) | 11-03 18:34:19 | 00:16:40 | **Draft checked projected source calls and complete root reconstruction.** A projected mutable call must extract the actual field, invoke the selected helper and rebuild the complete containing root in order. |
| [`3bdd4c5b6`](https://github.com/link-foundation/meta-language/commit/3bdd4c5b6e24a17528a560fbe5f57d7bdb2539fe) | 11-03 18:48:42 | 00:14:23 | **Draft checked projected String and Vec intrinsic replacements.** String/Vec intrinsics need the same actual selected-place writeback semantics as helpers, retaining unaffected sibling fields. |
| [`a314d7291`](https://github.com/link-foundation/meta-language/commit/a314d72914bbe474aab42e8622a0180ebdc40c1b) | 11-03 19:03:58 | 00:15:16 | **Draft complete Vec mutation helper interfaces and body checks.** A declared vector mutation interface is insufficient unless the actual compiler helper body independently agrees with every input/output slot. |
| [`c53a6b180`](https://github.com/link-foundation/meta-language/commit/c53a6b180ebccef1b26e9c61a61abb536cbf4517) | 11-03 19:30:03 | 00:26:05 | **Draft projected callback interfaces and actual lexical capture checks.** Callbacks must prove actual lexical captures and projected source roots; equal names or a callback label cannot authorize mutation. |
| [`55682bfb7`](https://github.com/link-foundation/meta-language/commit/55682bfb74eb77e05dc7326710631845a2a602cb) | 11-03 20:03:31 | 00:33:28 | **Draft actual record field place and containing-owner regions.** A field loan belongs to real containing storage, not the temporary binder used by the core projection pattern. |
| [`adae3e17b`](https://github.com/link-foundation/meta-language/commit/adae3e17b52bc97ea2116b46447e630e2f4fcec1) | 11-03 20:15:48 | 00:12:17 | **Draft fallible iterator compiler interfaces and complete fold body checks.** Fallible folds must preserve the actual iterator/helper body and first-error/state behavior, not just the result type. |
| [`f624d79b5`](https://github.com/link-foundation/meta-language/commit/f624d79b5f1edcaa2236f181300db0832f57ee7a) | 11-03 20:31:00 | 00:15:12 | **Draft generic Map and Set construction with actual initializer receiver reuse.** Map/Set constructors reuse genuine allocation and iterable initialization, including prototype/adder selection, order and abrupt completion. |
| [`220ae1137`](https://github.com/link-foundation/meta-language/commit/220ae11372b43bf99c39f9eadeb93e2aac224bda) | 11-03 20:47:49 | 00:16:49 | **Draft Array species construction through the shared Construct operation.** Array species must run through the common Construct operation with the real selected constructor and length, rather than a separately invented allocator. |
| [`6710151c4`](https://github.com/link-foundation/meta-language/commit/6710151c42e3452937989ffa9ff0d6411f308f85) | 11-03 21:20:06 | 00:32:17 | **Draft bound function metadata and ordinary call forwarding.** Bound functions need real target/receiver/prefix slots, ordered metadata reads and ordinary call forwarding on shared runtime identities. |
| [`370621bd1`](https://github.com/link-foundation/meta-language/commit/370621bd1bdb6a99f57ae8b2a10f04a4cc0466b8) | 11-03 21:35:33 | 00:15:27 | **Draft bound construction authority and instanceof delegation.** Construction and instanceof must follow bound-target authority and newTarget substitution; callability and source spelling do not prove constructor authority. |
| [`fe187eb5a`](https://github.com/link-foundation/meta-language/commit/fe187eb5af5638c1a27094417856c7360876c573) | 11-03 22:08:31 | 00:32:58 | **Draft live source handles and shared portable invocation driver.** Live public handles must preserve aliases and current heap state while using portable invocation continuations and observable host invariants. |
| [`925ac8be2`](https://github.com/link-foundation/meta-language/commit/925ac8be2a92548c519b4b1a5197a6ba1a2aea11) | 11-03 22:42:40 | 00:34:09 | **Draft reflective definitions and preconverted descriptor continuations.** Reflection/descriptor conversion has ordered getters and partial effects; suspending and resuming must not repeat already observed conversions. |
| [`8387f6621`](https://github.com/link-foundation/meta-language/commit/8387f6621f57b4cd40b8572536051fca3a706658) | 11-03 23:12:12 | 00:29:32 | **Draft exact function source text and restricted callable metadata.** Function text needs actual source intervals and realm-specific native metadata, independently of executable authority or function names. |
| [`8c6005d76`](https://github.com/link-foundation/meta-language/commit/8c6005d7665dc33d95dd311f50ac4100d2cffea2) | 11-03 23:23:31 | 00:11:19 | **Draft canonical Box record field ownership and source region proofs.** Canonical Box payload/field views must preserve actual owner layers and selected source regions rather than promote a field to fresh ownership. |
| [`c5002b3d8`](https://github.com/link-foundation/meta-language/commit/c5002b3d80a49008e3e789eb54a339c9eee30aa6) | 11-03 23:37:32 | 00:14:01 | **Draft owned Box helper arguments results and shared call positions.** Owned Box helper inputs/results and shared argument positions must retain complete canonical layers through the call interface. |
| [`fc308e1cb`](https://github.com/link-foundation/meta-language/commit/fc308e1cb9235aa5cdef694d2ac9465ebf7cc295) | 11-04 00:02:59 | 00:25:27 | **Draft mutable Box source state and exact projected field writeback.** Mutable Box storage needs real source binder identities and full projected writeback, not just scalar mutation of an erased payload. |
| [`0c42ff8d7`](https://github.com/link-foundation/meta-language/commit/0c42ff8d76d1dbd20d9586bae3ab5968655e3a82) | 11-04 00:17:27 | 00:14:28 | **Draft ordered scalar compound places through canonical Box layers.** Compound assignments must read the actual old selected scalar and evaluate operands in order before publishing the containing replacement. |
| [`73fe8da8e`](https://github.com/link-foundation/meta-language/commit/73fe8da8e6840f5fbed3047d30dd56bdbc5b0862) | 11-04 00:30:21 | 00:12:54 | **Draft Box payload assignments through shared ordered source places.** Payload assignment must retain the original allocation, nested layers and effects from the actual right-hand side before reconstruction. |
| [`6610de5ea`](https://github.com/link-foundation/meta-language/commit/6610de5ea315652330a8ea92fbf66d4f8c63af07) | 11-04 00:50:38 | 00:20:17 | **Draft checked Box payload calls and canonical mutable coercions.** Projected Box calls need complete caller/callee interfaces and checked actual coercion to the referent; the Box layer cannot be silently dropped. |
| [`347efd07f`](https://github.com/link-foundation/meta-language/commit/347efd07f73996cb642fd93f07ddb883de052e82) | 11-04 01:17:17 | 00:26:39 | **Draft Box receiver operations and method borrow activation.** Receiver calls combine actual method selection, reserved loans, operand evaluation and activation before String/Vec/record writeback. |
| [`b300cda38`](https://github.com/link-foundation/meta-language/commit/b300cda382e8fce1a922edc0a4d68a57d25dbf46) | 11-04 02:08:14 | 00:50:57 | **Draft independent record component ownership and field loan images.** Independent component roots are new shared ownership infrastructure: partial moves, selected loans, joins and containing lifetimes must agree. |
| [`10e72a77e`](https://github.com/link-foundation/meta-language/commit/10e72a77eab63699a603f968209f8ae312f4c6cd) | 11-04 02:22:10 | 00:13:56 | **Draft checked partial record initialization and selected field updates.** Reinitializing a missing field must preserve other missing/live components and genuine current state transport; a complete-looking replacement mask is insufficient. |
| [`63c905ff1`](https://github.com/link-foundation/meta-language/commit/63c905ff18e60549045617baaca31883dceaa2f9) | 11-04 02:36:19 | 00:14:09 | **Draft precise disjoint field lifetimes and storage revision joins.** Whole-read revisions and containing lifetime anchors must differ so a disjoint field write does not falsely invalidate an unchanged sibling loan. |
| [`325a5e75e`](https://github.com/link-foundation/meta-language/commit/325a5e75e17ee54eff62bc467f055bbab47db5c3) | 11-04 03:04:08 | 00:27:49 | **Draft irrefutable source move and reference place bindings.** Mixed move/ref/Copy patterns must capture every selected field before user shadows, initialize mutable leaves and preserve actual initializer storage. |
| [`d733d0af7`](https://github.com/link-foundation/meta-language/commit/d733d0af7f06393ccb73aa1276bdb8b068b8af6a) | 11-04 04:08:16 | 01:04:08 | **Draft owning source match guards and deferred actual arm transfers.** Guard-time owning names need readonly place views and private authority; actual moves occur only after success, while false guards and hygienic continuations retain the source. |
| [`a24f269a8`](https://github.com/link-foundation/meta-language/commit/a24f269a8a65eafc08de10dad743169b685b1888) | 11-04 04:16:17 | 00:08:01 | **Draft actual shared referent places for explicit source patterns.** Explicit shared-reference patterns reuse field/guard machinery after a checked one-layer referent view, retaining real loans and forbidding owning transfer. |
| [`2ae737862`](https://github.com/link-foundation/meta-language/commit/2ae7378627a95233d9fed45879d10037f61a2ecf) | 11-04 04:20:55 | 00:04:38 | **Draft source block arm boundaries across guarded match lowering.** Lowering a source match to a let must not change source comma rules; nested blocks, final else branches and following reference patterns retain their original syntax boundaries. |

The latest detailed source records are the handoffs for
[field component images](../../handoffs/issue-227-record-field-images-2026-11-04.md),
[partial initialization](../../handoffs/issue-227-partial-field-initialization-2026-11-04.md),
[disjoint revisions](../../handoffs/issue-227-disjoint-field-revisions-2026-11-04.md),
[irrefutable bindings](../../handoffs/issue-227-irrefutable-place-bindings-2026-11-04.md),
[guarded entry](../../handoffs/issue-227-guarded-place-matches-2026-11-04.md),
[explicit references](../../handoffs/issue-227-shared-reference-patterns-2026-11-04.md)
and [block-arm syntax](../../handoffs/issue-227-block-arm-syntax-2026-11-04.md).
They preserve actual code loci, deferred independent controls and remaining
protocols. Their new controls remain unexecuted; neither their existence nor
their counts establish verified delivery.

Two examples illustrate the difference between building infrastructure and
reusing it. Guarded owning entry follows its predecessor by **1h04m08s** and
introduces separate guard/body graphs, readonly place authority, ownership
snapshots and hygienic fallthrough. The following explicit shared-reference
step is **8m01s** later and reuses those field/region mechanisms. Block-arm
syntax follows **4m38s** later and reuses existing parser classification.
This supports investing in shared infrastructure. It does not prove an exact
64-minute implementation cost or that the next hard infrastructure gap will
fit another eight-minute interval.

## Long gaps, pauses and time that cannot be attributed

| Observed gap | Checkpoints | What the repository supports |
| --- | --- | --- |
| 25h05m20s | Oct 31 planning → Nov 1 resumed inventory draft | The stop is documented in the [October 31 checkpoint](../../handoffs/issue-227-checkpoint-2026-10-31.md), whose original `9405c25fb` version explicitly says the user requested stopping. The interval coincides with that stopped phase; Git does not measure precisely how much was inactive. |
| 23h00m17s | Oct 22 checkpoint → Oct 23 merge from main | These endpoint subjects establish a checkpoint and a later merge. The records inspected do not allocate the gap to waiting, merging, CI, sleep or implementation. |
| 9h58m41s | Oct 26 alias-context draft → later namespace/alias draft | A long checkpoint gap; its exact cause is unmeasured. |
| 9h14m31s | Oct 28 property-definition draft → registered-call/nominal-copy draft | A long checkpoint gap; its exact cause is unmeasured. |
| 7h15m25s | Nov 3 readonly/FnMut originals → later vector-read draft | A long checkpoint gap; its exact cause is unmeasured. |
| After Nov 4 04:20:55 through this report | Last implementation checkpoint → Nov 6 report request | No intervening local task commits were found in the pinned history. That is not proof of continuous development or a measured delay category. |

For the 535 in-phase continuation intervals, retaining only gaps at most
30 minutes sums to **75h21m17s**; using a 60-minute threshold gives about
**119.51 h**, 120 minutes gives **168.82 h**, and 240 minutes gives **199.74 h**.
The unfiltered span is **314.64 h**. The large change demonstrates why a
chosen “session” threshold cannot recover actual working time. None of these
figures is reported as person-hours or productive CPU time.

The conversation records disk exhaustion, cancelled/failed CI and requests to
stop or change agent count. The prior case study preserves that operational
context. Without historical telemetry and exact activity events, assigning a
percentage of total delay to each would invent data. This report neither
blames all elapsed time on those events nor assumes every unexplained gap was
idle. Current disk availability is also not historical resource telemetry.

## Why the project has accumulated so much time

1. **The required result spans more than parsing and printing.** Rust moves,
   reference lifetimes, callback effects and module identity must remain correct
   through translation, checking, both Links codecs and every required target.
   JavaScript adds live identity, descriptors, prototypes, exceptions and actual
   source-call execution. A syntax-only implementation can be fast and still
   leave these consuming paths unfinished.
2. **Several shared foundations were being added incrementally.** The latest
   sequence moves from whole records to component images, disjoint revisions,
   mixed bindings and successful/failed guards. Each depends on the previous
   proof interface. Independent semantics cannot safely be replaced with a
   flag in an archive. This creates serial integration work even when leaf
   feature authoring is parallel.
3. **Checkpoints are smaller than the promised outcome.** Numerous useful
   helpers, controls and source profiles were committed before one complete
   own-project translation path was closed. The earlier case study identifies
   repeated stops at unfinished frontend integration as a coordination failure.
   A handoff preserves work but does not itself finish that work.
4. **There is still no current full-scope completion denominator.** Historical
   mappings are not full implementations. Counting source paths, authored
   controls or commits as completed requirements previously made progress look
   more conclusive than it was. This report supplies all timing rows without
   converting them into a fabricated percentage.
5. **The current phase defers correctness feedback.** The instruction is still
   to draft all requirements before behavioral verification or CI repair.
   Source work continues, but unmeasured integration failures accumulate. The
   report respects that policy; the budget reserves a later repair phase.
6. **Automatic JS-to-Rust publication has a bootstrap dependency.** A new
   normative algorithm may need new translator support before it can be
   generated into native code. Handwritten native replacement semantics would
   hide that gap rather than close the authorized automatic translation path.
7. **Documentation and coordination also consume time.** Many small handoffs,
   repeated broad source reads and frequent checkpoints preserve detail but
   are not the same as finishing a module. Parallel arrivals also make the
   commit interval a poor unit of feature work. Fewer, dependency-complete
   integration batches can reduce these costs without discarding obligations.

These are source-grounded explanations of work structure. Exact causal hours
for these categories were not measured and cannot be reconstructed from Git.

## Remaining work and a provisional budget

The scope source is the [vision](../../vision.md) and the detailed handoff
chain, including the [remaining-work plan](../../handoffs/issue-227-remaining-work-plan-2026-10-31.md).
The following is a **planning allocation based on those remaining code protocols,
not a duration calculated from commit frequency or an audited requirement count**.
It is low confidence. A focused engineering hour is a budget unit for one
effective contributor; no measured conversion to this model's runtime, token
use or productive hours per day exists. Fast source drafts do not calibrate the
later correctness/consumer repair budget.

These ranges refine the older report's rough 1,500–3,500 hour full-draft and
4,000–8,000 hour full-delivery envelopes. They are not independent empirical
estimates or proof that the remaining work fits a fixed upper bound. Recent
commits are not subtracted at “one requirement per commit.” Unknown native/formal
repair and newly reconciled scope can exceed the ranges.

Every row below includes its own domain-specific independent control source.
D10 covers cross-domain consumer/delivery drivers, not another charge for the
same domain fixtures. D4 covers source integration and authoring recipes;
actual execution and repair belong to the later V rows.

| Package | Remaining executable code/control scope | Dependencies | Provisional focused hours |
| --- | --- | --- | ---: |
| D1 | Compact current source/construct inventory and full-scope draft review against captured vision/handoffs; reconcile ledger only when its mandatory online audit is permitted | Captured scope, current pinned sources | 16–40 |
| D2 | Rust variant/refutable/or/alias/guard-let matching, foreign guard effects, exclusive/stored references, product places, call/state/closure ABI, generic/trait/macro/module and required standard-owner gaps | D1, current ownership infrastructure | 160–480 |
| D3 | Remaining whole-project JavaScript source execution: module/live-cell/callback/effect integration, async, Function/dynamic code, derived/super, Proxy/private fields and required reflection paths | D1, common heap/call authority | 160–480 |
| D4 | Close complete own-source consuming modules and project driver in both directions; preserve edited CST/current graph emission; improve translator for complete automatic native authoring | D2/D3, priority CST generation | 80–240 |
| D5 | Full grammar-source unions and scanners across the supported catalog, priority JS/Rust/TS/Markdown, default lossless CST/optional AST, recursive any/all embedding and span/error handling | Existing generic grammar pipeline; D1 | 160–480 |
| D6 | Global link identities, recursively formal concept definitions, checked equivalence/unknown cases, ontology/taxonomy/root entry points and foundation integration | Common concept/definition graph | 120–400 |
| D7 | Complete graph edits and binding-safe transforms, grammar merges, RDF/OWL import/export/conversion/merge, dynamic reconstruction and public link-cli/library entry points | D5/D6, real graph identity | 120–360 |
| D8 | Required Lean/Rocq semantics and all directed target translations, modules/binders/universes/effects and actual prover-facing source/codecs | Common semantics plus D2/D3/D6 | 200–600 |
| D9 | Required grammar discovery algorithms, pinned primary-source/dataset research, discovery/refinement/minimization and public held-out-control integration | D5/D6, stable discovery interfaces | 120–400 |
| D10 | Cross-domain assertion consumers, fault injection, clean-package/downstream/release automation, source/naming/version/operations recipes and retained resource receipts | Domain interfaces and unchanged gates | 100–300 |
| D11 | Global code/control closure review, missing integrations and complete publication-input recipes | D1–D10 | 40–120 |
| **All remaining code/control drafting** | **Arithmetic sum, no presumed parallel overlap** | **Global phase boundary** | **1,276–3,900** |

The own-code JS/Rust draft allocation D1–D4 is **416–1,240 hours**, conditional
on the priority CST/concept dependencies actually being adequate. It is **not**
a separate promise excluding D5/D6 work that a real consuming module still needs,
and it is not an earlier permission to begin verification. Reusing the same
shared machinery broadly could reduce work; discovering that the current drafts
do not compose would increase it. General stored references and active variants
are currently concrete code gaps, not a measured CI bottleneck.

After the full code/control draft gate, allow separately for execution and repair:

| Package | Later delivery work | Provisional focused hours |
| --- | --- | ---: |
| V1 | Run automatic native/schema/asset publication and repair actual translator/output closure | 80–240 |
| V2 | Required JS/Rust original/product/runtime parity and integration repair at exact commits | 400–1,600 |
| V3 | Broader grammar/concept/RDF/formal/inference corpus, prover and independent-control repair | 600–2,200 |
| V4 | Actual clean consumers, downstream workloads, matching releases and final obligation reconciliation | 120–360 |
| **Later execution/repair subtotal** | **CI queue and external downtime excluded** | **1,200–4,400** |
| **Full bounded delivery including the draft** | **D + V, unchanged required quality** | **2,476–8,300** |

These figures explicitly separate drafting from delivery. They are not a claim
that tests must fail for a particular number of hours or that all existing code
must be rewritten. They are risk allowances until actual integrated execution
and a current requirement review can calibrate them. CI queue duration, external
service outages, future source/dependency changes and time between user-requested
work sessions are additional elapsed time, not included effort.

## Calendar conversion: conditional capacity, not a promised finish date

Current steering is solo work after earlier agents finish. No new agent is
assumed for this report. The earlier report's two-effective-worker schedule is
therefore a comparison scenario, not the current plan. Use:

`calendar weeks = remaining focused hours / effective focused hours delivered per week`

| Effective capacity assumption | Full code/control draft (1,276–3,900 h) | Full bounded delivery (2,476–8,300 h) |
| --- | ---: | ---: |
| 40 h/week: one effective contributor, 8 h × 5 days | 31.9–97.5 weeks | 61.9–207.5 weeks |
| 80 h/week: two effective contributors, if separately authorized and achieved | 16.0–48.8 weeks | 31.0–103.8 weeks |
| 168 h/week: theoretical uninterrupted one-contributor capacity | 7.6–23.2 weeks | 14.7–49.4 weeks |

The last row assumes every calendar hour is productive and is a sensitivity
bound, not an available service guarantee. Adding agents does not guarantee a
particular row: shared interfaces, integration and the final verification gate
remain serial. An AI agent's “engineering hour” cannot currently be translated
into these calendar capacities from repository metadata alone. Consequently no
calendar date or probability of completing by a date is established here.

Under the current global draft-first rule, own-code baseline verification waits
for **all D packages**, not just D1–D4. The own-code draft's shorter effort range
cannot be offered as the date of a working translator. Changing that phase
policy is a possible future scheduling decision; this report does not change it.

For comparison only, another **100 hypothetical similar checkpoints** at the
continuation median would occupy about **22.7 elapsed hours**; at its P90 gap,
about **107.5 hours**. We do not know that precisely 100 comparable steps remain.
Those calculations exclude feature-size differences, backlog review, repair and
release, so they must not be advertised as a full-delivery estimate. The complete
commit appendix makes this limitation visible rather than implying a calibrated
“requirements per hour” rate.

## How to get a tighter estimate with less wasted work

1. Keep one compact current dependency list linked to the retained handoffs.
   Select complete consuming modules, especially Rust `Type::children` plus
   `map_children`, and JavaScript Scope/loader/decoder paths, rather than another
   arbitrary collection of isolated helpers. Preserve all broader obligations.
2. Finish shared active-variant/reference/effect and common source-call protocols
   before building many adapters around incomplete interfaces. The eight-minute
   reuse step after guard infrastructure illustrates the intended payoff.
3. Group source, checker, public caller, codecs, automatic native authoring and
   independent controls in one coherent batch. A helper without its consumer
   stays in the same unfinished package instead of becoming a new completion.
4. Record future phase start/end, active contributor intervals, actual scope
   delta, interruptions and commit identities as ordinary planning data. Record
   per-tool durations when available. This can establish future active timing;
   historical gaps must retain their unknown attribution.
5. Reconcile the complete source/requirement set when its online audit is allowed.
   Classify full-scope source drafts against actual constructs and controls;
   do not reuse historical mapping coverage as a draft percentage.
6. After every draft is complete, execute the existing gates, measure actual
   native-generation and original/product parity repair, then reforecast D/V
   packages from observed complete outcomes. Native compilation remains CI-only.

The most useful next calibration is a complete own-project consuming path whose
source draft and independent controls are closed, followed by its first permitted
execution. Until then, the report supplies an exact checkpoint history and an
explicit, revisable planning budget rather than a fictitious precise finish time.

## Publication and reproducibility

The report commit adds this narrative, the complete timing appendix, calculation
notes and its standard-library generator. It changes no runtime, requirement
oracle, assertion, grammar or gate. The captured implementation head remains
pinned even after the report is committed and pushed. A status-only lookup
established the preceding workflow was terminal; no CI failure diagnosis or
repair was performed. The user explicitly requested committing and pushing
this report. The push carries the branch's preceding authorized source commits
as well as this documentation commit, because this is the existing continuation
branch rather than a separate history with replacement implementation.
