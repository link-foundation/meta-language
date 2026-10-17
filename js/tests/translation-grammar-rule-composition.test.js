import assert from 'node:assert/strict';
import { test } from 'node:test';
import { createHash } from 'node:crypto';
import { runInNewContext } from 'node:vm';
import * as rules from '../src/translation/grammar-rule-composition-rules.js';
import { readGrammarRuleCompositionSource, readGrammarRuleCompositionPolicy, readGrammarRuleCompositionLibraryStyles } from '../scripts/generate-grammar-rule-composition-rules.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { DecoratorSet } from '../src/decorators.js';
import { readPortableSourceParameterStyles, readPortableSourceLintStyles } from '../scripts/portable-source-library-style.mjs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks } from '../src/typed-program-links.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';

test('composition authoring admits only exact syntactic contexts and existing candidate profiles', () => {
  for (const [role, fields] of [['selection', 'kind|item|items'], ['slot', 'kind'],
    ['json-candidate', 'kind|name|mapping'], ['field-candidate', 'kind|name|mapping|selector'], ['', ''], ['future', '']]) {
    assert.equal(rules.readGrammarCompositionFieldOrder(role), fields);
  }
  for (const kind of ['normal', 'token', 'silent', 'external', '']) {
    for (const mode of ['any-grammar', 'all-grammar', 'choice', '']) {
      for (const slot of ['empty', 'literal', 'ref', '']) for (const treeDecorators of [false, true]) {
        const expected = kind !== 'normal' ? 'composition requires a normal syntactic rule'
          : !['any-grammar', 'all-grammar'].includes(mode) ? 'composition requires any-grammar or all-grammar'
          : slot !== 'empty' ? 'composition requires one empty authoring slot'
          : treeDecorators ? 'composition with executor or recovery decorators requires a source-witness proof' : '';
        assert.equal(rules.readGrammarCompositionContextIssue(kind, mode, slot, treeDecorators), expected);
      }
    }
  }
  for (const kind of ['grammar-field-candidate', 'grammar-candidate', 'grammar-range-candidate', 'ref', '']) {
    for (const mapping of ['field-ranges', 'json-string', 'included-ranges', 'identity', '']) {
      assert.equal(rules.acceptGrammarCompositionCandidateProfile(kind, mapping),
        kind === 'grammar-field-candidate' && mapping === 'field-ranges'
          || kind === 'grammar-candidate' && mapping === 'json-string');
    }
  }
  for (const [mappedSources, treeDecorators, expected] of [[false, false, ''], [false, true, ''], [true, false, ''],
    [true, true, 'a mapped grammar with executor or recovery decorators requires a source-witness proof']]) {
    assert.equal(rules.readGrammarCompositionPublicationIssue(mappedSources, treeDecorators), expected);
  }
});

test('composition candidates preserve complete ordered columns and every refusal boundary', () => {
  const selector = '(source-field-selector value include 0 1)';
  const cases = [
    [[], [], [], [], 'composition requires a nonempty complete candidate list'],
    [['grammar-candidate'], [], ['Json'], [''], 'composition requires a nonempty complete candidate list'],
    [['grammar-candidate'], ['json-string'], [], [''], 'composition requires a nonempty complete candidate list'],
    [['grammar-candidate'], ['json-string'], ['Json'], [], 'composition requires a nonempty complete candidate list'],
    [['grammar-candidate'], ['identity'], ['Json'], [''], 'composition uses an unsupported authoring profile'],
    [['grammar-candidate'], ['json-string'], [''], [''], 'composition candidates require nonempty source names'],
    [['grammar-field-candidate'], ['json-string'], ['Json'], [''], 'composition needs its exact candidate form'],
    [['grammar-candidate'], ['field-ranges'], ['Quoted'], [selector], 'composition needs its exact candidate form'],
    [['grammar-field-candidate'], ['field-ranges'], ['Quoted'], [''], 'composition field candidates require one common exact selector'],
    [['grammar-candidate'], ['json-string'], ['Json'], [selector], 'composition JSON candidates require their exact candidate form'],
    [['grammar-candidate', 'grammar-candidate'], ['json-string', 'field-ranges'], ['First', 'Second'], ['', ''], 'composition requires one common source mapping'],
    [['grammar-field-candidate', 'grammar-field-candidate'], ['field-ranges', 'field-ranges'], ['First', 'Second'], [selector, '(source-field-selector value exclude 0 1)'], 'composition field candidates require one common exact selector'],
    [['grammar-candidate', 'grammar-candidate'], ['json-string', 'json-string'], ['Same', 'Same'], ['', ''], 'composition candidate source names collide'],
    [['grammar-field-candidate', 'grammar-field-candidate'], ['field-ranges', 'field-ranges'], ['Same', 'Same'], [selector, selector], 'composition candidate source names collide'],
    [['grammar-candidate'], ['json-string'], ['Json'], [''], ''],
    [['grammar-candidate', 'grammar-candidate'], ['json-string', 'json-string'], ['First', 'Second'], ['', ''], ''],
    [['grammar-field-candidate'], ['field-ranges'], ['Quoted'], [selector], ''],
    [['grammar-field-candidate', 'grammar-field-candidate'], ['field-ranges', 'field-ranges'], ['First', 'Second'], [selector, selector], ''],
  ];
  for (const [kinds, mappings, names, selectors, expected] of cases) {
    const before = JSON.stringify([kinds, mappings, names, selectors]);
    assert.equal(rules.readGrammarCompositionCandidateIssue(kinds, mappings, names, selectors), expected);
    assert.equal(JSON.stringify([kinds, mappings, names, selectors]), before, 'read-only candidate columns are unchanged');
  }
  for (const missing of [new Array(1), ['']]) {
    assert.equal(rules.readGrammarCompositionCandidateIssue(['grammar-candidate'], ['json-string'], missing, ['']),
      'composition candidates require nonempty source names');
    assert.equal(rules.readGrammarCompositionCandidateIssue(missing, ['json-string'], ['Json'], ['']),
      'composition needs its exact candidate form');
  }
});

test('composition repetition compares the exact mode and complete validated ordered candidate key', () => {
  const keys = ['(grammar-candidate Json json-string)', '(grammar-candidate Other json-string)',
    '(grammar-candidate Json json-string) (grammar-candidate Other json-string)',
    '(grammar-candidate Other json-string) (grammar-candidate Json json-string)',
    '(grammar-field-candidate Quoted field-ranges (source-field-selector value include 0 1))',
    '(grammar-field-candidate Quoted field-ranges (source-field-selector value include 1 1))',
    '(grammar-field-candidate Quoted field-ranges (source-field-selector value exclude 0 1))'];
  for (const actualMode of ['', 'any-grammar', 'all-grammar']) for (const requestedMode of ['any-grammar', 'all-grammar']) {
    for (const actual of keys) for (const requested of keys) {
      assert.equal(rules.readGrammarCompositionDisposition(actualMode, actual, requestedMode, requested),
        actualMode === requestedMode && actual === requested ? 1 : 2);
    }
  }
});

test('the complete six-export composition module retains every actual owner through all emitters and both whole-program archives', () => {
  const closed = readGrammarRuleCompositionSource(), policy = readGrammarRuleCompositionPolicy(closed);
  assert.equal(closed.sourceClosure.inputs.length, 1, 'the complete import-free kernel has no omitted reader dependency');
  assert.equal(closed.sourceClosure.inputs[0].source, closed.source);
  assert.equal(closed.sourceClosure.inputs[0].bytes, Buffer.byteLength(closed.source));
  assert.equal(closed.sourceSha256, createHash('sha256').update(closed.source).digest('hex'));
  assert.deepEqual([...Object.keys(rules)].sort(), [...closed.exports].sort());
  assert.deepEqual(policy.decisions, closed.exports);
  const surface = parseJavaScript(closed.source, { referenceSemantics: false }), checked = checkProgram(surface);
  assert.deepEqual(surface.items.filter(item => item.generated !== true).map(item => item.name), closed.exports,
    'all original declarations are retained; compiler-added declarations are discovered separately');
  assert.equal(checked.declarations.size, surface.items.length, 'every original and generated declaration is checked');
  const owners = [...checked.declarations.values()];
  for (const [format, emit] of [['JavaScript', emitJavaScript], ['Rust', emitRust], ['Lean', emitLean], ['Rocq', emitRocq]]) {
    const emitted = emit(checked);
    assert.ok(emitted.text.length > 0, format);
    for (const owner of owners) assert.equal(emitted.mappings.filter(mapping => mapping.source === owner.fullName
      && mapping.kind === (owner.k === 'fn' ? 'function' : owner.k)).length, 1, format + '/' + owner.fullName);
  }
  const javascript = emitJavaScript(checked);
  const samples = [
    ['readGrammarCompositionFieldOrder', ['field-candidate']],
    ['readGrammarCompositionContextIssue', ['normal', 'all-grammar', 'empty', false]],
    ['acceptGrammarCompositionCandidateProfile', ['grammar-field-candidate', 'field-ranges']],
    ['readGrammarCompositionCandidateIssue', [['grammar-candidate'], ['json-string'], ['Json'], ['']]],
    ['readGrammarCompositionCandidateIssue', [['grammar-candidate', 'grammar-candidate'], ['json-string', 'json-string'], ['Json', 'Json'], ['', '']]],
    ['readGrammarCompositionDisposition', ['any-grammar', 'complete ordered key', 'any-grammar', 'complete ordered key']],
    ['readGrammarCompositionPublicationIssue', [true, true]],
    ['readGrammarCompositionPublicationIssue', [true, false]],
  ];
  for (const [name, args] of samples) {
    const owner = checked.declarations.get(name);
    const mapping = javascript.mappings.find(item => item.kind === 'function' && item.source === owner.fullName);
    const actual = runInNewContext(javascript.text + '\n' + mapping.target + '(...' + JSON.stringify(args) + ');', {}, { timeout: 1000 });
    assert.equal(actual, rules[name](...args), 'the complete actual emitted JavaScript executes ' + name);
  }
  const original = emitRust(checked, { arrayConsumption: true });
  const packet = encodeTypedProgramLinks(checked);
  for (const [format, network] of [['text', LinkNetwork.fromLino(packet.network.toLino())],
    ['binary', decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]]) {
    const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
    assert.deepEqual([...restored.declarations.keys()], [...checked.declarations.keys()], 'all complete owner identities survive ' + format);
    const replay = emitRust(restored, { arrayConsumption: true });
    for (const field of ['text', 'definitions', 'preludes', 'mappings', 'assumptions', 'encodings']) {
      assert.deepEqual(replay[field], original[field], 'complete actual Rust ' + field + ' survives ' + format);
    }
  }
});

function compositionLibraryStyleInput() {
  const artifact = readProjectData(new URL('../../parity/self-translation/grammar-rule-composition-rules-native.lino', import.meta.url));
  assert.equal(artifact.source, readGrammarRuleCompositionSource().source, 'the style proof uses the complete actual checked source');
  const emitted = artifact.emitted, header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
  const raw = [header, ...emitted.preludes, ...emitted.definitions].join('\n\n');
  const names = [...raw.matchAll(/^pub (?:fn|enum|struct|mod) (\w+)\b/gmu)].map(match => match[1]);
  const relocation = names.filter(name => raw.includes('crate::' + name))
    .map(name => ({ op: 'replace', field: 'source', from: 'crate::' + name, to: 'self::' + name }));
  const source = new DecoratorSet([{ id: 'composition-style-input', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: relocation }]).decorate('emitter', { format: 'Rust', source: raw }).source;
  const publicNames = [...source.matchAll(/^pub fn (\w+)\(/gmu)].map(match => match[1]);
  return readPortableSourceLintStyles(readPortableSourceParameterStyles(source, { publicNames }).source).source;
}

test('composition library styles retain complete actual owners and authenticate every String default and empty comparison', () => {
  const source = compositionLibraryStyleInput(), styled = readGrammarRuleCompositionLibraryStyles(source);
  const defaults = [...source.matchAll(/\.unwrap_or_else\(\|\|\s*String::new\(\)\)/gu)];
  const comparisons = [...source.matchAll(/\b(?:name|current_selector)\s*(?:==|!=)\s*""/gu)];
  assert.ok(defaults.length > 0 && comparisons.length > 0, 'the complete actual library contains the inspected style boundaries');
  assert.equal([...styled.source.matchAll(/\.unwrap_or_default\(\)/gu)].length,
    [...source.matchAll(/\.unwrap_or_default\(\)/gu)].length + defaults.length);
  assert.equal([...styled.source.matchAll(/\.is_empty\(\)/gu)].length,
    [...source.matchAll(/\.is_empty\(\)/gu)].length + comparisons.length);
  assert.doesNotMatch(styled.source, /\.unwrap_or_else\(\|\|\s*String::new\(\)\)/u);
  assert.doesNotMatch(styled.source, /\b(?:name|current_selector)\s*(?:==|!=)\s*""/u);
  const record = new DecoratorSet([{ id: 'composition-string-style', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: styled.actions }]);
  assert.equal(record.decorate('emitter', { format: 'Rust', source }).source, styled.source);
  let restored = styled.source;
  for (const action of [...styled.actions].reverse()) {
    assert.match(action.from, /^pub fn /u, 'each action retains a complete actual function owner');
    assert.equal(restored.split(action.to).length, 2, 'every actual owner has one inverse');
    restored = restored.replace(action.to, () => action.from);
  }
  assert.equal(restored, source, 'the complete source has a byte-exact style inverse');
  assert.deepEqual(readGrammarRuleCompositionLibraryStyles(styled.source), { source: styled.source, actions: [] });
});

test('composition String styles refuse shadowed readers bindings calls macros and commented expressions', () => {
  const source = compositionLibraryStyleInput();
  for (const refused of ['type String = custom::String;\n' + source, 'use custom::Option;\n' + source,
    source.replace('.cloned()', '.map(|item| item.clone())'),
    source + '\nunknown! { type String = custom::String; }\n']) {
    assert.deepEqual(readGrammarRuleCompositionLibraryStyles(refused), { source: refused, actions: [] });
  }
  const original = readGrammarRuleCompositionLibraryStyles(source).actions.find(action => /let name\s*=/u.test(action.from));
  assert.ok(original, 'use the actual complete owner of both String comparisons');
  for (const body of [
    original.from.replaceAll('self::ml_array::optional_at', 'custom::optional_at'),
    original.from.replaceAll(/\|\|\s*String::new\(\)/gu, '|| unknown()'),
    original.from.replaceAll(/\|\|\s*String::new\(\)/gu, '|| /* keep */ String::new()'),
    original.from.replaceAll(/\bml_result3\b/gu, 'index'),
  ]) {
    const changed = source.replace(original.from, () => body), styled = readGrammarRuleCompositionLibraryStyles(changed);
    assert.ok(styled.source.includes(body), 'the unproved complete owner is left byte-exact');
  }
  const shadowed = original.from.replace(/let name\s*=/u, 'let name = String::from("shadow"); let name =');
  const shadowedSource = source.replace(original.from, () => shadowed);
  const shadowedResult = readGrammarRuleCompositionLibraryStyles(shadowedSource).source;
  assert.match(shadowedResult, /\bname\s*==\s*""/u, 'a shadowed local has no String type proof');
  const comment = '/* .unwrap_or_else(|| String::new()) and name == "" */';
  const literal = '\npub fn unrelated() -> String { String::from("name == \\\"\\\"") }\n';
  const extra = source + '\n' + comment + literal;
  const extraResult = readGrammarRuleCompositionLibraryStyles(extra).source;
  assert.ok(extraResult.endsWith('\n' + comment + literal), 'unrelated literals and comments retain every byte');
});
