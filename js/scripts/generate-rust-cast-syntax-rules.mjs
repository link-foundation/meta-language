// Generate the shared Rust cast/type-path boundary and its recorded fixture correction.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { DecoratorSet } from '../src/decorators.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { simplifySourceTextRustParentheses } from './source-text-byte-library-style.mjs';

const originalSource = 'fn bounded(values:&[f64])->bool { values.len() as f64 < 3f64 } fn main(){}';
const originalSourceSha256 = '73a6ec2aba8875929a9d2a1f50e12b27eb22b75783df8a50a5feac8839b48ba2';
assert.equal(createHash('sha256').update(originalSource).digest('hex'), originalSourceSha256);
const policy = {
  schemaVersion: 1,
  model: 'Rust cast type path comparison boundary 1',
  sources: [
    'https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions',
    'https://doc.rust-lang.org/reference/paths.html#paths-in-types',
    'https://github.com/rust-lang/rust/issues/106413',
  ],
  ambiguousFollowingToken: '<',
  parenthesizedTypeStart: '(',
  reason: 'a cast before < requires parentheses',
  interpretation: 'A less-than token directly after an unparenthesized cast type path starts generic arguments, not comparison. A parenthesized target type is already delimited. A grouped cast expression closes before the comparison and therefore also retains its existing admission.',
  scope: 'Existing admitted Rust cast types only. Other type, cast, ownership and expression refusals remain independent. In particular this rule does not add pointer casts or generic targets.',
  sourceCorrection: {
    fixtureId: 'rust-array-source',
    caseId: 'slice-number-length',
    invalidId: 'slice-number-length-original-unparenthesized-cast',
    originalSource,
    originalSourceSha256,
    castSource: 'values.len() as f64',
    decoratorId: 'rust-array-source-cast-value-grouping',
    decoratorFile: 'parity/fixtures/rust-array-source-corrections.lino',
    interpretation: 'Retain this exact historical source as a negative. The same positive case and all its inputs and expected observations use only the recorded grouping decorator before parsing or native execution; the compiler harness never substitutes source.',
  },
};
const policyFile = new URL('../src/translation/data/rust-cast-syntax.lino', import.meta.url);
const copy = new URL('../../rust/src/data/rust-cast-syntax.lino', import.meta.url);
const check = process.argv.includes('--check');
if (check) {
  checkProjectData(policyFile, policy);
  assert.ok(readFileSync(copy).equals(readFileSync(policyFile)), 'both runtimes share exact cast syntax policy bytes');
} else {
  writeProjectData(policyFile, policy);
  writeFileSync(copy, readFileSync(policyFile));
}

const source = '// Generated from the linked Rust cast syntax policy. Do not edit.\n'
  + '/** @param {string} followingToken @param {boolean} parenthesizedType @returns {boolean} */\n'
  + 'export function acceptRustCastFollowingToken(followingToken, parenthesizedType) {\n'
  + `  return followingToken !== ${JSON.stringify(policy.ambiguousFollowingToken)} || parenthesizedType;\n}\n`
  + '/** @returns {string} */\n'
  + 'export function readRustCastSyntaxMessage() {\n'
  + `  return ${JSON.stringify(policy.reason)};\n}\n`;
const javascript = new URL('../src/translation/rust-cast-syntax-rules.js', import.meta.url);
if (check) assert.equal(readFileSync(javascript, 'utf8'), source, 'Rust cast JavaScript rules are fresh');
else writeFileSync(javascript, source);

const correction = policy.sourceCorrection;
assert.equal(originalSource.split(correction.castSource).length, 2, 'one actual historical cast owns the correction');
const correctedSource = originalSource.replace(correction.castSource, '(' + correction.castSource + ')');
const fixtureStyles = new DecoratorSet([{
  id: correction.decoratorId, level: 'transformation', order: 0,
  when: [['fixture', correction.fixtureId], ['case', correction.caseId], ['format', 'Rust']],
  actions: [{ op: 'replace', field: 'source', from: originalSource, to: correctedSource }],
}]);
const fixtureStyleFile = new URL('../../' + correction.decoratorFile, import.meta.url);
if (check) assert.equal(readFileSync(fixtureStyleFile, 'utf8'), fixtureStyles.toLino(), 'the exact fixture grouping decorator is fresh');
else writeFileSync(fixtureStyleFile, fixtureStyles.toLino());

if (!process.argv.includes('--javascript-only')) {
  const { parseJavaScript } = await import('../src/translation/javascript.js');
  const { checkProgram } = await import('../src/translation/check.js');
  const { emitRust } = await import('../src/translation/emit-rust.js');
  const definitions = emitRust(checkProgram(parseJavaScript(source))).definitions;
  assert.equal(definitions.length, 2, 'both complete checked declarations are translated');
  const raw = definitions.join('\n\n');
  const actions = [
    { op: 'replace', field: 'source', from: 'following_token: String', to: 'following_token: &str' },
    { op: 'replace', field: 'source', from: 'following_token.clone()', to: 'following_token' },
    { op: 'replace', field: 'source', from: 'String::from("<")', to: '"<"' },
  ];
  for (const name of ['accept_rust_cast_following_token', 'read_rust_cast_syntax_message']) {
    assert.ok(raw.includes('pub fn ' + name + '('), 'each generated declaration keeps its actual name');
    actions.push({ op: 'replace', field: 'source', from: 'pub fn ' + name + '(', to: '/// Read the shared Rust cast syntax boundary.\n#[must_use]\npub fn ' + name + '(' });
  }
  const intermediate = new DecoratorSet([{ id: 'rust-cast-syntax-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }])
    .decorate('emitter', { format: 'Rust', source: raw }).source;
  const grouped = simplifySourceTextRustParentheses(intermediate);
  if (grouped !== intermediate) actions.push({ op: 'replace', field: 'source', from: intermediate, to: grouped });
  const styles = new DecoratorSet([{ id: 'rust-cast-syntax-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]);
  const styleFile = new URL('../../parity/self-translation/rust-cast-syntax-rules-decorators.lino', import.meta.url);
  if (check) assert.equal(readFileSync(styleFile, 'utf8'), styles.toLino(), 'Rust cast library styles are fresh');
  else writeFileSync(styleFile, styles.toLino());
  const adapted = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
  assert.ok(adapted.includes('following_token: &str'), 'the native host borrows the actual token spelling');
  assert.ok(!adapted.includes('following_token.clone()'), 'no owned string clone remains after the borrowed adapter');
  const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: adapted, encoding: 'utf8' });
  if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
  const native = '// Generated from the checked JavaScript cast syntax decisions. Do not edit.\n' + formatted.stdout;
  const nativeFile = new URL('../../rust/src/translation/rust/cast_syntax_rules.rs', import.meta.url);
  if (check) assert.equal(readFileSync(nativeFile, 'utf8'), native, 'Rust cast native decisions are fresh');
  else writeFileSync(nativeFile, native);
}
console.log('Rust cast syntax: shared token boundary and exact archived source correction');
