This private JavaScript-only experiment reads five complete actual generated Rust
artifacts. The first is 1,419 bytes, with sha256
`be586151e33dbc40e596acdcf3205a094601bcfc11190546947b9c85090dc7d1`.
The fixture generator obtains each whole input from the real public Rust emitter;
the original tuple-call program observations and refusal fixtures remain unchanged.

Every token, raw spelling and intervening trivia code unit is represented in
Links Notation. The whole module produces a Surface program for the real checker.
Its source interpretation links actual header, call and lexical occurrences to
the existing checked declaration, parameter and local identities. String clones
also reference their actual checked receiver and unshadowed standard String owner.
The existing runtime-shell linked grammar supplies the inert attribute, empty
Unit worker, entry operations and four separate successful-runtime assumptions.
Each String clone or literal construction has its own operation-bound allocation
success condition. These conditions do not establish failure equivalence.

Use the experimental factory, reload checker and emitter together. The ordinary
typed codec stores the inspectable interpretation in root provenance, sharing
real core LinkIds. Generic public APIs alone do not enforce this experimental
proof. No original source string, cached emission, source envelope or selected
function substitute supplies semantic authority. Same-target replay preserves
original bytes until coherent semantic edits change live names, targets or values.
Trivia reuses the shared exact Rust whitespace and initial byte-order-mark rules.
Ordinary line comments and nested block comments retain every code unit. Outermost
doc comments require an attribute interpretation and are refused; doc-looking
contents nested within an ordinary block comment remain comment contents.
Canonical Rust emission reinterprets the complete actual emitted module and binds
its runtime and String conditions to the new operations and source intervals.
It retains any conditions reported by the underlying actual core emitter.

The JavaScript target reuses the actual core emitter and shared Unicode String
input decision, with inspectable linked input-guard decorators. Inputs must be
primitive Unicode scalar strings, signed i32 BigInts and primitive Booleans.
Unpaired UTF16 values and the separate generic JavaScript UTF16 text model are
refused. The shared unmodified standard `charCodeAt` realm assumption remains
explicit. Array identity, allocation counts, destruction timing and failure
behavior are outside the value observation.

Only immutable generated value bodies, direct String/i32 tuple helpers,
once-evaluated declared tuple calls, Boolean/i32 equality, conditionals and actual
standard clones are admitted. References, arbitrary methods, recursive helpers,
additional effects and runtime operations are refused. Edited String literals
currently admit ordinary Unicode scalars plus tab, LF and CR; other control
characters need an edited-literal quoting adapter. A saved negative literal needs
a new sign interpretation to become positive. Production selectors, native
pairing, complete failure behavior and the full binary formatter remain gaps.
The generated experimental Rust decision file is an automatic translation
artifact and is never compiled by this experiment. This is not ledger evidence.

Run the bounded checks from the repository root with Node 22:

```sh
node experiments/generated-owned-rust/generate-rules.mjs --check
node experiments/generated-owned-rust/generate-fixtures.mjs --check
TRANSLATION_CHECK_RUST=0 SELF_TRANSLATION_CHECK_RUST=0 node scripts/with-cache-cleanup.mjs --event test --no-preflight --no-docker -- node --test experiments/generated-owned-rust/model.test.mjs
```
