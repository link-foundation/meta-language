# Serial development batch authoring

[The vision](../../docs/vision.md#four-language-semantics-and-translation)
defines the architecture and priorities. This tool reduces repeated metadata
commands after a complete source batch; it implements no translator semantics
and establishes no requirement evidence.

`stages.lino` is generated automatically by `node experiments/development-batch/refresh.mjs --write-plan`.
`--list` shows the phases. Each phase invokes existing commands sequentially in
fresh JavaScript processes. The history phase first captures complete certified compiler and borrowing
successors and physical workflow succession, retains the grammar-context steps
around CI/native history and refreshes evidence-translation archives. Registration
appends canonical operations, their complete source bindings and the current
binding codec epoch in order, retaining every earlier row and epoch. Naming
refreshes both concept catalogs before deriving its supplemental views. Run final
verification after source-changing authors and manifest refreshes finish.

Use the existing resource and cache wrapper, one phase per bounded invocation:

```sh
env -u CARGO_TARGET_DIR -u CARGO_BUILD_TARGET_DIR node scripts/with-cache-cleanup.mjs --event test --project-owned-only --no-preflight --no-docker -- node experiments/development-batch/refresh.mjs --phase snapshot --receipt /tmp/meta-language-snapshot-authoring.lino
env -u CARGO_TARGET_DIR -u CARGO_BUILD_TARGET_DIR node scripts/with-cache-cleanup.mjs --event test --project-owned-only --no-preflight --no-docker -- node experiments/development-batch/refresh.mjs --phase history --receipt /tmp/meta-language-history-authoring.lino
env -u CARGO_TARGET_DIR -u CARGO_BUILD_TARGET_DIR node scripts/with-cache-cleanup.mjs --event test --project-owned-only --no-preflight --no-docker -- node experiments/development-batch/refresh.mjs --phase registration --receipt /tmp/meta-language-registration-authoring.lino
env -u CARGO_TARGET_DIR -u CARGO_BUILD_TARGET_DIR node scripts/with-cache-cleanup.mjs --event test --project-owned-only --no-preflight --no-docker -- node experiments/development-batch/refresh.mjs --phase naming --receipt /tmp/meta-language-naming-authoring.lino
env -u CARGO_TARGET_DIR -u CARGO_BUILD_TARGET_DIR node scripts/with-cache-cleanup.mjs --event test --project-owned-only --no-preflight --no-docker -- node experiments/development-batch/refresh.mjs --phase verification --receipt /tmp/meta-language-verification.lino
```

Verification runs eighteen read-only checks, beginning with the authenticated
online requirement-source audit before ledger access. It requires GitHub API
access. Its remaining commands check dependencies, scope, exports, runtime
inventory, documentation, test ownership, generated rules, source histories and
both concept catalogs. It also verifies every source epoch and current source
pin in the explicitly indexed historical naming documents. Stale outputs fail;
verification never refreshes them.
The snapshot phase refreshes the relative-character fixture through its existing
JavaScript oracle; final verification checks its exact source fingerprint,
unchanged input, ordered decisions and bounded package version.
`--check` leaves these already read-only commands unchanged.
The final command runs the existing JavaScript check that every embedded Rust
source input is packaged. It performs no Cargo invocation or compilation.

Append `--check` to authoring phases for their read-only freshness mode. This does
not replace required targeted controls, source reconciliation or the final
requirement-manifest refresh. If an author fails,
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
partial report as full coverage. CI 39857274367 generated all eight complete
reports for 347 modules, but its second job exceeded the thirty-minute deadline.
Complete report artifacts do not override that failed gate. Diagnostic groups
overlap and do not establish a completion percentage.
