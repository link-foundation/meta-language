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

## Open work

- A source syntax scan found an extra parenthesis in the ordinary class
  closure control. After that repair, static linking reaches the missing
  class, collection and module initialization codecs. A static import scan
  identified seven references to those three missing files. The Rust test
  modules also require the corresponding generated native codecs.
  The existing `draft-javascript-error-source-kernels.mjs --draft` recipe on
  supported Node 24.14.0 fails in `JavaScriptParser.typedefs` with
  `expected : in object type but found "?" at 264..265`. Complete optional
  field translation through the source checker, native emitter, and wire
  transport before generating class, collection and module-initialization
  codecs from their actual checked declarations. The initializer's separate
  author hits the same translator error at `1589..1590`.
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

Reviewed the four exact paper revisions named by #224. The public artifact
commits below were resolved independently on November 22, 2026. They identify
inputs for a comparison run. Benchmark results and portability remain open.

| Method | Pinned artifact and review findings |
| --- | --- |
| XVada | [Implementation and experiments](https://github.com/rifatarefin/xvada/tree/5221b183bc6e1ac583505299173fdbae0706da99), MIT. The repository ships training seeds and filename-based parser oracles, and the evaluator supports held-out inputs and a per-input memory cap. Its generated-sample precision is distinct from precision on a fixed labelled holdout. [Paper v4](https://arxiv.org/abs/2607.08959v4) |
| Crucio | [Implementation](https://github.com/Sinpersrect/crucio/tree/ce913637763f0b25017d38e0e01944f54cf86d38) and [benchmark](https://github.com/Sinpersrect/gi-benchmarks/tree/aab9b09e1af68eed73fb07231b8e0bbd4870cdf2). GitHub reports no SPDX license for either repository, and neither pinned root contains a license file. The paper used ANTLR grammars for the larger language oracles and removed comments from their samples. Preserve independent trivia-bearing controls for the Links CST requirements. [Paper v1](https://arxiv.org/abs/2601.12385v1) |
| NatGI | [Implementation and experiments](https://github.com/rifatarefin/natgi/tree/561f35a9bc9fa02c62de6d2ebfbc6266357fe414), MIT. The paper describes occasional non-determinism with temperature zero and seed 101. Reproducing the model-assisted mode needs the exact proposal transcripts or the model and a record of every resulting proposal. Oracle acceptance and checked concept equivalence remain separate. [Paper v2](https://arxiv.org/abs/2509.26616v2) |
| Attributed inference | The reviewed preprint and conference entry provide the method and seven preliminary subjects. Neither links the authors' implementation or held-out input files. The evaluated extraction covers successful paths of recursive-descent parsers and misses some semantic behaviors, including exception paths. The project still needs its own pinned implementation and separate exceptional-behavior controls. [Paper v1](https://arxiv.org/abs/2507.13117v1) · [Conference entry](https://conf.researchr.org/details/icsme-2025/icsme-2025-nier/6/Inferring-Attributed-Grammars-from-Parser-Implementations) |
