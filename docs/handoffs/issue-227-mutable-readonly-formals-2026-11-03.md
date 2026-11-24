# Mutable value parameters in readonly-result helpers

The three-profile source composition now initializes local storage for actual
mutable value formals of readonly-result functions. The existing source header
registers real owned slots. The parsed-callable adapter checks their parameter
index/name, source slot identity, direct closed referent and declaration inventory,
then publishes the existing initialize facet on a read of that original formal.
A lexical entry binding becomes the initial SSA state seen by the existing body
lowerer. No new parameter, return packet or synthetic source signature is added.

The wrapper is limited to actual readonly composition bodies and owned formals.
Historical independent profiles and closed/FnMut body paths remain unchanged.
Stored references, borrowed vectors and exclusive mutable formals are not relabeled
as owned value slots. Existing storage admission and final closed-region checks
retain those boundaries. A returned reference cannot escape from the locally
owned parameter allocation; it must derive from actual reference-bearing inputs.

The exact former mutable-formal refusal source is retained as a positive original,
including its empty main and empty stdout. Its earlier refusal was the explicit
missing-entry-carrier guard; this source batch supplies that carrier rather than
removing an oracle. Two additional originals exercise real scalar writes and owned
Vec mutation followed by references to an independent input. Two new refusals
cover returning the local parameter and unadmitted exclusive-formal composition.
All preceding input programs remain. The group has sixteen positive originals and
twelve refusals. The full positive CI driver inventory becomes 359 authored rows
(295 literal and 64 actual-source scenarios) across eighteen groups.

Deferred structural controls retain the original function signature and exact
formal token range, require one real entry initializer, and reject deleting it
before state writes. The existing per-original CI driver also retains typed Links
text/binary and original/product assertions for these appended inputs.

No tests, represented imports, source generation, validation gates or compilation
ran. Counts describe authored inputs, not completion, runtime evidence or proof of
whole-project translation. Remaining gaps include projected-field exclusive calls,
borrowed values inside closed callbacks, moved-owner mixed reinitialization and
standard collection/profile3 composition.
