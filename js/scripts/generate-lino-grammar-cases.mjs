#!/usr/bin/env node
// Generates parity/fixtures/lino-grammar-cases.json: LiNo sources with the
// links the official links-notation parser (the reference implementation of
// links-notation/src/grammar.pegjs) returns for them, or null where it rejects
// the source. Both runtimes check their built-in LiNo grammar CST against it.
//
//   node js/scripts/generate-lino-grammar-cases.mjs          # write
//   node js/scripts/generate-lino-grammar-cases.mjs --check  # verify
//
// A link is {id, values, children} with id null when the link is unnamed; a
// reference is a link with an id and no values or children.
import { readFile, writeFile } from 'node:fs/promises';
import { createRequire } from 'node:module';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { parse } from 'links-notation/src/parser-generated.js';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const outputPath = join(root, 'parity/fixtures/lino-grammar-cases.json');
const { version } = createRequire(import.meta.url)('links-notation/package.json');

const HAND_WRITTEN = [
  '', '\n\n', '1 1 1\n', '(papa (lovesMama: loves mama))\n(son lovesMama)\n', 'a\n\nb\n',
  'greeting:\n  hello world\n  (x y)\na b c\n', '  a\n  b\n', '(a\n b)\n', '"quoted ref" \'x\' `y`\n',
  'a: b c\n', '(broken\n', ')\n', 'a:\n', 'x\n  y\n    z\n  w\n', 'a\r\nb\r\n', '(a: b\n)\n', 'a b \n',
  '\t(a)\n', '(a)(b)\n', 'a:b\n', '"""x"""\n', '(a: (b: c) d)\n', 'a\n b\nc\n', 'a  \n\n  \n', '( )\n',
  '()\n', '"a""b"\n', '""\n', '"unclosed\n', 'x\n  (broken\n  y\n', '(obj_0: list obj_0)\n',
  'Hawaii: (state (of USA))\n', 'π: 3.14 ∞\n', 'a\n  b\n c\n', '(a b\n', 'a (b\n',
];

// A fixed linear congruential generator keeps the generated cases stable.
const ALPHABET = ['a', 'b', 'c', ' ', ' ', ' ', '\t', '\n', '\n', '\r', '(', ')', ':', '"', "'", '`', 'é'];
function generated(count) {
  let seed = 195;
  const random = () => ((seed = (seed * 1103515245 + 12345) % 2147483648) / 2147483648);
  const sources = [];
  for (let index = 0; index < count; index += 1) {
    const length = 1 + Math.floor(random() * 20);
    let source = '';
    for (let offset = 0; offset < length; offset += 1) {
      source += ALPHABET[Math.floor(random() * ALPHABET.length)];
    }
    sources.push(source);
  }
  return sources;
}

const normalize = (link) => ({
  id: link.id ?? null,
  values: (link.values ?? []).map(normalize),
  children: (link.children ?? []).map(normalize),
});

function official(source) {
  try {
    return parse(source).map(normalize);
  } catch {
    return null;
  }
}

// The conformance sources of the language inventory are always cases.
const inventory = JSON.parse(await readFile(join(root, 'parity/language-grammar-inventory.json'), 'utf8'));
const { source: inventorySource, recoverySource } = inventory.languages.find(({ name }) => name === 'LiNo');
const sources = [...new Set([inventorySource, recoverySource, ...HAND_WRITTEN, ...generated(600)])];
const expected = `${JSON.stringify({
  schemaVersion: 1,
  oracle: `links-notation@${version} src/parser-generated.js parse()`,
  cases: sources.map((source) => ({ source, links: official(source) })),
}, null, 1)}\n`;

if (process.argv.includes('--check')) {
  const committed = await readFile(outputPath, 'utf8');
  if (committed !== expected) {
    console.error(`${outputPath} is stale; run node js/scripts/generate-lino-grammar-cases.mjs`);
    process.exit(1);
  }
  console.log(`lino grammar cases: ${sources.length} cases match the official parser`);
} else {
  await writeFile(outputPath, expected);
  console.log(`wrote ${sources.length} cases to ${outputPath}`);
}
