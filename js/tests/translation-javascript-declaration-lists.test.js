import assert from 'node:assert/strict';
import test from 'node:test';
import { runInNewContext } from 'node:vm';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { readJavaScriptLexicalDeclarationStep } from '../src/translation/frontend-rules.js';
import { tokenize } from '../src/translation/lexer.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';

function observe(source) {
  const lines = [];
  try {
    runInNewContext(source, { console: { log: (...values) => lines.push(values.map(String).join(' ')) } }, { timeout: 1000 });
    return { lines, error: null };
  } catch (error) { return { lines, error: { name: error.name, message: error.message } }; }
}

function equivalent(source, expected) {
  assert.deepEqual(observe(source), expected, 'independent original JavaScript execution');
  const checked = checkProgram(parseJavaScript(source));
  const packet = encodeTypedProgramLinks(checked);
  const rust = emitRust(checked).text;
  const variants = [checked,
    decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root),
    decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)), packet.root)];
  for (const program of variants) {
    assert.deepEqual(observe(emitJavaScript(program).text), expected);
    assert.equal(emitRust(program).text, rust, 'complete native emission survives source-free replay');
  }
}

test('const lists retain ordered effects, initialized siblings, and the enclosing block scope', () => {
  equivalent(`function step(value) { console.log(value); return value + 1; }
function selected() { const first = step(1), second = step(first), third = second + first; return third; }
console.log(selected());`, { lines: ['1', '2', '5'], error: null });
});

test('top-level lists retain every literal binding for later owned functions', () => {
  equivalent(`const first = 2, second = 3;
function selected() { return first + second; }
console.log(selected());`, { lines: ['5'], error: null });
});

test('mixed array defaults, object fields and ordinary declarators evaluate each source once', () => {
  equivalent(`function values() { console.log('array'); return [2]; }
function record() { console.log('record'); return {left: 3, right: 5}; }
function selected() { const [first, next = first + 1] = values(), {left, right: last} = record(), total = first + next + left + last; return total; }
console.log(selected());`, { lines: ['array', 'record', '13'], error: null });
});

test('an initializer abort prevents subsequent declarators from executing', () => {
  equivalent(`/** @returns {number} */ function stop() { console.log('stop'); throw new Error('halt'); }
function later() { console.log('later'); return 9; }
function selected() { const first = stop(), second = later(); return first + second; }
console.log(selected());`, { lines: ['stop'], error: { name: 'Error', message: 'halt' } });
});

test('later const and let bindings shadow outer values throughout the block', () => {
  for (const declaration of ['const', 'let']) for (const body of [
    `${declaration} first = later, later = 2; return first;`,
    `${declaration} first = 1, later = later; return first;`,
    `console.log(later); ${declaration} first = 1, later = 2; return first;`,
    `${declaration} first = 1, Math = Math.floor(2); return first;`,
  ]) {
    const source = `function selected() { const later = 7; { ${body} } } console.log(selected());`;
    assert.equal(observe(source).error?.name, 'ReferenceError');
    assert.throws(() => checkProgram(parseJavaScript(source)), /temporal dead zone/u);
  }
});

test('nested initializer groups and strings cannot manufacture declaration separators', () => {
  const source = `function selected() { const first = [1, 2], text = ',const fake', {left, right: last} = {left: 3, right: 4}, total = first[0] + left + last; return total; }`;
  const { tokens } = tokenize(source, 'JavaScript');
  const start = tokens.findIndex(token => token.value === '{') + 1;
  const positions = [];
  let depth = 0;
  let declaration = false;
  for (let index = start; index < tokens.length; index += 1) {
    const step = readJavaScriptLexicalDeclarationStep(tokens[index].kind, tokens[index].value, depth, declaration);
    if (step === -1) break;
    if (step === 1) depth += 1;
    else if (step === 2) depth -= 1;
    else if (step === 5) declaration = false;
    if (step === 3 || step === 4) { declaration = true; positions.push(index + 1); }
  }
  assert.deepEqual(positions.map(position => tokens[position].value), ['first', 'text', '{', 'total']);
  equivalent(source + 'console.log(selected());', { lines: ['8'], error: null });
});

test('automatic semicolon insertion and nested scopes retain independent declaration lists', () => {
  equivalent(`function selected() {
const first = 2, second = first + 1
{ const first = 7, second = first + 1; console.log(first, second); }
const third = second + 1, fourth = third + 1
return fourth
}
console.log(selected());`, { lines: ['7 8', '5'], error: null });
});

test('initialized sibling captures work while unproved forward invocation timing stays refused', () => {
  equivalent('function selected(){const first=2, read=()=>first;return read();}console.log(selected());', { lines: ['2'], error: null });
  equivalent('function selected(){const read=(later)=>later, later=3;return read(later);}console.log(selected());', { lines: ['3'], error: null });
  for (const source of [
    'function selected(){const read=()=>later,later=2;return read();}',
    'function selected(){const read=()=>read,next=2;return next;}',
    'function selected(){const [first=later,later=2]=[],next=first;return next;}',
  ]) assert.throws(() => checkProgram(parseJavaScript(source)), /temporal dead zone/u);
});

test('invalid duplicate binders and unrepresented empty-pattern checks cannot silently lower', () => {
  for (const declaration of ['first=1,first=2', 'first=1,[first]=[2]', 'first=1,{value:first}={value:2}']) {
    const source = `function selected(){const ${declaration};return first;}`;
    assert.equal(observe(source).error?.name, 'SyntaxError');
    assert.throws(() => parseJavaScript(source), /duplicate lexical binding/u);
  }
  for (const pattern of ['[]', '[,]', '{}']) {
    const source = `function selected(){const ${pattern}=null,next=1;return next;}console.log(selected());`;
    assert.equal(observe(source).error?.name, 'TypeError');
    assert.throws(() => parseJavaScript(source), /empty (?:array|object) binding/u);
  }
});

test('partially returning nested branches preserve their outer continuation and closure scopes', () => {
  equivalent(`function selected(first, second) { if(first) { if(second) return 3; } return 7; }
function closed(first,second){const read=()=>{if(first){if(second)return 5;}return 9;};return read();}
console.log(selected(true,true),selected(true,false),selected(false,true),closed(true,true),closed(true,false));`,
  { lines: ['3 7 7 5 9'], error: null });
});
