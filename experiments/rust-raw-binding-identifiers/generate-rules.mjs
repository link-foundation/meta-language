// Experimental JavaScript-first interpretation; no public lexer or selector.
import assert from 'node:assert/strict';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData, checkProjectData } from '../../js/scripts/project-data.mjs';
import { parseJavaScript } from '../../js/src/translation/javascript.js';
import { checkProgram } from '../../js/src/translation/check.js';
import { emitRust } from '../../js/src/translation/emit-rust.js';
import { DecoratorSet } from '../../js/src/decorators.js';
import { formatProductRuleSource } from '../../js/scripts/product-rule-style-corrections.mjs';
import { stripRustSourceHeaderStyleParentheses } from '../../js/scripts/rust-source-header-library-style.mjs';

const units = readProjectData(new URL('../../js/src/translation/data/rust-pure-unit-calls.lino', import.meta.url));
export const rustRawBindingIdentifierPolicy = {
  schemaVersion: 1,
  model: 'Rust raw binding identifier interpretation 1',
  language: 'Rust',
  reference: 'https://doc.rust-lang.org/reference/identifiers.html#raw-identifiers',
  scopeReference: 'https://doc.rust-lang.org/reference/names/scopes.html',
  characterOperation: 'acceptRustPureUnitNameCharacter',
  characterSource: 'js/src/translation/rust-pure-unit-call-rules.js',
  characterProjection: 'The host projects every actual name character through the existing shared ASCII predicate; the pure name selector consumes that exact Boolean result.',
  rawPrefix: ['r', '#'],
  forbiddenNames: ['_', 'crate', 'self', 'super', 'Self'],
  ordinaryKeywords: units.forbiddenNames.filter(name => name !== 'main'),
  keywordSource: units.keywordSource,
  edition: units.edition,
  types: ['i32', 'bool'],
  primitiveKinds: ['fixed signed 32', 'bool'],
  aliases: { declarationPrefix: 'sourceFunction', parameterPrefix: 'sourceParameter' },
  fields: {
    proof: ['model', 'language', 'span', 'tokens', 'gaps', 'aliases', 'occurrences', 'worker'],
    alias: ['kind', 'value', 'checkedOwner', 'token'],
    occurrence: ['token', 'alias', 'checkedExpression'],
  },
  grammar: [
    'Complete module: one or more flat optional-pub value functions followed by exactly ordinary fn main(){}.',
    'Functions have zero or more named i32 or bool parameters and an explicit i32 or bool result.',
    'A body is a primitive parameter value, nonnegative i32 literal, Boolean literal, parenthesized value or direct unqualified declared call with such arguments.',
    'There are no imports, attributes, local bindings, nested blocks, operators, other source profiles or unrepresented whole-source operations.',
    'Self and mutual recursion refuse using the existing complete core recursion analysis, rather than a second call-graph algorithm.',
    'Raw identifiers are the exact contiguous production tokens r, # and one ASCII identifier; this experiment changes no production token path.',
  ],
  meaning: [
    'The raw prefix is an original spelling, not part of the semantic identifier: r#foo and foo resolve to the same name in the same scope.',
    'Declaration and parameter aliases are deterministic distinct temporary parser names; every inverse maps to an actual owned checked declaration or parameter object.',
    'Parameters shadow function items in their function body. Duplicate canonical declaration names and duplicate canonical parameters refuse.',
    'The original complete CST remains inspectable as tokens and exact trivia units. Edited owner links produce fresh source spellings and a newly checked complete interpretation.',
    'Every declaration and variable occurrence retains both its actual alias record and its actual checked call or variable expression identity in Links archives.',
    'Observation is over well-typed source invocations: primitive Boolean inputs and signed 32-bit integer inputs, represented as BigInt in emitted JavaScript. No Number arithmetic or overflow interpretation is introduced.',
    'Only explicit experimental entry points validate this provenance model; ordinary public typed-program decoding alone does not establish this source interpretation.',
  ],
};
export function rustRawBindingIdentifierRules() {
  const policy = rustRawBindingIdentifierPolicy;
  return '// Generated experimental raw binding decisions. Do not edit.\n'
    + '/** @param {string} model @returns {boolean} */\n'
    + `export function acceptRustRawBindingIdentifierModel(model) { return model === ${JSON.stringify(policy.model)}; }\n`
    + '/** @param {string} name @param {boolean} raw @param {boolean} charactersAccepted @returns {boolean} */\n'
    + 'export function acceptRustRawBindingIdentifierName(name, raw, charactersAccepted) {\n'
    + `  if (!charactersAccepted || name === "" || ${policy.forbiddenNames.map(name => 'name === ' + JSON.stringify(name)).join(' || ')}) return false;\n`
    + `  return raw || !(${policy.ordinaryKeywords.map(name => 'name === ' + JSON.stringify(name)).join(' || ')});\n}\n`
    + '/** @param {string} kind @param {boolean} signed @param {number} bits @returns {boolean} */\n'
    + 'export function acceptRustRawBindingIdentifierType(kind, signed, bits) { return kind === "bool" || kind === "fixed" && signed && bits === 32; }\n'
    + '/** @param {boolean} recursive @param {boolean} mutual @returns {boolean} */\n'
    + 'export function acceptRustRawBindingIdentifierClosure(recursive, mutual) { return !recursive && !mutual; }\n';
}
export const rustRawBindingIdentifierFixtures = {
  schemaVersion: 1,
  model: rustRawBindingIdentifierPolicy.model,
  cases: [
    { id: 'keyword names', source: 'pub fn r#match(r#type:i32)->i32{r#type}fn main(){}', name: 'match', arguments: ['7'], value: '7' },
    { id: 'ordinary and raw name equality', source: 'pub fn r#foo(r#value:bool)->bool{value}fn main(){}', name: 'foo', arguments: [true], value: true },
    { id: 'actual forward declaration occurrence', source: 'pub fn r#match(value:i32)->i32{r#type(value)}pub fn r#type(r#arg:i32)->i32{arg}fn main(){}', name: 'match', arguments: ['9'], value: '9' },
    { id: 'aliases collide with original names only as spellings', source: 'pub fn sourceFunction0(sourceParameter0_0:i32)->i32{r#sourceParameter0_0}pub fn r#type(value:i32)->i32{sourceFunction0(value)}fn main(){}', name: 'type', arguments: ['11'], value: '11' },
    { id: 'parameter shadows actual function', source: 'pub fn r#foo(r#foo:bool)->bool{foo}fn main(){}', name: 'foo', arguments: [false], value: false },
    { id: 'two actual parameter owners', source: 'pub fn r#choose(r#match:i32,r#type:i32)->i32{r#match}fn main(){}', name: 'choose', arguments: ['13', '17'], value: '13' },
    { id: 'raw Boolean keyword is a binding', source: 'pub fn r#true(r#false:bool)->bool{r#false}pub fn observe(value:bool)->bool{r#true(value)}fn main(){}', name: 'observe', arguments: [true], value: true },
  ],
  refusals: [
    'fn foo(value:i32)->i32{value}fn r#foo(value:i32)->i32{value}fn main(){}',
    'fn value(foo:i32,r#foo:i32)->i32{foo}fn main(){}',
    ...rustRawBindingIdentifierPolicy.forbiddenNames.map(name => 'fn r#' + name + '(value:i32)->i32{value}fn main(){}'),
    'fn value(r /* gap */ #type:i32)->i32{r#type}fn main(){}',
    'fn value(r# type:i32)->i32{r#type}fn main(){}',
    'fn r##foo(value:i32)->i32{value}fn main(){}',
    'fn match(value:i32)->i32{value}fn main(){}',
    'fn value(r#type:i32)->i32{type}fn main(){}',
    'fn value(r#foo:i32)->i32{r#foo()}fn main(){}',
    'fn r#foo(value:i32)->i32{value}fn caller(foo:i32)->i32{r#foo(foo)}fn main(){}',
    'fn value(value:i32)->i32{unknown(value)}fn main(){}',
    'fn value(value:i32)->i32{value+1}fn main(){}',
    'fn value(value:i32)->i32{let x=value;x}fn main(){}',
    'fn r#again(value:i32)->i32{again(value)}fn main(){}',
    'fn r#first(value:i32)->i32{second(value)}fn r#second(value:i32)->i32{first(value)}fn main(){}',
    'fn value(value:i32)->i32{value}fn main(){value(1);}',
  ],
};
async function translatedRules(check) {
  const raw = emitRust(checkProgram(parseJavaScript(rustRawBindingIdentifierRules()))).definitions.join('\n\n');
  const actions = [];
  let current = raw;
  const replace = (from, to) => {
    assert.ok(current.includes(from), 'the actual automatic translation owns its recorded style target');
    actions.push({ op: 'replace', field: 'source', from, to }); current = current.split(from).join(to);
  };
  for (const name of ['model', 'name', 'kind']) replace(name + ': String', name + ': &str');
  // Thirty-two is positive, finite and nonzero: equality to that fixed Number
  // is exactly bit equality for every f64, including both zeros and all NaNs.
  replace('bits == (32f64)', 'bits.to_bits() == 32f64.to_bits()');
  for (const match of [...current.matchAll(/pub fn accept_rust_raw_binding_identifier_[\s\S]*?\n\}/gu)]) {
    const definition = match[0], at = definition.indexOf('{');
    const body = stripRustSourceHeaderStyleParentheses(definition.slice(at + 1, -1));
    replace(definition, '/// Experimental raw binding identity decision.\n#[must_use]\n' + definition.slice(0, at + 1) + '\n    ' + body + '\n}');
  }
  replace('pub fn accept_rust_raw_binding_identifier_closure', 'pub const fn accept_rust_raw_binding_identifier_closure');
  const styles = new DecoratorSet([{ id: 'rust-raw-binding-identifier-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).toLino();
  const text = '// Experimental automatic translation; no native runtime integration.\n'
    + formatProductRuleSource(DecoratorSet.fromLino(styles).decorate('emitter', { format: 'Rust', source: raw }).source);
  for (const [name, value] of [['library-styles.lino', styles], ['rules.rs', text]]) {
    const file = new URL('./data/' + name, import.meta.url);
    if (check) assert.equal(readFileSync(file, 'utf8'), value, 'the actual translated artifact and recorded styles remain fresh');
    else writeFileSync(file, value);
  }
}
if (process.argv[1] === fileURLToPath(import.meta.url)) {
  mkdirSync(new URL('./data/', import.meta.url), { recursive: true });
  const check = process.argv.includes('--check');
  for (const [name, value] of [['policy', rustRawBindingIdentifierPolicy], ['fixtures', rustRawBindingIdentifierFixtures]]) {
    const file = new URL('./data/' + name + '.lino', import.meta.url);
    if (check) checkProjectData(file, value); else writeProjectData(file, value);
  }
  const source = rustRawBindingIdentifierRules(), file = new URL('./rules.mjs', import.meta.url);
  if (check) assert.equal(readFileSync(file, 'utf8'), source); else writeFileSync(file, source);
  checkProgram(parseJavaScript(source));
  // Once an artifact exists, even the JavaScript-first check verifies it. The
  // flag can defer initial generation; it cannot hide a stale paired artifact.
  if (!process.argv.includes('--javascript-only') || existsSync(new URL('./data/rules.rs', import.meta.url)) || existsSync(new URL('./data/library-styles.lino', import.meta.url))) await translatedRules(check);
  console.log('Experimental raw identifiers: linked policy, fixtures and checked shared decisions');
}
