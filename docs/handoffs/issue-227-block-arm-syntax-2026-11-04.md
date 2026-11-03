# Source match block-arm syntax draft

Continues [explicit shared-reference patterns](issue-227-shared-reference-patterns-2026-11-04.md)
under [vision](../vision.md). Source edits and textual inspection only. No test,
represented source, import, generation, formatting, validation gate, compilation
or CI feedback ran or was reviewed. Every authored control remains unexecuted.

`rust-source-place-matches.js` extends its bounded attempt planner to recognize
omitted commas after block/if/match arm bodies. Only a completed top-level brace
can begin another pattern; nested delimiters, condition blocks, source `else`
chains and explicit shared-reference patterns keep their actual boundaries.
Raw keyword identifiers remain pattern candidates. Ordinary comma-separated
expressions keep their existing planner path; unsupported planning falls back
or requests its real missing source protocol.

`rust.js` retains the existing source block classification on lowered actual
source match expressions, whose core kind may now be a let or conditional.
Nested match bodies can therefore omit their commas according to their source
syntax. Macro-provided graphs do not acquire new source block classification.
The actual arm's leading token prevents grouping and unary prefixes from
incorrectly inheriting a block-only separator exemption after reference erasure
or lowering. Postfix expression results retain ordinary comma requirements.
This classification is ordinary parser metadata and grants no ownership proof.

Seven positive native originals cover true/false owning guards, if/else bodies,
nested lowered matches, following shared patterns, ordered printing and condition
blocks, all without separators after their actual block expressions. Two invalid
native originals retain missing commas after grouped and postfixed blocks.
Three independent controls retain native source/output interpretation, invalid
source refusal, Links text/binary and all four target bodies for seven scenarios.
Retained Rust `>=1.90.0 <1.100.0`, edition 2024, and Node ranges apply. Existing
inputs and assertions remain unchanged; these additions supply no observations.

Contract: [Rust match expression syntax](https://doc.rust-lang.org/reference/expressions/match-expr.html).

Remaining work stays fully binding in the preceding detailed handoffs:

1. Complete general match entry for variants/refutable/or/alias patterns and guard
   let chains, with actual active-variant component images and independent wrong
   selector/path controls. Extend syntax planning to all supported block kinds,
   labels and macro forms with exact original source classification.
2. Transport native-valid foreign guard effects on both success and failure while
   retaining protected selected-place loans; complete internal closed-body regions,
   stored references, anonymous products and actual exclusive pattern leases.
3. Complete temporary lifetime extension, Drop/destructuring/indexed/custom-Deref
   semantics, call/state/closure/helper/loop ownership and all complete own-code
   CST-default bidirectional JavaScript/Rust translation and JavaScript runtime
   work. Every grammar/global formal link concept/ontology/taxonomy/embedding/
   RDF/OWL/dynamic/inference/distribution obligation remains in vision and the
   earlier detailed handoffs, without reduced scope.
4. After all actual code/control drafts, reconcile sources online before ledger
   use, publish native/schema/naming/features assets automatically, run authorized
   JavaScript and CI-only native gates, repair actual failures and record every
   assertion at the executed commit. Never compile Rust locally.

No audited denominator, completion percentage or full-support claim is supplied.
There is no external blocker to further source drafting.
