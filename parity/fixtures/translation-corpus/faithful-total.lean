def monus (a b : Nat) : Nat := a - b

def share (a b : Nat) : Nat := a / b

def spare (a b : Int) : Int := a % b

def big (n : Nat) : Nat := n * n * n * n * n

def main : IO Unit := do
  IO.println s!"monus 3 5 = {monus 3 5}"
  IO.println s!"share 9 0 = {share 9 0}"
  IO.println s!"spare -7 0 = {spare (-7) 0}"
  IO.println s!"big 10000 = {big 10000}"
