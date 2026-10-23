import assert from 'node:assert/strict';
import {restoreSharedArrayNativeFrontendSource} from '../scripts/shared-array-native-frontend-history.mjs';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { readSourceTextRustBlock } from '../scripts/source-text-byte-library-style.mjs';
import { generateRustLiteralMatchRules } from '../scripts/generate-rust-literal-match-rules.mjs';
import { generateRustLiteralMatchHosts, verifyRustLiteralMatchHosts } from '../scripts/generate-rust-literal-match-hosts.mjs';
import { LinkNetwork, LinkType, encodeLinkNetworkBinary, decodeLinkNetworkBinary,
  createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/index.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { parseRust } from '../src/translation/rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { tokenize } from '../src/translation/lexer.js';
import { acceptRustLiteralMatchReceiver, acceptRustLiteralMatchCatalogOwner, planRustLiteralMatchPattern } from '../src/translation/rust-literal-match-rules.js';

const root = new URL('../../', import.meta.url);
const policy = readProjectData(new URL('../src/translation/data/rust-literal-match-rules.lino', import.meta.url));
const digest = source => createHash('sha256').update(source).digest('hex');
const javascriptWords = 'arguments await break case catch class const continue debugger default delete do else enum eval export extends false finally for function if implements import in instanceof interface let new null package private protected public return static super switch this throw true try typeof var void while with yield'.split(' ');
const rustWords = 'as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn'.split(' ');
const reloads = network => [LinkNetwork.fromLino(network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))];
function evaluate(code, name, values, extra = {}) {
  return [...vm.runInNewContext(code + '\n' + JSON.stringify(values) + '.map(value => ' + name + '(value));', extra, { timeout: 1000 })];
}
function nodes(value, predicate) {
  if (!value || typeof value !== 'object') return [];
  return [...(predicate(value) ? [value] : []), ...Object.values(value).flatMap(child => nodes(child, predicate))];
}
function pattern(source) {
  const tokens = tokenize(source, 'Rust').tokens;
  return { tokens, kinds: tokens.map(token => token.kind), values: tokens.map(token => token.value) };
}

test('three complete actual Rust keyword declarations translate with independent keyword observations and both semantic archives', () => {
  assert.equal(policy.originalDeclarations.length, 3);
  for (const owner of policy.originalDeclarations) {
    const context = readFileSync(new URL(owner.path, root), 'utf8');
    assert.equal(context, owner.context, owner.path);
    assert.equal(Buffer.byteLength(context), owner.bytes);
    assert.equal(digest(context), owner.sourceSha256);
    assert.equal(readSourceTextRustBlock(context, 'fn', owner.name), owner.declaration);
    assert.equal(digest(owner.declaration), owner.declarationSha256);
    assert.ok(owner.declaration.startsWith('fn ' + owner.name + '('), 'no const, body, alternative or qualifier is removed');
    const expectedWords = owner.name === 'is_rust_keyword' ? rustWords : javascriptWords;
    const values = [...expectedWords, '', 'π', '😀', 'While', 'keyword', ...expectedWords.map(word => word + '_')];
    const expected = values.map(value => expectedWords.includes(value));
    const source = owner.declaration + '\nfn main(){}';
    const checked = checkProgram(parseRust(source));
    assert.deepEqual(evaluate(emitJavaScript(checked).text, owner.name, values), expected, owner.path);
    const packet = createTypedProgramLinks(source, 'Rust');
    for (const network of reloads(packet.network)) {
      assert.ok(network.links().every(link => !link.metadata().definition?.includes(owner.declaration)), 'canonical semantic links contain no hidden source program');
      const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
      assert.deepEqual(evaluate(emitJavaScript(restored).text, owner.name, values), expected, owner.path);
      assert.deepEqual(evaluate(emitTypedProgramLinks(network, packet.root, 'JavaScript').code, owner.name, values), expected);
      // Reverse the complete emitted JavaScript, then reparse every emitted
      // Rust definition. This is execution of actual translated functions,
      // separate from the concrete-source preservation assertion below.
      const reverse = checkProgram(parseJavaScript(emitJavaScript(restored).text));
      const definitions = emitRust(reverse, { arrayConsumption: true }).definitions.join('\n\n');
      assert.deepEqual(evaluate(emitJavaScript(checkProgram(parseRust(definitions + '\nfn main(){}'))).text, owner.name, values), expected);
    }
  }
});

test('ordinary full Rust CST of each actual declaration keeps all syntax comments whitespace and bytes through text and binary Links', () => {
  for (const owner of policy.originalDeclarations) {
    const source = '// actual declaration: ' + owner.path + '\n' + owner.declaration + '\n';
    const concrete = LinkNetwork.parse(source, 'Rust');
    assert.ok(concrete.links().some(link => link.metadata().term === 'function_item'));
    assert.ok(concrete.links().some(link => link.metadata().term === 'macro_invocation'));
    assert.ok(concrete.links().every(link => !link.metadata().flags.hasError && !link.metadata().flags.isMissing), 'ordinary grammar parsing succeeds without recovery');
    assert.ok(concrete.links().some(link => link.metadata().linkType === LinkType.SourceToken));
    assert.deepEqual(Buffer.from(concrete.reconstructText()), Buffer.from(source));
    for (const restored of reloads(concrete)) assert.deepEqual(Buffer.from(restored.reconstructText()), Buffer.from(source));
  }
});

test('literal matching retains Unicode raw strings delimiter-valued strings leading alternatives and one receiver evaluation', () => {
  const source = String.raw`fn provider(value:&str)->&str{value}
fn selected(value:&str)->bool{matches!(provider(value), | "π" | r#"😀"# | "|" | ")" | ",",)}
fn bracket(value:&str)->bool{matches![value, "π" | "😀"]}
fn brace(value:&str)->bool{matches!{value, "π" | "😀"}}
fn main(){}`;
  const values = ['π', '😀', '|', ')', ',', '', 'other'];
  const program = checkProgram(parseRust(source));
  const selected = program.declarations.get('selected');
  assert.equal(nodes(selected.body, node => node.k === 'call' && node.fn === 'provider').length, 1);
  assert.equal(nodes(selected.body, node => node.k === 'let').length, 1);
  const code = emitJavaScript(program).text.replace('function provider(value) {', 'function provider(value) { counter.calls += 1;');
  const counter = { calls: 0 };
  assert.deepEqual(evaluate(code, 'selected', values, { counter }), [true, true, true, true, true, false, false]);
  assert.equal(counter.calls, values.length, 'failed and successful alternatives evaluate the receiver exactly once');
  for (const name of ['bracket', 'brace']) assert.deepEqual(evaluate(emitJavaScript(program).text, name, values), [true, true, false, false, false, false, false]);
  const nested = 'fn selected(ml_rust_literal_match0:&str)->bool{matches!(ml_rust_literal_match0,"π") && matches!(ml_rust_literal_match0,"π"|"😀")}fn main(){}';
  assert.deepEqual(evaluate(emitJavaScript(checkProgram(parseRust(nested))).text, 'selected', values), [true, false, false, false, false, false, false]);
  const collision = nested.replaceAll('ml_rust_literal_match0', 'rust_literal_match_value0');
  assert.deepEqual(evaluate(emitJavaScript(checkProgram(parseRust(collision))).text, 'selected', values), [true, false, false, false, false, false, false]);
});

test('general patterns guards unknown owners and unrelated macros remain explicit refusals', () => {
  const patterns = ['', '_', 'binding', '"a" if true', '"a" | _', '"a"..="z"', '("a")', 'Some("a")', '"a", "b"', '"a" || "b"', '"a" |'];
  for (const pattern of patterns) assert.throws(() => checkProgram(parseRust('fn selected(value:&str)->bool{matches!(value,' + pattern + ')}fn main(){}')),
    /Rust literal matches!|expected/u, pattern);
  for (const source of [
    'fn selected(value:String)->bool{matches!(value,"a")}fn main(){}',
    'fn selected(value:bool)->bool{matches!(value,"a")}fn main(){}',
    'fn selected(value:&&str)->bool{matches!(value,"a")}fn main(){}',
    'enum str{Value} fn selected(value:&str)->bool{matches!(value,"a")}fn main(){}',
    'enum str{Value} mod nested{fn str()->bool{true}fn selected()->bool{matches!("a","a")}}fn main(){}',
    'fn selected(value:&str)->bool{other!(value,"a")}fn main(){}',
    'macro_rules! matches {()=>{true}} fn selected(value:&str)->bool{matches!(value,"a")}fn main(){}',
  ]) assert.throws(() => checkProgram(parseRust(source)), /outside|Rust literal matches!|unsupported|expected|macro/u, source);
  assert.throws(() => parseRust('const fn selected(value:&str)->bool{matches!(value,"a")}fn main(){}'), /expected/u, 'this batch does not erase existing const qualification boundaries');
});

test('shared pattern planner retains actual token positions and refuses malformed fact boundaries without mutation', () => {
  for (const [source, close] of [['"a"|"😀")', ')'], ['|"|"|r#")"# ,]', ']'], ['""}', '}']]) {
    const input = pattern(source), before = structuredClone(input);
    const plan = planRustLiteralMatchPattern(input.kinds, input.values, 0, close);
    assert.equal(plan[0], input.tokens.length - 1);
    assert.deepEqual(plan.slice(1), input.tokens.flatMap((token, index) => token.kind === 'string' ? [index] : []));
    assert.deepEqual(input, before);
  }
  const input = pattern('"a"|"b")');
  for (const start of [-1, 0.5, NaN, Infinity, -Infinity, input.tokens.length, '0', true]) assert.deepEqual(planRustLiteralMatchPattern(input.kinds, input.values, start, ')'), []);
  assert.deepEqual(planRustLiteralMatchPattern(input.kinds, input.values, 0, ';'), []);
  assert.deepEqual(planRustLiteralMatchPattern(input.kinds.slice(1), input.values, 0, ')'), []);
  for (let index = 0; index < input.tokens.length - 1; index++) {
    const kinds = [...input.kinds]; delete kinds[index];
    const values = [...input.values]; delete values[index];
    assert.deepEqual(planRustLiteralMatchPattern(kinds, input.values, 0, ')'), []);
    assert.deepEqual(planRustLiteralMatchPattern(input.kinds, values, 0, ')'), []);
  }
  assert.equal(acceptRustLiteralMatchCatalogOwner('data'), true);
  for (const kind of ['fn', 'module', 'externalCrate', 'ctor', '', 'bool']) assert.equal(acceptRustLiteralMatchCatalogOwner(kind), false);
  assert.equal(acceptRustLiteralMatchReceiver('string', 'string slice', true, 'reference', false), true);
  for (const args of [['string', 'owned string', true, '', false], ['string', 'string slice', false, 'reference', false],
    ['string', 'string slice', true, 'reference|reference', false], ['string', 'string slice', true, 'reference', true], ['bool', 'string slice', true, 'reference', false]]) {
    assert.equal(acceptRustLiteralMatchReceiver(...args), false);
  }
});

test('source-free literal edits change actual matching behavior after complete semantic rechecking', () => {
  const packet = createTypedProgramLinks('fn selected(value:&str)->bool{matches!(value,"π"|"😀")}fn main(){}', 'Rust');
  const literal = packet.network.links().find(link => link.metadata().definition === 'π');
  assert.ok(literal);
  const changed = LinkNetwork.fromLino(packet.network.toLino().split('\n').map(line => line.startsWith('(' + literal.id() + ': ')
    ? line.replace('(def: %CF%80)', '(def: %CE%B4)') : line).join('\n'));
  assert.notEqual(changed.toLino(), packet.network.toLino());
  for (const network of reloads(changed)) {
    const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
    assert.deepEqual(evaluate(emitJavaScript(restored).text, 'selected', ['π', 'δ', '😀', '']), [false, true, true, false]);
  }
});

test('the entire closed decision source and every generated owner survive automatic native emission and both honest archives', async () => {
  const result = await generateRustLiteralMatchRules({ check: true, native: true });
  assert.equal(result.policy.source, readFileSync(new URL('../src/translation/rust-literal-match-rules.js', import.meta.url), 'utf8'));
  assert.equal(result.artifact.inventory.filter(owner => !owner.generated).length, 3);
  assert.deepEqual(result.artifact.replays.map(replay => replay.format), ['text', 'binary']);
  assert.equal(result.artifact.checkedNamedSourceInventory.length, result.artifact.inventory.length);
});

test('certified complete native hosts replay offline and refuse damaged predecessors edits new sources and registration paths', () => {
  const { archive, files } = generateRustLiteralMatchHosts({ check: true });
  assert.equal(archive.hosts.length, 2);
  assert.equal(archive.additions.length, 2);
  assert.equal(files.length, 4);
  assert.deepEqual(verifyRustLiteralMatchHosts(archive), files);
  // This control owns the original literal-match source; actual successors
  // retain separate complete Git membership and current semantic tests.
  for (const row of files) assert.equal(restoreSharedArrayNativeFrontendSource(readFileSync(new URL(row.path, root), 'utf8'),row.path), row.source);
  const damage = mutate => {
    const changed = structuredClone(archive);
    mutate(changed);
    assert.throws(() => verifyRustLiteralMatchHosts(changed));
  };
  damage(value => { value.hosts[0].source += '\n'; });
  damage(value => { value.coordinateUnit = 'UTF-8 bytes'; });
  damage(value => { value.hosts[1].certificate.revision = '0'.repeat(40); });
  damage(value => { value.hosts[0].edits[0].removed += 'unowned'; });
  damage(value => { value.hosts[1].edits[0].inserted += '\n'; });
  damage(value => { value.additions[0].source += '\n'; });
  damage(value => { value.additions[1].path = 'rust/tests/unit/unregistered.rs'; });
  damage(value => { value.registrations[1].declaration = 'mod unregistered;'; });
});


test('literal matches preserves actual macro alias ownership in every module while function names use their separate namespace', () => {
  const examples = [
    'use crate::foreign::matches;fn selected(value:&str)->bool{matches!(value,"a")}fn main(){}',
    'fn selected(value:&str)->bool{matches!(value,"a")}use crate::foreign::matches;fn main(){}',
    'use crate::foreign::other as matches;fn selected(value:&str)->bool{matches!(value,"a")}fn main(){}',
    'mod nested{use crate::foreign::matches;fn selected(value:&str)->bool{matches!(value,"a")}}fn main(){}',
    'mod nested{fn selected(value:&str)->bool{matches!(value,"a")}use crate::foreign::other as matches;}fn main(){}',
  ];
  for (const source of examples) assert.throws(() => checkProgram(parseRust(source)), /Rust literal matches!/u, source);
  const source = 'fn matches(_value:&str)->bool{false}fn selected(value:&str)->bool{matches!(value,"a")}fn main(){}';
  const checked = checkProgram(parseRust(source));
  assert.deepEqual(evaluate(emitJavaScript(checked).text, 'selected', ['a', 'b']), [true, false]);
  const packet = createTypedProgramLinks(source, 'Rust');
  for (const network of reloads(packet.network)) {
    const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
    assert.deepEqual(evaluate(emitJavaScript(restored).text, 'selected', ['a', 'b']), [true, false]);
  }
});
