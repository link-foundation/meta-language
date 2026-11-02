# Complete Rust state source controls draft

This is source/control authoring under [the vision](../vision.md), not execution
evidence. `run-rust-state-source-originals.mjs` adds one reusable CI-only driver
over all complete positive originals in eight fixture groups: exclusive loans,
primitive compounds, owned Strings, owned vectors, exclusive collections,
standard collections, collection equality and ordered collections. It consumes
the original literal arrays directly, so new appended inputs stay included.

The driver requires explicit CI authorization before every native command. It
records the actual checked-out Git commit, Rust compiler version, target pointer
width and Node version. The initial frontend scope is Rust >=1.90 <1.100, edition
2024, 64-bit target, and Node >=22.15 <23 or >=24 <25. Uncovered versions fail
with diagnostics. This does not extend the frontend to the kernel's separate
32-bit domains or claim a native Rust frontend implementation.

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
Only its uniquely created scratch directory under `work/` is removed. Source
generation and native execution remain deferred during the current local phase.

Assertion correction: the exact empty-HashSet equality original previously named
“collection equality requires separate order-independent Eq interpretation”
retained `compilerAccepted:true`. The new genuine equality implementation makes
the old unsupported expectation obsolete. Its source input is preserved as an
appended positive in `rust-exclusive-collection-source.mjs`, with `true\n` stdout.
That input declares no exclusive function, so the fixture-specific “has exclusive
packet” assertion now applies to originals that actually require one. Its CST,
public profile, emitted stdout and graph coverage remain; every previous input
is retained. The current composition inventory has 13 positives and seven
refusals. This source correction is justified by the actual new shared Eq path
and the original Rust PartialEq contract, not a claimed observed pass.

Native frontend/codecs, complete downstream projects, every broader grammar and
ledger obligation still require their own full draft and later verification.
No tests, source imports, generation, compilation or validation ran locally.
