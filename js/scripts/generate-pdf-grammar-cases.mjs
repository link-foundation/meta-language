#!/usr/bin/env node
// Generates parity/fixtures/pdf-grammar-cases.json: PDF sources with the
// indirect objects pdf-lib's PDFParser (strict about invalid objects) reads
// from them, or null where it rejects the source. Both runtimes check their
// built-in PDF grammar CST against it: a clean CST must describe the same
// objects, and a source pdf-lib rejects must parse with errors.
//
//   node js/scripts/generate-pdf-grammar-cases.mjs          # write
//   node js/scripts/generate-pdf-grammar-cases.mjs --check  # verify
//
// An object is {ref: [number, generation], value}; values project as null,
// booleans, {number}, {name} (with #XX escapes decoded), {string} and {hex}
// (the text between the delimiters), {ref}, arrays, {dictionary: [[key,
// value]...]} and {stream: [[key, value]...], data}. Text is UTF-8.
import { readFile, writeFile } from 'node:fs/promises';
import { createRequire } from 'node:module';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  PDFArray, PDFBool, PDFDict, PDFHexString, PDFName, PDFNull, PDFNumber, PDFParser, PDFRawStream,
  PDFRef, PDFString,
} from 'pdf-lib';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const outputPath = join(root, 'parity/fixtures/pdf-grammar-cases.json');
const { version } = createRequire(import.meta.url)('pdf-lib/package.json');

const MINIMAL = '%PDF-1.7\n%%EOF\n';

// A stream whose /Length is the UTF-8 byte length of its data.
const stream = (entries, data, eol = '\n') =>
  `<< ${entries} /Length ${Buffer.byteLength(data)} >>\nstream${eol}${data}${eol}endstream`;

const HELLO = [
  '%PDF-1.4\n%âãÏÓ\n',
  '1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n',
  '2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n',
  '3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792]\n',
  '   /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>\nendobj\n',
  '4 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>\nendobj\n',
  `5 0 obj\n${stream('', 'BT /F1 24 Tf 72 712 Td (Hello, world!) Tj ET')}\nendobj\n`,
  'xref\n0 6\n0000000000 65535 f \n0000000015 00000 n \n0000000064 00000 n \n0000000121 00000 n \n',
  '0000000247 00000 n \n0000000317 00000 n \n',
  'trailer\n<< /Size 6 /Root 1 0 R /Info << /Title (Hello \\(world\\)) /Producer <FEFF0041> >> >>\n',
  'startxref\n410\n%%EOF\n',
].join('');

const HAND_WRITTEN = [
  MINIMAL,
  HELLO,
  // An incremental update redefines object 4 and appends a second trailer.
  `${HELLO}4 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Courier >>\nendobj\n`
    + 'xref\n4 1\n0000000600 00000 n \ntrailer\n<< /Size 6 /Root 1 0 R /Prev 410 >>\nstartxref\n680\n%%EOF\n',
  // Every object type, comments and the compact forms the delimiters allow.
  '%PDF-2.0\r\n% a comment\r\n7 0 obj % after obj\r\n[1 -2 +3 4. -.5 0.25 true false null /Name /A#20B /#2F'
    + ' (nested (paren) \\) \\\\ string) <48 65 6c> <> [] <<>> (é) /é 8 0 R]\r\nendobj\r\n'
    + '8 1 obj<</Kids[9 0 R 10 0 R]/Count 2/Flag true>>endobj\r\n%%EOF\r\n',
  // A form XObject content stream with an inline image and every operand type.
  `%PDF-1.7\n9 0 obj\n${stream('/Type /XObject /Subtype /Form /BBox [0 0 10 10]',
    'q 1 0 0 1 0 0 cm /GS1 gs [1 2] 0 d (text) Tj <4142> Tj\nBI /W 1 /H 1 /BPC 8 /CS /G ID x EI\nQ')}\n`
    + 'endobj\n%%EOF\n',
  // Filtered data stays opaque; a /Length reference finds the data through
  // `endstream`.
  `%PDF-1.7\n1 0 obj\n${stream('/Filter /ASCIIHexDecode', '48656C6C6F3E')}\nendobj\n`
    + '2 0 obj\n<< /Length 3 0 R >>\nstream\r\n0 0 m 10 10 l S\r\nendstream\nendobj\n3 0 obj\n15\nendobj\n%%EOF\n',
  // Stream data that is not a content stream despite having no filter.
  `%PDF-1.7\n1 0 obj\n${stream('/Type /Metadata /Subtype /XML', '<x:xmpmeta></x:xmpmeta>', '\r\n')}\nendobj\n%%EOF\n`,
  // Malformed sources.
  '',
  'hello\n',
  '%PDF-1.7\n1 0 obj\n<< /A 1 /B >>\nendobj\n%%EOF\n',
  '%PDF-1.7\n1 0 obj\n<< /A [1 2 ) 3\n2 0 obj\n(unterminated\n%%EOF\n',
  '%PDF-1.7\n1 0 obj << /Length 5 >> stream\nab',
  '%PDF-1.7\n1 0 obj endobj\n%%EOF\n',
  '%PDF-1.7\n1 0 obj\n<< /A 1\nendobj\n%%EOF\n',
  '%PDF-1.7\n1 0 obj\n[1 2\nendobj\n%%EOF\n',
  '%PDF-1.7\n1 0 obj\n42\n%%EOF\n',
  '%PDF-1.7\n1 0 obj 1 2 3 endobj garbage { } trailer startxref %%EOF\n',
  '%PDF-1.7\n1 0 obj\n<< 1 /A >>\nendobj\n%%EOF\n',
  '%PDF-1.7\n1 0 obj\n<zz>\nendobj\n%%EOF\n',
  '%PDF-1.7\n1 0 obj\n(a\\)\nendobj\n%%EOF\n',
  '%PDF-1.7\nxref\n0 1\n0000000000 65535 f \ntrailer\n%%EOF\n',
  '%PDF-1.7\n1 0 obj\n<< /Length 4 >>\nstream\nabcd\nendobj\n%%EOF\n',
  '1 0 obj\n1\nendobj\n%%EOF\n',
];

// A fixed linear congruential generator keeps the generated cases stable.
let seed = 195;
const random = () => ((seed = (seed * 1103515245 + 12345) % 2147483648) / 2147483648);
const pick = (items) => items[Math.floor(random() * items.length)];
const between = (low, high) => low + Math.floor(random() * (high - low + 1));

const NAMES = ['/Type', '/Kids', '/A', '/Size', '/F1', '/Long#20Name', '/é', '/Count', '/Parent', '/x'];
const OPERATORS = ['q', 'Q', 'cm', 'm', 'l', 'h', 'S', 'f', 'BT', 'ET', 'Tf', 'Td', 'Tj', "'", 'gs', 're', 'W', 'n'];

function scalar() {
  return pick([
    () => String(between(-50, 500)),
    () => `${between(-9, 9)}.${between(0, 99)}`,
    () => pick(['true', 'false', 'null']),
    () => pick(NAMES),
    () => `(${pick(['', 'text', 'a (nested) b', 'esc \\( \\n \\\\', 'ünï', 'line\nbreak'])})`,
    () => `<${pick(['', '48656C6C6F', 'ab cd', 'FEFF00E9'])}>`,
    () => `${between(1, 9)} 0 R`,
  ])();
}

function value(depth) {
  const kind = depth > 2 ? 0 : between(0, 3);
  if (kind === 1) {
    return `[${Array.from({ length: between(0, 4) }, () => value(depth + 1)).join(pick([' ', '\n', '']))}]`
      .replace(/\[\s+\]/u, '[]');
  }
  if (kind === 2) return dictionary(depth, between(0, 3));
  return scalar();
}

function dictionary(depth, size, entries = []) {
  const parts = entries.slice();
  for (let index = 0; index < size; index += 1) parts.push(`${pick(NAMES)} ${value(depth + 1)}`);
  return `<<${pick([' ', ''])}${parts.join(pick([' ', '\n']))}${pick([' ', ''])}>>`;
}

function contentData() {
  const operations = [];
  for (let index = between(0, 5); index > 0; index -= 1) {
    const operands = Array.from({ length: between(0, 3) }, () => pick([
      String(between(0, 99)), `${between(0, 9)}.5`, pick(NAMES), '(t)', '<41>', '[1 (x) 2]',
    ]));
    operations.push([...operands, pick(OPERATORS)].join(' '));
  }
  return operations.join(pick(['\n', ' ']));
}

function streamObject() {
  const content = random() < 0.6;
  let data = content ? contentData() : pick(['binary\u0001data', 'free text: 1 2 3', 'Ωmega', '']);
  if (data.includes('stream')) data = '';
  const byteLength = Buffer.byteLength(data);
  const entries = [];
  const lengthKind = between(0, 3);
  if (lengthKind <= 1) entries.push(`/Length ${byteLength}`);
  if (lengthKind === 2) entries.push(`/Length ${byteLength + between(1, 3)}`);
  if (content && random() < 0.5) entries.push('/Type /XObject /Subtype /Form /BBox [0 0 1 1]');
  if (!content && random() < 0.5) entries.push('/Filter /ASCIIHexDecode');
  const eol = pick(['\n', '\r\n']);
  return `${dictionary(1, 0, entries)}\nstream${eol}${data}${eol}endstream`;
}

function generatedFile() {
  const parts = [`%PDF-1.${between(0, 7)}\n`];
  const count = between(1, 5);
  for (let number = 1; number <= count; number += 1) {
    const body = random() < 0.3 ? streamObject() : value(0);
    parts.push(`${number} 0 obj${pick(['\n', ' '])}${body}${pick(['\n', ' '])}endobj\n`);
  }
  if (random() < 0.7) {
    const entries = Array.from({ length: count }, (_, index) => `${String(9 * (index + 1)).padStart(10, '0')} 00000 n \n`);
    parts.push(`xref\n0 ${count + 1}\n0000000000 65535 f \n${entries.join('')}`);
    parts.push(`trailer\n<< /Size ${count + 1} /Root 1 0 R >>\n`);
    parts.push(`startxref\n${between(9, 999)}\n`);
  }
  parts.push('%%EOF\n');
  return parts.join('');
}

const MUTATIONS = ['(', ')', '<<', '>>', '[', ']', 'endobj', ' obj ', '<', '/', '%', 'x', '{', '1 0 R', '\n'];

function mutated(source) {
  const characters = Array.from(source);
  const at = between(0, characters.length);
  if (random() < 0.5) {
    characters.splice(at, between(1, 6));
  } else {
    characters.splice(at, 0, pick(MUTATIONS));
  }
  return characters.join('');
}

function generated(count) {
  const sources = [];
  for (let index = 0; index < count; index += 1) {
    const source = generatedFile();
    sources.push(index % 2 === 0 ? source : mutated(source));
  }
  return sources;
}

const utf8 = (latin1) => Buffer.from(latin1, 'latin1').toString('utf8');

function project(object) {
  if (object === PDFNull) return null;
  if (object instanceof PDFBool) return object.asBoolean();
  if (object instanceof PDFNumber) return { number: object.asNumber() };
  if (object instanceof PDFName) return { name: utf8(object.decodeText()) };
  if (object instanceof PDFString) return { string: utf8(object.asString()) };
  if (object instanceof PDFHexString) return { hex: utf8(object.asString()) };
  if (object instanceof PDFRef) return { ref: [object.objectNumber, object.generationNumber] };
  if (object instanceof PDFArray) return object.asArray().map(project);
  if (object instanceof PDFRawStream) {
    return { stream: projectEntries(object.dict), data: Buffer.from(object.contents).toString('utf8') };
  }
  if (object instanceof PDFDict) return { dictionary: projectEntries(object) };
  throw new Error(`unexpected pdf-lib object ${object.constructor.name}`);
}

function projectEntries(dictionary) {
  return dictionary.entries().map(([key, entry]) => [utf8(key.decodeText()), project(entry)]);
}

async function oracle(source) {
  const warn = console.warn;
  console.warn = () => {};
  try {
    const parser = PDFParser.forBytesWithOptions(Buffer.from(source, 'utf8'), Infinity, true);
    const context = await parser.parseDocument();
    return context.enumerateIndirectObjects().map(([ref, object]) => ({
      ref: [ref.objectNumber, ref.generationNumber],
      value: project(object),
    }));
  } catch {
    return null;
  } finally {
    console.warn = warn;
  }
}

// The conformance sources of the language inventory are always cases.
const inventory = JSON.parse(await readFile(join(root, 'parity/language-grammar-inventory.json'), 'utf8'));
const { source: inventorySource, recoverySource } = inventory.languages.find(({ name }) => name === 'PDF');
const sources = [...new Set([inventorySource, recoverySource, ...HAND_WRITTEN, ...generated(400)])];
const cases = [];
for (const source of sources) cases.push({ source, objects: await oracle(source) });
const expected = `${JSON.stringify({
  schemaVersion: 1,
  oracle: `pdf-lib@${version} PDFParser.forBytesWithOptions(bytes, Infinity, true).parseDocument()`,
  cases,
}, null, 1)}\n`;

if (process.argv.includes('--check')) {
  const committed = await readFile(outputPath, 'utf8');
  if (committed !== expected) {
    console.error(`${outputPath} is stale; run node js/scripts/generate-pdf-grammar-cases.mjs`);
    process.exit(1);
  }
  console.log(`pdf grammar cases: ${sources.length} cases match pdf-lib`);
} else {
  await writeFile(outputPath, expected);
  console.log(`wrote ${sources.length} cases to ${outputPath}`);
}
