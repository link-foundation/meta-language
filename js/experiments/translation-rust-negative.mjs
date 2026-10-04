// Rust constructs outside the portable core must raise precise obligations.
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';

const main = 'fn main() { println!("x"); }';
const cases = {
  loop: `fn f(n: u64) -> u64 { let mut s = 0; s } ${main}`,
  whileLoop: `fn f(n: u64) -> u64 { while n > 0 { } n } ${main}`,
  vec: `fn f(v: Vec<u64>) -> u64 { 0 } ${main}`,
  closure: `fn f(n: u64) -> u64 { (|x: u64| x)(n) } ${main}`,
  method: `struct S; ${main}`,
  guard: `fn f(n: u64) -> u64 { match n { k if k > 2 => 1, _ => 0 } } ${main}`,
  debugFormat: `fn main() { println!("{:?}", 1u64); }`,
  stdUse: `use std::collections::HashMap; ${main}`,
  bitwise: `fn f(n: u64) -> u64 { n & 1 } ${main}`,
  shift: `fn f(n: u64) -> u64 { n >> 1 } ${main}`,
  earlyReturn: `fn f(n: u64) -> u64 { return n; } ${main}`,
  narrowing: `fn f(n: u64) -> u32 { n as u32 } ${main}`,
  float: `fn f(n: f64) -> f64 { n } ${main}`,
  genericFn: `fn f<T>(n: T) -> T { n } ${main}`,
  ifLet: `fn f(n: u64) -> u64 { if let 0 = n { 1 } else { 0 } } ${main}`,
  noMain: 'fn f(n: u64) -> u64 { n }',
};
for (const [name, source] of Object.entries(cases)) {
  try {
    checkProgram(parseRust(source));
    console.log(`${name}: ACCEPTED`);
  } catch (error) {
    console.log(`${name}: ${error.kind ?? 'CRASH'} ${error.message}`);
  }
}
