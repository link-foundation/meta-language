# Portable finite Rust text source certificate

This separate JavaScript experiment, `portable closed Rust text source
certificate 1`, certifies the complete finite source grammar of the frozen
[Rust text value projection experiment](../rust-text-value-projection/README.md).
Its original model, source recognizer, checker, policies and fixtures remain
byte-identical. The linked policy retains hashes for the
frozen source, checker, policy, fixtures and decision source.

The bridge assigns dense safe ordinal identities to actual token, declaration,
parameter, signature, source-position, expression, operation and empty-main
objects. A checked bijection maps each local ordinal to an actual object identity
restored from Links. Interned primitive string symbols express spelling and
value equality separately from owner identity. Null is zero, absence is minus
one, and real identities are positive.
Every table layout and role is inspectable in [data/policy.lino](data/policy.lino).

The generated kernel consumes the entire token stream. Data-selected finite
segments describe the existing header and five body-operation forms; no second
body AST is built. It checks ordered declaration and parameter coverage, actual
source signatures and result owners, source argument/receiver type layers,
actual expression owners/spans, every operation's ordered token/child links,
and the exact operation-bound input and allocation conditions. An owned move
requires no allocation condition. Dense role coverage, canonical symbol values,
symbol uniqueness, part/path/layer storage and null/absence remain checked.

[bridge.mjs](bridge.mjs) only validates descriptors, record/container shapes,
the existing complete lexer/CST and shared SourceSignature records, and projects
the actual fields and identities. Its checker does not invoke the frozen
JavaScript source recognizer, operation checker or deep object comparator.
Initial source creation reuses the existing complete production parser and
frozen factory to obtain the real program and source records. Subsequent saved
text/binary checks decide grammar and role ownership in the generated kernel.
This is a bounded transport over distinct original source occurrences; it does
not infer shared occurrence identity from equal-looking records.

The kernel itself uses typed flat Number arrays and portable loops. Its whole
source plus the actual existing name-character, text-domain, operation and
Unicode-scalar decisions pass through the real JavaScript parser/checker and
Rust emitter. The exact complete output, declaration list and all emitted
definitions are kept as links in [data/translation.lino](data/translation.lino).
No Rust function is handwritten, trimmed or substituted. The actual complete
kernel also executes after typed Links text and binary reload in JavaScript.

The initial feasibility probe found a concrete translator gap: dynamic array
`push` is unsupported on the checked Number-array type. The kernel instead
returns the fixed result array recorded in the policy: decision, consumed token
count, function count, input-condition count and allocation-condition count.
Negative decisions retain a bounded refusal stage. The complete automatic Rust
artifact retains the generic emitter style. Native compilation and execution
are CI checks; actual observations depend on the corresponding run's receipts.
Formatting and integration through the two public runtime packages require
checked decorators and paired graph adapters for this same algorithm.

The mandatory native CI oracle compiles the complete stored artifact unchanged
as a library and calls its actual public checker from one generated consumer.
It compares all five output values bit for bit with the complete JavaScript
kernel for 13 original source graphs and 27 malformed transports. Source hashes,
the ordered 17-vector interface, every declaration and exact compiler/execution
counts accompany the Links Notation receipt. Local oracle tests use injected
process contracts; real compilation requires both `CI=true` and
`RUST_TEXT_CERTIFICATE_NATIVE=1` in the selected native CI cell.

Run only these bounded checks:

```sh
node experiments/rust-text-certificate/feasibility.mjs
node experiments/rust-text-certificate/generate.mjs --check
node scripts/with-cache-cleanup.mjs --event test -- node --test experiments/rust-text-certificate/kernel.test.mjs
node scripts/with-cache-cleanup.mjs --event test -- node --test experiments/rust-text-certificate/native-observations.test.mjs
```
