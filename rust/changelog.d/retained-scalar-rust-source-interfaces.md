Draft an explicit Rust emission mode that reconstructs retained scalar source
headers, including `usize`, `isize`, immutable-reference inputs and lifetime
binders. Closed scalar calls use ordered temporary values and boundary
conversions. Select `rustSourceHeaderEmissionModel:
'retained scalar Rust function interface 1'` when emitting a program made with
`createRustSourceOwnerProgramLinks`. This mode requires 64-bit pointer targets.
The complete requirements draft and verification remain unfinished.
