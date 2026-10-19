import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import vm from 'node:vm';
import { readRustStringContinuationDraft } from '../../experiments/issue-199-rust-string-continuation-adapter.mjs';
import { acceptRustCookedStringEscapePrefix, acceptRustStringContinuationWhitespace, readRustSourceLineBreakEnd, readRustStringContinuationEnd } from '../../experiments/issue-199-rust-string-continuation-rules.mjs';
import { retainCompilerFeedback } from '../scripts/compiler-style-feedback.mjs';
import { LinkNetwork } from '../src/network.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';

const units = text => Array.from({ length: text.length }, (_, index) => text.charCodeAt(index));
const fixtures = [
  ['"alpha\\\n    beta"', 'alphabeta'],
  ['"α\\\n\t β"', 'αβ'],
  ['"😀\\\r\n \tβ"', '😀β'],
  ['"foo\\\n \n\r\n\tbar"', 'foobar'],
  ['"foo\\\n\u00a0bar"', 'foo\u00a0bar'],
  ['"foo\\\n\u000bbar"', 'foo\u000bbar'],
  ['"foo\\\n\u000cbar"', 'foo\u000cbar'],
  ['"foo\\\n\u0085bar"', 'foo\u0085bar'],
  ['"foo\\\n\u2028bar"', 'foo\u2028bar'],
  ['"foo\\\n\u2029bar"', 'foo\u2029bar'],
  ['"foo\\\n\u2003bar"', 'foo\u2003bar'],
  ['"\\\n\t "', ''],
  ['"foo\\\\\nbar"', 'foo\\\nbar'],
  ['"foo\\\\\\\n   bar"', 'foo\\bar'],
  ['"foo\r\nbar"', 'foo\nbar'],
  ['"foo\\nbar"', 'foo\nbar'],
  ['"foo\\\n  \\nbar"', 'foo\nbar'],
  ['"foo\\\n  \\u{1f600}"', 'foo😀'],
  ['"foo\\\n  \\\"bar"', 'foo"bar'],
];

function reloadRules() {
  const source = readFileSync(new URL('../../experiments/issue-199-rust-string-continuation-rules.mjs', import.meta.url), 'utf8');
  const archive = createTypedProgramLinks(source, 'JavaScript', { context: { referenceSemantics: false } });
  const text = archive.network.toLino();
  assert.ok(Buffer.byteLength(text) < 256 * 1024, 'only the closed lexical decisions belong to this bounded draft archive');
  return { network: LinkNetwork.fromLino(text), root: archive.root };
}

test('Rust continuation decisions consume only the four specified units and preserve original offsets', () => {
  const accepted = [9, 10, 13, 32];
  for (const unit of [...accepted, 0, 11, 12, 0x85, 0xa0, 0x2003, 0x2028, 0x2029, 0x3000, 92]) {
    assert.equal(acceptRustStringContinuationWhitespace(unit), accepted.includes(unit), String(unit));
  }
  for (const source of ['\\\n\t \n\r tail', '\\\r\n\t tail']) {
    assert.equal(readRustStringContinuationEnd(units(source), 0), source.indexOf('tail'));
  }
  for (const source of ['\\r', '\\\r', '\\\u2028', '\\\u2029', '\\ \n', '\\', '\n']) {
    assert.equal(readRustStringContinuationEnd(units(source), 0), -1, JSON.stringify(source));
  }
  for (const start of [-1, 0.5, 100]) assert.equal(readRustStringContinuationEnd(units('\\\n '), start), -1);
  assert.equal(readRustSourceLineBreakEnd(units('\r\n'), 0), 2);
  assert.equal(readRustSourceLineBreakEnd(units('\r'), 0), -1);
  assert.equal(readRustSourceLineBreakEnd(units('\n'), 0), -1);
  assert.equal(acceptRustStringContinuationWhitespace(undefined), false);
  assert.equal(acceptRustCookedStringEscapePrefix(undefined), false);
  for (const text of ['"', "'", '0', '\\', 'n', 'r', 't', 'u', 'x']) assert.equal(acceptRustCookedStringEscapePrefix(text.charCodeAt(0)), true);
  for (const text of ['b', 'f', 'v', '{', 'N']) assert.equal(acceptRustCookedStringEscapePrefix(text.charCodeAt(0)), false);
});

test('the cooked-string draft preserves every UTF-8 source byte while normalizing continuation values', () => {
  const prefix = '// 😀\n';
  for (const [literal, expected] of fixtures) {
    const source = prefix + literal + '; trailing';
    const token = readRustStringContinuationDraft(source, prefix.length);
    assert.equal(token.value, expected, literal);
    assert.equal(token.raw, literal);
    assert.equal(token.start, prefix.length);
    assert.equal(token.end, prefix.length + literal.length);
    assert.equal(token.utf8Start, Buffer.byteLength(prefix));
    assert.equal(token.utf8End, Buffer.byteLength(prefix + literal));
    assert.equal(token.pieces.map(piece => piece.raw).join(''), literal);
    let offset = token.start;
    let byteOffset = token.utf8Start;
    const bytes = Buffer.from(source);
    for (const piece of token.pieces) {
      assert.equal(piece.start, offset);
      assert.equal(piece.utf8Start, byteOffset);
      assert.equal(source.slice(piece.start, piece.end), piece.raw);
      assert.deepEqual(bytes.subarray(piece.utf8Start, piece.utf8End), Buffer.from(piece.raw));
      offset = piece.end;
      byteOffset = piece.utf8End;
    }
    assert.equal(offset, token.end);
    assert.equal(byteOffset, token.utf8End);
  }
  assert.throws(() => readRustStringContinuationDraft('"foo\\\n '), /unterminated string literal/u);
  assert.throws(() => readRustStringContinuationDraft('"foo\rbar"'), /isolated carriage return/u);
  assert.throws(() => readRustStringContinuationDraft('"foo\\\rbar"'), /unsupported string escape/u);
  assert.throws(() => readRustStringContinuationDraft('"foo\\\u2028bar"'), /unsupported string escape/u);
  assert.throws(() => readRustStringContinuationDraft('"\ud800"'), /Unicode scalar values/u);
  for (const escape of ['b', 'f', 'v', '{', 'N']) assert.throws(() => readRustStringContinuationDraft('"\\' + escape + '"'), /unsupported string escape/u);
  for (const source of ['r"foo"', 'b"foo"', 'c"foo"']) assert.throws(() => readRustStringContinuationDraft(source), /cooked string/u);
});

test('continuation decisions survive source-free LiNo reload and automatically emit both target languages', () => {
  const archive = reloadRules();
  const program = decodeTypedProgramLinks(archive.network, archive.root);
  assert.ok(program.declarations.has('readRustStringContinuationEnd'));
  const javascript = emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code;
  const rust = emitTypedProgramLinks(archive.network, archive.root, 'Rust').code;
  assert.match(rust, /pub fn read_rust_string_continuation_end/u);
  assert.match(rust, /pub fn read_rust_source_line_break_end/u);
  const context = vm.createContext({});
  vm.runInContext(javascript, context, { timeout: 1000 });
  for (const [literal] of fixtures) {
    const encoded = units(literal);
    for (let start = 0; start < encoded.length; start += 1) {
      assert.equal(context.readRustStringContinuationEnd(encoded, start), readRustStringContinuationEnd(encoded, start));
      assert.equal(context.readRustSourceLineBreakEnd(encoded, start), readRustSourceLineBreakEnd(encoded, start));
    }
  }
  for (const unit of [9, 10, 13, 32, 11, 12, 0xa0, 0x2003, 0x2028]) {
    assert.equal(context.acceptRustStringContinuationWhitespace(unit), acceptRustStringContinuationWhitespace(unit));
  }
  for (const unit of [34, 39, 48, 92, 110, 114, 116, 117, 120, 98, 102, 118, 123]) {
    assert.equal(context.acceptRustCookedStringEscapePrefix(unit), acceptRustCookedStringEscapePrefix(unit));
  }
});

test('CI checks the automatically emitted continuation rules and literal values against native Rust', {
  skip: process.env.TRANSLATION_CHECK_RUST !== '1' ? 'native Rust lexical verification runs only in CI' : false,
}, () => {
  assert.equal(process.env.CI, 'true', 'never compile Rust locally');
  const archive = reloadRules();
  const generated = emitTypedProgramLinks(archive.network, archive.root, 'Rust').code;
  // Rust reports lexical warnings for multiple skipped lines and unskipped
  // Unicode whitespace. All nineteen fixtures exercise the generated decisions
  // in native code; eleven warning-free original literals also provide the
  // strict compiler's independent string-value oracle.
  const nativeLiteralIndexes = new Set([0, 1, 2, 11, 12, 13, 14, 15, 16, 17, 18]);
  const cases = fixtures.map(([literal], index) => {
    const encoded = units(literal);
    const valueOracle = nativeLiteralIndexes.has(index) ? `    let value_${index}: &str = ${literal};
    println!("value\\t${index}\\t{}", value_${index}.as_bytes().iter().map(|byte| format!("{byte:02x}")).collect::<String>());\n` : '';
    return `${valueOracle}
    let units_${index} = vec![${encoded.map(unit => `${unit}.0`).join(', ')}];
    for start in 0..units_${index}.len() {
        println!("end\\t${index}\\t{start}\\t{}\\t{}", read_rust_string_continuation_end(units_${index}.clone(), start as f64), read_rust_source_line_break_end(units_${index}.clone(), start as f64));
    }`;
  }).join('\n');
  const directory = mkdtempSync(join(tmpdir(), 'meta-rust-continuation-'));
  const source = join(directory, 'continuation.rs');
  const executable = join(directory, 'continuation');
  try {
    writeFileSync(source, `mod rules {\n${generated}\n}\npub use rules::*;\nfn main() {\n${cases}\n}\n`);
    const compiled = spawnSync('rustc', ['--edition', '2024', '-O', '-D', 'warnings', '--error-format=json', source, '-o', executable],
      { encoding: 'utf8', timeout: 120_000, maxBuffer: 8 * 1024 * 1024 });
    retainCompilerFeedback({ root: fileURLToPath(new URL('../../', import.meta.url)), group: 'rust-string-continuation-draft',
      compiler: 'rustc', round: 0, stderr: compiled.stderr ?? '', inputs: [{ path: source, relativePath: 'generated/rust-string-continuation-draft.rs' }] });
    assert.equal(compiled.status, 0, compiled.error?.message ?? compiled.stderr);
    const executed = spawnSync(executable, [], { encoding: 'utf8', timeout: 120_000 });
    assert.equal(executed.status, 0, executed.error?.message ?? executed.stderr);
    const rows = executed.stdout.trimEnd().split('\n').map(line => line.split('\t'));
    const values = rows.filter(row => row[0] === 'value');
    assert.equal(values.length, nativeLiteralIndexes.size);
    for (const [kind, index, text] of values) {
      assert.equal(kind, 'value');
      assert.ok(nativeLiteralIndexes.has(Number(index)));
      assert.equal(text, Buffer.from(fixtures[Number(index)][1]).toString('hex'));
    }
    const ends = rows.filter(row => row[0] === 'end');
    assert.equal(ends.length, fixtures.reduce((count, [literal]) => count + literal.length, 0));
    for (const [, index, start, continuationEnd, lineBreakEnd] of ends) {
      const encoded = units(fixtures[Number(index)][0]);
      assert.equal(Number(continuationEnd), readRustStringContinuationEnd(encoded, Number(start)));
      assert.equal(Number(lineBreakEnd), readRustSourceLineBreakEnd(encoded, Number(start)));
    }
  } finally { rmSync(directory, { recursive: true, force: true }); }
});
