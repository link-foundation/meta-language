# Issue #227 CI source repairs, November 22, 2026

[PR #228](https://github.com/link-foundation/meta-language/pull/228) continues
[issue #227](https://github.com/link-foundation/meta-language/issues/227) and
[grammar discovery issue #224](https://github.com/link-foundation/meta-language/issues/224).
The [remaining-work plan](issue-227-remaining-work-plan-2026-10-31.md) and its
two requirement appendices retain the full scope.

## Published CI baseline

[Run 42129692629](https://github.com/link-foundation/meta-language/actions/runs/42129692629)
was triggered for head `3b77cb71a42c091c89d952f24acfe652ee95a1a1` and checked
out merge candidate `7db1c94b77f90523cdd989c4749f02c415fc9034`. GitHub reported
77 failures, 19 skipped jobs and two passes. The branch already contains
default-branch commit `ec9f4efb0e9ef936114e09823981352f6db13700`. Complete log
archives for the latest five runs and selected job logs are retained locally
in `ci-logs/`.

[Default-branch CI 39654536127](https://github.com/link-foundation/meta-language/actions/runs/39654536127)
reported 14 failures before cancellation. The current branch already contains
the null-domain guard in Rust emission, the binary-candidate refusal diagnostic,
and byte-balanced reporting for requested shard counts other than eight. Those
changes correspond to the published failures at log lines 2850, 859 and 799.
The raw descriptor failure at line 444 and the exact-commit outcome of the
remaining default-branch checks still need CI evidence.

| Failure | Draft repair and reproduction |
| --- | --- |
| Duplicate `error` declaration prevents the Rust frontend from loading | Give the callback-specific diagnostic its own binding. The existing borrowed iterator test imports the affected frontend. [Report job](https://github.com/link-foundation/meta-language/actions/runs/42129692629/job/127602873465) |
| All JavaScript group jobs encounter 42 unassigned files | Register each exact filename in an existing group. The existing test-group check verifies that every test file has one owner. Refresh the classifier history with the existing author. [Group job](https://github.com/link-foundation/meta-language/actions/runs/42129692629/job/127608680399) |
| The Rust source version job lacks a cache teardown event | Supply `--event ci-teardown` to the existing project-scoped command. The workflow cache policy check requires the event. [Group job](https://github.com/link-foundation/meta-language/actions/runs/42129692629/job/127608680399) |
| Package contracts detect stale inventories, an unknown callables source pin and missing case study metadata | Refresh the source register, requirement manifest, declaration inventory and compact naming history with their authors. Restore the historical index. [Contracts job](https://github.com/link-foundation/meta-language/actions/runs/42129692629/job/127602873881) |
| TypeScript 7.0.2 reports `TS2820` for the unchanged unknown const signature literal | Select the exact observed diagnostic by compiler version, retain the existing `TS2322` expectation for 5.9.2 and require uncovered versions to report the missing policy. The public declaration control exercises all three selections. [Generated package job](https://github.com/link-foundation/meta-language/actions/runs/42129692629/job/127602873736) |
| Dependency Refresh on `main` completes the audit but fails to publish its automation branch | Publish a forward-only commit that preserves the refreshed tree and prior branch history. The regression control uses a Git remote configured to reject non-fast-forward pushes. The retained log records `GH013` at line 928. [Dependency Refresh run](https://github.com/link-foundation/meta-language/actions/runs/43868840534) |
| The binary-buffer author refuses the current emitted function before applying the four reviewed compiler corrections | Recover the complete previous function from the exact emitter successor by reversing 51 generated name changes and one call's formatting. The normal source author generated all 74 owners and reproduced identical source through both typed archives. Native compilation and runtime checks remain CI work. |
| The module-initialization source author pulls in an unrelated call machine for one coordinate check | Share the original coordinate check directly. The normal authors produced both module-initialization libraries, their checked source assets and codecs, and the standalone literal fixture. The Rust source author applies formatting and the native assertion now passes the generated UTF-16 text input type. Logs `ci-logs/module-initialization-source-author-selected.log`, `ci-logs/module-initialization-companion-source.log` and `ci-logs/module-initialization-source-formatted.log` record the successful source commands. |
| Native test declarations include baseline fixture files and the Rust standard-sum method policy that were absent from the tree | Run the existing authors for the standard-sum method policy and 22 literal fixture files. The primitive-wrapper author preserves the exact allocation descriptors for NaN, Infinity, BigInt and unpaired UTF-16. The package allowlist and checked program catalogs include the generated policy. Log `ci-logs/baseline-dependent-source-publication.log` records the successful source command. |
| The Rust generic alias policy is missing its native source and checked Links program | Compose the complete source with lexical dependency aliases and append the existing template parameters in order. The author emitted the Rust source and paired Links assets. Log `ci-logs/baseline-alias-source-publication.log` records successful catalog publication. |
| The property descriptor author inferred a plain record in a tagged declaration | Register the referenced record alias before the tagged declaration and compare request flags in their original order. The source author produced the exact UTF-16 native type and paired Links assets. The existing native control now uses the generated type. Logs `ci-logs/javascript-property-descriptor-author.log` and `ci-logs/property-descriptor-source-publication.log` retain the source results. |
| The Array object source was refused during type checking, graph publication and Rust emission | Use the shared array projection for the lowered `for…of` length read, attach the IEEE semantics required by the typed Links archive to signed-zero SameValue divisions, and retain ordered Array property traversal in explicit source loops. The normal source author generated 14 Rust files and the paired Links assets. Logs `ci-logs/baseline-array-source-ordered-traversal-dependencies.log` and `ci-logs/dynamic-discovery-and-array-source-publication.log` record the output. |
| Mutable loan, owned record loop and update, and callable witness assets were absent | Retain the original value and work checks in source forms the translator can lower. The regular authors produced the native libraries and paired program assets. Logs `ci-logs/baseline-array-and-record-source-publication.log` and `ci-logs/baseline-callable-source-author-conditioned-whitespace.log` record authoring outcomes. Native compilation and control execution remain outstanding. |
| The Rust frontend declared missing baseline policy libraries | At the initial source inventory, 60 module declarations had no file. The normal authors produced 50 of those libraries and the matching JavaScript and Rust Links assets. Source logs include `ci-logs/baseline-recursion-and-disjoint-source-retry.log` and `ci-logs/baseline-class-loans-and-projection-sharing.log`. Rust compilation remains outstanding. |
| Closed source class composition and its supporting libraries were missing | The checked source provides the hidden class brand through the existing Unit-returning form, retains the closed export boundary, and preserves borrowed call arguments after nested generic parameter types. The authors produced the class lowerer, its observation and Error classifiers, and the two native Scope composition files. The class controls retain 72 positive and 45 refusal cases. Source generation completed. The controls still need execution. |
| The collection projection author exhausted its JavaScript heap | The optional memory trace located the failure after Rust emission. An inspector sample located repeated full-library scans in the equality lint proof. Shared token spans, detached action strings, and lazy contextual proof selection let the author progress without changing lint admission. The exact replacement pass now visits each identical generated block once. The source log is `ci-logs/baseline-collection-projection-exact-context.log`. |
| The first complete collection projection archive exceeded the catalog's 100,000-node limit | The existing ordered record-field encoding shares field layouts and retains each record occurrence. The resulting 41,704-line archive produces identical Rust source. The catalog publisher registered the projection, the iteration rules, the closed class composition, the Error class rules and the owned recursive datatype rules in both packages. Source logs `ci-logs/baseline-collection-projection-record-shapes.log` and `ci-logs/baseline-class-and-projection-catalog-ordered-records.log` retain those outcomes. |

## Open work

- A source syntax scan found an extra parenthesis in the ordinary class
  closure control. After that repair, static linking reached missing class,
  collection and module initialization codecs. The module initialization and
  Error class authors have since produced their libraries and codecs. The
  Rust tests still require the collection call library and its codecs. The collection
  key policy has generated Rust and paired Links sources. Its author uses
  constructor-specific field types and explicitly guarded merge ordering to
  retain the original comparison and storage order.
  The existing `draft-javascript-error-source-kernels.mjs --draft` recipe on
  supported Node 24.14.0 fails in `JavaScriptParser.typedefs` with
  `expected : in object type but found "?" at 264..265`. Complete optional
  field translation through the source checker, native emitter, and wire
  transport before generating the remaining codecs from their actual checked
  declarations. The same closure contains tagged unions composed from named
  alternatives, including `JavaScriptClosureObject` and
  `JavaScriptBoundFunctionObject` inside `JavaScriptOrdinaryObject`. The
  frontends currently require each alternative to start with a literal object
  type. A field-presence representation must retain omitted fields from older
  snapshot editions, own fields whose value is JavaScript `undefined`, and the
  distinction between an absent field on one union alternative and an optional
  field declared on that alternative.
  Required constructor fields now use the recursively parsed type grammar in
  both frontends. The source author generated the Rust boundary scanner from
  the JavaScript scanner. Both frontends now represent optional field presence
  and reuse previously declared alternatives. Source authoring completed with
  shared fixtures for nested type grammar, optional presence and literal tag
  selection. Exact runtime coverage and reverse translation remain open.
- Exact fixture templates can now substitute initialized text and retain its
  source and UTF-16 units. The native assertion author translated all 54
  borrowed-iterator assertions and all 15 array-object assertions from their
  actual sources. Generation of the mutable loan and callable rule libraries
  removes the initial authoring failures. The full native assertion author
  translated all 45 modules and registered their original fixture coordinates.
  The per-module receipts retain exact canonical Links text with bounded
  decoding and content hashes. See
  `ci-logs/baseline-native-policy-complete-compressed-receipts.log` for the
  completed source trace. Native compilation and execution remain outstanding.
- The compact naming author refreshed the current callables pin with both
  historical inventories intact. With bounded Git object reuse, the
  additional author captured all 1,202 selected epochs in 23 seconds. The
  timing trace located the first timeout during inventory loading. After
  retaining exact token validation through the faster reader, the next
  gate rejected a 91 MiB document at the existing 64 MiB ceiling. Bounded
  Git object reads and ordered continuation documents brought the author
  and independent reader within their existing deadlines. The normal
  `--additional-inventories` command authenticated 1,203 original sources,
  retained 567 ordered views and 3,884 names, and reproduced every byte of
  the 22 archive documents. The index SHA-256 is
  `01ee9f91273d69240668100c2488c69c06a0ab557711d58752004d91f0f9b28d`.
  The source-only run is retained in
  `ci-logs/naming-single-read-paged-fresh.log`. The storage, token and
  certificate regression controls still require execution.
- The raw consuming package job in run 39830836214 refused complete artifact
  `468566b3bd80bccc46a078d89dd8eef801f2189053927e7674b1d95cb6faac55`
  at line 469 of the retained log. Its existing archive metadata adapter
  recognizes that exact fingerprint. The current producer and receipt chain
  still need CI evidence, as do the earlier report and translation deadline
  failures. The module loading failure masked those paths in the latest
  published run. [Raw package job](https://github.com/link-foundation/meta-language/actions/runs/39830836214/job/120022678167)
- The register refresh matched 67 live requirement sources and 14 status
  reports. The authored manifest has 473 requirements, 494 fixture identities
  and 1,017 verification cells. Eighty requirements lack implementation entry
  points for at least one required runtime, including twelve in grammar
  inference. The exact-commit tests, native compilation, package installation,
  downstream workloads and acceptance observations remain outstanding.

## Grammar discovery research inputs

The source draft adds `language discover`, `language refine` and `language
install` in both command-line packages. They use the existing regular learner,
checkpoint journal and dynamic parser, with versioned text and binary archive
transport. The shared control deletes source example files as it goes and
checks both CST byte reconstruction and rejection after installation. The
command vocabulary and fixture authors completed without running the controls.
Finite scanner-state transitions can now be inferred from explicit mode,
spelling, token and successor-mode observations. The native scanner fragment
orders overlapping spellings by length, refuses conflicting observations and
round-trips through the grammar listing. The draft fixture exercises its
token boundaries in both runtimes. A second draft control discovers, refines
and installs the scanner without training files and parses Unicode input
through the installed grammar. The optional character class profile groups
observed Unicode scalars that have identical scanner transitions. Draft
controls check range gaps, longer literal prefixes, exact accepted trees,
checkpoint reload and refinement in both runtimes. Its source author produced
a checked program and native counterpart. Full before-and-after mode stacks
now supply the finite observations for native push, pop and replace. Shared
controls exercise nested pushes, the same pop restoring different parents,
conflicting effects and source-free archive replay with counterexamples. The
portable planner and its Rust counterpart are generated, and both controls
still require execution. A second shared learner derives recursive productions
from labelled token examples and disjoint delimiter pairs. Its draft library
controls cover nested inputs outside the training examples, source bytes,
crossed delimiters, contradictory merges and text and binary checkpoint replay.
The shared planner, native counterpart and archive integration are drafted.
Structural discovery has CLI dispatch and a checkpoint refinement journal in
both runtimes. Concrete parser observations constrain a finite native attribute
action vocabulary. The draft also projects attributes from an installed grammar
through checked Links programs to Rust and JavaScript source. Installed grammar
merges retain their source-qualified inputs and production provenance and are
rederived when the archive is loaded. The controls still need execution.
Generalization beyond observed characters, broader attributed inference,
translation across complete source languages and held-out benchmark measurements
remain open.

The pinned upstream link-cli importer reads one two-endpoint row per line.
Its native binary store archive also stores doublets, with exact addresses and
names. Both packages now draft a reversible projection of ordered n-ary links
and separate roots into that format. The shared command replay checks the full
31-step fixture in each package, with real Link CLI 3.1.1 import and export in
separate processes. It deletes the import packet before export, removes the
original grammar inputs after preparation and compares the canonical archive
bytes after eager restoration from all three formats. The preparation and
archive controls have yet to execute. The replay records elapsed time and
peak resident memory for the selected fixture. Catalog-wide workloads and
their measurements are still required.
[Pinned link-cli binary archive format](https://github.com/link-foundation/link-cli/blob/fbc90908772dd8117c4d1be05bc16defd1cb9540/docs/protocol/binary-links-notation.md#10-store-archive)

The source generator now handles 54 of the 60 previously ungenerated Rust
baseline modules. An independent native lint-policy draft resolved 39 policy
groups and produced 1,199 declaration checks. Six groups refused optional
JavaScript constructor fields at the last full authoring attempt. Since that
attempt, the frontends gained tagged field presence, declared alternatives
and finite conditional tags. The Error construction and ordinary class rule
authors generated native source. The Error class compiler passed its earlier
tagged object spread after both frontends learned to preserve optional-field
presence and evaluation order across finite literal choices. The JavaScript
frontend emitted the shared regression fixture to JavaScript and Rust.
The collection iterator callable source author produced its native code,
typed program and source receipt in
`ci-logs/javascript-collection-iterator-append-source.log`. The callable
directory shares its construction authority and source-span schema with the
call-code module. Its stable merge orders exact unsigned64 identities, and
the draft regression checks ties, wide identities and input preservation.
The checked call-source closure regenerated in `9232bc77c` with 25 Rust
parts and a pair of byte-identical Links archives. The generator memoizes
validation of shared type graphs and still checks expanded type size against
the original limit. The fixture codecs preserve absent fields and explicitly
present `undefined` fields. The shared regression exercises both forms in
JavaScript and Rust. Source authoring completed in
`ci-logs/javascript-call-source-presence-codecs.log`. The Error class compiler
generated 41 Rust parts and both fixture codecs in `47e83d773`. The qualified
direct-call match profile in `ad27f4d98` retains the checked call and ordered
arguments for each source pattern in its receipt. It brought the paired
archive from 109,062 to 96,052 links. The catalog and Cargo include the complete
archive. Source logs `ci-logs/javascript-error-class-source-direct-calls.log`
and `ci-logs/error-class-source-publication.log` record the authoring commands.
Native compilation, regression execution and execution of the generated
lint-policy assertions remain pending.

The draft JSON holdout measures RPNI, EDSM or ALERGIA in a separate process
against the same pinned JSONTestSuite corpus and budgets. It keeps training
examples out of scoring, reports resource refusals and oracle disagreements
per file, and compares corpus labels with Node and Python parsing. It also
measures byte reconstruction, leaf spans, grammar rules and bytes, query
counts, process peak memory and elapsed time. The experiment source has been
committed with a dedicated CI shard per strategy and artifact upload of
the emitted grammar, rejection diagnostics and disagreements. It awaits
execution. Its regular learners supply one baseline for the requested
comparison. [Pinned JSONTestSuite corpus](https://github.com/nst/JSONTestSuite/tree/1ef36fa01286573e846ac449e8683f8833c5b26a)

The XVada CI shard uses the same holdout with the upstream model-free
TreeVada and HDD configuration, Lark 1.2.2, and a seeded Python process.
XVada receives the positive training examples and queries a CPython JSON
membership oracle. The regular learners receive explicit positive and
negative examples. The report includes the actual XVada membership queries,
the generated grammar, the repeat outcome and the result for every eligible
holdout input. Training exhaustion leaves the comparison scores unmeasured.
The two process memory figures cover different programs and are recorded as
such. No XVada comparison has run yet.

The same comparison now admits the pinned model-free NatGI bracket-guided
learner and the pinned Crucio lexical and distributional learner. Crucio's
Cython extension builds in a temporary directory from the upstream source.
Its wheels and the source-only colored dependency have exact SHA-256 pins.
The job keeps the original checkout, progress diagnostics, membership traces
and resource-limit outcomes. There is a 90-second training allowance and a
separate 90-second evaluation allowance per learner run. Holdout rows whose
bytes occur in either membership trace are excluded from the score and
retained with their outcomes.

Every JSON strategy receives the same evaluation sources. In addition to
JSONTestSuite, the draft selects the 64 largest eligible JSON examples from the
pinned Crucio benchmark and the pinned npm CLI and TypeScript project
manifests. Rows outside the sample or byte budget remain in the report.
Node and CPython independently label project inputs, and scores are available
per corpus and per oracle. The experiment awaits CI execution.
[npm CLI source](https://github.com/npm/cli/tree/417daa72b09c5129e7390cd12743ef31bf3ddb83) · [TypeScript source](https://github.com/microsoft/TypeScript/tree/c63de15a992d37f0d6cec03ac7631872838602cb)

The attributed holdout is a bounded reproduction of the successful-path
trace approach. A separate scanner and regular expression characterize
ordered lists of ASCII decimal quantities with a common unit. The experiment
checks the inferred sum against that oracle and compares it with syntax-only
learning. It retains disjoint positive and adversarial holdouts, complete CST
projection, exact spans and bytes, ambiguity, and repeat outcomes. Broader
attribute inference and exceptional parser behavior still need coverage.

Reviewed the four exact paper revisions named by #224. The public artifact
commits below were resolved independently on November 22, 2026. They identify
inputs for a comparison run. Benchmark results and portability remain open.

| Method | Pinned artifact and review findings |
| --- | --- |
| XVada | [Implementation and experiments](https://github.com/rifatarefin/xvada/tree/5221b183bc6e1ac583505299173fdbae0706da99), MIT. The repository ships training seeds and filename-based parser oracles, and the evaluator supports held-out inputs and a per-input memory cap. Its generated-sample precision is distinct from precision on a fixed labelled holdout. [Paper v4](https://arxiv.org/abs/2607.08959v4) |
| Crucio | [Implementation](https://github.com/Sinpersrect/crucio/tree/ce913637763f0b25017d38e0e01944f54cf86d38) and [benchmark](https://github.com/Sinpersrect/gi-benchmarks/tree/aab9b09e1af68eed73fb07231b8e0bbd4870cdf2). GitHub reports no SPDX license for either repository, and neither pinned root contains a license file. The paper used ANTLR grammars for the larger language oracles and removed comments from their samples. Preserve independent trivia-bearing controls for the Links CST requirements. [Paper v1](https://arxiv.org/abs/2601.12385v1) |
| NatGI | [Implementation and experiments](https://github.com/rifatarefin/natgi/tree/561f35a9bc9fa02c62de6d2ebfbc6266357fe414), MIT. The paper describes occasional non-determinism with temperature zero and seed 101. Reproducing the model-assisted mode needs the exact proposal transcripts or the model and a record of every resulting proposal. Oracle acceptance and checked concept equivalence remain separate. [Paper v2](https://arxiv.org/abs/2509.26616v2) |
| Attributed inference | The reviewed preprint and conference entry provide the method and seven preliminary subjects. Neither links the authors' implementation or held-out input files. The evaluated extraction covers successful paths of recursive-descent parsers and misses some semantic behaviors, including exception paths. The project still needs its own pinned implementation and separate exceptional-behavior controls. [Paper v1](https://arxiv.org/abs/2507.13117v1) · [Conference entry](https://conf.researchr.org/details/icsme-2025/icsme-2025-nier/6/Inferring-Attributed-Grammars-from-Parser-Implementations) |
