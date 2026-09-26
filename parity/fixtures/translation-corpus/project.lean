-- Portable-core conformance project: control flow, recursion, data types,
-- modules, effects, and proofs in one Lean program.

namespace Arith

def fact : Nat → Nat
  | 0 => 1
  | n + 1 => (n + 1) * fact n

def sumTo (n : Nat) : Nat :=
  match n with
  | 0 => 0
  | k + 1 => sumTo k + (k + 1)

def fib : Nat → Nat
  | 0 => 0
  | 1 => 1
  | n + 2 => fib (n + 1) + fib n

def monus (a b : Nat) : Nat := a - b

def halve (x : Int) : Int := x / 2

def remainder (x y : Int) : Int := x % y

end Arith

inductive Tree where
  | leaf : Tree
  | node : Tree → Nat → Tree → Tree

namespace Tree

def size : Tree → Nat
  | leaf => 0
  | node l _ r => size l + 1 + size r

def total (t : Tree) : Nat :=
  match t with
  | .leaf => 0
  | .node l v r => total l + v + total r

def mirror : Tree → Tree
  | leaf => leaf
  | node l v r => node (mirror r) v (mirror l)

def insert (t : Tree) (x : Nat) : Tree :=
  match t with
  | leaf => node leaf x leaf
  | node l v r => if x < v then node (insert l x) v r else node l v (insert r x)

end Tree

def classify (n : Int) : String :=
  if n < 0 then "negative" else if n == 0 then "zero" else "positive"

def describe (n : Nat) : String :=
  let doubled := n + n
  let label := classify (Int.ofNat doubled - 10)
  s!"{n} doubled is {doubled} ({label})"

def sample : Tree :=
  Tree.insert (Tree.insert (Tree.insert (Tree.insert Tree.leaf 5) 2) 8) 3

theorem fact_five : Arith.fact 5 = 120 := by decide

theorem sumTo_formula (n : Nat) : 2 * Arith.sumTo n = n * (n + 1) := by
  induction n with
  | zero => rfl
  | succ k ih => simp [Arith.sumTo, Nat.mul_add, Nat.add_mul, ih]; omega

theorem mirror_mirror (t : Tree) : Tree.mirror (Tree.mirror t) = t := by
  induction t with
  | leaf => rfl
  | node l v r ihl ihr => simp [Tree.mirror, ihl, ihr]

theorem size_mirror (t : Tree) : Tree.size (Tree.mirror t) = Tree.size t := by
  induction t with
  | leaf => rfl
  | node l v r ihl ihr => simp [Tree.size, Tree.mirror, ihl, ihr]; omega

def main : IO Unit := do
  IO.println s!"fact 20 = {Arith.fact 20}"
  IO.println s!"sumTo 100 = {Arith.sumTo 100}"
  IO.println s!"fib 25 = {Arith.fib 25}"
  IO.println s!"monus 3 5 = {Arith.monus 3 5}"
  IO.println s!"halve -7 = {Arith.halve (-7)}"
  IO.println s!"remainder -7 3 = {Arith.remainder (-7) 3}"
  IO.println s!"remainder 7 0 = {Arith.remainder 7 0}"
  let t := sample
  IO.println s!"size = {Tree.size t}, total = {Tree.total t}"
  IO.println s!"mirrored total = {Tree.total (Tree.mirror t)}"
  IO.println (describe 3)
  IO.println (describe 7)
  IO.println (classify (-4))
