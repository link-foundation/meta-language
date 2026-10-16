import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {DecoratorSet} from '../src/decorators.js';
import {tokenize} from '../src/translation/lexer.js';
import {applyCompleteExpressionParenthesisStyles, completeExpressionParenthesisStylePrefix,
  replaceRecordedLibraryBaseStyle, simplifyCompleteRustExpressionParentheses} from '../scripts/complete-expression-parenthesis-styles.mjs';

test('complete-expression styles remove whole CI condition and tail wrappers while retaining every operand', () => {
  const before = 'pub fn choose(model:&str,kind:&str)->bool{if((model!="closed primitive borrowed Rust call interface 1")||(kind!="call")){(((kind=="a")||(kind=="b")))}else{((true))}}';
  const after = simplifyCompleteRustExpressionParentheses(before);
  assert.equal(after, 'pub fn choose(model:&str,kind:&str)->bool{if (model!="closed primitive borrowed Rust call interface 1")||(kind!="call"){(kind=="a")||(kind=="b")}else{true}}');
  const operands = source => tokenize(source, 'Rust').tokens.filter(token => token.kind !== 'punct' || !['(', ')'].includes(token.value)).map(token => [token.kind, token.value]);
  assert.deepEqual(operands(after), operands(before));
  assert.equal(simplifyCompleteRustExpressionParentheses(after), after);
});

test('complete-expression styles preserve tuples unit precedence calls closures and opaque source', () => {
  for (const source of [
    'pub fn pair(){(a,b)}', 'pub fn singleton(){(a,)}', 'pub fn unit(){()}',
    'pub fn grouped(){(a+b)*c}', 'pub fn cast(){if (value as usize)<limit {true}else{false}}',
    'pub fn call(){(factory())(argument)}', 'pub fn closure(){(||value)()}',
    'pub fn literal(){String::from("if ((x)) { (y) }")}',
    'pub fn raw(){r#"if ((x)) { (y) }"#}',
    'pub fn tree(){tokens!{if ((x)) { (y) }}}',
    'demo! { if (x) { (x) } }',
    '#[custom(if ((x)) { (y) })] pub fn value(){0}',
    '#![custom(if ((x)) { (y) })] pub fn value(){0}',
    'pub fn structure(){if (Thing { value:true }.value){true}else{false}}',
  ]) assert.equal(simplifyCompleteRustExpressionParentheses(source), source, source);
  assert.equal(simplifyCompleteRustExpressionParentheses('pub fn pair(){((a,b))}'), 'pub fn pair(){(a,b)}');
  assert.equal(simplifyCompleteRustExpressionParentheses('pub fn comment(){/* (x) */ (/* keep */ value /* ) */)}'), 'pub fn comment(){/* (x) */ /* keep */ value /* ) */}');
  assert.throws(() => simplifyCompleteRustExpressionParentheses('pub fn broken(){(value}'), /delimiter/u);
});

test('recorded whole-expression corrections append deterministically and refuse stale freshness', () => {
  const definition = {id: 'base', level: 'emitter', actions: [{op: 'replace', field: 'source', from: 'old', to: 'new'}]};
  const original = new DecoratorSet([definition]).toLino() + '\n';
  const source = 'pub fn choose(flag:bool)->bool{if (flag){(true)}else{(false)}}';
  assert.throws(() => applyCompleteExpressionParenthesisStyles(original, source), /stale/u);
  const first = applyCompleteExpressionParenthesisStyles(original, source, {update: true});
  assert.ok(first.document.startsWith(original));
  assert.equal(DecoratorSet.fromLino(first.document).decorators.filter(row => row.id.startsWith(completeExpressionParenthesisStylePrefix)).length, 1);
  assert.deepEqual(applyCompleteExpressionParenthesisStyles(first.document, source), first);
  assert.deepEqual(applyCompleteExpressionParenthesisStyles(first.document, source, {update: true}), first);
  const changed = source.replace('flag:bool', 'changed:bool').replace('if (flag)', 'if (changed)');
  assert.throws(() => applyCompleteExpressionParenthesisStyles(first.document, changed), /stale/u);
  const next = applyCompleteExpressionParenthesisStyles(first.document, changed, {update: true});
  assert.ok(next.document.startsWith(first.document));
  assert.equal(DecoratorSet.fromLino(next.document).decorators.filter(row => row.id.startsWith(completeExpressionParenthesisStylePrefix)).length, 2);
});

test('base style updates preserve unrelated recorded bytes and their complete-expression correction', () => {
  const base = {id: 'base', level: 'emitter', actions: [{op: 'replace', field: 'source', from: 'old', to: 'new'}]};
  const other = new DecoratorSet([{...base, id: 'unrelated'}]).toLino();
  const document = other + '\n' + new DecoratorSet([base]).toLino();
  const source = 'pub fn choose(flag:bool)->bool{if (flag){(true)}else{(false)}}';
  const recorded = applyCompleteExpressionParenthesisStyles(document, source, {update: true});
  const changed = replaceRecordedLibraryBaseStyle(recorded.document, {...base, actions: [{op: 'replace', field: 'source', from: 'before', to: 'after'}]});
  assert.ok(changed.startsWith(other + '\n'));
  const correction = recorded.document.slice(document.length);
  assert.ok(changed.endsWith(correction));
  assert.equal(replaceRecordedLibraryBaseStyle(changed, {...base, actions: [{op: 'replace', field: 'source', from: 'before', to: 'after'}]}), changed);
  assert.equal(applyCompleteExpressionParenthesisStyles(changed, source).source, recorded.source);
  const protectedSource = readFileSync(new URL('../scripts/product-rule-style-corrections.mjs', import.meta.url));
  assert.equal(createHash('sha256').update(protectedSource).digest('hex'), '9728ead881e57f16401321be9eb00e25416aab74cdb4d7164485cd0a3abbc216');
});
