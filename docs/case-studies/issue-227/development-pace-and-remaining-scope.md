# Issue 227: delivery pace, remaining requirements and forecast

Prepared 2026-11-01 while three GPT-6.1-sol medium agents continue source
drafting. The committed reference is
`a14ce9a1ab7549ae114b8cee90c4ee08c428fac6`; subsequent working-tree drafts are
identified separately. This is a case study and planning analysis. The
[vision](../../vision.md) remains the sole architecture specification.

The project has produced substantial committed code, but a current full-scope
delivery count cannot be established from that code or from commit frequency.
The immediate problem is closing real source-to-source translation paths across
multiple semantic layers, while the requested global draft-before-testing phase
defers the feedback needed to know whether the integrated implementation works.
Neither more commits nor another requirement mapping resolves that distinction.

The practical forecast is weeks for a selected integrated source draft and
months for broad verified own-project translation. The complete inherited scope
is a larger compiler, grammar and formal knowledge infrastructure program.
A finite, reliable finish date for its unrestricted interpretation cannot be
promised. The conditional estimates below expose their assumptions and the
effect of the current global verification gate.

## Evidence and limits of this analysis

The analysis uses read-only Git metadata, the vision, existing handoffs, current
source interfaces, and three active implementers' source-grounded assessments.
No represented program, test, build, generator, validation gate or compiler was
run. No new CI feedback was inspected. Git timestamp arithmetic is documentary
analysis, not project behavior verification. No installs or workload clones were
created, and unrelated journals and files were left untouched.

The [complete timing appendix](commit-timing-2026-11-01.md) retains all 348
captured first-parent commit identities, timestamps, subjects and intervals.
The [earlier checkpoint appendix](earlier-checkpoint-commit-timing.md) retains
the 1,474-commit PR 201 branch range and 130-commit PR 226 continuation range.
The [remaining-work plan](../../handoffs/issue-227-remaining-work-plan-2026-10-31.md)
contains the historical 473-row planning inventory, including links to the
[80 previously unmapped rows](../../handoffs/issue-227-unmapped-requirements-2026-10-31.md)
and [393 mapped rows needing review](../../handoffs/issue-227-mapped-requirements-review-2026-10-31.md).
Those are historical classifications, not a fresh ledger audit or completion
count. The online source-audit prerequisite and source-register reconciliation
remain deferred under the current no-gates phase; this analysis does not silently
substitute for them. New conversation instructions are preserved in a separate
[source capture](../../../parity/sources/development-pace-case-study-2026-11-01.txt).

Four kinds of progress must remain separate:

| Kind | What establishes it | What it does not establish |
| --- | --- | --- |
| Recorded obligation | Preserved source and reconciled atomic requirement | Implementation |
| Code/control draft | Actual implementation, public callers, automatic native authoring and independent control source for the complete scope | A successful execution |
| Committed checkpoint | Git object containing the draft | Correctness, runtime parity or published availability |
| Verified delivery | Every required assertion on its actual fixture/runtime at the recorded commit, plus applicable consumer/release obligations | Arbitrary future language versions or universal program equivalence |

There is no defensible current integer for “fully delivered requirements” in
this analysis. Historical bounded observations remain attached to their original
commits. They cannot be copied forward as observations for this much larger
unexecuted batch. There is also no audited integer for fully drafted rows:
393 mapped rows never meant 393 complete implementations, and 80 unmapped rows
never meant exactly 80 missing implementations. Row-level source review must
establish both before reporting a draft percentage.

## Measured commit cadence

Window: October 22, 00:28:24 through November 1, 16:12:47, UTC+07:00.
All captured author and committer dates agree. These are local Git timestamps;
they do not record push times, CI execution duration, time spent actively working,
token expenditure, or which contributor performed each edit.

| Measurement | Observed value |
| --- | ---: |
| Captured first-parent commits | 348 |
| Adjacent commit intervals | 347 |
| Median interval | 17.07 minutes |
| Mean interval | 44.22 minutes |
| 90th percentile, lower empirical order statistic | 72.40 minutes |
| Largest interval | 1,505.33 minutes, about 25 hours 5 minutes |
| November 1 captured commits | 11 |
| November 1 first-to-last span | 10.57 hours |
| November 1 median within-day interval | 53.81 minutes |

The longest interval runs from the October 31 planning checkpoint to the
November 1 resumed draft. The historical checkpoint explicitly records a
user-requested stop. Calling that entire interval an implementation delay would
be misleading. Daily first-to-last spans near 24 hours likewise do not establish
continuous active labor.

Recent integrated batches are larger than many earlier subset commits:

| Commit | Gap from predecessor | Files changed | Added/deleted lines | Concrete draft |
| --- | ---: | ---: | ---: | --- |
| `b0864c92` | 163.30 min | 235 | +6,817 / −380 | Class ownership archives and Rust record state consumers |
| `127b6863` | 51.42 min | 11 | +324 / −7 | Global object allocation and source record calls |
| `a1c23f19` | 14.10 min | 9 | +302 / −19 | Formal datatype placement and public names |
| `3b572e01` | 17.43 min | 114 | +2,788 / −297 | Class composition, ordered effects and owned containers |
| `8aec2da7` | 112.95 min | 156 | +3,732 / −662 | Error execution, formal placement and recursive ownership |
| `a14ce9a1` | 56.20 min | 93 | +1,998 / −170 | Finite loops, borrowed source positions and runtime integration |

The gaps are intervals between commits, not the development duration of the
named feature. Several contributors work concurrently; a change may span
multiple intervals. File and line counts include controls, metadata and
documentation, can count a file again in the next commit, and measure neither
semantic coverage nor useful output per hour. The November 1 inventory/policy
checkpoint alone adds 47,916 lines; that is another reason not to use line count
as the completion denominator.

The measured history supports “frequent drafting and checkpointing.” It does
not support “one completed requirement every seventeen minutes,” a delivery
velocity, or any extrapolation of 473 divided by the commit rate.

Earlier phases reinforce that conclusion:

| Local Git phase | Captured commits | Timestamp window (+07:00) | Median / mean interval |
| --- | ---: | --- | --- |
| PR 201 head, `ce0101fb^1..ce0101fb^2` | 1,474 | October 7 20:04:33 → October 19 15:18:00 | 2.05 / 11.54 min |
| PR 226 continuation, `ce0101fb..8f2f6c48` | 130 | October 19 17:10:30 → October 22 00:28:24 | 16.45 / 25.72 min |
| Current captured window | 348 | October 22 00:28:24 → November 1 16:12:47 | 17.07 / 44.22 min |

The last PR 226 head is also the first captured commit in the current window;
these rows are deliberately not summed. PR 201's very short median interval
shows how granular its checkpointing was; it does not establish proportionally
faster semantic delivery. PR 201 merged at `ce0101fb` on October 19. The retained
handoff records PR 226's October 22 checkpoint merge and issue 227 continuation
with unfinished scope. Both were checkpoints, not full-requirement completion.
Local first-parent data omits side-branch details behind merges and cannot
reconstruct an independent contributor-hours or CI-wait-time history.

## What the recent implementation actually contains

The [frontend consumer record](../../handoffs/issue-227-source-consumers-2026-11-01.md)
and its three area handoffs retain finer source details. The following describes
source integration inspected during this work, without claiming execution.

| Area | Committed draft through `a14ce9a1` | Continuing working-tree draft / remaining boundary |
| --- | --- | --- |
| JavaScript heap and calls | Global object/Symbol identities; descriptor operations; explicit object allocation; registered calls; source record construction | Inspectable Map/Set slots and live insertion cursors; machine/realm/constructor linkage is active |
| Classes and modules | Source ownership archives; class/re-export composition; actual Error class plans and call continuations | Ordinary class constructors/methods, mutable lexical snapshots and argument-order preservation; actual `Scope` composition remains active |
| Provider loading | Portable module initialization, TDZ, shared aliases, cached abrupt completion; qualified host capability protocol; Node facade | Actual project-data loader/provider/helper source bodies, shared cache cell, compact UTF16 text; decoder body remains an explicit capability boundary |
| Rust source ownership | Qualified nominal origins, standard sums, owned Box/Vec consumers, move/borrow result checks | Mutable-root loan leases, retained region validation, complete reference iterator/collect semantics |
| Rust control flow | Ordered mutable record updates/effects, ordinary completions, initial bounded finite record loops | Completed/nested loop paths, real exit state joins and current controls continue |
| Source contracts | Recursive borrowed type positions and explicit signature successor; source version boundaries | Persisted body expression positions, actual expression tagging and source-free whole-program binder/region proof |
| Formal output | Recursive datatype grouping, protected module placement, public aliases, proposition/Boolean bridges | General reference-state/heap effects and complete native prover consumers |
| Transport and consumers | Exact root/ID source drafts; source/checked graphs, native compilation consumers and automatic authoring recipes | New recipe inputs/fixtures need their later generation; all newly authored consumers remain unexecuted |

An important new bug found by source inspection illustrates the integration
cost. The old immutable JavaScript source profile could alias a local name to
its value register. Adding assignment makes `let copy = parameter; copy = 9`
incorrectly overwrite the parameter unless the local gets a fresh register.
The same problem affects earlier call arguments and property receivers when a
later operand reassigns their source variable. The additive snapshot lowering
is necessary to make the class frontend usable; merely adding a source syntax
case would have left wrong behavior.

Rust has the corresponding provenance problem. Actual
`rust/src/translation/types.rs::children` returns `Vec<&Self>` and combines
`&Box<Self>` coercion, pattern references, `iter().map(...).collect()`, and
`chain(once(ret.as_ref()))`. Parsing `Vec` or persisting its element type is
insufficient. The checker must retain the actual referent, binder and returned
loan through each producer, conversion and transport. The same file's
`map_children<E>` adds `impl FnMut` and generic Result behavior. Completing
`children` therefore does not complete even this whole source file.

## Detailed inherited requirement work breakdown

The family sizes below are copied from the October 31 planning snapshot and sum
to 473. They organize inherited scope; they are not newly audited counts and
not delivery labels. Every original atomic row, fixture, language, alias,
version, runtime and assertion remains in the linked appendices. The tasks
below refine that scope without creating a second architecture specification.

| Family | Historical rows | Concrete remaining code/control packages | Exit condition to establish later |
| --- | ---: | --- | --- |
| BASELINE | 17 | Complete tracked own/downstream source inventory; JS classes/prototypes, collections, closures, module/effect behavior; Rust generics/traits, borrows, moves/drops, macros and standard owners; edited common-graph emission in both directions; whole-project automation and automatic JS→Rust libraries | Actual original and generated projects agree under required observations, no carried executable items or source-body fallback, both runtimes and independent controls |
| GRAMMAR | 155 | Full source unions per language/version; generic operator/scanner lowering; first JS/Rust, then TS/TSX/Markdown, then complete catalog; default lossless CST and explicit AST projection; error/recovery/Unicode/real-source controls | Automatically produced native grammar union retains all required constructs, complete source/trivia/spans and independent accepted/rejected corpora |
| EMBED | 11 | One any/all-grammar invocation model; same-language recursion and arbitrary nested language changes; delimiter/escape/offset mappings; edit/reload/emission of child trees; cycle and ambiguity diagnostics | Deep Markdown→JS→JSON→Markdown/Python and other registered cases retain full nested CSTs without hidden opaque regions |
| MEANING | 32 | Common identity space for all grammars; source-qualified copies before deduplication; recursive formal definitions/binders/assumptions; checked one-to-one correspondences; ontology and taxonomy entry points; exact-ID integration | Sharing follows actual checked meaning correspondence, unresolved cases stay distinct, every root/entity/relation is represented as links |
| EDIT | 36 | All query/construct/clone/insert/delete/replace/move operations; actual lexical/reference graph updates; shadowing and capture avoidance; invalidate/rebuild affected semantics; emit edited current roots | Real mutation sequences preserve required syntax/semantics and reject capture/stale-cache counterexamples through both codecs and consumers |
| MERGE | 19 | Complete grammar-format feature import/export union; deterministic binding-aware names; conflicts/alternatives; automatic generation from merged inputs; external consumer cycles and quality controls | Real format/tool cycles and independently checked correspondences establish each claimed conversion/merge, without generated-file patches |
| DYNAMIC | 14 | Source-free reconstruction of CST/grammar/concept/semantic space; public library and link-cli paths; exact roots/IDs/allocator state; RDF/OWL import/export/conversion/merge; dynamic extension | Static and dynamic modes preserve declared graph/ontology semantics and round trips without authoritative hidden tables |
| INFERENCE | 14 | Existing discovery-issue linkage; pinned primary papers/datasets; active discovery, refinement/minimization, lexical/structural priors and constraints; public API/CLI; held-out comparative controls | Actual discovery outputs feed ordinary grammar/global-concept/translation paths and satisfy independent held-out correctness and resource measurements |
| FORMAL | 52 | Lean/Rocq syntax and semantics, modules, universes, notation/tactics, elaboration/proof dependencies; every directed translation among four languages; effects/abort/recursion encoding | Actual compiler/prover consumers preserve declared proof/effect behavior and edited graph semantics; unsupported foundations remain explicit |
| EVIDENCE | 33 | Complete independent assertion/fixture/runtime consumers; metamorphic/fuzz/edit controls; fault injection for skipped or weakened coverage; full failure accumulation; exact-commit observations | Every declared required cell executes and passes with retained observations, including checks that detect false success |
| DELIVERY | 15 | Remaining production dependency replacements; exact clean-consumer artifacts; matching npm/crate APIs/versions/assets; pinned downstream workloads; release/main reports and merge/continuation procedures | Matching installed publications and real downstream consumers establish delivery; a candidate or checkpoint alone does not |
| OPERATIONS | 65 | Source-only phase control; one pusher and coherent batches; safe disk/cache ownership; no local Rust; CI ordering/cancellation/failure reporting; Links conversion and temporary grammar downloads; concise contributor instructions; version-bounded assertions; canonical vocabulary | Applicable operational assertions hold with receipts; obsolete expectations are corrected with evidence while inputs/coverage remain |
| TRACE | 10 | Reconcile all captured/live sources; full row source review; implementation/control/runtime mappings; naming/feature/source publication; source register and precise claims | Current complete obligation set has traceable scope and actual evidence; no mapped-path or stale report is counted as a pass |

Additional detail for the most important cross-cutting packages follows. Each
item names a concrete dependency and deliverable rather than another subset
capability label.

1. **Actual project inventory.** Preserve original revision, path, full bytes,
   version and import owners. Cover runtime preludes and generated helpers as
   well as handwritten files. A frozen older lexical inventory needs a future
   rebuild; it cannot establish coverage of today's source.
2. **JS source execution.** Compose classes, live collections, callback owners,
   module cells and effects on one identity-preserving heap. Source-body calls
   must execute translated frames. Node IO is an explicit boundary, not a way
   to invoke an untranslated original implementation.
3. **Rust source semantics.** Keep nominal declaration identity, standard-owner
   qualification, actual Copy/Clone evidence, moved state, real regions and
   mutable-root loans separate. Complete iterator items and generic callable
   interfaces before claiming whole `types.rs`, not just an example method.
4. **Persistent semantic contracts.** Carry recursive type/body facets through
   surface IR, checked IR, projection, text/binary Links and every emitter.
   Rebuild executable provenance from actual binders after reload; serialized
   “proven” bits are not proof.
5. **Formal state and effects.** The ordinary VM and checked Core are distinct
   source profiles. Full shared semantic integration needs an actual bridge,
   plus reference/heap/effect representations for formal targets. A JS VM path
   does not by itself deliver Lean/Rocq or the ordinary Core translation path.
6. **Native authoring closure.** Improve the translator when new normative JS
   cannot be translated; never replace it with handwritten native semantics.
   Generate full model codecs, libraries and catalog assets reproducibly once
   generation is permitted. Missing generated outputs remain a delivery gap.
7. **Complete concept definitions.** Declare primitives, assumptions, binders,
   recursive dependencies and proof authority. Preserve multiple ontology roots,
   taxonomy specialization and doublet self-loops. The physical-wave discussion
   is attributed philosophical context, not an empirical or kernel proof.
8. **Conservative equivalence.** Arbitrary program/definition equivalence is not
   generally decidable. The required design needs sound checked certificates
   or declared decision fragments, with unknown results preserved. An estimator
   cannot promise discovery of every possible equivalence for every language.
9. **RDF/OWL completeness.** Retain IRI identity, blank-node scope, graph names,
   literals, datatypes, language tags, ontology axioms and merge conflicts.
   Generic graph serialization alone does not establish RDF/OWL semantics.
10. **Grammar discovery.** Separate algorithm implementation, current-paper
    research, datasets, held-out evaluation and integration. A separate issue
    preserves the requirement; creating it does not implement discovery.
11. **Associative stack.** Audit actual provider roles throughout both packages
    and tooling. Preserve pinned formal-ai and meta-theory foundations and the
    applicability of Router #761, formal-ai #1188 and relative-meta-logic #184.
    Do not confuse a source citation with implemented compatibility.
12. **Releases and recurring duties.** Publication, live dependency/version
    checks and workflow operation are execution duties, not source files that
    become permanently complete. Draft their procedures/consumers first;
    perform and record their actual runs later at the required checkpoints.

## New and currently active conversation requirements

These additions clarify execution and documentation; they do not remove any
inherited functionality. The identifiers in this table are local case-study
references, not fabricated atomic-ledger rows or execution observations.

| Reference | Obligation | Concrete record or work |
| --- | --- | --- |
| CASE-01 | Continue bulk implementation instead of stopping at unfinished frontend helpers | Root and three existing agents continue real callers, provenance, machine and source interfaces |
| CASE-02 | Use at most three GPT-6.1-sol medium subagents in the current phase | Three existing agents own JS/CST, Rust aliases/borrowed positions, and Rust sums/control/ownership; root integrates and alone pushes |
| CASE-03 | No tests, compilation, behavioral execution or costly local work during bulk drafting | Source edits and small read-only Git/document analysis only; Rust remains CI-only |
| CASE-04 | Analyze all earlier requirements and actual delivery pace | This case, complete captured commit appendix, inherited per-row planning appendices and refined work packages |
| CASE-05 | Explain why delivery is taking so long, with evidence and responsibility | Dependency examples, commit measurements, process failures and uncertainty below |
| CASE-06 | Give detailed realistic completion estimates | Explicit effort ranges, critical path, policy effects, assumptions and re-estimation triggers below |
| CASE-07 | Keep the case in the repository while agents keep working | This document and raw conversation source; commit/register integration follows normal authorized source workflow |

## Why it has taken so long

**The scope is substantially larger than a bidirectional parser/emitter.** It
includes multiple compiler frontends, a portable semantic model, ownership and
effect preservation, formal backends, complete grammar-source unions, dynamic
knowledge interchange, discovery algorithms, releases and downstream workloads.
Several of these are individually substantial projects. Their dependencies do
not disappear when grouped into a single issue.

**A new syntax case repeatedly exposes missing semantic infrastructure.** The
class register example and `Vec<&Self>` example above are concrete cases.
Module loading additionally needs TDZ, stable failure identity, shared mutable
cache state, actual capability authority and exact text transport. Those are
required behavior, not incidental polish. A translator cannot safely infer
ownership or object identity from equal names or structural resemblance.

**Earlier work often advanced subsets rather than whole consuming modules.**
Frequent small commits can leave many implemented helpers without one complete
own-project translation path. Continuing integration then uncovers missing
callers, schema fields, qualifiers or effect behavior. “Frontend integration
remains unfinished” describes precisely such a gap; repeating it at the end of
a turn did not satisfy the instruction to continue. That was a coordination
failure, and the current work must close actual consumers rather than use
another handoff as the stopping criterion.

**The progress denominator was unreliable.** Mapping a requirement to paths
does not show its complete implementation or controls. Treating historical
mapping coverage as a drafted percentage creates apparent near-completion
without measuring the remaining constructs. This case intentionally avoids
another percentage until that audit exists.

**Feedback is deliberately deferred.** Under the current instruction we cannot
measure current runtime success or use CI failures to correct the draft.
Source inspection catches some bugs, but not all parser, type inference,
generated Rust, target compiler, wire-format or semantic failures. Those
unknowns accumulate until verification. This policy explains part of the
forecast uncertainty; it is not authorization to violate the requested phase.

**Automation has a bootstrap cost.** Improving JS first and translating it to
Rust avoids permanent duplicated semantics, but an unsupported language feature
can require improving the translator before the new shared algorithm can ship.
Source schemas, codecs, catalogs, fixtures and public interfaces must agree.
Adding more isolated policy roots does not remove that integration work.

**Parallel work has serial joins.** Three agents can author separate areas, but
the global heap, call instruction model, source position schema and checker
boundaries have shared owners. Root must integrate them consistently. Four
workers therefore do not imply fourfold elapsed speed. Extra agents on the same
interfaces would add conflicts and duplicated assumptions.

**Resource and operational constraints are real.** The conversation records
disk exhaustion and explicit limits on cleanup and local computation. The
current source-only work avoids builds, clones and generated payloads. This
analysis has no complete historical resource telemetry, so it cannot assign a
percentage of delay to disk, CI, agent activity or user pauses. Doing so would
invent evidence. Likewise, commit gaps alone cannot show time “lost to CI.”

**The documentation and compatibility surface has grown.** The vision and
handoffs preserve a large history, while old assertions, source profiles and
aliases must survive. Re-reading or re-describing all of that on every change
costs time and obscures the current critical path. The remedy is one compact
current integration checklist linked to retained history, not deleting the
requirements or copying another enormous competing specification.

## Forecast model and assumptions

These are planning judgments, not a statistical confidence interval or a
promise. There is no measured end-to-end delivery velocity from which a
probabilistic finish date can be calculated. Novel proof and discovery work
has a particularly large upper tail.

An **engineering hour** below is an estimated hour of focused implementation,
integration or repair effort by one effective contributor. It is not a model
token budget or an observed hour of agent runtime. In particular, no measured
conversion from these judgment-based effort units to this model team's wall-clock
throughput exists yet; the calendar figures are conditional planning scenarios,
not empirically calibrated predictions. A **team day** assumes eight
active working hours with root and up to three agents, but only two contributors'
effective throughput after shared-interface coordination and serial integration:
approximately sixteen engineering hours per team day. Five such days form a
calendar workweek. These assumptions are deliberately less than a fourfold
speedup and do not assume around-the-clock availability.

The source owners supplied these narrower estimates on November 1:

| Area | Optimistic effort | Likely effort | Adverse effort | Important exclusions |
| --- | ---: | ---: | ---: | --- |
| Exact Scope/loader source path and stabilization | 24–48 h | 80–160 h | 240–480 h | Remaining whole-project JS constructs, broad formal integration |
| Bounded direct borrowed-vector / `children` profile | 12–20 h | 24–48 h | 72–120+ h | Whole `map_children<E>`, arbitrary traits/generics, complete downstream Rust |
| Selected destructor-free own-code control/ownership closure | 20–40 h | 60–120 h | 180–300+ h | Full Rust, unsafe/custom Drop; assumes generic/Vec/Box/signature integration lands |
| Full JS class/module/VM/decoder area | 160–300 h | 500–900 h | 1,200–2,000 h | Other Rust areas, grammar catalog and broader ontology/formal/discovery scope |

These are implementer estimates, not independent measurements, and their
boundaries overlap. Do not add the first row to the fourth. The borrowed-vector
estimate excludes unknown target/compiler repair. No CI queue time is included.
Whole-project and full-scope ranges below add explicit integration and repair
allowances rather than pretending these narrow estimates cover everything.

| Milestone, unchanged required quality | Optimistic remaining effort | Likely remaining effort | Adverse remaining effort |
| --- | ---: | ---: | ---: |
| Selected Scope + provider + borrowed method + completed-loop source integration, with controls drafted | 40–80 h | 120–240 h | 400–800 h |
| Entire own-project JS/Rust source code/control draft | 120–240 h | 300–700 h | 1,000–2,000 h |
| Entire own-project baseline including permitted execution, generated native closure and repair | 240–480 h | 1,000–2,000 h | 3,000–6,000 h |
| All inherited code/control drafting, including broader grammars, concepts, dynamic interchange, formal and inference work | 600–1,200 h | 1,500–3,500 h | 5,000+ h |
| Full bounded release scope, including all later verification, consumers and publication | 1,000–2,000 h | 4,000–8,000 h | 12,000+ h; unrestricted scope has no finite guarantee |

The “likely” own-project draft is approximately 19–44 team days, about four to
nine workweeks. Its verified baseline is approximately 63–125 team days, about
three to six calendar months **if its verification can start after that baseline
draft**. That last condition is not the current global phase policy.

Under the current instruction to draft **every** requirement before testing or
addressing CI feedback, the likely full code/control draft alone is about
94–219 team days, roughly five to eleven calendar months. The baseline's
verification then still has to execute and be repaired. Therefore the three-to-six
month baseline-delivery estimate cannot honestly be offered as the current-policy
finish date. Under that policy, a baseline delivery could instead fall roughly
six to fifteen months out; it depends on how much baseline repair overlaps the
broader verification phase. Full bounded delivery at 4,000–8,000 hours is about
250–500 team days, roughly twelve to twenty-four calendar months. These broad
estimates have low confidence and may move substantially after first execution.

The optimistic full-draft scenario is roughly two to four months and assumes
the current implementations cover far more than has been audited, most work is
generic reuse/integration, and no major redesign is needed. The adverse scenario
has no honest fixed completion date for open-ended semantic equivalence,
natural language or arbitrary future languages. Nothing in the commit history
supports a promise that the complete scope will finish in a few more hours.

## Critical path and how to reduce elapsed work

The current source critical path is:

`actual source inventory → shared JS heap/calls/modules and Rust ownership →
complete actual project consumers → persisted graph/contracts and all emitters →
automatic native generation closure → full broader code/control draft →
permitted execution and CI repair → clean consumers/publication`.

Grammar, concept and discovery tasks can be drafted alongside parts of that
path when they do not require the same shared interfaces. They cannot be
declared complete simply to unlock testing. The following improvements respect
the current source-only phase:

1. Close one **whole real source consumer** per batch: Scope, project-data
   provider, `children`, complete loop caller, then their importing modules.
   Require parser, checker, graph transport, emitter, generation recipe and
   independent control code together. A new unused helper is not that exit.
2. Reuse one shared algorithm for all runtimes; spend effort on missing
   translation constructs instead of duplicating behavior in Rust. Extend
   explicit semantic instructions and decorators where they unblock multiple
   actual modules.
3. Give each agent one owned interface and a concrete caller. Keep shared heap,
   instruction and schema edits coordinated. Root maintains a short critical
   path and integrates; root alone commits and pushes coherent batches.
4. Keep future consumer reports failure-accumulating and complete. Once
   verification is permitted, batch failures by shared cause rather than repair
   individual assertions independently. Preserve every original input and gate.
5. Keep requirement review distinct from runtime evidence. A source review can
   identify a missing body, codec or caller now, without running a fixture or
   manufacturing a success observation.
6. Use bounded, source-authenticated recipes for large fixture inputs and
   compact graph/text representations. Avoid tracked external grammars, local
   builds, duplicate clones and repeated expanded payloads.
7. Maintain one forecast with changed assumptions called out. More commits,
   more mapped paths, or more control declarations alone must not shorten it.

A baseline-first verification policy would shorten the time to an actually
usable translator and reduce accumulated integration risk, while keeping all
other obligations scheduled. It is presented here only as a scheduling
counterfactual. The active instruction remains full drafting before tests and
CI-feedback work; no phase change is made by this document.

## Re-estimation and documentation duties

Re-estimate after a complete actual module's source path is integrated, after
automatic generation first succeeds, after the first permitted full JavaScript
and native report, and after the first whole-project generated consumer. Record
the exact commit, scope, remaining construct families, effort assumption changes
and actual CI queue/run timings if later available. Do not infer unobserved
work hours from timestamps.

The later fresh source/ledger audit must reconcile newly captured instructions,
review every row's complete implementation/control/public/runtime scope and
separate actual verified, partially drafted, undrafted and ongoing operational
obligations. This case and its timing appendix must never be used as executable
acceptance evidence. The work continues while this analysis is written; a case
study, checkpoint or handoff does not finish the translator requirements.

## Capacity update after the November 1 analysis

The later user instruction is: “Slowly allow subagents to finish and continue
yourself.” Existing agents finish only their active coherent batches, without
new assignments; root then continues alone. The earlier two-effective-worker
calendar scenario is therefore historical and must not be quoted as the current
operating plan. The [verbatim steering](../../../parity/sources/agent-wind-down-2026-11-01.txt)
is retained for the later source reconciliation.

If the same uncalibrated effort ranges were retained and a solo contributor
provided eight focused hours per weekday, remaining calendar durations would be
approximately twice the earlier two-worker scenario. For example 300–700 hours
of own-project draft effort would mean roughly 38–88 solo days, while 1,500–3,500
hours of complete-scope drafting would mean roughly 188–438 solo days. These are
capacity arithmetic, not measured forecasts of this model's runtime. Actual
remaining effort may decrease as current batches finish; different daily active
hours change the conversion. The project still lacks an empirical conversion
from commit cadence or model throughput to verified end-to-end delivery. A
reliable current finish date cannot be inferred from the agent-count change.

## November 3 bulk drafting interval

The subsequent user authorization restores up to two subagents while root also
writes code. This supersedes the preceding solo-capacity scenario for the current
batch; it does not turn that scenario into a measured finish forecast.

The Git committer timestamps from `ac4531f03` (01:07:57 +07:00) through
`ff58130ab` (02:07:34 +07:00) cover 14 commits in 59 minutes 37 seconds. The 13
adjacent intervals average 4 minutes 35 seconds, with median 3 minutes 57 seconds;
the shortest is 19 seconds and longest 14 minutes 7 seconds. These are arrival
intervals for parallel source batches, not time spent implementing individual
requirements. For example independently drafted changes can commit seconds
apart after substantially overlapping work.

That interval drafts actual exclusive Rust source admission/composition, distinct
collection carriers, primitive compound writes, String accumulation, Vec packets,
closed Hash/BTree operations, Eq/Ord and ordered snapshots. Root connects typed
Links, project/CST ownership, package declarations and genuine library loading.
Five Array methods reuse existing VM opcodes: toReversed, with, sort, toSorted
and toSpliced. One generic native-control driver now includes 150 positive rows
from eight Rust state fixture groups, while the copying/sorting fixtures retain
62 original JavaScript rows. These are source/control inventories, with overlap
possible between examples; none is a requirement completion measurement or an
observed runtime pass.

The actions with broad reach are shared state/packet composition, genuine source
owner checks, one automatic library interface derivation, existing-opcode Array
composition and one reusable original/emitted-product matrix. Keeping primitive
and nominal semantics shared reduces independent native implementations. The
current local work writes source text only; no tests, source generation or Rust
compilation ran. The recorded project cache cleanup remains approximately
510 KiB with zero removed, and the inspected disk reports about 60 GiB free.

The next source frontier is materially larger than a leaf method: genuine generic
FnMut specialization needs original type/callback binders, concrete source body
reparse, mutable environment packets and first-error iterator completion.
Borrowed ordered loops need real source loan roots and control-state publication,
not an owned-array label. Stored/escaping references, general traits/macros,
source module initialization, native frontend/codec publication and complete
downstream programs remain outside the demonstrated source batches. Automatic
generation has not calibrated whether the new batches compose successfully.

Accordingly this faster commit cadence does not justify replacing the earlier
uncalibrated effort ranges with an asserted finish date. The next useful forecast
checkpoint is a complete actual own-project module's integrated source path and
the first permitted successful automatic generation, followed by measured
original/product/runtime results after the full draft gate. Until those inputs
exist, quote concrete drafted families and known gaps rather than a 100% date
derived from commits, fixture counts or mapped paths.

The later source continuation adds a bounded canonical Links reader before asset
insertion, borrowed BTree loop integration, a complete source stable-sort template
with one opaque payload type slot, a nominal UTF16 comparator adapter and literal
character String append. The same original/product driver now names fourteen
groups and 275 literal positive Rust originals. The increment from 150 describes
new authored controls; it is not 125 completed requirements. Owned Vec into-loop
admission is drafted to connect the sorting originals to the actual frontend
instead of changing those originals to bypass iteration. Generic FnMut admission
now has a bounded residual-only opaque-error proof: a body valid only for a
selected `u8` specialization cannot certify the original universal declaration.
Those composition obligations explain why a new leaf source body alone is not an
honest delivery boundary. They also identify reusable work: the owned vector loop
will share the existing control/state fold rather than create another emitter.

The next source-only integration grows that inventory to 279 literal originals
in fourteen groups and 51 independently declared scenarios over two actual
current-source method factories: 330 scenario rows in sixteen groups. These are
authored inputs, not observed successes or a count of delivered requirements.
The actual methods retain serde-bearing originals, explicit value projections,
and native calls into the actual locked crate as separate CI assertions. This
consolidates repeated native setup while preserving the reference distinctions.
Weak collections, Symbol registry and weak generic construction also connect to
the existing per-group native replay instead of adding another execution engine.
Agents finish only their existing batches; subsequent drafting proceeds solo.
No local behavioral execution, generation, validation or compilation calibrates
these source changes, so the earlier forecast limits still apply.

A further solo source pass preserves all 23 children-length scenarios and adds
nine independent child value/order observations. The authored driver inventory
becomes 339 scenario rows (279 literal originals and 60 actual-source scenarios).
This closes an oracle blind spot in the planned controls: correct lengths alone
do not distinguish exchanged, duplicated or incorrect child variants. These
observations remain unexecuted and do not establish pointer identity or complete
recursive equivalence. They cannot support a completion percentage or forecast.

Copied-reference indexing also promotes the exact former indexed matches source
input into the positive inventory. With that retained input, there are 340
authored positive scenario rows (279 literal originals and 61 actual-source
scenarios). Two added source-owner/static-lifetime refusal inputs cover indexed
results. These are source drafts without observations.

The final agent finished its current region batch at `fa925b892` and stopped.
Root now works alone. Integration adds eight literal mixed-region originals and
three complete actual method compositions to the same CI-only driver, yielding
351 authored positive rows across eighteen groups (287 literal and 64 actual
source scenarios). The new three-case composition group has explicit subset
scope; existing full-variant assertions remain unchanged. These inputs provide
more composition coverage but remain unexecuted and do not measure completed
requirements. No new agent or parallel batch is started.
