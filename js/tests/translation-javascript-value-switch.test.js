import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';

import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { readJavaScriptValueCaseForm, readJavaScriptValueSwitchInstruction,
  readJavaScriptValueSwitchPosition } from '../src/translation/javascript-surface-rules.js';
import { readProjectData } from '../scripts/project-data.mjs';
import { digest, runStages } from '../scripts/translation-stage-lib.mjs';
import frontendCases from './fixtures/translation-cases/frontend-javascript.mjs';
import checkCases from './fixtures/translation-cases/check.mjs';

const stages = readProjectData(new URL('../../parity/fixtures/translation-stages.lino', import.meta.url));
const record = result => result.error ? { error: result.error } : { sha256: digest(result.value) };

function observe(source) {
  const lines = [];
  vm.runInNewContext(source, { console: { log: (...values) => lines.push(values.map(String).join(' ')) } }, { timeout: 1000 });
  return lines;
}

function equivalent(source, expected) {
  assert.deepEqual(observe(source), expected);
  assert.deepEqual(observe(emitJavaScript(checkProgram(parseJavaScript(source))).text), expected);
}

test('all four original value switch programs retain every recorded pipeline stage', () => {
  const entries = [
    ...['number-case', 'value-switch-string-case', 'value-switch-name-case'].map(name => {
      const id = 'frontend-javascript-error-' + name;
      return ['frontend-javascript/' + id, frontendCases[id]];
    }),
    ['check/js-switch-literal', checkCases['js-switch-literal']],
  ];
  for (const [name, [extension, source]] of entries) {
    const fixture = stages.programs.find(program => program.name === name);
    assert.equal(fixture.source, source);
    assert.equal(fixture.extension, extension);
    const actual = runStages(extension, source);
    const observed = { parse: record(actual.parse) };
    if (actual.check) observed.check = record(actual.check);
    if (actual.emit) observed.emit = Object.fromEntries(Object.entries(actual.emit).map(([target, result]) => [target, record(result)]));
    const expected = { parse: fixture.parse };
    if (Object.hasOwn(fixture, 'check')) expected.check = fixture.check;
    if (Object.hasOwn(fixture, 'emit')) expected.emit = fixture.emit;
    assert.deepEqual(observed, expected, name);
  }
});

test('the shared instruction cursor orders fallback tests bodies and reverse branch construction', () => {
  for (const imperative of [false, true]) {
    const trace = [];
    const positions = [];
    for (let step = 0; ; step += 1) {
      const instruction = readJavaScriptValueSwitchInstruction(imperative, step, 2);
      trace.push(instruction);
      if (instruction === 'test') positions.push(readJavaScriptValueSwitchPosition(imperative, step, 2));
      if (instruction === 'bind') break;
    }
    assert.deepEqual(trace, ['capture', 'fallback', ...(imperative ? [] : ['reachability']), 'test', 'body', 'select', 'test', 'body', 'select', 'bind']);
    assert.deepEqual(positions, [1, 0]);
  }
  assert.equal(readJavaScriptValueCaseForm('string', 'true', '', false), 'value');
  assert.equal(readJavaScriptValueCaseForm('identifier', 'true', '', false), 'boolLit');
  assert.equal(readJavaScriptValueCaseForm('number', '1', '', true), 'value');
  assert.equal(readJavaScriptValueCaseForm('number', '1', 'n', true), 'numLit');
  assert.equal(readJavaScriptValueCaseForm('identifier', 'true', '', true), '');
});

test('pure value switches evaluate the discriminant once and dynamic tests only until the first match', () => {
  equivalent(`function subject(){console.log('subject');return 2;}
function candidate(value){console.log('test',value);return value;}
function choose(){switch(subject()){case candidate(1):return 'one';default:return 'none';case candidate(2):return 'two';case candidate(3):return 'three';}}
console.log(choose());`, ['subject', 'test 1', 'test 2', 'two']);
});

test('imperative value switches preserve grouped tests default placement and continuation state', () => {
  equivalent(`function candidate(value){console.log('test',value);return value;}
function choose(value){let result='initial';switch(value){case candidate(1):result='one';break;default:result='none';break;case candidate(2):case candidate(3):result='group';break;}return result;}
console.log(choose(3));console.log(choose(4));`, ['test 1', 'test 2', 'test 3', 'group', 'test 1', 'test 2', 'test 3', 'none']);
  equivalent(`function candidate(value){console.log('test',value);return value;}
function choose(value){let result='';switch(value){case candidate(1):result+='one';case candidate(2):result+='two';break;default:result='none';}return result;}
console.log(choose(1));`, ['test 1', 'onetwo']);
});

test('Number and string cases preserve strict equality and independent syntax refusals', () => {
  equivalent(`function numberWeight(value){switch(value){case -0:return 1;case 2.5:return 2;case NaN:return 3;default:return 4;}}
function word(value){switch(value){case 'true':return 5;default:return 6;}}
console.log(numberWeight(0),numberWeight(2.5),numberWeight(NaN),word('true'),word('false'));`, ['1 2 4 5 6']);
  equivalent(`function signed(value){let result='none';switch(value){case -1n:result='negative';break;case value:result='same';break;}return result;}
console.log(\`\${signed(-1n)} \${signed(2n)}\`);`, ['negative same']);
  for (const expression of ['-true', '-name']) {
    assert.throws(() => parseJavaScript('function choose(value){switch(value){case ' + expression + ':return 1;default:return 0;}}'), error => error.kind === 'unsupported' && error.details.construct === 'case test');
  }
  const mixed = '/** @param {bigint} value @returns {bigint} */function choose(value){switch(value){case "one":return 1n;default:return 0n;}}';
  assert.throws(() => checkProgram(parseJavaScript(mixed)), error => error.kind === 'type');
});
