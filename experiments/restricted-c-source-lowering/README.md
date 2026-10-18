This experiment lowers a supplied C CST into inspectable typed instructions and
stores the complete CST, source tokens, lowering inputs, declarations and remaining
obligations in Links data. It follows ordinary source calls from selected roots;
helper names cannot select replacement implementations. References to enumerators,
global variables, parameters and local variables retain their actual declaration
owners through archive reload. Replay rejects altered instructions and equal-looking
copies substituted for those owners.

Run the authored controls with
`cd js && node --test tests/translation-restricted-c-lowering.test.js`.
No native compiler or upstream corpus is involved. The project architecture and
delivery requirements remain in [the vision](../../docs/vision.md).

`capture-markdown.mjs` runs only in GitHub CI, reusing the existing pinned Cargo
archive and authenticated Markdown source receipt. It lowers the original block
scanner's fenced-code function and its ordinary declared call closure, retaining
the entire scanner CST. External sources stay in memory. The output is a `.lino`
artifact; it is never a vendored grammar or a behavioral support declaration.

The collector retains the full archive and a small transport-cost receipt before
replay. Encoding keeps the codec's default bounds; replay uses an explicit caller
budget within the CI input's fixed source-sized ceiling. Default traversal refusal
and complete data/owner comparison remain required. No CST or token is removed to
fit the transport budget.

The supplied CST is an explicit input assumption. Its reconstruction and internal
lowering replay do not independently prove grammar derivation. C memory layout,
pointer behavior, external primitives, preprocessor semantics, arithmetic and
sequencing still need proofs and executable lowering. The declared ABI is an input,
not an observed compiler ABI. `checkedPortableProgram` remains `null`; this
experiment does not establish executable C translation or complete Markdown support.
