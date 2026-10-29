import Std

/- Algebraic laws only. The adapter must separately establish sound classes,
   actual constructor ownership, the captured alternative, scope and evaluation
   order. This does not certify the JavaScript adapter or floating-point boxing. -/
namespace MetaLanguageConstructorSelection
universe u v w

theorem stable_default_argument {Index : Type u} {Value : Type v}
    (classes : Index → Nat) (values : Index → Value)
    (sameClass : ∀ first second, classes first = classes second → values first = values second)
    (fallback selected : Index) :
    (if classes selected = classes fallback then values fallback else values selected) = values selected := by
  by_cases same : classes selected = classes fallback
  · simp only [if_pos same]
    exact (sameClass selected fallback same).symm
  · simp only [if_neg same]

theorem constructor_result {Index : Type u} {Value : Type v} {Result : Type w}
    (classes : Index → Nat) (values : Index → Value)
    (sameClass : ∀ first second, classes first = classes second → values first = values second)
    (construct : Value → Result) (fallback selected : Index) :
    construct (if classes selected = classes fallback then values fallback else values selected) =
      construct (values selected) :=
  congrArg construct (stable_default_argument classes values sameClass fallback selected)

theorem binary_condition {Value : Type u} {Result : Type v}
    (choice : Bool) (construct : Value → Result) (left right : Value) :
    (if choice then construct left else construct right) = construct (if choice then left else right) := by
  cases choice <;> rfl
end MetaLanguageConstructorSelection
