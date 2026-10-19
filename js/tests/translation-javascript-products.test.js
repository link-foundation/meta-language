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

test('top-level nested product destructuring keeps one source evaluation and lexical bindings', () => {
  equivalent(`function make(){console.log('read');return Object.freeze([Object.freeze([5,true]),2]);}
const [[value,flag],increment]=make();console.log(flag?value+increment:value);`, ['read', '7']);
});

test('nested fresh freezes follow actual owned local bindings rather than tuple annotations', () => {
  equivalent(`function make(){const inner=Object.freeze([5,true]);return Object.freeze([inner,2]);}
const value=make();console.log(value[0][0]===5);console.log(value[0][1]===true);`, ['true', 'true']);
  assert.throws(() => checkProgram(parseJavaScript(`/** @param {[number,boolean]} inner */ function make(inner){return Object.freeze([inner,2]);}
const value=make([5,true]);console.log(value[0][0]);`)), /immutable tuple product/u);
});

test('fixed numeric negative zero selects the same product position as positive zero', () => {
  const { surface } = equivalent(pair + '\nconsole.log(value[-0]===value[0]);', ['true', 'true', 'true']);
  const projection = surface.main.effects.at(-1).expr.left;
  assert.equal(projection.k, 'tupleIndex');
  assert.equal(projection.index, 0);
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
  const record = 'const record={value:Object.freeze([5,true])};console.log(record.value[0]);';
  assert.throws(() => parseJavaScript(record), error => error.kind === 'unsupported'
    && error.details.construct === 'immutable tuple product use'
    && error.reason === 'immutable tuple product use: a product only supports owned bindings, direct calls, and fixed positional reads');
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

test('shadowed global numeric names follow actual bindings and temporal dead zones', () => {
  for (const name of ['NaN', 'Infinity']) {
    equivalent(`function make(${name}){return Object.freeze([${name},true]);}
console.log(make(5)[0]===5);`, ['true']);
    assert.throws(() => parseJavaScript(`function make(){const value=${name};const ${name}=5;return Object.freeze([value,true]);}`), /temporal dead zone/u);
    assert.throws(() => parseJavaScript(`function make(){return Object.freeze([${name},true]);}const ${name}=5;console.log(make()[0]===5);`), /global binding/u);
  }
  equivalent('function value(undefined){return undefined;}console.log(value(5)===5);', ['true']);
  assert.throws(() => parseJavaScript('function value(){const result=undefined;const undefined=5;return result;}'), /temporal dead zone/u);
});

test('whole product conditions retain the explicit position-only obligation', () => {
  for (const [condition, diagnostic] of [['value', /immutable tuple product use/u], ['value?true:false', /immutable tuple product use/u], ['!value', /immutable tuple product use/u], ['value&&true', /tuple.*boolean/u], ['value||false', /tuple.*boolean/u]]) {
    assert.throws(() => parseJavaScript(`function check(){const value=Object.freeze([5,true]);if(${condition})return true;return false;}console.log(check());`), diagnostic);
  }
});

test('parameter defaults follow the same product caller and escape proof as function bodies', () => {
  const externals = [{ k: 'fn', name: 'escape', params: [{ name: 'value', type: { kind: 'tuple', elements: [{ kind: 'float' }, { kind: 'bool' }] } }], ret: { kind: 'bool' } }];
  assert.throws(() => parseJavaScript('function run(flag=escape(Object.freeze([5,true]))){return flag;}console.log(run());', { externals }), /unknown or external caller/u);
  equivalent('function read(value=Object.freeze([5,true])){return value[0];}console.log(read()===5);console.log(read(Object.freeze([7,true]))===7);', ['true', 'true']);
});

test('default initializers cannot substitute global constants for current or later parameter bindings', () => {
  for (const name of ['NaN', 'Infinity', 'undefined']) {
    for (const params of [`${name}=${name}`, `value=${name},${name}=5`, `${name}=\`\${${name}}\``]) {
      assert.throws(() => parseJavaScript(`function make(${params}){return Object.freeze([5,true]);}make();`), /default reading parameter|temporal dead zone/u);
    }
  }
  equivalent('function value(x=((x)=>x)(5)){return x;}console.log(value());', ['5']);
});
