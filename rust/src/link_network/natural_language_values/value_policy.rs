// Generated from js/src/natural-language-value-policy.js.
pub(super) fn calendar_month_days(year: u16, month: u8) -> u8 {
    match month {
        2 if year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400)) => {
            29
        }
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        1..=12 => 31,
        _ => 0,
    }
}

pub(super) fn preferred_grouping_delimiter(locale: &str, delimiter: char) -> bool {
    if locale == "ru" {
        delimiter == '.'
    } else {
        delimiter == ','
    }
}
