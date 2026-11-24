From Stdlib Require Import Arith.PeanoNat Bool.Bool.
Set Universe Polymorphism.

(* Algebraic laws under explicit class soundness. The source adapter, captured
   choice, lexical owners and effect order require their separate obligations. *)
Section ConstructorSelection.
Context {Index Value : Type}.
Variable classes : Index -> nat.
Variable values : Index -> Value.
Hypothesis same_class : forall first second,
  classes first = classes second -> values first = values second.

Lemma stable_default_argument (fallback selected : Index) :
  (if Nat.eq_dec (classes selected) (classes fallback)
   then values fallback else values selected) = values selected.
Proof.
  destruct (Nat.eq_dec (classes selected) (classes fallback)) as [same | different].
  - symmetry. apply same_class. exact same.
  - reflexivity.
Qed.

Lemma constructor_result (Result : Type) (construct : Value -> Result)
    (fallback selected : Index) :
  construct (if Nat.eq_dec (classes selected) (classes fallback)
    then values fallback else values selected) = construct (values selected).
Proof.
  f_equal. apply stable_default_argument.
Qed.

Lemma binary_condition (Result : Type) (choice : bool)
    (construct : Value -> Result) (left right : Value) :
  (if choice then construct left else construct right) =
  construct (if choice then left else right).
Proof.
  destruct choice; reflexivity.
Qed.
End ConstructorSelection.
