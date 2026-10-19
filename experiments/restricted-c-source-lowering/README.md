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

The size-query adapter retains `sizeof` as a source-owned layout query with
`byteSize:null`. It reads operand type facts without emitting calls, updates or
loads. A selected ABI supplies the unsigned result type; a typedef named
`size_t` cannot invent that target ABI. Arrays, potentially variably modified
types, unknown expressions and aggregate layout remain explicit obligations.

The C parser can select an ambiguous type/expression alternative. The adapter
retains that complete CST and its `sourceForm`, then records a separate namespace
refinement through the actual preceding type, value or function declaration.
Value shadowing wins over a type spelling; missing, later and ambiguous owners
stay unresolved. Actual typedef declarator roles also remain inspectable when
the parser projects a header type name as a primitive node. These finite
refinements leave general C parsing and source derivation as open obligations.

The [GNU type-size reference](https://www.gnu.org/software/c-intro-and-ref/manual/html_node/Type-Size.html)
describes ordinary operand non-evaluation and the variable-array exception;
[GCC's array documentation](https://gcc.gnu.org/onlinedocs/gcc/Variable-Length.html)
describes retained dynamic bounds. The prototype conservatively refuses those
array cases and GNU void/function extensions. These are semantic references,
not local compiler observations. The existing malloc/primitive-refusal input and
all eight original control groups remain; its obsolete unresolved-`sizeof`
assertions now require the source query and unresolved physical layout instead.

`data/sizeof-source-publication.lino` retains both complete original files,
the exact source edits, the adapter and all new controls. Run
`node experiments/restricted-c-source-lowering/sizeof-source-publication.mjs --check`
from the repository root to validate replay and complete original inverses.
Explicit `--write` regenerates the three source files from that approved archive.
Source hashes and byte counts complement the well-formed Unicode requirement;
malformed surrogate source text refuses before UTF-8 conversion.
