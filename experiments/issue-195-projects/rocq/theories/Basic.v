Universe u.

Inductive box (A : Type) : Type := Box : A -> box A.

Definition double (n : nat) : nat := n + n.

Fixpoint sum_to (n : nat) : nat :=
  match n with
  | 0 => 0
  | S m => n + sum_to m
  end.

Lemma double_eq : forall n : nat, double n = n + n.
Proof. reflexivity. Qed.

Create HintDb util.

Ltac solve_double := intros; unfold double; reflexivity.

Notation "x ++2" := (double x) (at level 1).
