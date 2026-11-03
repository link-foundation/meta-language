# Immediate whole replacement after a main join

A source-derived local kill projection now complements actual-write projection.
It applies to the exact main if expression or a directly adjacent lexical
binding in a callable body when the immediately following actual effect is a private complete-root write to an existing outer slot. Its
RHS is inspected within the existing source graph budget. Any access to that
root, unknown private operation, field path, intervening effect or unsupported
root form prevents the optimization. The private descriptor is not serialized
as user data and cannot authorize arbitrary missing state.

Only this exact join omits the replaced value from its state packet. Nested
joins retain their independent state sets; callable join descriptors are scoped
in a private WeakMap and restored on every lexical exit. Every branch still evaluates in the
original order, including replacement values and writes; other roots remain in
the packet as required. Joined SSA environments retain the old name only as an
unread dead placeholder until the next complete write creates a fresh epoch.
The independent region visitor still rejects reading its moved/stale storage.
Admitted owned storage remains destructor-free; no user Drop behavior is removed.

The exact previous mixed move/write input becomes a retained positive. Additional
originals exercise the written arm and an independently updated scalar. Refusals
preserve a RHS read of the possibly moved root and an intervening print: the latter
is native-valid but needs a more general carrier/continuation proof. This narrow
optimization does not claim that general uninitialized state transport is solved.
Condition-side replacement of this root, nested joins and non-immediate
overwrites retain separate proof boundaries. Two additional readonly-result
function originals exercise both branches of adjacent lexical replacement.

A deferred checked-graph control requires that the scalar packet survives while
only the immediately killed vector root is absent. All original/product and
text/binary typed graph assertions remain in the existing reusable CI driver.
The mixed fixture now has 27 positive originals and 16 refusals. Driver inventory
is 370 authored positives: 306 literal and 64 actual-source, across 18 groups.
No tests, represented execution, generation, validation gates or compilation ran.
The scope above is source drafting, not runtime evidence or overall completion.
