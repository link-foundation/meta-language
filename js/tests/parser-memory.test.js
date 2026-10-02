// Memory guards for the grammar-backed parser. Importing the package once
// compiled all 60 vendored grammars into the runtime's WebAssembly memory, and
// every parse built an object per source character to map string offsets to
// UTF-8 bytes and points, which together pushed whole-suite runs past a few
// gigabytes. Grammars now load on first use, and the offset index keeps one
// typed-array checkpoint per CHECKPOINT_INTERVAL characters.
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { CHECKPOINT_INTERVAL, sourceBoundaries } from '../src/source-boundaries.js';

const encoder = new TextEncoder();

// The per-character map the index replaces, as the reference.
function referenceBoundaries(text) {
  const result = new Map([[0, { byte: 0, row: 0, column: 0 }]]);
  let byte = 0;
  let row = 0;
  let column = 0;
  for (let offset = 0; offset < text.length;) {
    const character = String.fromCodePoint(text.codePointAt(offset));
    const byteLength = encoder.encode(character).length;
    byte += byteLength;
    if (character === '\n') {
      row += 1;
      column = 0;
    } else {
      column += byteLength;
    }
    offset += character.length;
    result.set(offset, { byte, row, column });
  }
  return result;
}

test('the source boundary index agrees with a per-character map at every offset and byte', () => {
  const unit = 'aé€😀\n\uD800b\r\n\t​';
  const text = Array.from({ length: 40 }, (_, index) => `${unit}${index}`).join('');
  const reference = referenceBoundaries(text);
  const boundaries = sourceBoundaries(text);
  for (let offset = -1; offset <= text.length + 1; offset += 1) {
    assert.deepEqual(boundaries.get(offset), reference.get(offset), `offset ${offset}`);
  }
  const offsets = new Map([...reference].map(([offset, { byte }]) => [byte, offset]));
  const bytes = encoder.encode(text).length;
  for (let byte = -1; byte <= bytes + 1; byte += 1) {
    assert.equal(boundaries.offsetOf(byte), offsets.get(byte), `byte ${byte}`);
  }
  assert.deepEqual(sourceBoundaries('').get(0), { byte: 0, row: 0, column: 0 });
  assert.equal(sourceBoundaries('').offsetOf(0), 0);
});

test('the source boundary index stores one checkpoint per interval, not one per character', () => {
  const text = 'x'.repeat(CHECKPOINT_INTERVAL * 1000 + 7);
  const boundaries = sourceBoundaries(text);
  assert.equal(boundaries.checkpointCount, 1000 + 2);
  assert.deepEqual(boundaries.get(text.length), { byte: text.length, row: 0, column: text.length });
});

test('grammars load on first use, not when the package is imported', () => {
  const parser = new URL('../src/programming-language-parser.js', import.meta.url).href;
  const index = new URL('../src/index.js', import.meta.url).href;
  const script = `
    const { loadedGrammarIds } = await import(${JSON.stringify(parser)});
    const { LinkNetwork } = await import(${JSON.stringify(index)});
    const imported = loadedGrammarIds();
    LinkNetwork.parse('# Title\\n\\nSome *text*.\\n', 'Markdown');
    LinkNetwork.parse('fn main() {}\\n', 'Rust');
    LinkNetwork.parse('fn other() {}\\n', 'Rust');
    console.log(JSON.stringify({ imported, parsed: loadedGrammarIds() }));
  `;
  const output = execFileSync(process.execPath, ['--input-type=module', '-e', script], {
    cwd: fileURLToPath(new URL('..', import.meta.url)),
    encoding: 'utf8',
  });
  assert.deepEqual(JSON.parse(output), { imported: [], parsed: ['markdown', 'markdown_inline', 'rust'] });
});

// PowerShell's generated lexer is one function of about 360 KB. Tiered up by
// TurboFan in the background, it grew the process by about a gigabyte after
// the parses had returned.
test('grammar code is not tiered up after parsing ends', () => {
  const index = new URL('../src/index.js', import.meta.url).href;
  const inventory = new URL('../../parity/language-grammar-inventory.json', import.meta.url);
  const script = `
    const { readFileSync } = await import('node:fs');
    const { LinkNetwork } = await import(${JSON.stringify(index)});
    const { languages } = JSON.parse(readFileSync(new URL(${JSON.stringify(inventory.href)}), 'utf8'));
    const { source } = languages.find(({ name }) => name === 'PowerShell');
    for (let round = 0; round < 20; round += 1) LinkNetwork.parse(source, 'PowerShell');
    const parsed = process.memoryUsage().rss;
    await new Promise((resolve) => setTimeout(resolve, 1500));
    console.log(JSON.stringify({ parsed, idle: process.memoryUsage().rss }));
  `;
  const { parsed, idle } = JSON.parse(
    execFileSync(process.execPath, ['--input-type=module', '-e', script], { encoding: 'utf8' }),
  );
  assert.ok(idle - parsed < 100 * 1024 * 1024, `grew ${Math.round((idle - parsed) / 1048576)} MB while idle`);
});
