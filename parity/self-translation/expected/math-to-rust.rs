// meta-language:self-translation:v1 source=JavaScript target=Rust sha256=30bcbd127071e11c019c05303ccb5478227517a8d3855da657807bac0a40db05 bytes=202

// meta-language:prelude begin
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]
// meta-language:prelude end

// meta-language:translated JavaScript export_statement items=1 sha256=a0993bf6d194732bfa3701b377deaff87d9f226e774554f8a507d1d25d307be8
// | // The sibling module whose items imports.mjs reads through a relative import.
// | export const GREETING = 'hello';
pub const GREETING: &str = "hello";

// meta-language:translated JavaScript export_statement items=1 sha256=e13747d9ef84cac9f40dd6e798a359ae0f9bed7dbece1095764e4b507b5fde6d
// | /** @param {number} x @returns {number} */
// | export function double(x) {
// |   return x * 2;
// | }
pub fn double(x: f64) -> f64 {
    (x * 2f64)
}
