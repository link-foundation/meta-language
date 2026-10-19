import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { runInNewContext } from 'node:vm';
import { translateProgram } from '../src/program-translation.js';
import { parseRust } from '../src/translation/rust.js';
import rustCases from './fixtures/translation-cases/frontend-rust.mjs';

const main = 'fn main() { println!("x"); }';
const vectorSource = 'fn f(v: Vec<u64>) -> u64 { 0 } ' + main;
const u64 = { kind: 'fixed', bits: 64, signed: false };
const surface = source => JSON.parse(JSON.stringify(parseRust(source), (key, value) => key === 'span' ? undefined : value));

test('unfinished Rust visibility reports exact EOF diagnostics instead of stalling the frontend', () => {
  const cases = [main + '\npub(crate', main + '\npub(', main + '\npub(in crate::m', 'fn main() { println!("😀"); }\npub(crate'];
  const worker = `
    import {readFileSync} from 'node:fs';
    import {parseRust} from ${JSON.stringify(new URL('../src/translation/rust.js', import.meta.url).href)};
    const results = JSON.parse(readFileSync(0, 'utf8')).map(source => {
      try { parseRust(source); return { admitted: true }; }
      catch(error) { return { kind: error.kind, message: error.message, span: error.span }; }
    });
    process.stdout.write(JSON.stringify(results));
  `;
  const output = spawnSync(process.execPath, ['--max-old-space-size=384', '--input-type=module', '-e', worker], {
    input: JSON.stringify(cases), encoding: 'utf8', timeout: 5000, maxBuffer: 64 * 1024,
  });
  assert.equal(output.error, undefined, 'the bounded parser process terminates');
  assert.equal(output.status, 0, output.stderr);
  const results = JSON.parse(output.stdout);
  assert.equal(results.length, cases.length);
  for (const [index, source] of cases.entries()) {
    const expected = { kind: 'syntax', message: `expected ) in visibility but found end of input at ${source.length}..${source.length}`, span: { start: source.length, end: source.length } };
    assert.deepEqual(results[index], expected);
    assert.deepEqual(translateProgram(source, 'Rust', 'JavaScript').diagnostic, expected);
  }
});

test('the complete native Rust frontend rejection matrix keeps its current JavaScript diagnostics', () => {
  const native = readFileSync(new URL('../../rust/tests/unit/translation_rust_frontend.rs', import.meta.url), 'utf8');
  const body = native.split('fn rejected_programs_raise_the_javascript_errors() {')[1].split('\n#[test]')[0];
  const quoted = '"(?:[^"\\\\]|\\\\.)*"';
  const rows = [...body.matchAll(new RegExp(`\\(\\s*(${quoted}),\\s*ErrorKind::([A-Za-z]+),\\s*(${quoted}),\\s*\\)`, 'g'))];
  assert.equal(rows.length, 12);
  assert.ok(rows.some(row => JSON.parse(row[1]).includes('List<u64>')));
  assert.ok(rows.some(row => JSON.parse(row[1]) === main + '\npub(crate'));
  for (const row of rows) {
    const source = JSON.parse(row[1]);
    assert.throws(() => parseRust(source), error => {
      assert.equal(error.kind, row[2].toLowerCase(), source);
      assert.equal(error.message, JSON.parse(row[3]), source);
      const match = / at ([0-9]+)\.\.([0-9]+)$/u.exec(error.message);
      assert.deepEqual(error.span, { start: Number(match[1]), end: Number(match[2]) });
      return true;
    });
  }
});

test('the exact original Vec source retains its supported scalar array surface and executable result', () => {
  assert.equal(vectorSource, rustCases['frontend-rust-negativeVec'][1]);
  assert.equal(createHash('sha256').update(vectorSource).digest('hex'), 'f0e7dc655f6f6e21afa3ccbd2a7e4e196c9151cdf1125cc83b0dc31a0c9f2347');
  assert.deepEqual(surface(vectorSource), {
    language: 'Rust',
    items: [{ k: 'fn', name: 'f', params: [{ name: 'v', type: { kind: 'array', element: u64 } }], ret: u64, body: { k: 'num', value: '0' } }],
    main: { effects: [{ k: 'print', expr: { k: 'str', value: 'x' }, style: 'rust' }] },
  });
  const result = translateProgram(vectorSource, 'Rust', 'JavaScript');
  assert.equal(result.diagnostic, null);
  const lines = [];
  const context = { console: { log: value => lines.push(value) } };
  runInNewContext(result.code, context, { timeout: 1000 });
  assert.equal(context.f([7n]), 0n);
  assert.deepEqual(lines, ['x']);
});

test('closed Rust visibility forms preserve the same complete surface program', () => {
  const source = visibility => `mod m { ${visibility} fn f(n: u64) -> u64 { n } }\n${main}`;
  const expected = surface(source(''));
  for (const visibility of ['pub', 'pub(crate)', 'pub(self)', 'pub(super)', 'pub(in crate::m)']) {
    assert.deepEqual(surface(source(visibility)), expected, visibility);
  }
});
