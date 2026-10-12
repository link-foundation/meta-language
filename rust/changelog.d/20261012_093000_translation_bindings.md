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
