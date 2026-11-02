# Primitive compound writes through actual exclusive source state

Unexecuted source draft only. No tests, source import execution, generation,
validation, compilation, parity observations or ledger updates ran.

Concrete own-code uses include `self.count += 1` in
`rust/src/translation/output.rs:284`, `self.temporary += 1` in
`rust/src/translation/rust/owned_local_state.rs:100`, and `index += 1` in
`rust/src/translation/ir.rs:916`. Their containing functions still have
independent source obligations; this batch does not claim their complete
admission. In particular, actual `Scope<'a>` also stores a borrowed dyn Fn,
uses BTreeSet/iterator conversion, performs formatted candidate construction,
and has an unbounded source loop. The existing source project composer already
handles supplied module composition; these are semantic source obligations.

`rust-exclusive-source-compound-rules.js` selects only built-in primitive
`+=`, `-=`, `*=`, `/=` and `%=`. The actual retained primitive owner, width,
signedness, direct wrappers and proven position must agree with the existing
primitive-source policy. String/custom AddAssign dispatch, bitwise/shift
compound operations, raw pointers, indexed places and nonprimitive order keep
their explicit boundaries. Unselected old profiles keep their refusal paths.
The existing exclusive model/version boundary remains Rust >=1.90.0 <1.100.0;
deferred edition-2024 originals cover Node >=22.15 <23 or >=24 <25.

The [Rust Reference compound-assignment rule](https://doc.rust-lang.org/reference/expressions/operator-expr.html#compound-assignment-expressions)
specifies that when both operand types are known primitives, the right operand
is evaluated before the assigned place. This differs from simply building a
binary expression that reads its left operand first. The new reader holds the
actual RHS exactly once in an ordinary let, then reads the selected place from
the latest source-state root. Existing ordered state lowering therefore
preserves an RHS call that writes the same root, and preserves sibling changes
when reconstructing a nested field. Ordinary checked binary arithmetic retains
the existing overflow, division and remainder interpretation. There is no new
native arithmetic body or runtime mutation cell.

The parser hooks `readRustExclusiveSourceCompoundAssignment(parser,path)`
before existing ordinary state assignment. It admits only an actual mutable
lexical slot and its real field layout, or an actual exclusive scalar formal
dereference. Source types are known before primitive admission; no custom
operator is guessed from an identifier spelling. State availability is checked
after reading the RHS, and actual read/write/loan guards remain in force.

The writer seam is now exported as
`recordRustExclusiveSourcePlaceWrite(parser,place,value,path,span)` from
`rust-exclusive-source-calls.js`. Its actual place shape is
`{slot,layouts,fields,field}` from existing parser-owned state inventory. It
requires the current callable, real available root, nonoverlapping surviving
loans, compatible value/source position and the active argument write guard.
It authors the existing private write packet for consumption by the ordinary
effect continuation; no arbitrary root/type summary grants authority.

An independent static defect in field Vec push was corrected in the same seam:
the previous code spread the whole enclosing slot root when the receiver was a
Vec field. The source now projects the real selected Vec through the retained
field layouts before appending, then reconstructs all owning ancestors using
the existing writer. Whole-Vec receiver behavior keeps the empty layout path.

`rust-exclusive-source-compounds.mjs` contains 12 positive literal originals
and eight independently compiler-classified refusal inputs. Positive controls
cover scalar/field/nested writes, same-root RHS effects, primitive signed
division/remainder, f64 multiplication, unit result, RHS early completion and
Vec field accumulation with sibling counting. Deferred tests also cover the
independent primitive owner rule, concrete text/binary transport, typed Links
and every emitter. They are future controls, not evidence or a claim that
automatic source projection/native frontend integration has run.
