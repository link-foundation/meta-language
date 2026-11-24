# Copied shared-reference vector indexing source draft

The source array-index adapter now distinguishes an actual signature3 standard
`Vec<&T>` from direct scalar arrays. It requires the existing complete canonical
vector/element owner, matching element type and shared-reference position,
closed referent admission, and live original source loans. Historical scalar
arrays keep their unchanged receiver/usize checks. The reference itself is Copy;
its nominal referent receives no Copy, Clone or mutable authority.

An indexed result keeps the element's original lifetime and referent-region
loans. It does not borrow the vector allocation merely because the reference
was copied from that vector. The existing exact usize conversion and checked
index operation retain bounds/error behavior. No in-bounds proof is fabricated.
The region adapter reads the actual checked vector and coordinate, then preserves
its element position and referent regions. Root adds only the explicit index operation and dedicated visitor dispatch;
the current agent retains ownership of its independent state-packet changes.

All 23 existing actual Type children-length scenarios remain. Nine independent
value/order scenarios distinguish Nat/Int/Float children, with length guards
before each index. Record observations retain original field order rather than
borrowing the mapper's sorted output oracle. The exact former indexed `matches!`
input is retained and promoted into the positive group: source inspection shows
its old indexed-operation refusal is superseded by the new dedicated adapter,
and the existing ordinary matches expansion already preserves actual patterns.
No native-invalid input becomes a successful source program. Two additional
refusals retain source-owner invalidation and reject an invented static lifetime.

The current actual children inventory has 33 positive scenarios and six refusal
inputs. Together with mapper/other groups the positive native-control inventory
contains 340 authored rows, not 340 completed requirements. Source generation,
behavioral execution, graph validation gates and compilation are deferred. No
observation or passing-runtime claim is recorded here.
