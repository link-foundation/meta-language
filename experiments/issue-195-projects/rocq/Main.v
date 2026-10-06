From Util Require Import Basic.

Definition quadruple (n : nat) : nat := double (double n).

Definition boxed : box nat := Box nat 3.

Definition total : nat := sum_to 4.

#[export] Hint Resolve double_eq : util.

Definition eight : nat := 4 ++2.

Lemma double_two : double 2 = 4.
Proof. solve_double. Qed.

Lemma quadruple_eq : forall n : nat, quadruple n = double n + double n.
Proof. intros n. unfold quadruple. apply double_eq. Qed.
