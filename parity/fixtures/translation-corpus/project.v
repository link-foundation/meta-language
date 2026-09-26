(* Portable-core conformance project: control flow, recursion, data types,
   modules, effects, and proofs in one Rocq program. *)

From Stdlib Require Import NArith ZArith String DecimalString List Lia Recdef.
Import ListNotations.
Open Scope string_scope.

Definition show_N (n : N) : string := NilEmpty.string_of_uint (N.to_uint n).
Definition show_nat (n : nat) : string := NilEmpty.string_of_uint (Nat.to_uint n).
Definition show_Z (z : Z) : string := NilZero.string_of_int (Z.to_int z).

Module Arith.

Function fact (n : N) {measure N.to_nat n} : N :=
  if N.eqb n 0 then 1%N else (n * fact (N.pred n))%N.
Proof.
  intros n Hn. apply N.eqb_neq in Hn. lia.
Defined.

Fixpoint sumTo (n : nat) : nat :=
  match n with
  | O => 0
  | S k => sumTo k + S k
  end.

Fixpoint fib (n : nat) : nat :=
  match n with
  | 0 => 0
  | 1 => 1
  | S (S m as p) => fib p + fib m
  end.

Definition monus (a b : N) : N := (a - b)%N.

Definition halve (x : Z) : Z := (x / 2)%Z.

Definition remainder (x y : Z) : Z := Z.modulo x y.

End Arith.

Inductive Tree : Type :=
| leaf : Tree
| node : Tree -> N -> Tree -> Tree.

Module Tree.

Fixpoint size (t : Tree) : N :=
  match t with
  | leaf => 0%N
  | node l _ r => (size l + 1 + size r)%N
  end.

Fixpoint total (t : Tree) : N :=
  match t with
  | leaf => 0%N
  | node l v r => (total l + v + total r)%N
  end.

Fixpoint mirror (t : Tree) : Tree :=
  match t with
  | leaf => leaf
  | node l v r => node (mirror r) v (mirror l)
  end.

Fixpoint insert (t : Tree) (x : N) : Tree :=
  match t with
  | leaf => node leaf x leaf
  | node l v r => if N.ltb x v then node (insert l x) v r else node l v (insert r x)
  end.

End Tree.

Definition classify (n : Z) : string :=
  if (n <? 0)%Z then "negative" else if (n =? 0)%Z then "zero" else "positive".

Definition describe (n : N) : string :=
  let doubled := (n + n)%N in
  let label := classify (Z.of_N doubled - 10) in
  show_N n ++ " doubled is " ++ show_N doubled ++ " (" ++ label ++ ")".

Definition sample : Tree :=
  Tree.insert (Tree.insert (Tree.insert (Tree.insert leaf 5) 2) 8) 3.

Theorem fact_five : Arith.fact 5 = 120%N.
Proof. reflexivity. Qed.

Theorem sumTo_formula (n : nat) : 2 * Arith.sumTo n = n * (n + 1).
Proof.
  induction n as [| k ih].
  - reflexivity.
  - simpl Arith.sumTo. nia.
Qed.

Theorem mirror_mirror (t : Tree) : Tree.mirror (Tree.mirror t) = t.
Proof.
  induction t as [| l ihl v r ihr].
  - reflexivity.
  - simpl. rewrite ihl, ihr. reflexivity.
Qed.

Theorem size_mirror (t : Tree) : Tree.size (Tree.mirror t) = Tree.size t.
Proof.
  induction t as [| l ihl v r ihr]; simpl; lia.
Qed.

Definition main : list string :=
  ("fact 20 = " ++ show_N (Arith.fact 20)) ::
  ("sumTo 100 = " ++ show_nat (Arith.sumTo 100)) ::
  ("fib 25 = " ++ show_nat (Arith.fib 25)) ::
  ("monus 3 5 = " ++ show_N (Arith.monus 3 5)) ::
  ("halve -7 = " ++ show_Z (Arith.halve (-7))) ::
  ("remainder -7 3 = " ++ show_Z (Arith.remainder (-7) 3)) ::
  ("remainder 7 0 = " ++ show_Z (Arith.remainder 7 0)) ::
  let t := sample in
  ("size = " ++ show_N (Tree.size t) ++ ", total = " ++ show_N (Tree.total t)) ::
  ("mirrored total = " ++ show_N (Tree.total (Tree.mirror t))) ::
  describe 3 ::
  describe 7 ::
  classify (-4) ::
  nil.

Eval vm_compute in main.
