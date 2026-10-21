import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';
import { selfTranslate, selfTranslationSignatures } from '../src/self-translation.js';
import { acceptDeclarationDependencyFacts, readDeclarationDependencyComponents,
  acceptRecursiveDeclarationGroup, acceptRecursiveDeclarationPublication, acceptDeclarationModuleScope } from '../src/translation/declaration-dependency-rules.js';
import { generateDeclarationDependencyRules, readDeclarationDependencySourceClosure } from '../scripts/generate-declaration-dependency-rules.mjs';
import { recursivePairSource, recursivePairCalls, recursivePairExpected,
  readDeclarationDependencyModuleFixtures } from '../scripts/declaration-dependency-fixtures.mjs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { LinkNetwork } from '../src/network.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks } from '../src/typed-program-links.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';

function versions() {
  const [major, minor] = process.versions.node.split('.').map(Number);
  assert.ok(major === 22 && minor >= 15 || major === 24, 'uncovered Node version ' + process.versions.node);
}
function archives(network) {
  return [LinkNetwork.fromLino(network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))];
}
function facts(matrix) {
  const starts = [], counts = [], targets = [];
  for (const row of matrix) {
    starts.push(targets.length);
    for (let target = 0; target < row.length; target += 1) if (row[target]) targets.push(target);
    counts.push(targets.length - starts.at(-1));
  }
  return [starts, counts, targets];
}
// Independent finite transitive-closure oracle, deliberately unlike the two
// iterative DFS passes of the production rule.
function oracle(matrix) {
  const reachable = matrix.map((row, source) => row.map((edge, target) => edge || source === target));
  for (let middle = 0; middle < matrix.length; middle += 1) for (let source = 0; source < matrix.length; source += 1)
    for (let target = 0; target < matrix.length; target += 1)
      reachable[source][target] ||= reachable[source][middle] && reachable[middle][target];
  return matrix.map((_, source) => reachable.findIndex((row, target) => row[source] && reachable[source][target]));
}
function output(source, calls) {
  return runInNewContext(source.replace(/^export /gmu, '') + '\n[' + calls.join(',') + '].map(String).join("|");', {}, { timeout: 1000 });
}

const pair = recursivePairSource;
const calls = recursivePairCalls;
const expected = recursivePairExpected;

test('all complete shared module fixtures retain independent output, refusal and exact source expectations', () => {
  versions();
  for (const fixture of readDeclarationDependencyModuleFixtures()) {
    const translated = selfTranslate(fixture.source, fixture.language, 'Rust');
    assert.deepEqual(translated.items.filter(item => ['function_declaration', 'export_statement'].includes(item.term)).map(item => item.status),
      fixture.functionStatuses, fixture.id);
    assert.deepEqual(selfTranslationSignatures(fixture.source, fixture.language).map(signature => signature.name), fixture.signatures, fixture.id);
    assert.equal(output(fixture.source, fixture.calls).replaceAll('|', '\n') + '\n', fixture.expectedLines, fixture.id);
    assert.equal(selfTranslate(translated.code, 'Rust', fixture.language).code, fixture.source, fixture.id);
    if (fixture.sourceMode === 'Script') {
      assert.doesNotMatch(fixture.source, /\bexport\b/u, 'the original valid var-rebinding Script is not converted from an invalid ES module');
      assert.equal(runInNewContext(fixture.source + '\nfirst;'), fixture.sourceOwnerValue);
      assert.throws(() => runInNewContext(fixture.source + '\nsecond(0);'), error => error.name === fixture.sourceOwnerCallError);
      assert.doesNotMatch(translated.code, /pub fn first\(|pub fn second\(/u);
    }
  }
});

test('dependency rules preserve complete source and independently partition every three-node graph', async () => {
  versions();
  await generateDeclarationDependencyRules({ check: true, native: true });
  assert.deepEqual(readDeclarationDependencyComponents([], [], []), []);
  for (let mask = 0; mask < 512; mask += 1) {
    const matrix = Array.from({ length: 3 }, (_, source) => Array.from({ length: 3 }, (_, target) => Boolean(mask & 1 << (source * 3 + target))));
    const input = facts(matrix);
    assert.equal(acceptDeclarationDependencyFacts(...input), true);
    assert.deepEqual(readDeclarationDependencyComponents(...input), oracle(matrix), 'graph=' + mask);
  }
  for (const input of [[[], [0], []], [[1], [0], []], [[0], [-1], []], [[0], [0.5], []],
    [[0], [1], [-1]], [[0], [1], [1]], [[0], [1], [0.5]], [[0], [1], [NaN]], [[0], [1], [Infinity]], [[0], [0], [0]]]) {
    assert.equal(acceptDeclarationDependencyFacts(...input), false);
    assert.throws(() => readDeclarationDependencyComponents(...input), /invalid declaration dependency facts/u);
  }
  const sparse = [0]; delete sparse[0];
  assert.throws(() => readDeclarationDependencyComponents(sparse, [0], []), /missing required Number fact/u);
  assert.equal(acceptRecursiveDeclarationGroup(2, true, true), true);
  for (const input of [[1, true, true], [2.5, true, true], [Infinity, true, true], [NaN, true, true],
    [2, false, true], [2, true, false]]) assert.equal(acceptRecursiveDeclarationGroup(...input), false);
  assert.equal(acceptRecursiveDeclarationPublication(2, 2, 2, true, true, true), true);
  for (const input of [[2, 1, 2, true, true, true], [2, 2, 1, true, true, true],
    [2, 2, 2, false, true, true], [2, 2, 2, true, false, true], [2, 2, 2, true, true, false]])
    assert.equal(acceptRecursiveDeclarationPublication(...input), false);
  assert.equal(acceptDeclarationModuleScope(['comment', 'named function declaration', 'named import', 'primitive literal binding'], false), true);
  for (const form of ['var declaration', 'assignment', 'eval', 'namespace import', 'const binding list', 'class declaration', 'unknown module form']) {
    assert.equal(acceptDeclarationModuleScope(['named function declaration', form], false), false, form);
    assert.equal(acceptDeclarationModuleScope(['named function declaration', form], true), true, form + ' requires the real complete checker');
  }
});

test('the complete shared algorithm survives both semantic codecs, independent rechecking and actual emission', () => {
  versions();
  const source = readDeclarationDependencySourceClosure().source;
  const checked = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  const saved = encodeTypedProgramLinks(checked);
  const rust = emitRust(checked, { arrayConsumption: true }).text;
  for (const network of archives(saved.network)) {
    const program = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, saved.root), { wholeProgram: true });
    assert.equal(emitRust(program, { arrayConsumption: true }).text, rust);
    const api = runInNewContext(emitJavaScript(program).text + '\n({readDeclarationDependencyComponents});', {}, { timeout: 1000 });
    for (let mask = 0; mask < 512; mask += 1) {
      const matrix = Array.from({ length: 3 }, (_, source) => Array.from({ length: 3 }, (_, target) => Boolean(mask & 1 << (source * 3 + target))));
      assert.deepEqual(Array.from(api.readDeclarationDependencyComponents(...facts(matrix))), oracle(matrix), 'codec graph=' + mask);
    }
    for (const input of [[[0], [1], [2]], [[0], [0], [0]]])
      assert.throws(() => api.readDeclarationDependencyComponents(...input), /invalid declaration dependency facts/u);
  }
});

test('complete cyclic bodies execute, keep mutual recursion through both codecs and restore exact source', () => {
  versions();
  assert.equal(output(pair, calls), expected);
  const checked = checkProgram(parseJavaScript(pair));
  assert.equal(checked.declarations.get('isEven').mutual, true);
  assert.equal(checked.declarations.get('isOdd').mutual, true);
  const saved = encodeTypedProgramLinks(checked);
  for (const network of archives(saved.network)) {
    const replay = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, saved.root), { wholeProgram: true });
    assert.equal(output(emitJavaScript(replay).text, calls), expected);
    assert.equal(emitRust(replay, { arrayConsumption: true }).text, emitRust(checked, { arrayConsumption: true }).text);
    replay.declarations.get('isOdd').body.type = { kind: 'string' };
    assert.throws(() => recheckLinkedTypedProgram(replay, { wholeProgram: true }));
  }
  for (const source of [pair, '\r\n// recursive café\r\n' + pair.replaceAll('\n', '\r\n') + '\r\n']) {
    const translated = selfTranslate(source, 'JavaScript', 'Rust');
    assert.ok(translated.items.every(item => item.status === 'translated'));
    assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, source);
    assert.deepEqual(selfTranslationSignatures(source, 'JavaScript').map(item => item.name), ['isEven', 'choose']);
    for (const network of archives(LinkNetwork.parse(source, 'JavaScript'))) assert.equal(network.reconstructText(), source);
  }
});

test('noncontiguous recursive declarations do not absorb unrelated unsupported bodies or their dependants', () => {
  versions();
  const marker = '/** @param {number} value @returns {boolean} */\nfunction isOdd';
  assert.equal(pair.split(marker).length, 2);
  const unavailable = 'function unavailable() { return new Date(); }\n';
  const source = pair.replace(marker, unavailable + marker)
    + 'export function independent() { return 17; }\nexport function blocked() { return unavailable(); }\n';
  const translated = selfTranslate(source, 'JavaScript', 'Rust');
  const functions = translated.items.filter(item => ['function_declaration', 'export_statement'].includes(item.term));
  assert.deepEqual(functions.map(item => item.status), ['translated', 'carried', 'translated', 'translated', 'translated', 'carried']);
  assert.deepEqual(selfTranslationSignatures(source, 'JavaScript').map(item => item.name), ['isEven', 'choose', 'independent']);
  assert.match(translated.code, /pub fn is_even\(/u);
  assert.match(translated.code, /pub fn is_odd\(/u);
  assert.doesNotMatch(translated.code, /pub fn unavailable\(|pub fn blocked\(/u);
  assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, source);
  assert.equal(output(source, calls), expected);
  const edited = translated.code.replace('17f64', '19f64');
  assert.notEqual(edited, translated.code);
  assert.notEqual(selfTranslate(edited, 'Rust', 'JavaScript').code, source);
});

test('failed complete cycles publish no provisional signatures while independent functions retain their proofs', () => {
  versions();
  for (const second of ['if(value <= 0) return new Date(); return first(value - 1);',
    'if(value <= 0) return "wrong"; return first(value - 1);']) {
    const source = 'export function first(value) { if(value <= 0) return true; return second(value - 1); }\n'
      + 'function second(value) { ' + second + ' }\n'
      + 'export function caller(value) { return first(value); }\n'
      + 'export function independent() { return 23; }\n';
    const translated = selfTranslate(source, 'JavaScript', 'Rust', { retainDiagnostics: true });
    assert.deepEqual(translated.items.filter(item => ['function_declaration', 'export_statement'].includes(item.term)).map(item => item.status),
      ['carried', 'carried', 'carried', 'translated']);
    assert.deepEqual(selfTranslationSignatures(source, 'JavaScript').map(item => item.name), ['independent']);
    assert.doesNotMatch(translated.code, /pub fn first\(|pub fn second\(|pub fn caller\(/u);
    assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, source);
  }
});

test('module assignment and eval cannot be omitted between or after complete recursive declarations', () => {
  versions();
  const first = 'function first(value) { if(value <= 0) return true; return second(value - 1); }\n';
  const second = 'function second(value) { return first(value); }\n';
  for (const effect of ['first = 0;\n', 'eval("first = 0");\n']) for (const position of ['between', 'after']) {
    const source = first + (position === 'between' ? effect : '') + second
      + (position === 'after' ? effect : '') + 'function independent() { return 31; }\n';
    assert.equal(runInNewContext(source + '\nfirst;'), 0);
    assert.throws(() => runInNewContext(source + '\nsecond(0);'), error => error.name === 'TypeError');
    const translated = selfTranslate(source, 'JavaScript', 'Rust');
    assert.doesNotMatch(translated.code, /pub fn first\(|pub fn second\(/u);
    assert.match(translated.code, /pub fn independent\(/u);
    assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, source);
  }
});

test('a safe wider output module retains the actual complete-check route to its original cyclic bodies', () => {
  versions();
  const source = pair + 'console.log(choose(6));\n';
  const lines = [];
  runInNewContext(source.replace(/^export /gmu, ''), { console: { log: value => lines.push(String(value)) } });
  assert.deepEqual(lines, ['true']);
  const translated = selfTranslate(source, 'JavaScript', 'Rust');
  assert.deepEqual(translated.items.filter(item => ['function_declaration', 'export_statement'].includes(item.term)).map(item => item.status),
    ['translated', 'translated', 'translated']);
  assert.deepEqual(selfTranslationSignatures(source, 'JavaScript').map(signature => signature.name), ['isEven', 'choose']);
  assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, source);
});

test('ordinary translated named-import signatures bind an external dependency without provisional peers', () => {
  versions();
  const dependency = '/** @param {number} value @returns {number} */\nexport function lower(value) { return value - 1; }\n';
  const source = 'import { lower as step } from "./dependency.js";\n'
    + 'export function left(value) { if(value <= 0) return true; return right(step(value)); }\n'
    + 'function unavailable() { return new Date(); }\n'
    + 'function right(value) { if(value <= 0) return false; return left(step(value)); }\n';
  const options = { imports: { './dependency.js': selfTranslationSignatures(dependency, 'JavaScript') } };
  const translated = selfTranslate(source, 'JavaScript', 'Rust', options);
  assert.deepEqual(translated.items.filter(item => ['function_declaration', 'export_statement'].includes(item.term)).map(item => item.status),
    ['translated', 'carried', 'translated']);
  assert.deepEqual(selfTranslationSignatures(source, 'JavaScript', options).map(signature => signature.name), ['left']);
  assert.match(translated.code, /use crate::dependency::lower as step;/u);
  assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, source);
});

test('the actual complete typed callable statement traversal supplies a source-derived own-code candidate', () => {
  versions();
  const source = readFileSync(new URL('../src/translation/callable-source-rules.js', import.meta.url), 'utf8');
  assert.match(source, /export function readCallableStatementUses\(/u);
  assert.match(source, /function callableStatementUses\(/u);
  assert.match(source, /const next = callableStatementUses\(/u);
  assert.match(source, /readCallableStatementUses\(forms, texts, starts, counts, labels, targets, callableField/u);
  // This records actual complete-source candidate ownership only. The native
  // author/checker tests cover semantics; no new whole-module gain is claimed.
  for (const network of archives(LinkNetwork.parse(source, 'JavaScript'))) assert.equal(network.reconstructText(), source);
});
