import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { selfTranslate } from '../src/self-translation.js';
import { nativeRustChecksEnabled } from './support/native-rust-checks.js';

const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/self-translation-bindings.json', import.meta.url), 'utf8'));
fixture.cases = fixture.cases.map((entry) => entry.module ? { ...entry, source: readFileSync(new URL(`../../${entry.module}`, import.meta.url), 'utf8') } : entry);

test('sibling bindings share a checked scope, execute faithfully and restore their source', () => {
  const directory = nativeRustChecksEnabled() ? mkdtempSync(path.join(tmpdir(), 'self-translation-bindings-')) : null;
  try {
    for (const entry of fixture.cases) {
      const translated = selfTranslate(entry.source, 'JavaScript', 'Rust');
      assert.ok(translated.items.every(({ status }) => status === 'translated'), entry.name);
      assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, entry.source, entry.name);
      for (const observation of [entry, ...(entry.additionalCalls ?? [])]) {
        const name = observation === entry ? entry.name : `${entry.name}-${observation.name}`;
        const expected = execFileSync(process.execPath, ['--input-type=module', '-e', `${entry.source}\nconsole.log(${observation.call});`], { encoding: 'utf8', timeout: 10_000 });
        assert.equal(expected, observation.expected, name);
        if (!directory) continue;
        const source = path.join(directory, `${name}.rs`);
        const binary = path.join(directory, process.platform === 'win32' ? `${name}.exe` : name);
        writeFileSync(source, `${translated.code}\nfn main() { println!("{}", ${observation.rustCall}); }\n`);
        execFileSync('rustc', ['--edition', '2024', source, '-o', binary], { stdio: 'pipe', timeout: 60_000 });
        const actual = execFileSync(binary, [], { encoding: 'utf8', timeout: 10_000 });
        assert.equal(actual, expected, name);
      }
    }
  } finally { if (directory) rmSync(directory, { recursive: true, force: true }); }
});

test('a carried sibling and its caller stay carried without poisoning an independent function', () => {
  const source = '/** @returns {number} */\nfunction good() { return 7; }\n\nfunction unavailable() { return new Date(); }\n\n/** @returns {number} */\nexport function caller() { return unavailable(); }\n';
  const translated = selfTranslate(source, 'JavaScript', 'Rust');
  const functions = translated.items.filter(({ term }) => term === 'function_declaration' || term === 'export_statement');
  assert.deepEqual(functions.map(({ status }) => status), ['translated', 'carried', 'carried']);
  assert.ok(!translated.code.includes('pub fn unavailable('));
  assert.ok(!translated.code.includes('pub fn caller('));
  assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, source);
});


test('mutable, nonliteral and temporal-dead-zone bindings stay unbound', () => {
  for (const entry of fixture.refusals) {
    const translated = selfTranslate(entry.source, 'JavaScript', 'Rust');
    const functions = translated.items.filter(({ term }) => term === 'function_declaration' || term === 'export_statement');
    assert.deepEqual(functions.map(({ status }) => status), ['carried'], entry.name);
    assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, entry.source, entry.name);
  }
});
