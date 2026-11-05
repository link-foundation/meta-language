# Closed standard collection equality draft

Authored JavaScript sources and deferred controls only. No tests, original Rust,
behavior imports, source generation, validation, builds or compilation ran for
this batch. No ledger observations or completion claim are made.

`rust-standard-collection-equality.js` implements extensional HashMap and
HashSet equality over the existing closed primitive Eq/String domains. Map
keys and owned values both compare; Set membership compares. Allocation roots,
mutation revisions and the private entry-vector order do not contribute to Eq.
No observable hash iteration order, host Map/Set fallback or custom hasher/Eq
implementation is supplied.

The exported begin/step/run/execute API returns explicit continue, completed,
invalid or resource-limit transitions. Each comparison charges primitive or
String-storage work. A bounded run retains both complete operands and the exact
next comparison on exhaustion. External continuations recheck their complete
actual equal prefix, selected key pair and charged work; cursors cannot skip
unequal entries or forge already-paid comparison work. Supported source versions
remain Rust >=1.90.0 <1.100.0; primitive domains and pointer-width rules come
from the existing state validator. A producer within that bounded range may be
compared by another supported active version without erasing its source origin.

`compareRustStandardCollectionValuePackets` supplies a genuine shared callable
body in the automatically projected value-packet library. Source admission now
requires packet, kernel and equality bindings with actual complete bodies and
original declaration owners, plus exact whole source-checked graph correspondence.
An equality-shaped descriptor or provenance role alone is insufficient. The
library owner facet gains the explicit `equality` role and its checked interface.

`rust-standard-collection-equality-source.js` lowers direct `==`/`!=`, actual
standard `.eq(&other)`/`.ne(&other)` calls and unshadowed core/std PartialEq UFCS
calls (including genuine imported trait aliases). Both operands retain the same
actual concrete nominal carrier/domain identity. Private wrappers unwrap only at
the shared kernel boundary and inspect its real completion variants. Actual
operand expressions occur once in source order. Source ownership checks and
actual RHS packet/place effects prevent mutation of the already borrowed left
collection place. Ordinary ordering, nested aggregate collection comparison,
Debug assertion formatting and custom implementations retain explicit refusals.

The existing optional profile is unchanged:
`standardCollectionSourceModel: 'Rust closed standard collections 1'`, a supplied
actual projected library and explicit 64-bit source target. Raw kernel 32-bit
states remain available independently. The public parser hooks now call Eq
lowering before collection structural-observation refusals; unrelated comparisons
and default-profile guards remain in place. The companion Vec helper finalizer
is imported into rust.js immediately after owned-record effect lowering and
before collection source finishing/private-marker inspection.

Deferred controls include extensional order/root/revision independence, exact
String/integer values, zero-work size preflight, resource continuation resumption,
forged cursors, provenance/domain/version rejection and real projected source
parsing/checking. The independent literal file retains 24 native original inputs
and six explicit source refusals. Expected outputs are declarations, not
executed evidence. Earlier collection inputs and refusals are preserved.

Remaining frontiers include genuine Hash iteration/Debug formatting, recursive
aggregate Eq, custom Hash/Eq/hasher behavior, collection source target32 and
actual automatic generation/native verification of the complete shared library.
