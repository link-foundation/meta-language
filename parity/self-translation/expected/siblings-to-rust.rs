// meta-language:self-translation:v1 source=JavaScript target=Rust sha256=bee605380143ca101cd61417c8711a548cfa34a40ab97b2e747c4e20485193c0 bytes=912

// meta-language:prelude begin
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]
// meta-language:prelude end

// Items that call the other top-level items of their module.

// meta-language:translated JavaScript function_declaration items=1 sha256=ae35fae354953fa836141bcaeaea7546e1171261e4de59e852319cda8daf205c
// | /** @param {number} x @returns {number} */
// | function square(x) {
// |   return x * x;
// | }
pub fn square(x: f64) -> f64 {
    (x * x)
}

// meta-language:translated JavaScript export_statement items=1 sha256=5c020b94957026dbebff30d8399e0844882c4f616b068f59d75d0911bfa09eed
// | /** @param {number} x @param {number} limit @returns {number} */
// | export function cappedSquare(x, limit) {
// |   const squared = square(x);
// |   return squared > limit ? limit : squared;
// | }
pub fn capped_square(x: f64, limit: f64) -> f64 {
    {
        let squared = square(x);
        if (squared > limit) {
            limit
        } else {
            squared
        }
    }
}

// meta-language:translated JavaScript export_statement items=1 sha256=4e4304e43a1008b16ac170d71666b579b844e0b080d255e3b41bbac7a7efbb2a
// | /** @param {number} x @returns {number} */
// | export function cappedTwice(x) {
// |   return halve(cappedSquare(x, 3)) * 4;
// | }
pub fn capped_twice(x: f64) -> f64 {
    (halve(capped_square(x, 3f64)) * 4f64)
}

// meta-language:translated JavaScript lexical_declaration items=1 sha256=4dee13e039b359d5b90be59212153e21b296c7d3055481d5da863f7526fd445f
// | // Declared after its caller, as JavaScript hoists a function declaration.
// | const halve = (x) => x / 2;
pub fn halve(x: f64) -> f64 {
    (x / 2f64)
}

// meta-language:translated JavaScript export_statement items=1 sha256=885f66ac0132cafdb1a73e5796c7237df664813e3246917c3c3d4cddf73730a4
// | /** @param {string} name @returns {boolean} */
// | export function greets(name) {
// |   return name === greeting();
// | }
pub fn greets(name: String) -> bool {
    (name == (greeting()))
}

// meta-language:translated JavaScript function_declaration items=1 sha256=79396675e1153410cd066700ea48e5d978fbedc32cbbb453e1f617b9c06bc2ed
// | function greeting() {
// |   return 'hello';
// | }
pub fn greeting() -> String {
    String::from("hello")
}

// meta-language:carried JavaScript export_statement (type)
// | /** @param {number} x @returns {number} */
// | export function boxed(x) {
// |   return unbox(x);
// | }

// meta-language:carried JavaScript function_declaration (unsupported)
// | function unbox(x) {
// |   return new Box(x).value;
// | }

// meta-language:carried JavaScript class_declaration (unsupported)
// | class Box {
// |   constructor(value) {
// |     this.value = value;
// |   }
// | }
