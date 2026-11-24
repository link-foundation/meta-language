# Bounded source library asset draft

This is a source draft, not an execution report. No tests, generation, validation
or compilation were run for this change.

`js/src/canonical-network-reader.js` uses the existing Links Notation streaming
parser and canonical metadata decoder. It checks input bytes, node count and
reference count before insertion, rejects duplicate identities and dangling
references, and retains registered terms and self links. It is an internal Node
asset reader; the public general grammar and network APIs retain their contracts.

The Rust standard collection library loader now uses this bounded reader and
checks both shipped asset files before reading them. Receipt and full source/body
correspondence checks remain independent requirements. Missing generated assets
refuse; they do not select a host collection implementation. Deferred controls in
`js/tests/canonical-network-reader.test.js` retain lossless, duplicate, dangling,
budget and registered-term cases. The source author registers their automatic
Links conversion; publication is deferred until the full draft is complete.

The public TypeScript source state consumer obligations retain all five original
consumer inputs. `js/scripts/rust-source-state-assertion-policies.mjs` adds exact
TypeScript 5.9.2 and 7.0.2 ranges to the ordinary policy generator. These authored
diagnostic expectations are not observations. Other active compiler versions must
be reported as uncovered. The positive consumer also exercises the explicit
concrete FnMut profile with the exclusive source profile selected.

Next: generate assets and policies only after the complete draft gate, then run
the relevant JavaScript and clean-consumer controls through the existing gates.
Native Rust products and parity remain CI-only. Correct any disproved expectation
with evidence while retaining its input and coverage.
