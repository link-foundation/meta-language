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
and program fixtures. The Formal AI consumer retains 147 JavaScript workloads,
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

```sh
node js/scripts/check-automated-translation-rollout.mjs /tmp/automated-translation-rollout.lino
```
