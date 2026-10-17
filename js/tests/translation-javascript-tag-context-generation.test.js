// Whole-source JavaScript/Links/Rust-TEXT controls; no native compiler, library
// writer, formatter or generated-file mutation participates in these tests.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import {
  JAVASCRIPT_TAG_CONTEXT_DECISIONS, readJavaScriptTagContextSource,
  readJavaScriptTagContextPolicy, readJavaScriptTagContextOwnerMappings,
  readJavaScriptTagContextNativeParameters, prepareJavaScriptTagContextTranslation,
  readJavaScriptTagContextLibraryStyles,
} from '../scripts/generate-javascript-tag-context-rules.mjs';

test('the complete tag context generation input retains all five exports and the original selector bytes', () => {
  const source = readJavaScriptTagContextSource(), policy = readJavaScriptTagContextPolicy(source);
  assert.equal(policy.source, source);
  assert.equal(policy.sourceSha256, createHash('sha256').update(source).digest('hex'));
  assert.equal(policy.sourceClosureSha256, policy.sourceSha256);
  assert.equal(source, readFileSync(new URL('../src/translation/javascript-tag-context-rules.js', import.meta.url), 'utf8'),
    'the actual current source is returned without any comment or body rewrite');
  assert.equal(policy.originalSelector.bytes, 578);
  assert.equal(createHash('sha256').update(policy.originalSelector.source).digest('hex'),
    '925ebeab62dc79a723dcb6312b422c9574009d09f0f68a9f51e9039d418eb065');
  assert.equal(policy.currentSelector.bytes, 577);
  assert.equal(policy.currentSelector.source, Buffer.from(source).subarray(0, 577).toString('utf8'));
  assert.equal(policy.currentSelector.sha256, '162a9c343561a5206abda6b9c33d0eb7862bdf2e94d6e98e1672f6e251ea11e0');
  assert.deepEqual(policy.selectorCommentTransport.from, '// PRIVATE JS-first selection decision.');
  assert.deepEqual(policy.selectorCommentTransport.to, '// Shared JS-first selection decision.');
  assert.equal(policy.originalSelector.source, policy.selectorCommentTransport.from
    + policy.currentSelector.source.slice(policy.selectorCommentTransport.to.length));
  assert.deepEqual(policy.decisions, ['readJavaScriptTagOwnerPosition', 'readJavaScriptTagReceiverForm',
    'readJavaScriptTagBindingStep', 'acceptJavaScriptTagHeaderFact', 'readJavaScriptTagDeclaredOwner']);
  assert.deepEqual(policy.decisions, [...JAVASCRIPT_TAG_CONTEXT_DECISIONS]);
  assert.match(policy.boundary, /not evidence of native execution/u);
});

test('tag context library styles retain both complete owners Number boundaries and lazy constructor identity', async () => {
  const {DecoratorSet} = await import('../src/decorators.js');
  const {readPortableSourceParameterStyles, readPortableSourceLintStyles} = await import('../scripts/portable-source-library-style.mjs');
  const {readSourceTextRustBlock} = await import('../scripts/source-text-byte-library-style.mjs');
  const {parseRust} = await import('../src/translation/rust.js');
  const {checkProgram} = await import('../src/translation/check.js');
  const {emitJavaScript} = await import('../src/translation/emit-javascript.js');
  const {readJavaScriptTagBindingStep} = await import('../src/translation/javascript-tag-context-rules.js');
  const prepared = await prepareJavaScriptTagContextTranslation(), emitted = prepared.emitted;
  assert.equal(prepared.sourceOwners.length, 5);
  assert.equal(prepared.checkedInventory.length, 7);
  const header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
  assert.ok(header);
  const raw = [header, ...emitted.preludes, ...emitted.definitions].join('\n\n');
  const declarations = new Set([...raw.matchAll(/^pub (?:fn|enum|struct|mod) (\w+)\b/gmu)].map(match => match[1]));
  const relocation = [...declarations].filter(name => raw.includes('crate::' + name))
    .map(name => ({op: 'replace', field: 'source', from: 'crate::' + name, to: 'self::' + name}));
  const relocated = new DecoratorSet([{id: 'tag-context-owner-control', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: relocation}]).decorate('emitter', {format: 'Rust', source: raw}).source;
  const publicNames = [...relocated.matchAll(/^pub fn (\w+)\(/gmu)].map(match => match[1]);
  const reviewed = readPortableSourceLintStyles(readPortableSourceParameterStyles(relocated, {publicNames}).source).source;
  const result = readJavaScriptTagContextLibraryStyles(reviewed);
  assert.equal(result.actions.length, 2);
  assert.deepEqual(result.actions.map(action => /^pub fn (\w+)/u.exec(action.from)[1]),
    ['read_java_script_tag_binding_step', 'read_java_script_tag_declared_owner']);
  const [binding, owner] = result.actions;
  assert.equal(binding.to, binding.from.replace(/^pub fn /u, 'pub const fn '));
  assert.equal(owner.to, owner.from.replace('.unwrap_or_else(|| String::new())', '.unwrap_or_default()'));
  const historical = owner.from.replace('.unwrap_or_else(|| String::new())', '.unwrap_or_else(String::new)');
  assert.equal(readJavaScriptTagContextLibraryStyles(reviewed.replace(owner.from, historical)).source, result.source,
    'the historical function-item input retains the same lazy standard String default');
  const recorded = new DecoratorSet([{id: 'tag-context-library-control', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: result.actions}]);
  assert.equal(recorded.decorate('emitter', {format: 'Rust', source: reviewed}).source, result.source);
  assert.deepEqual(readJavaScriptTagContextLibraryStyles(result.source), {source: result.source, actions: []});
  let inverse = result.source;
  for (const action of [...result.actions].reverse()) {
    assert.equal(inverse.split(action.to).length, 2);
    inverse = inverse.replace(action.to, action.from);
  }
  assert.equal(inverse, reviewed, 'the full library inverse retains all owners, bodies, order and runtime bytes');
  for (const name of [...declarations].filter(name => !['read_java_script_tag_binding_step', 'read_java_script_tag_declared_owner'].includes(name))) {
    const kind = /^Ml/u.test(name) ? 'enum' : name === 'ml_array' ? 'mod' : 'fn';
    assert.equal(readSourceTextRustBlock(result.source, kind, name), readSourceTextRustBlock(reviewed, kind, name));
  }
  const contexts = [binding.from, binding.to].map(source => {
    const ordinary = source.replace(/^pub const fn /u, 'pub fn ');
    assert.equal(source.startsWith('pub const fn ') ? ordinary.replace(/^pub fn /u, 'pub const fn ') : ordinary, source);
    const replay = {};
    vm.runInNewContext(emitJavaScript(checkProgram(parseRust(ordinary + '\nfn main() {}'))).text, replay, {timeout: 1000});
    return replay;
  });
  for (const boundary of [-Infinity, -Number.MAX_VALUE, -1, -Number.MIN_VALUE, -0, 0, Number.MIN_VALUE, 1, Number.MAX_VALUE, Infinity, NaN]) {
    for (const owns of [false, true]) for (const context of contexts) {
      const actual = context.read_java_script_tag_binding_step(owns, boundary);
      assert.ok(Object.is(actual, readJavaScriptTagBindingStep(owns, boundary)), 'complete ordinary Rust body retains each primitive Number boundary');
    }
  }
  for (const counterfeit of [
    binding.from.replace('boundary: f64', 'boundary: &f64'),
    binding.from.replace('boundary: f64', 'boundary: f32'),
    binding.from.replace('0f64', '1f64'),
    binding.from.replace('1f64', 'effect()'),
    binding.from.replace('owns_binding: bool, boundary: f64', 'boundary: f64, owns_binding: bool'),
    binding.from.replace('read_java_script_tag_binding_step', 'other_owner'),
    owner.from.replace('path: &[String]', 'path: &[Other]'),
    owner.from.replace('String::new()', 'effect()'),
    owner.from.replace('|| String::new()', '|| { observe(); String::new() }'),
    owner.from.replace('|| String::new()', '|| /* retained */ String::new()'),
    'type f64=Custom;\n' + binding.from, 'type bool=Custom;\n' + binding.from,
    'struct String;\n' + owner.from, 'type Option<T>=Custom<T>;\n' + owner.from, 'use custom::*;\n' + owner.from,
    'macro_rules! opaque { () => {' + binding.from + '} }',
    '#[custom(' + binding.from + ')]\npub fn untouched() {}',
    binding.from + '\n' + binding.from,
    binding.from + '\npub fn hidden()->&\'static str{r#"' + binding.from + '"#}',
  ]) {
    assert.notEqual(counterfeit, binding.from);
    assert.deepEqual(readJavaScriptTagContextLibraryStyles(counterfeit), {source: counterfeit, actions: []},
      'custom types, captures, effects, duplicate owners and opaque copies cannot borrow a source proof');
  }
});

test('tag context generation refuses original-selector edits dependencies omitted owners and extra declarations', () => {
  const source = readJavaScriptTagContextSource();
  for (const counterfeit of [
    source.replace('owners.length === 1', 'owners.length === 2'),
    source.replace('// Shared JS-first selection decision.', '// Unrecorded JS-first selection decision.'),
    source.replace('// Shared JS-first selection decision.', '// Shared JS-first selection decision.\n// extra comment'),
    source.replace('// Shared JS-first selection decision.', '// PRIVATE JS-first selection decision.'),
    source.slice(0, 578) + "\nimport { hidden } from './foreign.js';\n" + source.slice(578),
    source.replace('function readJavaScriptTagReceiverForm(', 'function counterfeitReceiverForm('),
    source + '\nexport function extraDecision(){return 0;}\n',
    source + '\nfunction unrecordedDecision(){return 0;}\n',
    source + '\nexport function readJavaScriptTagDeclaredOwner(){return "";}\n',
  ]) {
    assert.notEqual(counterfeit, source);
    assert.throws(() => readJavaScriptTagContextSource(counterfeit), assert.AssertionError);
  }
});

test('every checked tag context owner requires its own exact emitted function mapping', () => {
  const inventory = JAVASCRIPT_TAG_CONTEXT_DECISIONS.map(name => ({ kind: 'fn', fullName: name }));
  const mappings = inventory.map((owner, ordinal) => ({ kind: 'function', source: owner.fullName, target: 'actual_' + ordinal }));
  assert.deepEqual(readJavaScriptTagContextOwnerMappings({ mappings }, inventory), mappings);
  for (const counterfeit of [
    mappings.slice(1), [...mappings, mappings[0]],
    mappings.map((row, at) => at ? row : { ...row, kind: 'data' }),
    mappings.map((row, at) => at ? row : { ...row, source: 'foreignOwner' }),
  ]) assert.throws(() => readJavaScriptTagContextOwnerMappings({ mappings: counterfeit }, inventory), assert.AssertionError);
  assert.deepEqual(readJavaScriptTagContextNativeParameters('left: Vec<String>, right: String'), ['left:Vec<String>', 'right:String']);
  assert.deepEqual(readJavaScriptTagContextNativeParameters('left: Vec<String>, right: String,\n'), ['left:Vec<String>', 'right:String']);
  assert.deepEqual(readJavaScriptTagContextNativeParameters(''), []);
  for (const malformed of [',left: String', 'left: String,,right: String', 'left: String, ,right: String', 'left: String,,', ',']) {
    assert.throws(() => readJavaScriptTagContextNativeParameters(malformed), /one optional trailing parameter comma/u);
  }
});

test('the complete five-helper source translates and preserves independent decisions through both whole-program archives', async () => {
  const prepared = await prepareJavaScriptTagContextTranslation();
  assert.equal(prepared.source, readFileSync(new URL('../src/translation/javascript-tag-context-rules.js', import.meta.url), 'utf8'),
    'the complete actual source enters translation unchanged');
  assert.equal(prepared.sourceOwners.length, 5);
  assert.deepEqual(prepared.sourceOwners.map(row => row.fullName), [...JAVASCRIPT_TAG_CONTEXT_DECISIONS]);
  assert.ok(prepared.checkedInventory.length >= 5, 'actual lowering continuations are retained rather than assumed absent');
  assert.ok(prepared.emitted.definitions.length >= 5, 'complete generated definitions are retained without assuming one per export');
  assert.ok(prepared.emitted.text.length > 0);
  assert.deepEqual(prepared.replays.map(row => row.format), ['text', 'binary']);
  const digest = source => createHash('sha256').update(source).digest('hex');
  for (const replay of prepared.replays) {
    assert.deepEqual(replay.checkedInventory, prepared.checkedInventory);
    assert.equal(replay.rustSha256, digest(prepared.emitted.text));
    assert.equal(replay.javascriptSha256, digest(prepared.javascript.text));
  }
  // The actual complete original module executes without extraction. Only its
  // five export keywords are removed for this Script transport; restoring them
  // at their exact original offsets must recover every input source character.
  const actions = [...prepared.source.matchAll(/^export (?=function )/gmu)].map(match => ({ at: match.index, text: match[0] }));
  assert.equal(actions.length, 5);
  let executable = prepared.source;
  for (const action of [...actions].reverse()) executable = executable.slice(0, action.at) + executable.slice(action.at + action.text.length);
  let inverse = executable;
  for (const action of actions) {
    inverse = inverse.slice(0, action.at) + action.text + inverse.slice(action.at);
  }
  assert.equal(inverse, prepared.source, 'only the five exact export transport tokens changed');
  const original = {};
  vm.runInNewContext(executable, original, { timeout: 1000 });
  assert.deepEqual(prepared.javascriptPrograms.map(row => row.format), ['original', 'text', 'binary']);
  const programs = prepared.javascriptPrograms.map(({ format, emission }) => {
    const context = {};
    vm.runInNewContext(emission.text, context, { timeout: 1000 });
    return { format, context, mappings: readJavaScriptTagContextOwnerMappings(emission, prepared.sourceOwners) };
  });
  const expectedCases = [
    [0, [[], 'First'], -1], [0, [['Only'], ''], 0],
    [0, [['First', 'Second'], 'First'], 0], [0, [['Second', 'First'], 'First'], 1],
    [0, [['First', 'First'], 'First'], -1], [0, [['First', 'Second'], ''], -1],
    [0, [['First', 'Second'], 'first'], -1], [0, [['δοκιμή😀', 'Other'], 'δοκιμή😀'], 0],
    [1, ['app', 'name', ['crate', 'make'], 'crate'], 1],
    [1, ['name', '', ['crate', 'make'], 'crate'], 1],
    [1, ['name', '', ['state'], 'crate'], 2],
    [1, ['app', 'name', ['make'], 'crate'], 0],
    [1, ['app', 'functionValue', ['crate', 'make'], 'crate'], 0],
    [1, ['app', 'name', ['crate', 'api', 'make'], 'crate'], 0],
    [1, ['name', '', ['crate', 'api', 'make'], 'crate'], 0],
    [1, ['name', '', ['other', 'make'], 'crate'], 0],
    [1, ['method', 'name', ['crate', 'make'], 'crate'], 0],
    [1, ['name', '', [], 'crate'], 0],
    [2, [true, 0], 1], [2, [true, 1], 1], [2, [true, 2], 1],
    [2, [false, 0], 0], [2, [false, -0], 0], [2, [false, 1], 2],
    [2, [false, 2], 2], [2, [false, -1], 2], [2, [false, NaN], 2], [2, [false, Infinity], 2],
    [3, [false, false, false], false], [3, [false, false, true], false],
    [3, [false, true, false], true], [3, [false, true, true], false],
    [3, [true, false, false], false], [3, [true, false, true], false],
    [3, [true, true, false], false], [3, [true, true, true], false],
    [4, ['data', [], 'First', 'crate'], 'First'],
    [4, ['named', ['crate', 'First'], '', 'crate'], 'First'],
    [4, ['named', ['other', 'First'], '', 'crate'], ''],
    [4, ['named', ['crate', 'api', 'First'], '', 'crate'], ''],
    [4, ['option', ['crate', 'First'], 'First', 'crate'], ''],
    [4, ['union', ['crate', 'First'], 'First', 'crate'], ''],
    [4, ['array', ['crate', 'First'], 'First', 'crate'], ''],
    [4, ['', [], 'First', 'crate'], ''],
    [4, ['named', ['crate', 'δοκιμή😀'], '', 'crate'], 'δοκιμή😀'],
  ];
  for (const [ordinal, args, expected] of expectedCases) {
    const before = structuredClone(args), name = JAVASCRIPT_TAG_CONTEXT_DECISIONS[ordinal];
    assert.equal(original[name](...args), expected, 'independent original decision: ' + name);
    for (const program of programs) assert.equal(program.context[program.mappings[ordinal].target](...args), expected,
      'actual complete emitted decision ' + program.format + ': ' + name);
    assert.deepEqual(args, before, 'actual source values and Number bits are unchanged');
  }
});
