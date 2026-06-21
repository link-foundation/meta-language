# Grammar architecture

Owned by [F1](../case-studies/issue-93/proposed-issues/F1-grammar-subsystem-docs.md);
architecture anchor for [A1](../case-studies/issue-93/proposed-issues/A1-grammar-ir.md),
[A3](../case-studies/issue-93/proposed-issues/A3-grammar-concept-ontology.md),
and [C6](../case-studies/issue-93/proposed-issues/C6-concept-aligned-translation.md).

The grammar layer uses one intermediate representation for every grammar-facing
feature. The core types live in [`src/grammar/mod.rs`](../../rust/src/grammar/mod.rs):

| Type | Role |
| --- | --- |
| `Grammar` | Ordered collection of rules, optional start rule, and optional source format. |
| `GrammarRule` | Named rule with an expression, `RuleKind`, optional concept, and optional documentation. |
| `GrammarExpr` | Expression algebra for terminals, non-terminals, sequences, choices, repetition, predicates, captures, character classes, and empty expressions. |
| `RuleKind` | Parse participation marker: normal, atomic, silent, or token. |
| `GrammarFormat` | Source or target notation tag such as `MetaLanguage`, `Bnf`, `Ebnf`, `Abnf`, `Peg`, `Antlr`, `Lark`, `Gbnf`, `TreeSitter`, or `Inferred`. |

## Minimal example

The rustdoc example on [`src/grammar/mod.rs`](../../rust/src/grammar/mod.rs) is the
doctested reference for building a grammar, encoding it as links, and decoding it
back. The invariant is:

```text
Grammar::from_links(Grammar::to_links(grammar)) == grammar
```

In code, use `Grammar::builder()` for rules and `Grammar::expr()` for expression
constructors. Then pass the value through `LinksEncoder` and `LinksDecoder`.

## Grammar as links

A grammar is data in the network. The links codec in
[`src/grammar/links.rs`](../../rust/src/grammar/links.rs) implements `ToLinks` and
`FromLinks` for `Grammar`. Each encoded grammar node is inserted with
`LinkType::Grammar` from [`src/link_network.rs`](../../rust/src/link_network.rs) and a
stable term such as `grammar::grammar`, `grammar::rule`, or
`grammar::expr::sequence`.

The self-description root in
[`src/self_description.rs`](../../rust/src/self_description.rs) declares the
`grammar` root as `LinkType::Grammar` with references to `language` and
`relation link`. That makes grammar structure part of the same self-describing
network as links, references, concepts, fields, regions, and objects.

Because grammar data is encoded as ordinary links:

- Link projections can include or hide grammar links according to normal network
  rules.
- Grammar data can be serialized through the existing links codec and LiNo
  helpers.
- Other stages can accept a `Grammar` value, a grammar root link, or a serialized
  links network without inventing a parallel storage path.

## Pipeline

```text
                            +--------------------+
                            | authoring surface  | A2, E4
                            +----------+---------+
                                       |
                            +----------v---------+
                            | external importers | B1-B7
                            +----------+---------+
                                       |
                                       v
+------------------+        +--------------------+        +------------------+
| example corpora  +------->| Grammar IR links   +------->| emitters         |
| and observations | D1,D5,D6| A1                 | C1-C3  | BNF/EBNF/etc.   |
+------------------+        +---------+----------+        +------------------+
                                      |
                +---------------------+----------------------+
                |                     |                      |
                v                     v                      v
        parser codegen          concept translation     runtime parser
        C4, C5, E5              A3, C6                 E1, E2
```

The rule for new work is simple: parse or infer into the IR first, then emit,
translate, generate code, or run from that IR. This keeps every front end and
back end comparable.

## Concept alignment

Grammar concepts in [`src/grammar/concepts.rs`](../../rust/src/grammar/concepts.rs)
name constructs such as rules, terminals, non-terminals, sequences, choices, and
repetition. [A3](../case-studies/issue-93/proposed-issues/A3-grammar-concept-ontology.md)
owns the full ontology. [C6](../case-studies/issue-93/proposed-issues/C6-concept-aligned-translation.md)
uses that ontology so translation can preserve meaning even when two notations
spell a construct differently.

This mirrors the document-format path described in
[`docs/cross-format-fidelity.md`](../cross-format-fidelity.md): parse into a
shared concept layer, then render in the target format with documented fallbacks
where exact fidelity is impossible.

## See also

- [Authoring](authoring.md) for the native surface syntax.
- [Import and export](import-export.md) for external grammar notations.
- [Translation](translation.md) for concept-aligned grammar conversion.
- [CLI and runtime](cli-and-runtime.md) for running grammars through user-facing
  commands and parser registration.
