# Original Rust item capture source draft

`captureRustConcreteSourceItems` reads the supplied current full CST and original
source, checks exact reconstruction, and selects actual named item nodes through
their name/body/type fields and actual syntax-child edges. Enclosing source
items must be ancestors in the supplied graph, as well as enclosing source
ranges; a coincident span alone is insufficient. It retains full original item/declaration slices,
hashes, link identities, UTF8 byte ranges and UTF16 source offsets. Enclosing
inline module names and inherent/trait implementation type fields distinguish
otherwise identical method names. A namespace inferred from a filename does not
replace those source owners.

The selector accepts canonical grammar rule names and their exact registered
source names. The current native adapter projects rules through `oracleKinds`,
so public CST nodes retain `mod_item`, `impl_item`, `struct_item`, `enum_item`
and `const_item`. Capture preserves that actual node term and identity. A
canonical selector alias never creates an alternate node owner.
Leading outer attributes are associated through canonical Rust trivia lexing;
enclosed attribute nodes, including field annotations, retain their own IDs,
coordinates and text. This does not execute a represented body or invoke a new
grammar parse. Ambiguous selectors/fields, nested function declarations, foreign
language metadata, mismatched source and exhausted byte/node/field/selector/work
budgets refuse. Work is bounded independently of the graph size and selectors.

The returned source capture is provenance, not a proof that removing a derive or
attribute preserves behavior. Actual serde-bearing Type declarations must remain
available in full. A later standalone value-declaration projection must explicitly
identify every removed serde annotation/token while preserving other derives,
all fields/variants and exact original method bodies. Full dependency/crate
behavior and a projected standalone control remain separate assertions.

Deferred controls retain actual annotations and methods through text/binary CST
transport, distinguish same-name methods/modules, verify byte/unit ranges after
a supplementary Unicode comment, and reject forged duplicate name owners and
bounded resource failures. Public declarations expose the exact capture shape.
No tests, source generation, validation or compilation ran for this source draft.
