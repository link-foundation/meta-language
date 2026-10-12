The JavaScript translation frontend now reads local arrow functions and
function expressions, immutable captures, array bindings in declarations and
for-of loops, string and Number switch cases, literal record constants,
nullable and structural JSDoc types, nullish defaults, and optional fields.
These forms enter the checked translation representation and target emitters;
mutable captures retain a precise diagnostic.

Both lexical runtimes distinguish statement block braces from expression
braces when deciding whether a slash starts a regular expression. The shared
frontend decisions also distinguish Rust Unicode whitespace trimming from
JavaScript trimming, with Rust decisions generated from JavaScript and
reviewed Links Notation style decorators.

Optional JSDoc parameters retain an omitted `undefined` value, separately from
an actual JavaScript default expression. Named functions can be higher order
arguments and have the JavaScript `typeof` result `function`. Decimal Rust
binary64 literals use shared generated lexical boundary decisions in both
runtime packages; ranges, member names and integer suffixes keep their own
tokens, and binary32 remains a separately reported translation gap.
