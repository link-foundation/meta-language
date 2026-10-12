import assert from 'node:assert/strict';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';

function output(source) {
  const values = [];
  runInNewContext(source, { console: { log: (...args) => values.push(args.map(String).join(' ')) } }, { timeout: 1000 });
  return values;
}
function equivalent(source, expected) {
  const surface = parseJavaScript(source);
  const checked = checkProgram(surface);
  assert.deepEqual(output(source), expected);
  assert.deepEqual(output(emitJavaScript(checked).text), expected);
  assert.ok(emitRust(checked).definitions.some(text => text.includes('pub fn')));
  return { surface, checked };
}
const pair = `/** @returns {[number,boolean]} */ function make(){return Object.freeze([5,true]);}
/** @param {[number,boolean]} value @returns {number} */ function choose(value){const [number,flag]=value;return flag?number+2:number;}
const value=make();console.log(choose(value)===7);console.log(value[0]===5);`;

test('closed frozen products preserve the selected model and all actual positional callers', () => {
  const { surface, checked } = equivalent(pair, ['true', 'true']);
  assert.equal(surface.items[0].body.k, 'tuple');
  assert.equal(surface.items[0].body.productModel, 'immutable value product');
  assert.equal(checked.declarations.get('make').body.frozen, true);
  assert.equal(checked.declarations.get('make').body.productModel, 'immutable value product');
  assert.equal(surface.main.effects[2].expr.left.k, 'tupleIndex');
});

test('nested product destructuring evaluates a fresh source once and binds positions in order', () => {
  equivalent(`function make(){console.log('read');return Object.freeze([Object.freeze([5,true]),2]);}
function choose(){const [[value,flag],increment]=make();return flag?value+increment:value;}
console.log(choose()===7);`, ['read', 'true']);
});

test('product ownership requires every caller and refuses mutable arrays despite tuple annotations', () => {
  for (const source of [
    pair.replace('Object.freeze([5,true])', '[5,true]'),
    pair + '\nconsole.log(choose([5,true]));',
    pair.replace('function make()', 'export function make()'),
    pair + '\nconst leaked=choose;',
    pair + '\nconsole.log(value===value);',
    pair + '\nfunction read(index){return value[index];}console.log(read(0));',
  ]) assert.throws(() => checkProgram(parseJavaScript(source)), /immutable tuple|unsupported|function value|unknown name/u);
});

test('fresh freeze admission refuses shallow mutable children, zero products and shadowed built-ins', () => {
  for (const source of [
    'function make(){return Object.freeze([[5,true],2]);}',
    'function make(){return Object.freeze([]);}',
    'function Object(value){return value;}function make(){return Object.freeze([5,true]);}',
    'function make(Object){return Object.freeze([5]);}',
    'function make(){return Object.freeze([...[5,true]]);}',
  ]) assert.throws(() => checkProgram(parseJavaScript(source)), /immutable tuple|shadow|method|unknown/u);
});

test('nested arbitrary array iterators retain an explicit obligation', () => {
  assert.throws(() => parseJavaScript('function read(){const [[value]]=[[5]];return value;}console.log(read());'), /nested array binding/u);
});

test('imperative closures retain immutable outer bindings while their loops are lifted', () => {
  equivalent(`function total(first,second,offset){const count=(value)=>{let result=offset;for(let index=0;index<value;index+=1)result+=1;return result;};return count(first)+count(second);}
console.log(total(2,3,4));`, ['13']);
});

test('solved expression identities follow reference representation clones without becoming declared evidence', () => {
  const surface = parseJavaScript(pair);
  assert.equal(surface.referenceSemantics, true);
  assert.equal(surface.inferredExpressionTypes.get(surface.items[0].body).kind, 'tuple');
  assert.equal(Object.keys(surface).includes('inferredExpressionTypes'), false);
  assert.equal(JSON.stringify(surface).includes('inferredExpressionTypes'), false);
});
