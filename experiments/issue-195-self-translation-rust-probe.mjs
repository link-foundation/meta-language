// Probes selfTranslate Rust → JavaScript → Rust and compiles the Rust.
import { execFileSync } from 'node:child_process';
import { mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { selfTranslate } from '../js/src/self-translation.js';
const rs = `//! Small arithmetic.
use std::fmt;

/// Adds two numbers.
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

fn is_even(n: u32) -> bool {
    n % 2 == 0
}

pub struct Point {
    x: i32,
}
`;
const js = selfTranslate(rs, 'Rust', 'JavaScript');
console.log(js.code);
console.log(js.items.map((i) => `${i.term}:${i.status}:${i.reason}`).join('\n'));
const back = selfTranslate(js.code, 'JavaScript', 'Rust');
console.log('round trip identical', back.code === rs);
const fromJs = selfTranslate(`/** @param {number} a @returns {number} */\nexport function twice(a) { return a * 2; }\n/** @param {bigint} n @returns {bigint} */\nexport function sq(n) { return n * n; }\n`, 'js', 'rs');
console.log(fromJs.code);
const dir = mkdtempSync(path.join(tmpdir(), 'self-'));
writeFileSync(path.join(dir, 'lib.rs'), fromJs.code);
try {
  execFileSync('rustc', ['--edition', '2021', '--crate-type', 'lib', '-o', path.join(dir, 'lib.rlib'), path.join(dir, 'lib.rs')], { stdio: 'inherit' });
  console.log('rustc ok');
} catch { console.log('rustc failed'); }
