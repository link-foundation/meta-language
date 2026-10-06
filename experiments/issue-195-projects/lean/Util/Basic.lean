namespace Util

universe u

structure Box (α : Type u) where
  value : α

def double (n : Nat) : Nat := n + n

def sumTo : Nat → Nat
  | 0 => 0
  | n + 1 => (n + 1) + sumTo n

def greet (name : String) : IO Unit := IO.println name

theorem double_eq (n : Nat) : double n = n + n := rfl

notation "twice " x => double x

end Util
