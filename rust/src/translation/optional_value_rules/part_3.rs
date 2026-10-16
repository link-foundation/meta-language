// Generated from the complete checked optional kernel and its recorded source dependencies.
// Do not edit by hand.
use super::*;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_render_optional_strict_equality_loop2(
    format: &str,
    left_policy: &str,
    right_policy: &str,
    prefix: &str,
    separator: &str,
    mut cases: String,
    mut right_count: f64,
    left_absence: &str,
    mut second: f64,
) -> String {
    loop {
        if second < right_count {
            {
                let right_absence = self::read_optional_equality_absence(right_policy, second);
                {
                    let same = if left_absence == right_absence {
                        String::from("true")
                    } else {
                        String::from("false")
                    };
                    {
                        let cases_2 = format!(
                            "{}{}",
                            cases,
                            (format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (format!(
                                        "{}{}",
                                        (format!(
                                            "{}{}",
                                            (format!(
                                                "{}{}",
                                                (format!(
                                                    "{}{}",
                                                    (format!(
                                                        "{}{}",
                                                        separator.to_owned(),
                                                        prefix.to_owned()
                                                    )),
                                                    (String::from("("))
                                                )),
                                                (self::render_optional_equality_absent(
                                                    format,
                                                    left_policy,
                                                    left_absence
                                                ))
                                            )),
                                            (String::from(", "))
                                        )),
                                        (self::render_optional_equality_absent(
                                            format,
                                            right_policy,
                                            &(right_absence)
                                        ))
                                    )),
                                    (String::from(") => "))
                                )),
                                same
                            ))
                        );
                        {
                            let second_2 = second + 1f64;
                            {
                                (cases, second) = (cases_2, second_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { cases.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_render_optional_strict_equality_loop1(
    format: &str,
    left_policy: &str,
    right_policy: &str,
    prefix: &str,
    separator: &str,
    mut cases: String,
    mut left_count: f64,
    mut right_count: f64,
    mut first: f64,
) -> String {
    loop {
        if first < left_count {
            {
                let left_absence = self::read_optional_equality_absence(left_policy, first);
                {
                    let second = 0f64;
                    {
                        let cases_2 = self::ml_render_optional_strict_equality_loop2(
                            format,
                            left_policy,
                            right_policy,
                            prefix,
                            separator,
                            cases.clone(),
                            right_count,
                            &(left_absence),
                            second,
                        );
                        {
                            let first_2 = first + 1f64;
                            {
                                (cases, first) = (cases_2, first_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { cases.clone() };
    }
}
