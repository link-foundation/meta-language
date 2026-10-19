# CI scratch runner

Run `node experiments/grammar-outcome-network/native-transport/runner/run.mjs`
from the checkout for a source plan. It authenticates the frozen Git snapshot,
published authoring sources and generated candidates without running Cargo,
the JavaScript oracle or a native probe, and without writing compiler scratch.
Run `node --test experiments/grammar-outcome-network/native-transport/runner/runner.test.mjs`
for injected process controls. These checks establish source and process
contracts; native assertions require CI execution.

`publication.lino` retains the complete reviewed runner sources, original policy
and exact invertible publication edits. `generate-publication.mjs --check`
replays those edits and refuses source drift. The bootstrap conversion uses
`--original-dir` with the original authenticated reviewed packet. No original
candidate or probe pin is replaced by a publication hash.

The called Rust workflow runs the native branch after its JavaScript
prerequisite and native dependency check. Native execution requires `--native`,
`META_LANGUAGE_B95_NATIVE_OUTCOME=1`, `CI=true`, `GITHUB_ACTIONS=true`, Linux,
the actual workspace and an external `RUNNER_TEMP`. These guards precede every
compiler/version command, native scratch allocation and injected native process.

The scratch crate contains the exact frozen sources from
`3780504667f4abe167a270263453e54be2b1cc43`, the small original Agda JavaScript
build-helper closure and three explicit candidate/probe overlays. Current source
hashes are recorded separately. Only the existing JavaScript dependencies are
shared. Cargo runs offline and locked with one job in a fresh target directory.
The existing checksum-verifying downloader acquires only the frozen registry's
native oracle build assets into that scratch overlay under a separate ninety
second bound. The artifact receipt records the exact registry, downloader and
acquired compressed/decoded source hashes. External grammars remain untracked.
Compilation has a fifteen minute bound; probe execution has a sixty second bound.
Each process uses its own Linux process group and retains at most eight MiB
per diagnostic stream. Artifacts retain the oracle, source inventory, tool
versions, native report when produced and every process result after failures.

This is finite baseline candidate evidence. It makes no claim about current
production APIs, default CST transport, arbitrary depth, container identity or
ledger obligations. See the [authoritative vision](../../../../docs/vision.md).

## Compiler and target observations

The source-only planner derives one additive `std::primitive::usize::BITS`
observation in the existing post-assertion native report. Its source receipt
restores the complete original probe exactly. It does not execute the constant
locally or change any original case or assertion.

Native CI retains verbose compiler diagnostics and binds the selected probe's
actual `--extern serde_json=...` argument to its exact compiler-artifact record.
The same locked package can have several feature units; their feature sets are
never combined. The collector checks complete build output, original stream
hashes, source and executable paths and explicit version bounds: Cargo and the
actually selected Rust compiler
`>=1.90.0 <1.100.0`, JSON dependency `>=1.0.151 <1.0.152`. Missing, ambiguous or
uncovered profiles refuse.

The retained historical compiler fixture comes from a failed native probe. Its
two distinct dependency units and the synthetic selectors in JavaScript tests
do not establish a current native observation. Only successful guarded CI
execution supplies the new profile receipt. Original DTO reconstruction,
native number variants and static-reference resolution remain required.
