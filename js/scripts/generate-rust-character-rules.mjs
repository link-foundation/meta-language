// Every dependency declaration is retained. No generated Rust is authored here.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { tokenize } from '../src/translation/lexer.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';
import { readSourceTextRustBlock } from './source-text-byte-library-style.mjs';
import { readRustLibraryItems } from './rust-library-packaging.mjs';
import { flattenRustSourceHeaderStyleFormat } from './rust-source-header-library-style.mjs';

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

/** Record finite styles over the actual complete character dependency owners. */
export function readRustCharacterLibraryStyles(source) {
  const items = readRustLibraryItems(source).items;
  const inspect = text => {
    const { tokens, comments } = tokenize(text, 'Rust'), matching = new Map(), stack = [];
    for (let at = 0; at < tokens.length; at += 1) {
      if (tokens[at].kind !== 'punct') continue;
      if (['(', '[', '{'].includes(tokens[at].value)) stack.push(at);
      else if ([')', ']', '}'].includes(tokens[at].value)) {
        const open = stack.pop();
        assert.equal(tokens[open]?.value, ({ ')': '(', ']': '[', '}': '{' })[tokens[at].value]);
        matching.set(open, at); matching.set(at, open);
      }
    }
    assert.equal(stack.length, 0);
    const words = (start, end) => tokens.slice(start, end).filter(token => token.kind !== 'eof')
      .map(token => token.kind === 'number' ? token.raw.replaceAll('_', '') : token.value).join('');
    return { tokens, comments, matching, words };
  };
  const whole = inspect(source), protectedNames = new Set(['f64', 'bool', 'str', 'String', 'Box', 'Clone', 'ToOwned', 'format', 'RustUnicodeEscape']);
  for (let at = 0; at < whole.tokens.length; at += 1) {
    const token = whole.tokens[at];
    if (['type', 'struct', 'enum', 'mod', 'trait'].includes(token.value)
      && protectedNames.has(whole.tokens[at + 1]?.value)
      && !(token.value === 'enum' && whole.tokens[at + 1].value === 'RustUnicodeEscape')) return { source, actions: [] };
    if (!['use', 'impl'].includes(token.value)) continue;
    let end = at + 1;
    while (whole.tokens[end] && ![';', '{'].includes(whole.tokens[end].value) && whole.tokens[end].kind !== 'eof') end += 1;
    if (whole.tokens.slice(at + 1, end).some(part => token.value === 'use'
      ? protectedNames.has(part.value) || part.value === '*' : part.value === 'RustUnicodeEscape')) return { source, actions: [] };
  }
  const functions = new Map();
  for (const item of items.filter(item => item.kind === 'fn')) {
    assert.ok(!functions.has(item.name), 'a character style owns one actual complete function');
    const parsed = inspect(item.source), fn = parsed.tokens.findIndex(token => token.value === 'fn');
    const close = parsed.matching.get(fn + 2);
    const body = parsed.tokens.findIndex((token, index) => index > close && token.value === '{');
    functions.set(item.name, { ...parsed, item, fn, close, body });
  }
  const spelling = text => { const parsed = inspect(text); return parsed.words(0, parsed.tokens.length); };
  const enumOwners = items.filter(item => item.kind === 'enum' && item.name === 'RustUnicodeEscape');
  const ownedEnum = enumOwners.length === 1 && spelling(enumOwners[0].source)
    === '#[derive(Clone,Debug,PartialEq)]pubenumRustUnicodeEscape{Scalar(f64,f64),Malformed,Unsupported(f64,f64),}';
  const actions = [];
  let adapted = source;
  const replace = (from, to) => {
    if (from === to) return;
    assert.equal(adapted.split(from).length, 2, 'a character style retains its unique complete source owner');
    actions.push({ op: 'replace', field: 'source', from, to });
    adapted = adapted.replace(from, to);
  };
  const displayFormat = (parsed, text, at) => {
    const literal = parsed.tokens[at + 2];
    if (literal?.kind !== 'string') return false;
    let format;
    try { format = JSON.parse(text.slice(literal.start, literal.end)); } catch { return false; }
    if (typeof format !== 'string') return false;
    const fields = format.replaceAll('{{', '').replaceAll('}}', '');
    return !/[{}]/u.test(fields.replace(/\{(?:[A-Za-z_][A-Za-z_0-9]*)?\}/gu, ''));
  };
  const decoder = functions.get('decode_rust_unicode_escape');
  if (ownedEnum && decoder) {
    const { item, tokens, matching, words } = decoder;
    let styled = item.source;
    const edits = [];
    for (let at = decoder.body + 1; at < tokens.length; at += 1) {
      if (words(at, at + 2) !== 'self::' || !/^MlDecodeRustUnicodeEscapeLoop[0-9]+Result$/u.test(tokens[at + 2]?.value ?? '')
        || tokens[at + 3]?.value !== '::' || tokens[at + 5]?.value !== '(') continue;
      const owner = tokens[at + 2].value, variant = tokens[at + 4].value;
      const close = matching.get(at + 5), payload = tokens[at + 6]?.value;
      if (![payload, payload + ','].includes(words(at + 6, close)) || tokens[close + 1]?.value !== '=>'
        || tokens[close + 2]?.value !== '{') continue;
      const end = matching.get(close + 2);
      if (words(close + 3, end) !== 'let' + payload + '=*' + payload + ';' + payload + '.clone()'
        || decoder.comments.some(comment => comment.start >= tokens[close + 3].start && comment.end <= tokens[end - 1].end)) continue;
      const declarations = items.filter(item => item.kind === 'enum' && item.name === owner);
      if (declarations.length !== 1 || !spelling(declarations[0].source).includes(variant + '(Box<self::RustUnicodeEscape>)')) continue;
      let open = at - 1;
      while (open >= 0 && !(tokens[open].value === '{' && matching.get(open) > end && tokens[open - 2]?.value === 'match')) open -= 1;
      const state = tokens[open - 1]?.value;
      if (open < 0 || tokens.filter(token => token.kind === 'identifier' && token.value === state).length !== 2) continue;
      const binding = tokens.findIndex((token, index) => token.value === 'let' && tokens[index + 1]?.value === state && tokens[index + 2]?.value === '=');
      const call = binding + 3;
      if (binding < 0 || words(call, call + 2) !== 'self::' || tokens[call + 3]?.value !== '(') continue;
      const target = functions.get(tokens[call + 2].value), callEnd = matching.get(call + 3);
      if (!target || tokens[callEnd + 1]?.value !== ';' || callEnd >= open
        || target.words(target.close + 1, target.body) !== '->self::' + owner) continue;
      // An owned return signature and its actual Box field establish a move.
      // The primitive-only derived result has no custom Clone/Drop behavior.
      edits.push([tokens[close + 3].start, tokens[end - 1].end, '*' + payload]);
    }
    for (const [start, end, text] of edits.reverse()) styled = styled.slice(0, start) + text + styled.slice(end);
    replace(item.source, styled);
  }
  for (const fn of functions.values()) {
    if (!fn.item.name.startsWith('render_rust_source_header_')) continue;
    const parameters = fn.words(fn.fn + 3, fn.close).split(',');
    if (parameters.at(-1) === '') parameters.pop();
    if (!parameters.length || parameters.some(parameter => !/^[A-Za-z_][A-Za-z_0-9]*:&str$/u.test(parameter))) continue;
    const names = new Set(parameters.map(parameter => parameter.split(':')[0]));
    const { tokens, matching, comments } = fn, edits = [];
    if (tokens.slice(fn.body + 1).some(token => ['let', 'match', 'for', 'fn', '|'].includes(token.value)
      || token.kind === 'macro' && token.value !== 'format')) continue;
    if (tokens.some((token, index) => token.kind === 'macro' && token.value === 'format'
      && comments.some(comment => comment.start >= token.start && comment.end <= tokens[matching.get(index + 1)].end))) continue;
    for (let at = fn.body + 1; at + 4 < tokens.length; at += 1) {
      if (tokens[at].kind !== 'identifier' || !names.has(tokens[at].value)
        || fn.words(at + 1, at + 5) !== '.to_owned()' || ['.', '::'].includes(tokens[at - 1]?.value)) continue;
      const formats = tokens.flatMap((token, index) => token.kind === 'macro' && token.value === 'format'
        && index < at && matching.get(index + 1) > at + 4 && displayFormat(fn, fn.item.source, index) ? [index] : []);
      if (!formats.length || comments.some(comment => comment.start >= tokens[at + 1].start && comment.end <= tokens[at + 4].end)) continue;
      edits.push([tokens[at + 1].start, tokens[at + 4].end]);
    }
    let styled = fn.item.source;
    for (const [start, end] of edits.reverse()) styled = styled.slice(0, start) + styled.slice(end);
    // Reuse the existing pure source-header concatenation proof, including
    // escaped literal braces and opaque conditional branches, to inline names.
    for (let pass = 0; pass < 30; pass += 1) {
      const current = inspect(styled);
      let changed = false;
      for (let at = current.tokens.length - 2; at >= 0; at -= 1) {
        if (current.tokens[at].kind !== 'macro' || current.tokens[at].value !== 'format') continue;
        const end = current.matching.get(at + 1);
        if (!displayFormat(current, styled, at) || current.comments.some(comment => comment.start >= current.tokens[at].start && comment.end <= current.tokens[end].end)) continue;
        const from = styled.slice(current.tokens[at].start, current.tokens[end].end);
        const to = flattenRustSourceHeaderStyleFormat(from);
        if (from === to) continue;
        styled = styled.slice(0, current.tokens[at].start) + to + styled.slice(current.tokens[end].end);
        changed = true; break;
      }
      if (!changed) break;
      assert.ok(pass < 29, 'finite character header format styles converge');
    }
    replace(fn.item.source, styled);
  }
  // These are the complete finite numeric/Boolean leaf and caller owners.
  // A callee enters the const set only after its own body has been proved.
  const candidates = new Set(['accept_rust_cooked_string_escape_prefix', 'read_rust_ascii_escape_digit',
    'accept_rust_source_whitespace', 'accept_rust_source_byte_order_mark', 'read_rust_character_simple_escape_value',
    'read_rust_ascii_escape_value', 'read_rust_character_byte_escape_value']);
  const constants = new Set();
  for (let pass = 0; pass < candidates.size; pass += 1) {
    let changed = false;
    for (const name of candidates) {
      if (constants.has(name) || !functions.has(name)) continue;
      const fn = functions.get(name), { tokens, words } = fn;
      if (!['->f64', '->bool'].includes(words(fn.close + 1, fn.body))) continue;
      const parameters = words(fn.fn + 3, fn.close).split(',');
      if (parameters.at(-1) === '') parameters.pop();
      if (parameters.some(parameter => !/^[A-Za-z_][A-Za-z_0-9]*:(?:f64|bool)$/u.test(parameter))) continue;
      const names = new Set(parameters.map(parameter => parameter.split(':')[0]));
      let accepted = true;
      for (let at = fn.body + 1; at < tokens.length - 1; at += 1) if (tokens[at].value === 'let') {
        if (tokens[at + 1]?.kind !== 'identifier' || tokens[at + 2]?.value !== '=' || names.has(tokens[at + 1].value)) { accepted = false; break; }
        names.add(tokens[at + 1].value);
      }
      const allowed = new Set(['if', 'else', 'let', 'return', 'true', 'false', '(', ')', '{', '}', ',', ';', '=', '+', '-', '*', '/', '<', '>', '<=', '>=', '==', '!=', '&&', '||', '!']);
      for (let at = fn.body + 1; accepted && at < tokens.length - 1; at += 1) {
        const token = tokens[at];
        if (token.kind === 'number') { accepted = token.suffix === 'f64'; continue; }
        if (token.value === 'self' && tokens[at + 1]?.value === '::' && constants.has(tokens[at + 2]?.value) && tokens[at + 3]?.value === '(') { at += 2; continue; }
        if (names.has(token.value) && tokens[at + 1]?.value !== '(' || allowed.has(token.value)) continue;
        accepted = false;
      }
      if (!accepted) continue;
      constants.add(name); changed = true;
      if (tokens[fn.fn - 1]?.value !== 'const') replace(fn.item.source,
        fn.item.source.slice(0, tokens[fn.fn].start) + 'const ' + fn.item.source.slice(tokens[fn.fn].start));
    }
    if (!changed) break;
  }
  return { source: adapted, actions };
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
    const files = writeRecordRuleLibrary(emitted, 'rust_character_rules', { check, sourceStyles: readRustCharacterLibraryStyles });
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
