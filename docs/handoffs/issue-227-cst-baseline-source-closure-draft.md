# Issue 227: own-source concrete baseline source draft

This handoff records source inspection and deferred controls under the existing
[vision](../vision.md#concrete-and-abstract-syntax-trees). It introduces no
architecture specification and supplies no execution or completion evidence.
The requirement ledger and its source register were not read or changed: their
online prerequisite is deferred with runtime execution in this drafting phase.

## Drafted files

- `rust/src/link_network.rs`: corrected the documentation of `LinkNetwork::parse`
  to describe its actual registered-grammar concrete-tree dispatch. No executable
  Rust statement changed and no handwritten native semantics were introduced.
- `js/tests/issue-227-own-source-concrete-baseline.test.js`: future controls for
  complete current `js/src/network-projection-rules.js` and
  `rust/src/network_projection.rs`, independently bounded to 16 KiB each.

The controls explicitly select Node `>=22.15.0 <23.0.0` or `>=24.0.0 <25.0.0`,
the package's declared web-tree-sitter `0.27.0` dependency, and the repository's
Rust edition `2024`/declared minimum `1.90`. Rust source is parsed as text by the
JavaScript package; this is not Rust toolchain execution or native equivalence.
Uncovered Node/dependency/source-manifest versions fail the future control
rather than silently skipping an assertion. Grammar implementations remain the
actual selected checkout's catalog/lock inputs; the controls do not assign an
unverified independent grammar version to them.

Every selected file must retain clean recovery, nested grammar structure,
actual named declarations, fields, trivia, exact physical token intervals,
distinct identities and spans for repeated same-text occurrences, and exact
original bytes. The tests exercise omitted concrete selection before and after
explicit abstract selection, complete text/binary graph reload while source
parsing is trapped, subsequent source reconstruction and deterministic reparse.
Binary replay additionally compares its complete archive bytes. Assertions use
the actual current complete file and original byte intervals; declarations are
independently named in the control, not discovered as their own expected values.

These two files are a bounded control pair, not the repository denominator.
There is no source hash baseline, whole-source inventory claim, full independent
CST-row oracle, native parity fixture, or semantic behavior certificate here.
The complete own-source inventory and original independent tree expectations
remain necessary. No observation recorder or status field is added.

## Inspected production path

`js/src/programming-language-parser.js:parseProgrammingLanguage` selects the
registered grammar. `parseGrammarCst` retains its tree and physical source gaps;
`js/src/network.js:LinkNetwork._insertProgrammingLanguage` stores actual tokens,
and `_insertProgrammingTree` stores syntax hierarchy and field relationships.
`LinkNetwork.parse` does not request abstract syntax. `projectedLinks()` defaults
to `ConcreteSyntax`, with shared membership supplied by
`js/src/network-projection-rules.js`; explicit selection leaves the source graph
intact. A lexical fallback for an unknown language is a separate existing path,
not proof of a complete registered-language tree.

`rust/src/link_network.rs:LinkNetwork::parse` delegates to the built-in language
parser and retains recovery/provenance. `reconstruct_text` selects physical
nonmissing token links with spans and terms, sorts by start byte then identity,
and suppresses already-covered intervals. Its default CST documentation now
agrees with this registered-parser boundary.

## Source gaps requiring coherent follow-up

1. **Spanless constructed source tokens.** JavaScript
   `LinkNetwork.reconstructText` retains nonmissing source tokens without spans,
   using the existing `sourceOrder` comparator. Native `reconstruct_text`
   filters them out through `span()?`; `rust/src/source_generation.rs:render_source`
   instead supports constructed roots and, when physical tokens exist, places
   spanless tokens after physical tokens. JavaScript `renderSource` also places
   spanless tokens last. These are observably distinct public operations.
   A follow-up must inventory callers and retain the existing JavaScript
   constructed-token behavior before drafting a shared explicit source-defined
   ordering/ownership policy and translating it automatically. Future controls
   must include a purely spanless graph, mixtures inserted in both orders,
   absent terms, zero-width spans, missing recovery tokens, separate documents,
   and equal-text distinct occurrences. Dropping tokens to align the runtimes
   would conceal this obligation. No source policy or adapter semantics changed
   in this batch.
   There is also a static ordering issue in JavaScript's `sourceOrder`: it
   compares byte starts only when both tokens have spans, and compares IDs
   otherwise. Consider A(id 1, start 20), B(id 2, no span), C(id 3, start 10).
   The comparator gives A before B, B before C, and C before A. This is a
   source-derived counterexample to transitivity, not an executed observation.
   A coherent shared total order and its compatibility controls must cover this
   mixed case; a platform's incidental sort result cannot define the oracle.
2. **Embedded overlays and editable ownership.**
   `LinkNetwork._attachEmbeddedRegions` inserts the host physical tree before
   child trees, preserves absolute offset spans, and keeps actual Region links
   plus shared budget/ancestor decisions. `reconstructText` merges all token
   links by start then identity. Earlier host tokens therefore suppress child
   tokens covered by the same interval. This preserves a passive original host
   stream; it does not define how editing an embedded child rewrites that host
   token, escapes it, or updates derived source planes. `renderSource(language)`
   narrows language membership but does not resolve an edited child against its
   physical owner. Do not reverse token priority: that could change unchanged
   host bytes and select unrelated overlapping alternatives. Follow-up controls
   must independently specify physical and decoded source planes, duplicate
   nested-region boundaries, same-language embeddings, edited/deleted child
   leaves, ancestor cycles, and text/binary reload. Shared Links data must choose
   ownership and supported edit propagation before public emission integration;
   unsupported ambiguous edits require an explicit refusal, not cached original
   text presented as edited emission.
3. **Full own-source acceptance.** The bounded pair cannot replace every real
   tracked JavaScript/Rust module, complete function/type/effect/ownership
   inventory, whole-graph exact identity support, independent concrete rows,
   native replay or behavioral translation. Keep the complete denominator and
   original refusal cases when joining this draft to that work. The public
   exact-ID source adapters and the existing full replay controls require their
   own full inventory and native execution before any closure claim.

No tests, source programs, generators, validators, compilation, package install,
network requests, CI inspection, ledger operation or Git mutation ran for this
subtask. Only targeted source reads and file edits were performed. The next
phase must review the whole drafted batch before executing its required checks.

Coordinator integration adds a deferred aggregate policy-control file at
`js/tests/rust-baseline-source-policies.test.js` for the separate alias/sum
policies. It does not exercise or resolve the CST emission gaps above.
