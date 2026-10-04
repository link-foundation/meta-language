import Util.Basic

open Util

def quadruple (n : Nat) : Nat := double (double n)

def boxed : Util.Box Nat := { value := 3 }

def total : Nat := sumTo 4

def main : IO Unit := greet "lean"

attribute [simp] double_eq

def six : Nat := twice 3

theorem quadruple_eq (n : Nat) : quadruple n = double n + double n :=
  double_eq (double n)

theorem double_zero : double 0 = 0 := by
  rw [double_eq]
