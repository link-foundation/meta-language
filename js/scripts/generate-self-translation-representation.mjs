// Preserve the existing representation policy and emitted runtime as shared data.
// Native policy generation is an explicit second phase after the JS batch.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync, readFileSync, writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {STRING_PRELUDE} from '../src/translation/string-runtime.js';
import {checkProjectData, readProjectData, writeProjectData} from './project-data.mjs';
import {applyCompleteExpressionParenthesisStyles} from './complete-expression-parenthesis-styles.mjs';

const policyPath = new URL('../src/data/self-translation-representation.lino', import.meta.url);
const nativePolicyPath = new URL('../../rust/src/data/self-translation-representation.lino', import.meta.url);
const rulePath = new URL('../src/self-translation-representation-rules.js', import.meta.url);
const selectorPath = new URL('../src/self-translation.js', import.meta.url);
const digest = value => createHash('sha256').update(value).digest('hex');

function originalSelector() {
  if (existsSync(policyPath)) return readProjectData(policyPath).originalSelector;
  const source = readFileSync(selectorPath, 'utf8');
  const selector = source.match(/const stringMethods = new Set\(\[[^\n]+\]\);/u)?.[0];
  assert.ok(selector, 'initial extraction requires the actual pre-extraction selector');
  return {source: selector, sha256: digest(selector)};
}

export function selfTranslationRepresentationPolicy() {
  const selector = originalSelector();
  assert.equal(digest(selector.source), selector.sha256);
  assert.match(selector.source, /^const stringMethods = new Set\(\[(?:'[^']+'(?:, )?)+\]\);$/u);
  const memberNames = [...selector.source.matchAll(/'([^']+)'/gu)].map(match => match[1]);
  assert.equal(new Set(memberNames).size, memberNames.length);
  const additionalMemberNames = ['charCodeAt'];
  assert.equal(new Set([...memberNames, ...additionalMemberNames]).size, memberNames.length + additionalMemberNames.length);
  return {
    schemaVersion: 1,
    model: 'shared self translation UTF16 representation 1',
    originalSelector: selector,
    memberNames,
    additionalMemberNames,
    selection: 'An explicit request, or a JavaScript-family module containing an actual member_expression property with one of these exact source spellings. Receiver types do not change this retained syntax policy.',
    nativeBoundary: 'Representation selection and exact emitted prelude bytes do not establish native Text lowering. Actual selected String values require independently paired rendering support.',
    prelude: STRING_PRELUDE,
    preludeSha256: digest(STRING_PRELUDE),
  };
}

function rules(policy) {
  return '// Generated from the shared self-translation representation policy. Do not edit.\n'
    + '/** @param {string} name @returns {boolean} */\n'
    + 'export function acceptSelfTranslationUtf16Member(name) {\n  return '
    + [...policy.memberNames, ...policy.additionalMemberNames].map(name => `name === ${JSON.stringify(name)}`).join(' || ') + ';\n}\n\n'
    + '/** @param {boolean} explicit @param {string} family @param {boolean} member @returns {boolean} */\n'
    + "export function requireSelfTranslationUtf16Representation(explicit, family, member) {\n  return explicit || (family === 'JavaScript' && member);\n}\n\n"
    + '/** @param {string} kind @returns {boolean} */\n'
    + "export function requireSelfTranslationUtf16Text(kind) {\n  return kind === 'string';\n}\n";
}

export function generateSelfTranslationRepresentation(check = false) {
  const policy = selfTranslationRepresentationPolicy(), source = rules(policy);
  if (check) {
    checkProjectData(policyPath, policy);
    checkProjectData(nativePolicyPath, policy);
    assert.equal(readFileSync(rulePath, 'utf8'), source, 'representation rules are stale');
  } else {
    writeProjectData(policyPath, policy);
    writeProjectData(nativePolicyPath, policy);
    writeFileSync(rulePath, source);
  }
  return {policy, source};
}

export async function generateSelfTranslationRepresentationNative(check = false, updateStyles = false, updateCompleteStyles = false) {
  assert.ok(!check || !(updateStyles || updateCompleteStyles), 'freshness cannot modify recorded styles');
  const {source} = generateSelfTranslationRepresentation(check);
  const [{parseJavaScript}, {checkProgram}, {emitRust}, {DecoratorSet}, {tokenize}, {formatProductRuleSource}] = await Promise.all([
    import('../src/translation/javascript.js'), import('../src/translation/check.js'), import('../src/translation/emit-rust.js'),
    import('../src/decorators.js'), import('../src/translation/lexer.js'), import('./product-rule-style-corrections.mjs'),
  ]);
  const program = checkProgram(parseJavaScript(source)), emitted = emitRust(program);
  assert.equal(program.items.length, 3);
  assert.equal(emitted.definitions.length, 3);
  const raw = emitted.definitions.join('\n\n');
  const stylePath = new URL('../../parity/self-translation/self-translation-representation-decorators.lino', import.meta.url);
  if (updateStyles) {
    let current = raw;
    const actions = [];
    const replace = (from, to) => {
      assert.ok(current.includes(from), 'a representation library style lost its source');
      actions.push({op: 'replace', field: 'source', from, to}); current = current.split(from).join(to);
    };
    for (const name of ['name', 'family', 'kind']) replace(`${name}: String`, `${name}: &str`);
    // All generated string values occur solely as exact comparison operands.
    for (const match of [...new Set(current.match(/String::from\("[^"\\]*"\)/gu))]) {
      replace(match, match.slice('String::from('.length, -1));
    }
    // Remove only parentheses wrapping an entire block's one return value.
    // No interior operator grouping, branch or scope is changed.
    for (;;) {
      const {tokens} = tokenize(current, 'Rust'), stack = [], matching = new Map();
      for (const [index, token] of tokens.entries()) if (token.kind === 'punct') {
        if (['(', '[', '{'].includes(token.value)) stack.push(index);
        if ([')', ']', '}'].includes(token.value)) matching.set(stack.pop(), index);
      }
      assert.equal(stack.length, 0);
      const index = tokens.findIndex((token, index) => token.kind === 'punct' && token.value === '(' && tokens[index - 1]?.value === '{' && tokens[matching.get(index) + 1]?.value === '}');
      if (index < 0) break;
      const close = tokens[matching.get(index)];
      replace(current.slice(tokens[index].start, close.end), current.slice(tokens[index].end, close.start));
    }
    for (const name of ['accept_self_translation_utf16_member', 'require_self_translation_utf16_representation', 'require_self_translation_utf16_text']) {
      replace(`pub fn ${name}(`, `/// Apply the retained shared UTF16 representation decision.\n#[must_use]\npub fn ${name}(`);
    }
    writeFileSync(stylePath, new DecoratorSet([{id: 'self-translation-representation-library', level: 'emitter', order: 0, when: [['format', 'Rust']], actions}]).toLino());
  }
  const document = readFileSync(stylePath, 'utf8');
  const decorators = DecoratorSet.fromLino(document);
  let reviewed = raw;
  for (const entry of decorators.forLevel('emitter').filter(({when}) => !when.some(([field]) => field === 'scope'))) for (const action of entry.actions) {
    assert.equal(action.op, 'replace'); assert.equal(action.field, 'source'); assert.ok(reviewed.includes(action.from));
    reviewed = reviewed.split(action.from).join(action.to);
  }
  const decorated = decorators.decorate('emitter', {format: 'Rust', source: raw}).source;
  assert.equal(decorated, reviewed);
  const correction = applyCompleteExpressionParenthesisStyles(document, formatProductRuleSource(decorated), {
    update: updateStyles || updateCompleteStyles, format: formatProductRuleSource,
  });
  if (correction.document !== document) writeFileSync(stylePath, correction.document);
  const native = '// Automatically translated from the complete shared representation policy. Do not edit.\n' + correction.source;
  const path = new URL('../../rust/src/self_translation/representation_rules.rs', import.meta.url);
  if (check) assert.equal(readFileSync(path, 'utf8'), native, 'native representation rules are stale');
  else writeFileSync(path, native);
  return native;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const check = process.argv.includes('--check');
  generateSelfTranslationLayoutRules({check});
  if (process.argv.includes('--native')) {
    console.log('Native shared representation rules: ' + Buffer.byteLength(await generateSelfTranslationRepresentationNative(check, process.argv.includes('--update-styles'), process.argv.includes('--update-complete-expression-styles'))) + ' bytes');
  } else {
    const {policy} = generateSelfTranslationRepresentation(check);
    console.log(`Shared self-translation representation: ${policy.memberNames.length} retained members; exact prelude ${Buffer.byteLength(policy.prelude)} bytes`);
  }
}

import {generateSelfTranslationLayoutRules} from './self-translation-layout-generation.mjs';
