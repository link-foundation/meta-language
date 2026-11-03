# Overwrite liveness and readonly function printing source draft

This batch replaces the adjacent-only overwrite lookup with one bounded actual
source continuation scan. Main uses an index into its existing effect array;
callables follow actual lexical let/print bodies. No array suffix copies,
generators, original execution or effect reordering are needed. The first event
for an outer root must be a complete private whole write whose RHS cannot access
it. Intervening reads, partial writes and unknown private operations prevent that
root's omission. Other roots are tracked independently. A complete continuation
scan may also prove a root unused until scope exit. Existing storage admission
remains destructor-free; this is not an optimization for arbitrary Rust Drop.

Actual if, nominal match and short-circuit join identities select the descriptor.
Rebuilt match operands do not invent a second source owner. Each private callable
descriptor remains scoped/restored through the existing WeakMap. Only the selected
join omits that value; nested joins keep their own proof obligations. Unknown
loop effects remain conservative barriers. No uninitialized value, root read or
arbitrary sparse state packet is fabricated. The independent region visitor still
checks stale/moved binder epochs. A live joined owner continues to require its
actual state packet slot.

The checked region visitor now handles print text and its continuation separately,
using the existing display/borrow guards and actual temporary lifetime boundaries.
The ordered print lowerer places original result qualifiers on a held result at
the print sequence point, and removes them from the continuation wrapper. A Unit
statement qualifier cannot incorrectly requalify a borrowed-vector continuation.
The value is held exactly once and has its real original result type. No native
IO implementation, replacement body or formatting fallback is introduced.

Source review also found a transport mismatch with the existing native grammar:
`expression_statement` accepts `expression_ending_with_block` without a semicolon
in parity/grammars/native/rust.lino, rules at lines 33 and 116. Main now uses this
supported source boundary. Function implicit block handling checks actual opening
and closing tokens; parenthesized expressions do not inherit semicolon omission.
Existing specialized loop/block statement admission remains intact. No generated
grammar file is edited and non-Unit main expressions retain their existing guard.

The exact former intervening-print refusal is retained as a positive original,
with literal `checkpoint\ntrue\n` stdout. Additional originals cover unrelated
writes and print order, readonly prints and mutable parameter epochs, nominal
matches, short-circuit results, direct Unit blocks, unused roots and live joined
owners. Refusals preserve intervening reads, parenthesized statement boundaries
and unknown loop effects. That overwrite-only checkpoint contained 49 positives and 21
refusals. With the later loop-state batch, the shared original/product driver
declares 428 authored positives in 19 groups: 364 literal originals and 64
actual-source scenarios. These counts
are inputs, not completed requirements or observed passing evidence.

No tests, represented imports, source generation, validation gates, formatting or
compilation ran. No CI feedback was inspected. Root works alone and batches source
work ahead of pushes. Remaining broad source frontiers include nested/unknown
state carriers, projected-field exclusive calls, internal borrowed callback values,
standard collection/profile3 composition and complete own-project module coverage.

The integrated scope pass threads actual enclosing continuations into nested
lexical bodies. Branch ends cannot masquerade as callable ends. Exclusive
formals/callback environments remain mandatory-live even before later overwrites.
Abrupt completion packets and aborts are conservative scan barriers. New originals
retain nested live carriers, outer overwrites, original print order and mandatory
writeback values. Complete-scope flags are supplied only by full callable adapters
and complete main continuations; loop fragments do not receive that authority.

Print delimiter handling now accepts brace-delimited statement macros and final
Unit print tails without requiring a semicolon, while non-brace invocations before
another statement retain that requirement. Earlier boundary and non-Unit guards
remain intact. This source batch includes 22 appended positive programs relative
to the 370-row checkpoint, preserving one exact former refusal as positive.

The separate Unit-fixture author also appends four original function controls:
semicolon-free conditional, nominal match and direct Unit block statements,
and a parenthesized conditional with its required semicolon. Earlier inputs and
assertions remain. Its generated Links fixture is deliberately not regenerated
during source drafting; these declarations do not increase the 392 state-driver
inventory and provide no execution evidence.
