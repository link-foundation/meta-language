// Generated from complete js/src/translation/proposition-value-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_proposition_value_form(kind: &str) -> String {
    if kind == "bool" {
        String::from("value")
    } else if kind == "not" {
        String::from("negation")
    } else if ((kind == "and") || (kind == "or")) || (kind == "implies") {
        kind.to_owned()
    } else if kind == "forall" {
        String::from("finite quantification")
    } else if self::accept_proposition_value_comparison(kind) {
        String::from("comparison")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_proposition_value_conditional_branch(operation: &str, truth: bool) -> String {
    if operation == "and" {
        if truth {
            String::from("right")
        } else {
            String::from("false")
        }
    } else if operation == "or" {
        if truth {
            String::from("true")
        } else {
            String::from("right")
        }
    } else if operation == "implies" {
        if truth {
            String::from("right")
        } else {
            String::from("true")
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_proposition_value_quantifier_join() -> String {
    String::from("and")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_proposition_value_comparison(operation: &str) -> bool {
    (((((operation == "eq") || (operation == "ne")) || (operation == "lt")) || (operation == "le"))
        || (operation == "gt"))
        || (operation == "ge")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_proposition_value_quantifier(kind: &str) -> Vec<f64> {
    if kind == "bool" {
        vec![0f64, 1f64]
    } else if kind == "unit" {
        vec![0f64]
    } else {
        Vec::<f64>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_proposition_value_expansion_limit() -> f64 {
    256f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_proposition_value_domain_error(kind: &str) -> String {
    {
        let ml_library_condition_1 = {
            let ml_value = (self::plan_proposition_value_quantifier(kind)).len() as f64;
            ml_value != 0f64 && !ml_value.is_nan()
        };
        if ml_library_condition_1 {
            String::new()
        } else {
            String::from(
                "a proposition value requires an exhaustive Bool or Unit quantifier domain",
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rocq_proposition_equality_value(equality: &str, negate: bool) -> String {
    {
        let decision = format!(
            "(if (ltac:(decide equality) : {{{}}} + {{~ ({})}}) then true else false)",
            equality.to_owned(),
            equality.to_owned()
        );
        if negate {
            format!("(negb {decision})")
        } else {
            decision
        }
    }
}
