# Issue #248 · Temporary pause on automated translation

> **Historical case study.** See [the vision](../../vision.md#javascript-first-and-self-translation) for the current policy.

[Issue #248](https://github.com/link-foundation/meta-language/issues/248) asks
for a reversible CI pause so work on other features and fixes can proceed with
manual ports. [evidence.lino](evidence.lino) records the exact issue body and
digest, the open pull requests at collection time, and baseline job excerpts.

## What the existing checks do

[Prior work in PR #228](https://github.com/link-foundation/meta-language/pull/228)
split the self-translation report into separate jobs. The report inventories
source modules, emits translation artifacts, and checks them against the tool's
current output. The stricter acceptance stages also run native JavaScript,
Rust, Lean and Rocq controls, compare results, and verify the immutable
evidence for each assertion. The standalone JavaScript workflow on the
baseline commit `ec9f4efb0e9ef936114e09823981352f6db13700` ran for approximately
54 minutes and 29 seconds. That duration is wall time for the whole workflow,
including scheduling and independent work. It does not measure peak memory
or the time a paused PR run will save. Six failed translation-related jobs
accounted for 39 minutes and 59 seconds of job time. Those intervals overlap
and include setup and teardown. The workflow also had cancelled translation
and report shards.
[Baseline run](https://github.com/link-foundation/meta-language/actions/runs/45059254685)

| Baseline job | Observation and implication |
| --- | --- |
| Translation 1/6 | Local and historical relative-array inputs failed their provenance checks at log lines 903 and 1095. These are kept with the translation suite for opt-in runs. [Job log](https://github.com/link-foundation/meta-language/actions/runs/45059254685/job/137321255822) |
| Translation 4/6 | The report sizing assertion failed at lines 916 and 924 with shard sizes of 1,106,047, 1,117,099 and 1,554,120 units. Sharding distributes this work and still requires all report cells. [Job log](https://github.com/link-foundation/meta-language/actions/runs/45059254685/job/137321256171) |
| Generated package checks | The owned-tuple profile failed at line 2971. Separate jobs reported failures in the native source-model check and the raw consuming emitter. The pause gates these executable translation paths. [Generated](https://github.com/link-foundation/meta-language/actions/runs/45059254685/job/137333777730), [native projection](https://github.com/link-foundation/meta-language/actions/runs/45059254685/job/137333777690), [consuming emitter](https://github.com/link-foundation/meta-language/actions/runs/45059254685/job/137333777743) |
| Binary candidates | The modified-input regression at line 1000 expected a frozen candidate model refusal and received a digest assertion. The native candidate profiles and their original assertions remain in the opt-in path. [Job log](https://github.com/link-foundation/meta-language/actions/runs/45059254685/job/137333777739) |
| Conformance and tooling | Naming failed at lines 854–879 and an inherited assertion's source comparison failed at line 1145. These checks need independent fixes and remain active. [Conformance](https://github.com/link-foundation/meta-language/actions/runs/45059254685/job/137321256178), [tooling](https://github.com/link-foundation/meta-language/actions/runs/45059254685/job/137321255968) |

The first complete tooling shard for the pause surfaced assertions tied to
earlier inputs at log lines 1157, 3571, 4049 and 8411. The source checks still
authenticate the original compact instructions and the original six-part
matrix. The contributor fixture also authenticates the single clause that
issue #248 revises. The workflow and source discovery tests exercise both
translation modes and include the new issue and comment endpoints.
[PR tooling job](https://github.com/link-foundation/meta-language/actions/runs/45086449179/job/137406065515)

Tooling partition 4 reached its 15-minute job deadline at log line 1220.
The two new tests had moved the native coverage history and fixture storage
replays into the same partition as the gate mutation test. The scheduler
preserves every established tooling assignment and adds the two controls
to the next slots. A test verifies the full union and placement.
[Tooling partition 4](https://github.com/link-foundation/meta-language/actions/runs/45086449179/job/137406065487)

The retained partition 2 log identified a stale generated manifest at line
955, the historical grammar workflow comparison at line 1440, and a missing
self-translation reference in the contributor guide at line 5557. The grammar
comparison still verifies the complete predecessor through authenticated
source replay. The contributor guide links to the retained opt-in workflow.
[Tooling partition 2](https://github.com/link-foundation/meta-language/actions/runs/45086449179/job/137406065607)

Conformance partition 2 reached the same 15-minute deadline after the runtime
concept record assertions passed. The next suite repeatedly reloaded the
unchanged naming registers and re-extracted source descriptions for in-memory
mutations. The mutation tests now reuse the complete extracted naming inputs
and invoke the same validators as the normal naming command for every mutation.
The historical assertion still verifies its 208 existing rows and now verifies
the 63 pre-pause owners added by this change against their certified source.
The archive retains the complete previous test module.
[Conformance partition 2, log line 914](https://github.com/link-foundation/meta-language/actions/runs/45092978360/job/137426480751)

On the next run, conformance partition 2 passed in 6 minutes 51 seconds, and
tooling partition 4 passed in 12 minutes 12 seconds. Both jobs ran with the
existing 15-minute deadline.
[Conformance partition 2](https://github.com/link-foundation/meta-language/actions/runs/45098269657/job/137442894195),
[tooling partition 4](https://github.com/link-foundation/meta-language/actions/runs/45098269657/job/137442894138)

Tooling partition 1 failed the scratch checkout comparison at log line 1607.
The fixture copied the original required source files and omitted the additional
files that contributed historical naming rows. It now copies every contributing
file, verifies the complete comparison, and checks each altered file. The
original required sources still fail on any change. Altered files added during
the pause lose the historical rows they would otherwise authenticate. The
Git-certified archive retains the previous test file and its naming record.
[Tooling partition 1](https://github.com/link-foundation/meta-language/actions/runs/45098269657/job/137442894177)

Tooling partition 2 compared the current Formal AI validator to an earlier
12,317-byte prefix and looked for four previous naming bindings in the current
Node dependency test file. The original digest, validator export and all four
bindings are now checked against the Git-certified sources. The current
placement test still exercises inventory discovery and rejection of altered
owner expressions. The naming gate also validates the original Formal AI test
body as a witness for its existing concept record.
[Tooling partition 2, log lines 1431 and 1489](https://github.com/link-foundation/meta-language/actions/runs/45098269657/job/137442894199)

All 15 package jobs passed, including the 11 raw schema and consumer
assertions. The JavaScript package job reported the failed tooling shards at
line 1239. The aggregate named the same upstream jobs at line 58 and also
reported their skipped dependents. The pause gate now checks independent
results after the delivery-candidates prerequisite succeeds. The earlier
gate still reports the cause when the prerequisite is missing.
[CI run](https://github.com/link-foundation/meta-language/actions/runs/45098269657),
[JavaScript package job](https://github.com/link-foundation/meta-language/actions/runs/45098269657/job/137485087833),
[aggregate](https://github.com/link-foundation/meta-language/actions/runs/45098269657/job/137485771286)

On `d460599c7`, Delivery Candidates packed both packages at log line 535
and uploaded them at line 589. The clean consumers, Relative Meta Logic
workloads, and Formal AI partitions still showed as skipped. GitHub's default
success condition propagates the skipped translation report through the
dependency chain. Each of these jobs now explicitly requires successful
Delivery Candidates and a run that has not been cancelled.
[Delivery Candidates](https://github.com/link-foundation/meta-language/actions/runs/45112972070/job/137540929225),
[PR run](https://github.com/link-foundation/meta-language/actions/runs/45112972070),
[GitHub job dependencies](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#jobsjob_idneeds)

That run completed with a failed Rust check. Cargo Check reported three calls
to a missing observation helper and four uses of unsupported hexadecimal
formatting from log lines 1149 through 2388. The test repair reuses the existing
SHA-256 string encoder and restores the JavaScript subprocess observations.
The same run found an inline test under `src/` and two files over the line
limit at log lines 1138–1140 and 4425–4426. The corresponding bodies moved
to dedicated files and retain their assertions and inputs.
[Cargo Check](https://github.com/link-foundation/meta-language/actions/runs/45112972070/job/137557959187),
[Rust lint](https://github.com/link-foundation/meta-language/actions/runs/45112972070/job/137557959068)

Clippy reported 199 diagnostics in the same Rust job at log line 4305.
The reviewed corrections preserve the native lint command and pass through
the existing JavaScript generators for generated Rust. The review data keeps
the input module digests, the exact edited spans and their original rendered
diagnostic locations. This data records the source review. Native compile,
lint and runtime results come from CI on the updated revision.
[Rust lint](https://github.com/link-foundation/meta-language/actions/runs/45112972070/job/137557959068),
[review data](../../../parity/self-translation/native-source-style-review-137557959068.lino)

On `35ce88bc8`, nine assertions across three tooling partitions and the
JavaScript downstream partition found that the crate allowlist entry for the
relocated ownership metadata test was missing from the dependency inventory.
The declaration refresh appended that entry and regenerated the audit document
with the same audit date and prior 818 entries. The focused package, inventory,
certified history, and delivery checks passed all 22 tests, including all nine
failing assertions.
[Tooling partition 1, log lines 1262–1324](https://github.com/link-foundation/meta-language/actions/runs/45144165350/job/137589211095),
[tooling partition 3, log lines 762–1025](https://github.com/link-foundation/meta-language/actions/runs/45144165350/job/137589211407),
[tooling partition 4, log lines 920–944](https://github.com/link-foundation/meta-language/actions/runs/45144165350/job/137589211838),
[downstream partition, log lines 750–791](https://github.com/link-foundation/meta-language/actions/runs/45144165350/job/137589211619)

Tooling partition 2 found a stale acceptance manifest at log lines 956–964.
The naming changes had updated the canonical concept register while three
fixtures still carried its previous digest. The manifest refresh updates
those three fixtures and the two fixtures whose input is the extended
dependency inventory. All fixture identities, requirements and assertions
remain in the generated manifest.
[Tooling partition 2](https://github.com/link-foundation/meta-language/actions/runs/45144165350/job/137589212106)

The package contracts job repeated the stale manifest and missing inventory
errors at log lines 626 and 669. Its naming check passed at line 685 with
16,688 concept records and all 25 WordNet fixtures. The separate raw schema
and consumer job passed all 11 tests, with zero skips at lines 679–684.
[Package contracts](https://github.com/link-foundation/meta-language/actions/runs/45144165350/job/137639287457),
[raw schema and consumer checks](https://github.com/link-foundation/meta-language/actions/runs/45144165350/job/137639287454)

The naming conformance partition on that revision passed its 81 active tests.
It checked the current and historical naming owners at log lines 950–951.
Two executable translation tests reported the pause at lines 873 and 981.
The bulk grammar pipeline also passed and published its 67-language matrix,
summarized at log line 1028.
[Conformance partition 2](https://github.com/link-foundation/meta-language/actions/runs/45144165350/job/137589212564),
[bulk grammar pipeline](https://github.com/link-foundation/meta-language/actions/runs/45144165350/job/137589213975)

On `3be026bb2`, all four tooling partitions, the JavaScript downstream
partition, all 15 package partitions, and the JavaScript aggregate passed.
Package contracts checked 474 requirements, 495 fixture identities and 819
dependency entries. The raw schema job passed 11 tests with zero skips. The
bulk grammar pipeline again produced the 67-language matrix.
[JavaScript aggregate](https://github.com/link-foundation/meta-language/actions/runs/45166331688/job/137750507373),
[package contracts](https://github.com/link-foundation/meta-language/actions/runs/45166331688/job/137693504082),
[raw schema and consumer checks](https://github.com/link-foundation/meta-language/actions/runs/45166331688/job/137693504298),
[bulk grammar pipeline](https://github.com/link-foundation/meta-language/actions/runs/45166331688/job/137664120628)

Delivery Candidates failed Rust package verification with E0382 at log lines
549–567. Our first correction for Clippy's `redundant_clone` diagnostic at
log line 4157 removed the clone and moved the original lexer mode before
the replay array consumed it. The corrected generator review borrows the
mode for rendering, keeps `DEFAULT_MODE` for the default mode, and transfers
the original mode to replay. A regression checks that the generated function
borrows the mode before preserving it in the replay array. It failed before
the correction and passed after regeneration. The focused run passed 19 tests,
including ordinary modes, default modes and ordered actions. One automated
translation test reported the pause.
[Delivery Candidates compiler error](https://github.com/link-foundation/meta-language/actions/runs/45166331688/job/137753170118),
[original Clippy diagnostic](https://github.com/link-foundation/meta-language/actions/runs/45112972070/job/137557959068),
[JavaScript lexer tests](../../../js/tests/grammar-lexer-rules.test.js),
[native lexer tests](../../../rust/tests/unit/grammar_lexer_transport.rs)

## Requirements and implementation plan

| Requirement | Alternatives considered | Selected implementation and verification |
| --- | --- | --- |
| Pause required automated translation until the maintainer resumes it | Filter the entire workflow by path, use conditional jobs and steps, or mark failures tolerated. Workflow filtering can leave required checks pending. Tolerating failures would execute the expensive work. | Use native job and step conditions with one Boolean input and a repository variable. Check the default, opt-in, cancellation and failed-prerequisite cases. |
| Keep the code, checks, tests and strict results | Remove commands, move the whole translator to a different workflow, or retain the existing commands and artifact graph with opt-in conditions. | Retain the checked commands and strict evidence evaluator in place. Compare commands with their certified predecessors and verify that paused jobs cannot seal or supply complete translation evidence. |
| Disable failing automated translation checks | Match failures by name, split shared jobs by role, or ignore entire shared jobs. | Gate translation execution and select native libtest exclusions. Keep parser, manifest, naming, packaging and downstream checks active. Clean consumers check installation, public entry points, and offline parse and program behavior. Run the actual gate script with fault inputs and inspect the latest PR checks for regressions. |
| Permit manual translation between supported languages | Permit independent implementations without common tests, or keep the JavaScript-first batch and shared behavior contract. | Allow manual ports in the vision and contributor instructions. Use shared fixtures and independent native tests for affected runtimes. Maintain `parity/language-features.lino` when public behavior changes. |
| Apply the pause to all open pull requests | Change repository branch rules, push each branch, or land one common base change and update each branch. | [PR #250](https://github.com/link-foundation/meta-language/pull/250) is the rollout source. After that commit enters the base, use each pull request's branch-update path and verify a fresh eligible run at its new head or test merge commit. Other refs remain outside this PR's push scope. |
| Capture data in the requested folder and research each possible solution | Record only links, keep full CI logs in Git, or capture exact issue data and referenced excerpts. | Keep the source body, digests, branch inventory and selected log lines in [evidence.lino](evidence.lino). Retain full logs outside regenerable scratch. Review workflow condition semantics, same-revision reuse, dispatch and validation tooling. |
| Deliver all changes in one pull request | Split the policy, CI and record changes across PRs, or review a single coordinated change. | Keep the implementation and case study in PR #250. Review the change against the default branch and verify the active checks at the exact PR revision. Record rollout only from observed runs on each PR. |

GitHub's native conditions can skip individual jobs while keeping the normal
workflow event and unrelated checks. An aggregate job can use `always()` to
inspect failed prerequisites. A call to a reusable workflow via a local path
resolves it at the caller's commit. A manual dispatch requires the workflow on
the default branch, and a dispatch run does not satisfy an eligible PR check.
[Required-check behavior](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks),
[reusable workflows](https://docs.github.com/en/actions/how-tos/reuse-automations/reuse-workflows),
[manual dispatch](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/manually-run-a-workflow)

The existing test classifiers already separate JavaScript translation and
report tests, and the existing Rust runner inventories filters against real
libtest names. Reusing them keeps each opted-in command and its scope available.
[actionlint](https://github.com/rhysd/actionlint) can validate GitHub expression
contexts and YAML structure. Repository tests cover mode and artifact behavior
using the workflows' current conditions and shell commands.

## Operating the pause

Normal PR, push and release runs leave `automated_translation` off. Clean
consumers still exercise the installed or published packages against parsing
and program fixtures. The raw schema partition still runs its graph integrity
and consumer contracts. The raw execution-plan partition and the projection
test that emits JavaScript from a Rust-derived graph require opt-in.
The Formal AI consumer retains 147 JavaScript workloads,
134 Rust workloads, and its cross-runtime probes. Its one JavaScript workload
that runs Meta Language's translator in six directions and the matching probe
operation require the opt-in. The rest still exercise Formal AI's own grammar
projections. The paused report can validate this reduced execution scope for
ordinary CI. Strict acceptance requires the original full workload.

The Rust coverage floor of 84.30% is calibrated for the full suite, including
its translation shard. The opt-in still runs that entire coverage gate and the
complete release-delivery translation checks. After the workflow is available on
the default branch, a maintainer can run a diagnostic on a selected ref with this
command.

```sh
gh workflow run ci.yml --repo link-foundation/meta-language --ref <ref> -f automated_translation=true
```

Set the repository Actions variable
`META_LANGUAGE_AUTOMATED_TRANSLATION=true` when the maintainer decides to
restore translation validation on normal events. A change of the variable
applies to newly evaluated runs. The complete issue #195 evidence path still
evaluates the original assertions in that mode. Retain separate links to the
ordinary and strict runs when recording findings.

## Open pull request rollout

| Open PR | Branch recorded at intake |
| --- | --- |
| [#218](https://github.com/link-foundation/meta-language/pull/218) | `issue-217-wrap-long-rust-lines` |
| [#228](https://github.com/link-foundation/meta-language/pull/228) | `issue-227-translation-continuation-2026-10-22` |
| [#229](https://github.com/link-foundation/meta-language/pull/229) | `automation/dependency-refresh` |
| [#246](https://github.com/link-foundation/meta-language/pull/246) | `issue-245-3cb95ea1992b` |
| [#247](https://github.com/link-foundation/meta-language/pull/247) | `issue-232-3c9786b3dbc0` |
| [#249](https://github.com/link-foundation/meta-language/pull/249) | `issue-197-d5dad2c0071f` |
| [#250](https://github.com/link-foundation/meta-language/pull/250) | `issue-248-3afae7c35b4a` |

A rerun at an older commit uses the older workflow. Once the policy reaches
the default branch, enumerate the open PRs again, update their branches through
GitHub where necessary, and record the new head and eligible run for each PR.
Confirm the translation stages are skipped and the applicable independent
checks ran. A conflicting branch requires its own conflict resolution before
it can be verified. The captured branch inventory is a rollout input.

The post-merge report records the newest eligible workflow run and its latest
attempt for each current pull request head and base pair. It requires a
successful pause step and aggregate. It checks the workflow's pull request
number and verified head and base parents even when GitHub omits the association
for a fork. It reads the current base branch ref directly because an unmergeable
pull request can report an older base commit. Run the same observation from a
clean checkout with GitHub CLI access.
[Fork workflow run for PR #246](https://github.com/link-foundation/meta-language/actions/runs/45089575521)

GitHub's head-filtered workflow list returned no runs for PR #249 at
`f1d961d1`, while the unfiltered list included its queued run with the same
head and base. The audit pages through the unfiltered workflow list and checks
the event, head and base locally before choosing the latest run.
[Queued workflow run for PR #249](https://github.com/link-foundation/meta-language/actions/runs/45112870920)

```sh
node js/scripts/check-automated-translation-rollout.mjs /tmp/automated-translation-rollout.lino
```
