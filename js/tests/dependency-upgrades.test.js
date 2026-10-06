import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { readFile } from 'node:fs/promises';
import test from 'node:test';
import { deflateSync, inflateSync } from 'node:zlib';

import peggy from 'peggy';
import { PDFDocument } from 'pdf-lib';
import * as pako from 'pako';

const compression = pako.default ?? pako;

const inventory = JSON.parse(await readFile(
  new URL('../../parity/dependency-inventory.json', import.meta.url), 'utf8'));
const lock = JSON.parse(await readFile(new URL('../package-lock.json', import.meta.url), 'utf8'));

test('the repository npm lockfile uses each audited current stable release', () => {
  for (const item of inventory.items.filter(({ category }) => category === 'npm')) {
    assert.equal(item.pinned, item.current, item.id);
    assert.equal(lock.packages[`node_modules/${item.name}`]?.version, item.current, item.id);
  }
});

test('updated Peggy dependencies generate executable parsers, source maps and CLI output', () => {
  const grammar = 'start = digits:[0-9]+ { return Number(digits.join("")); }';
  const parser = peggy.generate(grammar);
  assert.equal(parser.parse('195'), 195);
  assert.throws(() => parser.parse('195!'));
  const generated = peggy.generate(grammar, {
    output: 'source-and-map', format: 'es', grammarSource: 'dependency-probe.pegjs',
  }).toStringWithSourceMap({ file: 'dependency-probe.js' });
  assert.match(generated.code, /function peg\$parse/);
  const map = JSON.parse(generated.map.toString());
  assert.ok(map.sources.includes('dependency-probe.pegjs'));
  assert.ok(map.mappings.length > 0);
  const version = execFileSync(process.execPath, ['node_modules/peggy/bin/peggy.js', '--version'], {
    cwd: new URL('..', import.meta.url), encoding: 'utf8',
  }).trim();
  assert.equal(version, lock.packages['node_modules/peggy'].version);
});

test('updated compression and TypeScript helpers preserve PDF creation and independent zlib interoperability', async () => {
  const source = Buffer.from('Unicode: λ 😀\n'.repeat(16));
  assert.deepEqual(Buffer.from(compression.inflate(deflateSync(source))), source);
  assert.deepEqual(inflateSync(compression.deflate(source)), source);
  const document = await PDFDocument.create();
  document.addPage().drawText('dependency upgrade');
  const bytes = await document.save();
  const loaded = await PDFDocument.load(bytes);
  assert.equal(loaded.getPageCount(), 1);
  assert.ok(bytes.length > 0);
});
