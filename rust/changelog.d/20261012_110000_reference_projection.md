JavaScript source projection now selects reference semantics for mutation and
inferred structural identity comparisons, preserves receivers and selected
fields or indices in write operations, and represents mutable closure captures
with explicit shared cell allocation, reads and writes. Scalar equality keeps
the existing value representation.

For-of source projection distinguishes a live iterable from an explicit
Array.from snapshot. Generated array loop bounds are retained as internal proof
metadata; this does not establish that a sparse array slot contains a value.
