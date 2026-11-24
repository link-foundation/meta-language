// meta-language:self-translation:v1 source=JavaScript target=Rust sha256=41937dbefce372e530a063bc4b03c0221d06b9bf70ab4892e47775daf3decfb5 bytes=815

// meta-language:prelude begin
#![allow(
    unused,
    unreachable_patterns,
    non_snake_case,
    non_camel_case_types,
    invalid_nan_comparisons
)]
// meta-language:prelude end

// meta-language:translated JavaScript export_statement items=1 sha256=1148900f7b32ec385527697db3a2848530917eff0b75531729f3f6ab08f1ab92
// | // The Links Notation versioned reference boundary consumes a tilde, one or
// | // more decimal digits, an opening brace, and the payload through its closing
// | // brace. Hosts supply one code unit at a time and use the upstream decoder.
// | // State 3 marks a reserved prefix, state 4 a complete token, and -1 a mismatch.
// | /** @param {number} state @param {number} unit @returns {number} */
// | export function advanceReferenceLiteral(state, unit) {
// |   if (state === 0) return unit === 126 ? 1 : -1; // ~
// |   const digit = unit >= 48 && unit <= 57;
// |   if (state === 1) return digit ? 2 : -1;
// |   if (state === 2) {
// |     if (digit) return 2;
// |     return unit === 123 ? 3 : -1; // {
// |   }
// |   if (unit === 125) return 4; // }
// |   if (unit === 32 || unit === 9 || unit === 13 || unit === 10 || unit === 40 || unit === 41) return -1;
// |   return 3;
// | }
pub fn advance_reference_literal(state: f64, unit: f64) -> f64 {
    if (state == (0f64)) {
        if (unit == (126f64)) { 1f64 } else { (-1f64) }
    } else {
        {
            let digit = ((unit >= (48f64)) && (unit <= (57f64)));
            if (state == (1f64)) {
                if digit { 2f64 } else { (-1f64) }
            } else {
                if (state == (2f64)) {
                    if digit {
                        2f64
                    } else {
                        if (unit == (123f64)) { 3f64 } else { (-1f64) }
                    }
                } else {
                    if (unit == (125f64)) {
                        4f64
                    } else {
                        if ((((((unit == (32f64)) || (unit == (9f64))) || (unit == (13f64)))
                            || (unit == (10f64)))
                            || (unit == (40f64)))
                            || (unit == (41f64)))
                        {
                            (-1f64)
                        } else {
                            3f64
                        }
                    }
                }
            }
        }
    }
}
