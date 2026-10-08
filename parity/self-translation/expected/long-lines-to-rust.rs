// meta-language:self-translation:v1 source=JavaScript target=Rust sha256=f65cb2f76aaa915baebbe8615da022a31b1fea1339760c922e370b5dc033b9d2 bytes=547

// meta-language:prelude begin
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]
// meta-language:prelude end

// meta-language:translated JavaScript export_statement items=1 sha256=537129fbd25052dae588569d279db5a8504082b9e69838861741a4e4ea775f51
// | // Expressions too long for one Rust line, which the layout breaks one item
// | // per line the way rustfmt would (issue #217).
// | export const KEYWORDS = ['abstract', 'arguments', 'await', 'boolean', 'break', 'byte', 'case', 'catch', 'char', 'class', 'const', 'continue'];
pub static KEYWORDS: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(
    || vec![
        String::from("abstract"),
        String::from("arguments"),
        String::from("await"),
        String::from("boolean"),
        String::from("break"),
        String::from("byte"),
        String::from("case"),
        String::from("catch"),
        String::from("char"),
        String::from("class"),
        String::from("const"),
        String::from("continue"),
    ]
);

// meta-language:translated JavaScript export_statement items=1 sha256=84cb1b833d26facfa573dcd036cf5eadc44a63fbe3a563828346ed043fae60ba
// | /** @param {number} first @param {number} second @param {number} third @param {number} fourth @returns {number} */
// | export function weightedSum(first, second, third, fourth) {
// |   return (first * 1000 + second * 100 + third * 10 + fourth) * (first + second + third + fourth + 1);
// | }
pub fn weighted_sum(first: f64, second: f64, third: f64, fourth: f64) -> f64 {
    (
        (
            (((first * 1000f64) + (second * 100f64)) + (third * 10f64)) + fourth
        ) * ((((first + second) + third) + fourth) + 1f64)
    )
}
