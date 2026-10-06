// meta-language:self-translation:v1 source=JavaScript target=Rust sha256=b8c4fc2851f7049854fe831a477ccf4543d46f5e9112fc0b4fefde506a8b65b2 bytes=508

// meta-language:prelude begin
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]
// meta-language:prelude end

// meta-language:carried JavaScript import_statement (no definition)
// | // Arithmetic the portable core translates, beside items it carries.
// | import { strict as assert } from 'node:assert';

// meta-language:translated JavaScript export_statement items=1 sha256=adca809dcf16c4ab258d1602d17f45ee2f96918eb30071d97c19a43d857611fc
// | /**
// |  * The sum of `a` and `b`.
// |  * @param {number} a
// |  * @param {number} b
// |  * @returns {number}
// |  */
// | export function add(a, b) {
// |   return a + b;
// | }
pub fn add(a: f64, b: f64) -> f64 {
    (a + b)
}

// meta-language:translated JavaScript export_statement items=1 sha256=8f2bb35a2a2769aa367d29dd6f2f6165cf1c116f613b92a4b33ba43f78ee1868
// | /**
// |  * Whether `name` names Rust.
// |  * @param {string} name
// |  * @returns {boolean}
// |  */
// | export function isRust(name) {
// |   return name === 'Rust';
// | }
pub fn is_rust(name: String) -> bool {
    name == "Rust" || name == "rs"
}

// meta-language:carried JavaScript export_statement (unsupported)
// | export class Box {
// |   constructor(value) {
// |     this.value = value;
// |   }
// | }

// meta-language:carried JavaScript expression_statement (type)
// | assert.equal(add(1, 2), 3);
