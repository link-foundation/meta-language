# Serial development batch authoring

[The vision](../../docs/vision.md#four-language-semantics-and-translation)
defines the architecture and priorities. This tool reduces repeated metadata
commands after a complete source batch; it implements no translator semantics
and establishes no requirement evidence.

`stages.lino` is generated automatically by `node experiments/development-batch/refresh.mjs --write-plan`.
`--list` shows the phases. Each phase invokes the existing normal authors,
sequentially in fresh JavaScript processes. The six-stage history phase retains
the intentional grammar-context steps before and after CI/native history.
The naming phase runs last, after source-changing authors finish.

Use the existing resource and cache wrapper, one phase per bounded invocation:

```sh
env -u CARGO_TARGET_DIR -u CARGO_BUILD_TARGET_DIR node scripts/with-cache-cleanup.mjs --event test --project-owned-only --no-preflight --no-docker -- node experiments/development-batch/refresh.mjs --phase snapshot --receipt /tmp/meta-language-snapshot-authoring.lino
env -u CARGO_TARGET_DIR -u CARGO_BUILD_TARGET_DIR node scripts/with-cache-cleanup.mjs --event test --project-owned-only --no-preflight --no-docker -- node experiments/development-batch/refresh.mjs --phase history --receipt /tmp/meta-language-history-authoring.lino
env -u CARGO_TARGET_DIR -u CARGO_BUILD_TARGET_DIR node scripts/with-cache-cleanup.mjs --event test --project-owned-only --no-preflight --no-docker -- node experiments/development-batch/refresh.mjs --phase naming --receipt /tmp/meta-language-naming-authoring.lino
```

Append `--check` for each existing author's read-only freshness mode. This does
not replace required targeted controls, source reconciliation, source binding
registration or the final requirement-manifest refresh. If an author fails,
the receipt includes its actual result and no dependent author runs. Inspect
the error, fix its cause and rerun the phase; earlier generated changes are
retained for review. Do not run multiple phases concurrently.

The runner clears inherited Cargo targets, `NODE_OPTIONS` and CI activation,
disables translation compiler checks, selects a 352-MiB JavaScript heap and an
80-second child deadline. The task's outer 90-second/768-MiB guard is still
required. No stage invokes Cargo or selects a full grammar/raw workload.
Native compilation, full reports, acceptance and runtime evidence remain CI-only.

To choose the next semantic batch, reuse CI artifacts with
`js/scripts/self-translation-report-blockers.mjs --report ... --out ...`.
Repeat `--report` to retain all shards; use `--group` to emit the existing
source-bound replay plan. Never generate full reports locally or interpret a
partial report as full coverage. The seven available reports for CI
39654536127 cover 296 modules; the eighth shard timed out. Those diagnostic
groups overlap and do not establish a completion percentage.
