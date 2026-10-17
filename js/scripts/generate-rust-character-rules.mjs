// Every dependency declaration is retained. No generated Rust is authored here.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { tokenize } from '../src/translation/lexer.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';
import { readSourceTextRustBlock } from './source-text-byte-library-style.mjs';

const root = new URL('../../', import.meta.url);
const digest = source => createHash('sha256').update(source).digest('hex');
const sourcePath = 'js/src/translation/rust-character-rules.js';
const sourceEntries = [
  ['js/src/translation/source-text-byte-rules.js', []],
  ['js/src/translation/rust-cooked-string-rules.js', []],
  ['js/src/translation/rust-unicode-escape-rules.js', ["import {readRustAsciiEscapeDigit} from './rust-cooked-string-rules.js';\n"]],
  ['js/src/translation/rust-source-header-emission-rules.js', []],
  ['js/src/translation/rust-number-rules.js', []],
  ['js/src/translation/rust-source-whitespace-rules.js', ["import { acceptRustNumberDigitBoundary } from './rust-number-rules.js';\n"]],
  [sourcePath, [
    "import { readSourceTextScalar } from './source-text-byte-rules.js';\n",
    "import { acceptRustCookedStringEscapePrefix, readRustAsciiEscapeDigit, readRustAsciiEscapeValue } from './rust-cooked-string-rules.js';\n",
    "import { decodeRustUnicodeEscape } from './rust-unicode-escape-rules.js';\n",
    "import { acceptRustSourceHeaderLifetimeCharacter } from './rust-source-header-emission-rules.js';\n",
    "import { acceptRustSourceWhitespace } from './rust-source-whitespace-rules.js';\n",
  ]],
];

export function readRustCharacterSourceClosure() {
  const captures = [], parts = [];
  for (const [path, imports] of sourceEntries) {
    const source = readFileSync(new URL(path, root), 'utf8');
    let body = source;
    const removedImports = [];
    for (const statement of imports) {
      assert.equal(source.split(statement).length, 2, 'one actual module import: ' + path);
      assert.equal(body.split(statement).length, 2, 'each import is removed once');
      const start = source.indexOf(statement), end = start + statement.length;
      removedImports.push({ statement, start, end,
        byteStart: Buffer.byteLength(source.slice(0, start)),
        byteEnd: Buffer.byteLength(source.slice(0, end)) });
      body = body.replace(statement, '');
    }
    assert.doesNotMatch(body, /^\s*import\b/mu, 'an unrecorded source dependency must refuse: ' + path);
    captures.push({ path, source, bytes: Buffer.byteLength(source), sourceSha256: digest(source), removedImports });
    parts.push(body);
  }
  return { source: captures.at(-1).source, sourceSha256: captures.at(-1).sourceSha256,
    closure: parts.join('\n\n'), captures };
}

function checkedOwners(program, inventory) {
  return inventory.map(source => {
    const declaration = program.declarations.get(source.name);
    assert.ok(declaration, 'the complete checked closure retains ' + source.name);
    assert.equal(declaration.k, source.kind, 'actual source kind: ' + source.name);
    assert.equal(declaration.name, source.name, 'actual source name: ' + source.name);
    return { sourceOrdinal: source.ordinal, kind: declaration.k, name: declaration.name,
      fullName: declaration.fullName, modulePath: declaration.modulePath };
  });
}

function ownerMappings(emitted, inventory) {
  return inventory.map(owner => {
    const mappings = emitted.mappings.filter(mapping => mapping.source === owner.fullName
      && mapping.kind === (owner.kind === 'fn' ? 'function' : owner.kind));
    assert.equal(mappings.length, 1, 'one actual emitted mapping for ' + owner.fullName);
    return mappings[0];
  });
}

function readRustCharacterLibraryParameters(source) {
  const parameters = source.split(',').map(parameter => parameter.trim());
  if (parameters.at(-1) === '') parameters.pop();
  assert.ok(parameters.length > 0 && parameters.every(parameter => parameter.length > 0),
    'a character library signature has no empty parameter slots');
  return parameters;
}

// The existing writer records/replays its source-derived loan and lint actions.
// Verify the actual resulting interfaces, rather than declaring that loans ran.
export function readRustCharacterLibraryInterface(files, emitted, inventory) {
  const source = files.map(file => file.source).join('\n\n');
  const mappings = ownerMappings(emitted, inventory);
  for (const [index, owner] of inventory.entries()) {
    const target = mappings[index].target;
    assert.match(target, /^[A-Za-z_][A-Za-z_0-9]*$/u, 'one flat actual library declaration');
    const declarations = [...source.matchAll(new RegExp('^pub (?:const )?'
      + (owner.kind === 'fn' ? 'fn' : '(?:enum|struct)') + ' ' + target + '\\b', 'gmu'))];
    assert.equal(declarations.length, 1, 'packaging retains the actual source owner ' + owner.fullName);
  }
  const interfaces = [];
  for (const name of ['read_source_text_scalar', 'decode_rust_unicode_escape',
    'finish_rust_character_literal', 'decode_rust_character_literal']) {
    const block = readSourceTextRustBlock(source, 'fn', name);
    const header = /\bfn \w+\(([^)]*)\)/u.exec(block);
    assert.ok(header, 'the actual generated entry has its complete header: ' + name);
    const original = /\bfn \w+\(([^)]*)\)/u.exec(readSourceTextRustBlock(emitted.text, 'fn', name));
    assert.ok(original, 'the complete original generated entry remains available: ' + name);
    const expected = readRustCharacterLibraryParameters(original[1]).map(parameter => parameter
      .replace(/^units:\s*Vec<f64>$/u, 'units: &[f64]').replace(/\s+/gu, ''));
    assert.deepEqual(readRustCharacterLibraryParameters(header[1]).map(parameter => parameter.replace(/\s+/gu, '')), expected,
      'only the full-source parameter loan changes the actual argument interface: ' + name);
    assert.match(header[1], /(?:^|,)\s*units:\s*&\[f64\](?:,|$)/u,
      'the full original source vector is borrowed: ' + name);
    interfaces.push({ name, signature: header[0] });
  }
  const tokens = tokenize(source, 'Rust').tokens;
  for (let at = 0; at + 4 < tokens.length; at += 1) {
    assert.ok(!(tokens[at].kind === 'identifier' && tokens[at].value === 'units'
      && tokens[at + 1].value === '.' && ['clone', 'to_vec'].includes(tokens[at + 2].value)
      && tokens[at + 3].value === '(' && tokens[at + 4].value === ')'),
    'packaged full-source input must not be copied at a generated call');
  }
  return { interfaces, files: files.map(file => ({ path: file.path ?? 'rust_character_rules.rs',
    bytes: Buffer.byteLength(file.source), sourceSha256: digest(file.source) })) };
}

export async function generateRustCharacterRules({ check = false, native = false } = {}) {
  const closed = readRustCharacterSourceClosure();
  const policy = { schemaVersion: 1, model: 'bounded unsuffixed Rust character token draft 1',
    sourcePath, source: closed.source, sourceSha256: closed.sourceSha256,
    sourceClosure: { source: closed.closure, sourceSha256: digest(closed.closure), inputs: closed.captures },
    input: 'Original dense primitive Number UTF16 source units and an absolute integral Number offset.',
    output: 'Closed character or byte-character token/refusal plans with absolute source boundaries; not typed char or byte admission.',
    boundary: 'Nonliteral apostrophes remain on the original lifetime/label path. Suffixes remain explicitly unsupported in this bounded profile. Character-to-String, character-to-number and byte-to-fixed semantic coercions are not admitted.' };
  for (const relative of ['js/src/translation/data/rust-character-rules.lino', 'rust/src/data/rust-character-rules.lino']) {
    const file = new URL(relative, root);
    if (check) checkProjectData(file, policy);
    else writeProjectData(file, policy);
  }
  let artifact = null;
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const { encodeTypedProgramLinks, decodeTypedProgramLinks } = await import('../src/typed-program-links.js');
    const { recheckLinkedTypedProgram } = await import('../src/typed-program-checked-projection.js');
    const { LinkNetwork } = await import('../src/network.js');
    const { encodeLinkNetworkBinary, decodeLinkNetworkBinary } = await import('../src/link-network-binary.js');
    const surface = parseJavaScript(closed.closure, { referenceSemantics: false });
    const inventory = surface.items.map((item, ordinal) => {
      assert.ok(['fn', 'data'].includes(item.k) && typeof item.name === 'string',
        'every actual source item is classified; none is silently filtered');
      return { ordinal, kind: item.k, name: item.name, span: item.span };
    });
    const checked = checkProgram(surface);
    const owners = checkedOwners(checked, inventory);
    assert.equal(owners.length, checked.declarations.size, 'all checked declarations belong to the captured source inventory');
    const emitted = emitRust(checked, { arrayConsumption: false });
    ownerMappings(emitted, owners);
    const packet = encodeTypedProgramLinks(checked);
    const replays = [];
    for (const [format, network] of [
      ['text', LinkNetwork.fromLino(packet.network.toLino())],
      ['binary', decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))],
    ]) {
      const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
      const restoredOwners = checkedOwners(restored, inventory);
      assert.deepEqual(restoredOwners, owners, 'the complete owner inventory survives ' + format);
      const replay = emitRust(restored, { arrayConsumption: false });
      ownerMappings(replay, restoredOwners);
      assert.equal(replay.text, emitted.text, 'complete raw native source survives ' + format);
      assert.deepEqual(replay.definitions, emitted.definitions, 'every full native definition survives ' + format);
      assert.deepEqual(replay.preludes, emitted.preludes, 'every native prelude survives ' + format);
      replays.push({ format, checkedNamedSourceInventory: restoredOwners,
        emittedTextBytes: Buffer.byteLength(replay.text), emittedTextSha256: digest(replay.text) });
    }
    const files = writeRecordRuleLibrary(emitted, 'rust_character_rules', { check });
    const libraryInterface = readRustCharacterLibraryInterface(files, emitted, owners);
    artifact = { schemaVersion: 1, model: policy.model,
      sourceSha256: closed.sourceSha256, sourceClosure: policy.sourceClosure,
      inventory, checkedNamedSourceInventory: owners, replays,
      emitted: { ...emitted, definitions: emitted.definitions, preludes: emitted.preludes },
      emittedTextBytes: Buffer.byteLength(emitted.text), emittedTextSha256: digest(emitted.text),
      libraryInterface,
      boundary: 'Actual complete source generation, archive recheck and recorded library packaging only. Native compilation and execution are CI observations, not this source receipt.' };
    const output = new URL('parity/self-translation/rust-character-rules-native.lino', root);
    if (check) checkProjectData(output, artifact);
    else writeProjectData(output, artifact);
  }
  return { policy, artifact };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  assert.ok(process.argv.slice(2).every(argument => ['--check', '--native'].includes(argument)), 'only source generation/freshness options are admitted');
  await generateRustCharacterRules({ check: process.argv.includes('--check'), native: process.argv.includes('--native') });
  console.log('Rust character scanner: complete original dependency closure and recorded borrowed library');
}
