From Stdlib Require Import NArith ZArith String DecimalString List.
Open Scope string_scope.

Definition show_N (n : N) : string := NilEmpty.string_of_uint (N.to_uint n).
Definition show_Z (z : Z) : string := NilZero.string_of_int (Z.to_int z).

Definition monus (a b : N) : N := (a - b)%N.

Definition share (a b : N) : N := (a / b)%N.

Definition spare (a b : Z) : Z := (a mod b)%Z.

Definition big (n : N) : N := (n * n * n * n * n)%N.

Definition main : list string :=
  ("monus 3 5 = " ++ show_N (monus 3 5)) ::
  ("share 9 0 = " ++ show_N (share 9 0)) ::
  ("spare -7 0 = " ++ show_Z (spare (-7) 0)) ::
  ("big 10000 = " ++ show_N (big 10000)) ::
  nil.

Eval vm_compute in main.
