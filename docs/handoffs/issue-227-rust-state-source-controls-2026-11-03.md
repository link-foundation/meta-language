# Complete Rust state source controls draft

This is source/control authoring under [the vision](../vision.md), not execution
evidence. `run-rust-state-source-originals.mjs` adds one reusable CI-only driver
over all complete positive originals in fourteen fixture groups: exclusive loans,
primitive compounds, owned Strings, owned vectors, exclusive collections,
standard collections, collection equality, ordered collections, borrowed ordered
collection loops, stable UTF16 nominal field sorting and literal character String
append/prefix insertion, owned vector into-loops, concrete mutable source functions
and primitive vector sorting. The source inventory contains 279 complete positive
originals. This count records authored inputs,
not passing assertions or completed requirements. It consumes
the original literal arrays directly, so new appended inputs stay included.

The driver requires explicit CI authorization before every native command. It
records the actual checked-out Git commit, Rust compiler version, target pointer
width and Node version. The initial frontend scope is Rust >=1.90 <1.100, edition
2024, 64-bit target, and Node >=22.15 <23 or >=24 <25. Uncovered versions fail
with diagnostics. This does not extend the frontend to the kernel's separate
32-bit domains or claim a native Rust frontend implementation.
Each original's declared language range is checked against that interpretation.
The existing runtime-data version selector also checks the active Links Notation
dependency; its actual version, Unicode/V8 versions and platform/architecture
are retained with the observations. A separate assertion requires the original's
complete CST to be clean, while text/binary transports are still attempted as
their own checks. Actual concrete graphs and JavaScript products are retained.

For each original, the driver independently compiles/runs its original Rust,
parses/checks the actual source through the shared JavaScript frontend, emits
and runs JavaScript, emits/compiles/runs Rust, and compares both generated stdout
results with the original and literal oracle. Default full CST reconstruction
and Links text/binary transports have separate assertions. All assertions are
attempted even after another fails; an unavailable predecessor becomes an
explicit failed assertion. No planned operation is reported as an observation.
Kernel refusals and the existing negative source controls remain independent
inventories; this positive-original matrix does not claim their assertions.

Collection interfaces load their complete authored package asset and source
receipt through the public automatic loader. Missing/stale assets fail; no
descriptor-only or host-container replacement is supplied. Typed body/owner
correspondence is checked before source admission. A shared test helper now
builds equivalent genuine source assets and delegates binding derivation to
that same public loader, replacing the older two-callable manual setup. The
required packet/kernel/equality callable inventory remains three.

The Rust reusable workflow runs the new job only after its check gate; the
parent CI already waits for JavaScript success before calling that workflow.
Checkout selects actual pre-merge head inputs. The job needs no local Cargo
target, uses bounded subprocesses and one disposable executable directory at a
time, and retains actual sources, products, command output and every assertion
diagnostic as Links under `issue-195-artifacts/rust-state-source/<commit>/`.
Typed program text and binary transports independently decode and recheck the
current complete graph, compare their emitted JavaScript and Rust bytes with the
selected products, and run each restored JavaScript product against the original
and literal stdout. Exact Rust byte equality relates those transports to the
separate emitted Rust compilation assertion; it does not report a compilation
when that assertion fails. Transport hashes, byte/node counts and actual stdout
are retained. Complete common typed libraries are not redundantly written for
every fixture, keeping CI artifact growth bounded by receipts and products rather
than repeated full library archives. FnMut groups select their explicit source
profile through the same driver; other groups retain their previous selection.
Only its uniquely created scratch directory under `work/` is removed. Source
generation and native execution remain deferred during the current local phase.

Assertion correction: the exact empty-HashSet equality original previously named
“collection equality requires separate order-independent Eq interpretation”
retained `compilerAccepted:true`. The new genuine equality implementation makes
the old unsupported expectation obsolete. Its source input is preserved as an
appended positive in `rust-exclusive-collection-source.mjs`, with `true\n` stdout.
That input declares no exclusive function, so the packet-inventory assertion
compares every original with its actual header requirement, including an explicit
false expectation for this input. No assertion cell is skipped. Its CST,
public profile, emitted stdout and graph coverage remain; every previous input
is retained. The current composition inventory has 13 positives and seven
refusals. This source correction is justified by the actual new shared Eq path
and the original Rust PartialEq contract, not a claimed observed pass.

Native frontend/codecs, complete downstream projects, every broader grammar and
ledger obligation still require their own full draft and later verification.
No tests, source imports, generation, compilation or validation ran locally.

The current driver also adds 28 actual `Type::map_children` and 33 actual
`Type::children` controls through current-source factories. This gives 340
authored scenario rows across sixteen groups (279 literal originals plus 61
current-source scenarios). Counts do not measure requirement completion. See
[the native reference handoff](issue-227-actual-type-native-reference-controls-2026-11-03.md)
for full original, projected and actual crate reference scopes. None ran locally.
