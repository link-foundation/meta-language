import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { parseRust } from '../src/translation/rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { projectLinkedProgramExpression } from '../src/typed-program-expression-projection.js';

function project(program) {
  const expression = value => projectLinkedProgramExpression(value, program.declarations);
  return {
    language: program.sourceLanguage,
    items: program.items.map(entry => entry.k === 'data' ? {
      k: 'data', name: entry.name, ctors: entry.ctors, span: entry.span, generated: entry.generated,
    } : {
      k: 'fn', name: entry.name, params: entry.params, ret: entry.ret,
      body: expression(entry.body), span: entry.span, generated: entry.generated,
    }),
    main: program.main && { ...program.main, effects: program.main.effects.map(effect => ({
      ...effect, expr: expression(effect.expr), style: 'js-console',
    })) },
  };
}

function normalize(value) {
  if (value instanceof Map) return [...value].map(([name, child]) => [name, normalize(child)]);
  if (Array.isArray(value)) return value.map(normalize);
  if (value && typeof value === 'object') return Object.fromEntries(Object.entries(value)
    .filter(([key, child]) => child != null && !(child === false && ['generated', 'mutual', 'frozen', 'sequentialAsync', 'constant'].includes(key)))
    .map(([key, child]) => [key, normalize(child)]));
  return value;
}

test('data, constructors and match scopes project through the existing checker unchanged', () => {
  const sources = [
    [parseRust, 'enum Choice{Empty,Value(i32)} fn read(item:Choice)->i32{match item{Choice::Empty=>0,Choice::Value(value)=>value}} fn main(){println!("{}",read(Choice::Value(7)));println!("{}",read(Choice::Empty));}'],
    [parseRust, 'enum Inner{Value(i32)} enum Outer{Empty,Wrap(Inner)} fn read(item:Outer)->i32{match item{Outer::Empty=>0,Outer::Wrap(inner)=>match inner{Inner::Value(value)=>value}}} fn main(){println!("{}",read(Outer::Wrap(Inner::Value(7))));println!("{}",read(Outer::Empty));}'],
    [parseJavaScript, '/** @typedef {{ $: "some", value: number } | { $: "none" }} Choice */\n/** @returns {Choice} */ function make(){return {$:"some",value:7};}\n/** @param {Choice} item @returns {number} */ function read(item){if(item.$==="some") return item.value; return 0;}\nconsole.log(read(make())); console.log(read({$:"none"}));'],
  ];
  for (const [parse, source] of sources) {
    const checked = checkProgram(parse(source));
    const restored = checkProgram(project(checked));
    assert.deepEqual(normalize(restored), normalize(checked));
    const output = [];
    vm.runInNewContext(emitJavaScript(restored).text, {
      console: { log: (...values) => output.push(values.map(String).join(' ')) }, process: { argv: [] },
    }, { timeout: 1000 });
    assert.deepEqual(output, ['7', '0']);
  }
});

test('constructor projection preserves explicit immutability and actual field names', () => {
  const source = '/** @typedef {{ $: "some", value: number } | { $: "none" }} Choice */\n/** @returns {Choice} */ function make(){return Object.freeze({$:"some",value:7});}';
  const checked = checkProgram(parseJavaScript(source));
  const restored = checkProgram(project(checked));
  assert.deepEqual(normalize(restored), normalize(checked));
  const result = vm.runInNewContext(`${emitJavaScript(restored).text}\n[Object.isFrozen(make()), make().value]`, {}, { timeout: 1000 });
  assert.deepEqual(Array.from(result), [true, 7]);
  assert.throws(() => projectLinkedProgramExpression(checked.declarations.get('make').body), /constructor projection differs/u);
});

test('projection retains checker refusal for missing constructor fields and nonexhaustive matches', () => {
  const checked = checkProgram(parseRust('enum Choice{Empty,Value(i32)} fn read(item:Choice)->i32{match item{Choice::Empty=>0,Choice::Value(value)=>value}} fn main(){}'));
  const nonexhaustive = project(checked);
  nonexhaustive.items.find(entry => entry.k === 'fn').body.cases.pop();
  assert.throws(() => checkProgram(nonexhaustive), /match on Choice misses Value/u);
  const incorrect = project(checked);
  const kase = incorrect.items.find(entry => entry.k === 'fn').body.cases[1];
  kase.pattern = { ...kase.pattern, binds: [] };
  assert.throws(() => checkProgram(incorrect), /binds 0 of 1 fields/u);
});
