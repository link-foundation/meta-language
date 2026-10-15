import assert from 'node:assert/strict';
import { test } from 'node:test';
import vm from 'node:vm';
import { LinkNetwork, createTypedProgramLinks, emitTypedProgramLinks } from '../src/index.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { tokenize } from '../src/translation/lexer.js';
import { acceptJavaScriptLineTerminator } from '../src/translation/javascript-line-terminator-rules.js';
import { findJavaScriptLineTerminator } from '../src/translation/javascript-line-terminators.js';
import { readProjectData } from '../scripts/project-data.mjs';

const terminators = ['\n', '\r', '\u2028', '\u2029', '\r\n'];
function execute(source) {
  const output = [];
  vm.runInNewContext(source, { console: { log: (...values) => output.push(values.map(String).join(' ')) } }, { timeout: 1000 });
  return output;
}
function replay(source) {
  const packet = createTypedProgramLinks(source, 'JavaScript');
  return emitTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root, 'JavaScript').code;
}

test('the shared ECMAScript boundary recognizes exactly its four normative UTF-16 units', () => {
  const policy = readProjectData(new URL('../src/translation/data/javascript-line-terminators.lino', import.meta.url));
  assert.deepEqual(policy.codeUnits, [10, 13, 8232, 8233]);
  const expected = new Set([10, 13, 8232, 8233]);
  for (let unit = 0; unit < 65536; unit += 1) assert.equal(acceptJavaScriptLineTerminator(unit), expected.has(unit));
  for (const value of [-1, 65536, NaN, Infinity, 10.5]) assert.equal(acceptJavaScriptLineTerminator(value), false);
});

test('single-line comment endings retain every active statement through public source-free Links replay', () => {
  for (const separator of terminators) {
    const source = `// hidden${separator}console.log('ACTIVE');\nconsole.log('END');`;
    assert.deepEqual(execute(source), ['ACTIVE', 'END']);
    assert.deepEqual(execute(replay(source)), execute(source));
    const { comments, tokens } = tokenize(source, 'JavaScript');
    assert.deepEqual(comments, [{ text: '// hidden', start: 0, end: 9 }]);
    const first = tokens[0];
    assert.equal(first.value, 'console');
    assert.equal(first.start, source.indexOf('console'));
    assert.equal(source.slice(first.start, first.end), 'console');
  }
  const trailing = tokenize('console.log(1); // no terminator', 'JavaScript');
  assert.equal(trailing.comments[0].end, 'console.log(1); // no terminator'.length);
});

test('nonterminating whitespace remains inside comments and other source-language comment boundaries stay intact', () => {
  for (const space of ['\v', '\f', '\u0085', '\u00a0', '\u2000', '\ufeff', '😀']) {
    const source = `// hidden${space}console.log('HIDDEN');\nconsole.log('END');`;
    assert.deepEqual(execute(source), ['END']);
    assert.deepEqual(execute(replay(source)), ['END']);
  }
  for (const separator of ['\r', '\u2028', '\u2029']) {
    const source = `// hidden${separator}fn hidden() {}\nfn visible() {}`;
    assert.equal(tokenize(source, 'Rust').tokens.filter(token => token.value === 'fn').length, 1);
  }
  assert.equal(findJavaScriptLineTerminator('😀text\u2028tail', 0), 6);
  assert.equal(findJavaScriptLineTerminator('😀text\u2028tail', 0, 6), 6);
});

test('return ASI across every actual line terminator refuses unreachable values rather than changing undefined to a value', () => {
  for (const separator of terminators) for (const gap of [separator, `/*${separator}*/`, `// comment${separator}`]) {
    const source = `/** @returns {number} */ function f(){return${gap}1;} console.log(f());`;
    assert.deepEqual(execute(source), ['undefined']);
    assert.throws(() => createTypedProgramLinks(source, 'JavaScript'), error => error.kind === 'unsupported' && error.reason.startsWith('unreachable statement:'));
    const empty = `function f(){return${gap};} f();`;
    assert.deepEqual(execute(replay(empty)), execute(empty));
  }
  const ordinary = 'function f(){return/* no line break */1;} console.log(f());';
  assert.deepEqual(execute(replay(ordinary)), ['1']);
});

test('line terminators after throw retain the native JavaScript syntax failure and exact source span', () => {
  for (const separator of terminators) for (const gap of [separator, `/*${separator}*/`, `// comment${separator}`]) {
    const source = `function f(){throw${gap}new Error('no');}`;
    assert.throws(() => new vm.Script(source), { name: 'SyntaxError' });
    assert.throws(() => parseJavaScript(source), error => error.kind === 'syntax' && error.reason === 'line terminator after throw'
      && error.span.start === source.indexOf('throw') && error.span.end === source.indexOf('throw') + 5);
  }
  assert.doesNotThrow(() => parseJavaScript("function f(){throw/* ordinary */new Error('no');}"));
});

test('break and continue ASI ignore following unreachable identifiers rather than inventing labels', () => {
  for (const jump of ['break', 'continue']) for (const separator of terminators) {
    const source = `function f(){let n=0;while(n<1){n++;${jump}/*${separator}*/missing();}return n;} console.log(f());`;
    assert.deepEqual(execute(source), ['1']);
    assert.throws(() => parseJavaScript(source), error => error.kind === 'unsupported' && error.reason.startsWith('unreachable statement:'));
    const labelled = `function f(){while(true){${jump} label;}}`;
    assert.throws(() => parseJavaScript(labelled), error => error.kind === 'unsupported' && error.reason.startsWith(`labelled ${jump}:`));
  }
});
