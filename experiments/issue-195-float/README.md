# JavaScript Number helpers for the translation targets (issue 195)

JavaScript `number` values are IEEE-754 binary64. Each target has to reproduce
two things that are not built in:

- `Number::toString`. This is the shortest decimal that rounds back to the
  value. If two candidates are equally short, it takes the nearer one, and on
  a tie the even one. The result is laid out as ECMAScript prescribes.
- The `%` operator. This is C `fmod`: the exact truncated remainder, with the
  sign of the dividend.

| Helper | Target | Checked against V8 |
| --- | --- | --- |
| `js-number.lean` | Lean `Float` | 20 039 values (`gen-bits.mjs`) and 20 000 tie-heavy values (`gen-ties.mjs`) |
| `js-number-rocq.v.head` | Rocq `PrimFloat.float` | 300 values (`gen-rocq.mjs`); it runs in `vm_compute` |
| `js_number.rs` | Rust `f64` | both sets. `{:e}` rounds a shortest-digit tie away from zero, so the helper checks the exact expansion and picks the even neighbour |
| `float-rem.lean` | Lean `%` | 1041 pairs (`gen-rem.mjs`) |
| `float-rem.v.head` | Rocq `%` and SameValue | 1041 pairs (`gen-rem-rocq.mjs`), using exact hexadecimal float literals |

To reproduce:

```sh
node gen-bits.mjs && node gen-ties.mjs && node gen-rem.mjs
lean --run js-number.lean < bits.txt | diff - expected.txt
lean --run js-number.lean < ties-bits.txt | diff - ties-expected.txt
rustc -O js_number.rs -o /tmp/js_number && /tmp/js_number < ties-bits.txt | diff - ties-expected.txt
lean --run float-rem.lean < rem-cases.txt
node gen-rem-rocq.mjs && rocq c float_rem_test.v
node gen-rocq.mjs && rocq c js_number_test.v
```
