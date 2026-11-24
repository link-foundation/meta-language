# Actual source-state write projection

One shared bounded graph walk now derives outer write sets from private record
write and exclusive call/write nodes. Actual call writeback roots come from its
retained borrow slots. Unknown private operations and loops return the existing
conservative all-state projection. Ordinary data cannot grant write authority.
All nested operands remain inspected within the existing source budget.

Conditional joins and main-effect packets include only selected actual outer
roots. Omitted bindings retain their exact original slot and SSA name; each
continuation checks that invariant and preserves the full environment inventory.
Linear/read-only main expressions need no state packet when the write set is
empty. This supersedes the earlier linear-only optimization and shares the same
projection with if, short-circuit and match joins. No branch or effect is skipped.

A conditional move of an otherwise unchanged root can now remain outside the
packet until a later complete assignment supplies a fresh epoch. Its earlier
maybe-moved state is still unavailable before assignment. An unrelated written
scalar is carried independently. Mixed move/write arms for the same selected
root remain a native-valid refusal: that selected packet slot still requires a
real uninitialized carrier, and the write projection does not pretend to provide
one. The exact previous no-write conditional input is retained as a positive.

Four appended positive originals cover the retained conditional input, its
nonmoving branch a separately written scalar, and a genuine FnMut writeback beside an unrelated moved vector. Refusals retain a premature
read and explicitly record the mixed move/write carrier frontier. A deferred
checked-graph control requires the scalar-only state packet shape and retains
original/emitted stdout assertions. The mixed group now has 22 positives and
15 refusals. The full original/product driver declares 365 positive rows across
18 groups (301 literal originals and 64 actual-source scenarios).

All changes remain source drafts. No tests, represented imports, source
generation, validation gates or compilation ran. These counts are inventories,
not observed passes, completion percentages or full-project translation evidence.
