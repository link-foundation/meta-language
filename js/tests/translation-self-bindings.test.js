import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { selfTranslate } from '../src/self-translation.js';

const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/self-translation-bindings.json', import.meta.url), 'utf8'));

test('sibling bindings share a checked scope, execute faithfully and restore their source', () => {
  const directory = mkdtempSync(path.join(tmpdir(), 'self-translation-bindings-'));
  try {
    for (const entry of fixture.cases) {
      const translated = selfTranslate(entry.source, 'JavaScript', 'Rust');
      assert.ok(translated.items.every(({ status }) => status === 'translated'), entry.name);
      assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, entry.source, entry.name);
      const source = path.join(directory, `${entry.name}.rs`);
      const binary = path.join(directory, entry.name);
      writeFileSync(source, `${translated.code}\nfn main() { println!("{}", ${entry.rustCall}); }\n`);
      execFileSync('rustc', ['--edition', '2024', source, '-o', binary], { stdio: 'pipe' });
      const actual = execFileSync(binary, [], { encoding: 'utf8' });
      const expected = execFileSync(process.execPath, ['--input-type=module', '-e', `${entry.source}\nconsole.log(${entry.call});`], { encoding: 'utf8' });
      assert.equal(expected, entry.expected, entry.name);
      assert.equal(actual, expected, entry.name);
    }
  } finally { rmSync(directory, { recursive: true, force: true }); }
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
