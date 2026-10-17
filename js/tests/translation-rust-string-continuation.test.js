import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { encodeFormalConceptDefinition, proveFormalConceptEquivalence, verifyFormalConceptEquivalence } from '../src/formal-concept-definition.js';
import { LinkNetwork } from '../src/network.js';
import { translateProgram } from '../src/program-translation.js';
import { tokenize } from '../src/translation/lexer.js';
import { checkProgram } from '../src/translation/check.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';

const root = fileURLToPath(new URL('../../', import.meta.url));
const fixtures = readProjectData(new URL('../../parity/fixtures/translation-rust-string-continuations.lino', import.meta.url)).fixtures;
const sourceFor = literal => `// 😀\nfn main(){println!("{}",${literal});}\n`;
const execute = code => {
  const output = [];
  vm.runInNewContext(code, { console: { log: value => output.push(String(value)) } }, { timeout: 1000 });
  return output;
};

test('production Rust tokens normalize every original continuation fixture and preserve source spans', () => {
  assert.equal(fixtures.length, 19);
  for (const { id, literal, value } of fixtures) {
    const source = sourceFor(literal);
    const token = tokenize(source, 'Rust').tokens.find(token => token.raw === literal);
    assert.ok(token, id);
    assert.equal(token.kind, 'string', id);
    assert.equal(token.value, value, id);
    assert.equal(token.start, source.indexOf(literal), id);
    assert.equal(token.end, token.start + literal.length, id);
    assert.equal(source.slice(token.start, token.end), literal, id);
    const bytes = Buffer.from(source);
    const byteStart = Buffer.byteLength(source.slice(0, token.start));
    const byteEnd = Buffer.byteLength(source.slice(0, token.end));
    assert.deepEqual(bytes.subarray(byteStart, byteEnd), Buffer.from(literal), id);
  }
});

test('Rust continuations translate through source-free semantic links into JavaScript and reimportable canonical literals', () => {
  for (const { id, literal, value } of fixtures) {
    const source = sourceFor(literal);
    const direct = translateProgram(source, 'Rust', 'JavaScript');
    assert.equal(direct.diagnostic, null, id);
    assert.deepEqual(execute(direct.code), [value], id);
    const created = createTypedProgramLinks(source, 'Rust');
    const network = LinkNetwork.fromLino(created.network.toLino());
    assert.deepEqual(decodeTypedProgramLinks(network, created.root), decodeTypedProgramLinks(created.network, created.root), id);
    assert.deepEqual(execute(emitTypedProgramLinks(network, created.root, 'JavaScript').code), [value], id);
    const canonicalRust = emitTypedProgramLinks(network, created.root, 'Rust').code;
    const emittedLiteral = tokenize(canonicalRust, 'Rust').tokens.find(token => token.kind === 'string' && token.value === value);
    assert.ok(emittedLiteral, id);
    // Reimport the actual emitted literal in the already supported source main.
    // Full generated runtime-wrapper admission is a separate frontend gap.
    const imported = translateProgram(sourceFor(emittedLiteral.raw), 'Rust', 'JavaScript');
    assert.equal(imported.diagnostic, null, id);
    assert.deepEqual(execute(imported.code), [value], id);
  }
});

test('Rust skips its selected continuation whitespace while JavaScript retains following indentation', () => {
  const literal = '"a\\\n \t b"';
  assert.equal(tokenize(literal, 'Rust').tokens[0].value, 'ab');
  assert.equal(tokenize(literal, 'JavaScript').tokens[0].value, 'a \t b');
  assert.equal(tokenize('"a\\\r\n  b"', 'Rust').tokens[0].value, 'ab');
  assert.equal(tokenize('"a\\\r\n  b"', 'JavaScript').tokens[0].value, 'a  b');
  for (const literal of ['"a\\\r b"', '"a\\\u2028 b"', '"a\\\u2029 b"', '"a\\q"']) {
    const prefix = '// 😀\n';
    assert.throws(() => tokenize(prefix + literal, 'Rust'), error => error.kind === 'syntax'
      && error.message.includes('unsupported string escape')
      && error.span.start === prefix.length + literal.indexOf('\\')
      && error.span.end === error.span.start + 2);
  }
});

test('multiple Rust continuation tokens reuse one lazy source encoding', () => {
  const source = Array.from({ length: 40 }, (_, index) => `let text${index} = "α\\\n  β";`).join('\n');
  const original = String.prototype.charCodeAt;
  let calls = 0;
  let result;
  try {
    String.prototype.charCodeAt = function (index) { calls += 1; return original.call(this, index); };
    result = tokenize(source, 'Rust');
  } finally { String.prototype.charCodeAt = original; }
  assert.equal(calls, source.length);
  assert.equal(result.tokens.filter(token => token.kind === 'string').length, 40);
  assert.ok(result.tokens.filter(token => token.kind === 'string').every(token => token.value === 'αβ'));
});

test('the production Rust rule source and linked fixture corpus regenerate without native compilation', () => {
  for (const script of ['generate-rust-string-rules.mjs', 'build-rust-string-continuation-fixtures.mjs']) {
    execFileSync(process.execPath, [`js/scripts/${script}`, '--check'], { cwd: root, timeout: 30_000, stdio: 'pipe' });
  }
  const generated = readFileSync(new URL('../../rust/src/translation/frontend_rules/rust_strings.rs', import.meta.url), 'utf8');
  assert.ok(generated.includes('units: &[f64]'));
  assert.ok(!generated.includes('Vec<f64>'));
  assert.ok(!generated.includes('units.clone()'));
  assert.ok(generated.includes('crate::translation::frontend_rules::ml_array::optional_at'));
});

test('production continuation definitions preserve a checked correspondence with the original closed draft', () => {
  const original = parseJavaScript(readFileSync(new URL('../../experiments/issue-199-rust-string-continuation-rules.mjs', import.meta.url), 'utf8'));
  // The unrelated experimental invalid-escape classifier stays outside this
  // production change. Every original reachable continuation definition stays.
  original.items = original.items.filter(item => item.name !== 'acceptRustCookedStringEscapePrefix');
  const first = encodeFormalConceptDefinition(checkProgram(original));
  const second = encodeFormalConceptDefinition(createTypedProgramLinks(
    readFileSync(new URL('../src/translation/rust-string-rules.js', import.meta.url), 'utf8'), 'JavaScript'));
  const reload = ({ network, root }) => ({ network: LinkNetwork.fromLino(network.toLino()), root });
  const proof = proveFormalConceptEquivalence(reload(first), reload(second));
  assert.ok(proof, 'the declared checked-program primitive model proves the entire closed correspondence');
  assert.equal(verifyFormalConceptEquivalence(reload(first), reload(second), reload(proof)), true);
});
