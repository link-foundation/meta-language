# Owned text certificate adapter experiment

This JavaScript-only prototype attaches the unchanged
[`closed standard Rust text value projection 1`](../rust-text-value-projection/README.md)
proof directly to the actual program as `sourceTextValueProjection`. It invokes
the unchanged [complete portable certificate kernel](../rust-text-certificate/README.md)
at actual codec, validation, recheck and emission entry points. No production
file, package index, public declaration, source option, old model, fixture or
acceptance assertion is changed.

[data/overlays.lino](data/overlays.lino) retains the original provider receipts.
[data/overlays-current.lino](data/overlays-current.lino) records the selected reviewed source hashes, checked
Links decorators and resulting hashes for five actual production modules. The
reusable [overlay.mjs](overlay.mjs) materializes only those complete modules in
a temporary directory, preserves their original resource base, links imports
to actual providers and removes the directory after use. It does not extract
functions or replace the production parser. Unknown source bytes or ambiguous
hook strings refuse rather than silently producing a different adapter.

`captureOwnedRustTextCertificate(source)` reuses the existing complete source
factory and attaches its actual proof. `createRustTextCertificatePublicOverlay()`
returns temporary versions of the actual typed codec, validator, rechecker and
both emitters. The existing public source factory remains unchanged; there is
no new production selection option. Programs without the facet retain their
existing paths, including nonsemantic user provenance. The checked program root
must be an actual plain data record; function objects and inherited/custom host
prototypes refuse before selection without reading source accessors.
An existing main container likewise requires an own data descriptor and a plain
record value; an accessor or inherited/function container cannot hide source
facts in a generic fallback. Selected programs must
pass their owned facet before returning from encode/decode, recheck or emission.
The validation view removes exactly that one root field; all nested source,
signature, parameter, core, operation and main objects retain actual identity.
After that original graph passes, emission alone reuses the existing typed Links
encode/decode adapter to obtain ordinary record representations and checks the
restored graph with the same kernel. This preserves null-prototype record values,
ordered fields and shared aliases without mutating the caller or replacing saved
LinkId authentication. It costs an extra selected-program codec roundtrip and
kernel check per emission. The frozen replay checker therefore sees its expected
record representation without changing its source or meaning.

The raw boundary checks the selected program-body graph before ordinary
decoding. It refuses duplicated, misplaced or non-record facets, unknown fields,
foreign values and equal-looking copied owners. An ephemeral view hides only
the facet relation and exposes the **same original facet LinkId** through the
existing experimental codec entry. That codec resolves proof references through
memoized actual core anchors. The unchanged portable kernel then checks the
complete grammar, actual ordered owners, operations and input/allocation
conditions. The original network and its user provenance remain byte-identical.
Equal names and structural equality never establish shared owner identity.

[data/identity-transport.lino](data/identity-transport.lino) records all seventeen
table layouts, original raw role sources, dense identity categories, absent/null
distinctions and ordering contracts. A native bridge must run before Serde:
the existing RuntimeShell codec normalization is a concrete hook example,
and existing scope validation already authenticates semantic binding targets.
The bridge must use original `Data::Map` entries and `Decoder.fields` vectors;
`BTreeMap` values and sorted formal graphs lose the relevant order. Its smallest
missing primitive is an original-LinkId anchor context that interns each actual
node once and retains ordered relations. A new source-specific handwritten Rust
owner checker would not close the automatic translation requirement.

The prototype still indexes field relations for the **whole network** and clones
that network, including unrelated roots. The selected traversal has node/depth
bounds, but those bounds do not limit global indexing or cloning. Large dynamic
global concept spaces require a bounded selected-graph or proxy view that keeps
original IDs. This experiment makes no scalability claim.

The decision kernel has a complete automatically emitted Rust artifact, but this
adapter does not execute it natively. The JavaScript table bridge still uses
dynamic arrays and object/Map projection, and source capture/replay still uses
the frozen JavaScript adapters. Native raw identity projection, direct native
program representation, re-encoding, paired capture/replay and the complete
kernel interface remain gaps. The Rust replay adapter also retains the frozen
JavaScript checker as a redundant check. No native parity, production feature or
requirement-completion claim follows from this prototype.

The bounded tests reuse all thirteen complete source modules and twenty-six
exact refusals, exercise actual text and binary archives, execute emitted
JavaScript and replay complete Rust source. Coherent clone retargets, names and
Unicode/control-code edits remain possible; forged owners and assumptions refuse
at direct and linked boundaries. Input Unicode-scalar and successful allocation
conditions remain explicit, and a plain owned move creates no allocation fact.
The independently verified complete kernel algorithm is unchanged, so this
adapter does not repeat its expensive whole-kernel translation test.

Run from the repository root:

```sh
node experiments/rust-text-certificate-adapter/generate.mjs --check
node scripts/with-cache-cleanup.mjs --event test -- node --test \
  experiments/rust-text-certificate-adapter/overlay-builder.test.mjs \
  experiments/rust-text-certificate-adapter/overlay.test.mjs \
  experiments/rust-text-certificate-adapter/model.test.mjs \
  experiments/rust-text-certificate-adapter/raw-identity.test.mjs
```

These commands perform JavaScript checks only; they never invoke native
compilation or the CI-only original-Rust oracle.
