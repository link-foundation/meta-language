/-- ECMAScript `%` on binary64 (C `fmod`): the exact remainder, truncated,
with the sign of the dividend; computed on the bits, so it is exact. -/
def ml_float_rem (x y : Float) : Float :=
  let decompose (f : Float) : Nat × Int :=
    let b : Nat := f.toBits.toNat
    let exponent : Nat := (b / 2 ^ 52) % 2048
    let fraction : Nat := b % 2 ^ 52
    if exponent == 0 then (fraction, -1074) else (fraction + 2 ^ 52, (exponent : Int) - 1075)
  if x.isNaN || y.isNaN || x.isInf || y == 0 then 0 / 0
  else if y.isInf || x == 0 then x
  else
    let (mx, ex) := decompose x
    let (my, ey) := decompose y
    let e := min ex ey
    let r := (mx * 2 ^ (ex - e).toNat) % (my * 2 ^ (ey - e).toNat)
    let magnitude := Float.scaleB (Float.ofNat r) e
    if x.toBits.toNat ≥ 2 ^ 63 then -magnitude else magnitude

def main : IO Unit := do
  let stdin ← IO.getStdin
  let mut line ← stdin.getLine
  let mut bad := 0
  let mut total := 0
  while line != "" do
    match (line.trimAsciiEnd.copy.splitOn " ").map String.toNat? with
    | [some a, some b, some c] =>
      total := total + 1
      let r := ml_float_rem (Float.ofBits (UInt64.ofNat a)) (Float.ofBits (UInt64.ofNat b))
      let ok := r.toBits.toNat == c || (r.isNaN && (Float.ofBits (UInt64.ofNat c)).isNaN)
      if !ok then
        bad := bad + 1
        IO.println s!"mismatch {a} {b}: {r.toBits.toNat} vs {c}"
    | _ => pure ()
    line ← stdin.getLine
  IO.println s!"{total} cases, {bad} mismatches"
