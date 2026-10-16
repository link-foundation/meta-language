import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {createHash} from 'node:crypto';
import {DecoratorSet} from '../src/decorators.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {
  applyRecordedFamilyLibraryStyles, flattenFamilyLibraryElseIf,
  mergeFamilyLibraryIdenticalBranches, qualifyFamilyLibraryBooleanFunctions,
  invertFamilyLibraryInequalityBranches, bindFamilyLibraryInequalityConditions, simplifyFamilyLibraryNegatedConjunctions, liftFamilyLibraryTruthinessConditions,
} from '../scripts/recorded-family-library-style.mjs';

function readDecision(source, {ordinaryConstBody = false} = {}) {
  const context = {};
  const options = source.includes('const fn') ? {constFunctionModel: 'closed pure Rust const predicates 1'} : {};
  let surface = parseRust(source + '\nfn main() {}', options);
  if (ordinaryConstBody) {
    const original = surface;
    surface = {...original, items: original.items.map(entry => {
      const copy = {...entry};
      delete copy.sourceConstFunction;
      assert.equal(copy.body, entry.body, 'the runtime-only seam retains the actual parsed body object');
      assert.equal(copy.params, entry.params);
      return copy;
    })};
    assert.ok(original.items.every(entry => Object.hasOwn(entry, 'sourceConstFunction')));
  }
  vm.runInNewContext(emitJavaScript(checkProgram(surface)).text, context, {timeout: 1000});
  return context.decide;
}

test('family branch syntax preserves exact bodies and every Boolean branch observation', () => {
  const before = 'pub fn decide(first:bool,second:bool,third:bool)->bool{if first{third}else{if second{third}else{!third}}}';
  const flattened = flattenFamilyLibraryElseIf(before);
  assert.notEqual(flattened, before);
  const after = mergeFamilyLibraryIdenticalBranches(flattened);
  assert.match(after, /if \(\s*first\s*\) \|\| \(\s*second\s*\)/u);
  const original = readDecision(before), transformed = readDecision(after);
  for (const first of [false, true]) for (const second of [false, true]) for (const third of [false, true]) {
    assert.equal(transformed(first, second, third), original(first, second, third));
  }
  assert.equal(mergeFamilyLibraryIdenticalBranches(after), after);
  assert.equal(flattenFamilyLibraryElseIf(after), after);
});

test('family identical-body merge keeps callback evaluation and opaque syntax intact', () => {
  const before = 'pub fn decide(first:bool,second:bool)->bool{if observe_first(first){true}else if observe_second(second){true}else{false}}';
  const after = mergeFamilyLibraryIdenticalBranches(before);
  const read = source => {
    const context = {events: []};
    const helpers = 'fn observe_first(value:bool)->bool{value} fn observe_second(value:bool)->bool{value}';
    let emitted = emitJavaScript(checkProgram(parseRust(helpers + source + ' fn main(){}'))).text;
    emitted = emitted.replace('function observe_first(value) {', 'function observe_first(value) { events.push("first");')
      .replace('function observe_second(value) {', 'function observe_second(value) { events.push("second");');
    vm.runInNewContext(emitted, context, {timeout: 1000});
    return context;
  };
  for (const first of [false, true]) for (const second of [false, true]) {
    const original = read(before), transformed = read(after);
    assert.equal(transformed.decide(first, second), original.decide(first, second));
    assert.deepEqual(transformed.events, original.events);
    assert.deepEqual(transformed.events, first ? ['first'] : ['first', 'second']);
  }
  for (const source of [
    'pub fn text()->String{String::from("if first {same} else {if second {same} else {different}}")}',
    'macro_rules! sample { () => { if first { same } else { if second { same } else { different } } }; }',
    'pub fn decide(first:bool)->bool{if first{true}else{let value=false;if first{true}else{value}}}',
    'pub fn decide(first:bool,second:bool)->bool{if first{/*first*/true}else if second{/*second*/true}else{false}}',
    'pub fn decide(first:bool,second:bool)->bool{if first{/*same*/true}else if second{/*same*/true}else{false}}',
    'pub fn decide(first:bool,second:bool)->bool{if first{true}/*keep*/else if second{true}else{false}}',
  ]) {
    assert.equal(flattenFamilyLibraryElseIf(source), source);
    assert.equal(mergeFamilyLibraryIdenticalBranches(source), source);
  }
});

test('family const qualification requires only actual Boolean parameters and primitive syntax', () => {
  for (const body of ['first || second', 'if first { second } else { !second }']) {
    const before = `pub fn decide(first:bool,second:bool)->bool{${body}}`;
    const after = qualifyFamilyLibraryBooleanFunctions(before);
    assert.match(after, /^pub const fn decide/u);
    assert.equal(qualifyFamilyLibraryBooleanFunctions(after), after);
    const constIf = body.startsWith('if ');
    if (constIf) {
      assert.throws(() => checkProgram(parseRust(after + '\nfn main() {}', {constFunctionModel: 'closed pure Rust const predicates 1'})),
        error => error.reason === 'Rust const function proof: the expression has no primitive const predicate interpretation',
        'the existing const source proof still refuses checked If; this control compares only the retained ordinary Boolean body');
    }
    const original = readDecision(before), transformed = readDecision(after, {ordinaryConstBody: constIf});
    for (const first of [false, true]) for (const second of [false, true]) assert.equal(transformed(first, second), original(first, second));
  }
  for (const source of [
    'pub fn decide(first:f64)->bool{first>0.0}',
    'pub fn decide(first:bool)->bool{other(first)}',
    'pub fn decide(first:bool)->bool{let value=first;value}',
    'pub fn decide(first:bool)->bool{first==true}',
    'pub fn decide(first:bool)->bool{first()}',
  ]) assert.equal(qualifyFamilyLibraryBooleanFunctions(source), source);
});

test('family records append after historical bytes and require fresh exact replay', () => {
  const historical = new DecoratorSet([{id: 'historical-base', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: [{op: 'replace', field: 'source', from: 'unrelated', to: 'preserved'}],
  }]).toLino();
  const source = 'pub fn decide(first:bool,second:bool)->bool{first||second}';
  const baseRecordHashes = [createHash('sha256').update(historical).digest('hex')];
  assert.throws(() => applyRecordedFamilyLibraryStyles(historical, source, {family: 'test-family'}), /stale/u);
  const recorded = applyRecordedFamilyLibraryStyles(historical, source, {family: 'test-family', update: true, baseRecordHashes});
  assert.ok(recorded.document.startsWith(historical));
  assert.equal(DecoratorSet.fromLino(recorded.document).size, 2);
  assert.deepEqual(applyRecordedFamilyLibraryStyles(recorded.document, source, {family: 'test-family'}), recorded);
  assert.throws(() => applyRecordedFamilyLibraryStyles(recorded.document, source.replace('first||second', 'first&&second'), {family: 'test-family'}), /stale/u);
  assert.throws(() => applyRecordedFamilyLibraryStyles(recorded.document, source, {family: 'another-family'}), /stale/u);
  assert.throws(() => applyRecordedFamilyLibraryStyles(recorded.document.replace('historical-base', 'rewritten-base'), source,
    {family: 'test-family', baseRecordHashes}), /historical family style record/u);
  const records = DecoratorSet.fromLino(recorded.document).decorators;
  const forged = new DecoratorSet(records.map(record => record.id === 'historical-base' ? record : {...record,
    actions: record.actions.map(action => ({...action, to: action.to.replace('first||second', 'false')})),
  })).toLino();
  assert.throws(() => applyRecordedFamilyLibraryStyles(forged, source, {family: 'test-family', baseRecordHashes}), /stale/u,
    'a modified post-base body cannot certify its own semantics');
  assert.doesNotMatch(recorded.source, /#\[(?:allow|expect)\(/u);
});

test('family whole inequality inversion preserves unordered Numbers and both original arms', () => {
  const before = 'pub fn decide(first:f64,second:f64)->bool{if (first) != (second) {/*different*/true} /*between*/ else {/*equal*/false}}';
  const after = invertFamilyLibraryInequalityBranches(before);
  assert.notEqual(after, before);
  for (const comment of ['/*different*/', '/*between*/', '/*equal*/']) assert.equal(after.split(comment).length, 2);
  const original = readDecision(before), transformed = readDecision(after);
  for (const first of [NaN, -Infinity, -1, -0, 0, 1, Infinity]) for (const second of [NaN, -Infinity, -1, -0, 0, 1, Infinity]) {
    assert.equal(transformed(first, second), original(first, second));
  }
  for (const source of [
    'pub fn decide(first:bool,second:bool)->bool{if first!=second&&first{true}else{false}}',
    'pub fn decide(first:f64,second:f64)->bool{if !(first<second){true}else{false}}',
    'pub fn decide(first:bool)->bool{if let Some(value)=first{true}else{false}}',
    'pub fn decide<T:PartialEq>(first:T,second:T)->bool{if first!=second{true}else{false}}',
    'struct Custom;impl PartialEq for Custom{fn eq(&self,other:&Self)->bool{true}fn ne(&self,other:&Self)->bool{true}}pub fn decide(first:Custom,second:Custom)->bool{if first!=second{true}else{false}}',
    'struct String;pub fn decide(first:String,second:String)->bool{if first!=second{true}else{false}}',
    'pub fn decide(first:f64,second:f64)->bool{let (first,other)=custom();if first!=second{true}else{false}}',
    'pub fn decide(first:f64,second:f64)->bool{match custom(){Some(first)=>{if first!=second{true}else{false}},None=>false}}',
    'macro_rules! sample { () => { if first != second { different } else { equal } }; }',
  ]) assert.equal(invertFamilyLibraryInequalityBranches(source), source);
  assert.equal(invertFamilyLibraryInequalityBranches(after), after);
  const vector = 'pub fn decide(first:Text)->bool{if first!=Text::from_units(vec![48]){true}else{false}}';
  assert.equal(invertFamilyLibraryInequalityBranches(vector), vector, 'an unproved custom Text name cannot stand in for the derived provider');
  const provider = 'pub mod ml_string{#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord)]pub struct Text(pub Vec<u16>);impl Text{pub fn from_units(units:Vec<u16>)->Self{Self(units)}}}';
  const proven = provider + 'pub fn decide(first:self::ml_string::Text)->bool{if first!=self::ml_string::Text::from_units(vec![48]){true}else{false}}';
  const projected = invertFamilyLibraryInequalityBranches(proven);
  assert.notEqual(projected, proven);
  assert.equal(projected.split('Text::from_units(vec![48])').length, 2, 'a complete vector literal stays opaque and occurs once');
  const strings = 'pub fn decide(first:&str,second:String)->bool{if first!=second{true}else{false}}';
  assert.notEqual(invertFamilyLibraryInequalityBranches(strings), strings);
  const helpers = 'fn observe(value:f64)->f64{value}pub fn decide(first:f64,second:f64)->bool{let value=observe(first);if value!=observe(second){true}else{false}}';
  assert.notEqual(invertFamilyLibraryInequalityBranches(helpers), helpers, 'actual unique local helper return types supply the operand proof');
  const overridden = 'struct Custom;impl PartialEq for Custom{fn eq(&self,other:&Self)->bool{true}fn ne(&self,other:&Self)->bool{true}}'
    + 'pub fn decide(first:Custom,second:Custom,ml_library_inequality_1:bool)->bool{if first!=second{/*different*/true}else{/*equal*/false}}';
  assert.equal(invertFamilyLibraryInequalityBranches(overridden), overridden);
  const bound = bindFamilyLibraryInequalityConditions(overridden);
  assert.notEqual(bound, overridden);
  assert.equal(bound.split('first!=second').length, 2, 'the custom ne operator is retained exactly once');
  assert.ok(bound.startsWith(overridden.slice(0, overridden.indexOf('pub fn decide'))), 'the independently overridden eq and ne implementations retain their bytes');
  assert.match(bound, /let ml_library_inequality_2 =\s*first!=second;/u);
  assert.match(bound, /if ml_library_inequality_2/u);
  assert.equal(bound.split('/*different*/true').length, 2);
  assert.equal(bound.split('/*equal*/false').length, 2);
  assert.equal(bindFamilyLibraryInequalityConditions(bound), bound);
  const callback = 'pub fn decide(first:f64,second:f64)->bool{if observe_first(first)!=observe_second(second){true}else{false}}';
  assert.equal(invertFamilyLibraryInequalityBranches(callback), callback, 'unproved callees remain outside inversion');
  const callbackBound = bindFamilyLibraryInequalityConditions(callback);
  const observe = source => {
    const context = {events: []};
    const functions = 'fn observe_first(value:f64)->f64{value}fn observe_second(value:f64)->f64{value}';
    let code = emitJavaScript(checkProgram(parseRust(functions + source + 'fn main(){}'))).text;
    code = code.replace('function observe_first(value) {', 'function observe_first(value) { events.push("first");')
      .replace('function observe_second(value) {', 'function observe_second(value) { events.push("second");');
    vm.runInNewContext(code, context, {timeout: 1000});
    return context;
  };
  for (const first of [NaN, -0, 0, 1]) for (const second of [NaN, -0, 0, 1]) {
    const original = observe(callback), transformed = observe(callbackBound);
    assert.equal(transformed.decide(first, second), original.decide(first, second));
    assert.deepEqual(transformed.events, ['first', 'second']);
    assert.deepEqual(transformed.events, original.events);
  }
  const opaque = 'pub fn decide(first:Custom)->bool{if unrepresented!(first)!=first{true}else{false}}';
  assert.equal(bindFamilyLibraryInequalityConditions(opaque), opaque);
});

test('family whole unary conjunction style retains short circuit order and unordered comparisons', () => {
  const helpers = 'fn observe_first(value:bool)->bool{value}fn observe_second(value:bool)->bool{value}';
  const before = 'pub fn decide(first:bool,second:bool)->bool{if !observe_first(first)&&!observe_second(second){true}else{false}}';
  const after = simplifyFamilyLibraryNegatedConjunctions(before);
  assert.notEqual(after, before);
  const read = source => {
    const context = {events: []};
    let emitted = emitJavaScript(checkProgram(parseRust(helpers + source + 'fn main(){}'))).text;
    emitted = emitted.replace('function observe_first(value) {', 'function observe_first(value) { events.push("first");')
      .replace('function observe_second(value) {', 'function observe_second(value) { events.push("second");');
    vm.runInNewContext(emitted, context, {timeout: 1000});
    return context;
  };
  for (const first of [false, true]) for (const second of [false, true]) {
    const original = read(before), transformed = read(after);
    assert.equal(transformed.decide(first, second), original.decide(first, second));
    assert.deepEqual(transformed.events, original.events);
    assert.deepEqual(transformed.events, first ? ['first'] : ['first', 'second']);
  }
  const numbers = 'pub fn decide(first:f64,second:f64)->bool{if !(first<second)&&!(first>second){true}else{false}}';
  const original = readDecision(numbers), transformed = readDecision(simplifyFamilyLibraryNegatedConjunctions(numbers));
  for (const first of [NaN, -Infinity, -0, 0, 1, Infinity]) for (const second of [NaN, -Infinity, -0, 0, 1, Infinity]) {
    assert.equal(transformed(first, second), original(first, second));
  }
  for (const source of [
    'pub fn decide(first:bool,second:bool)->bool{if !first&&second{true}else{false}}',
    'pub fn decide(first:bool,second:bool)->bool{if !first||!second{true}else{false}}',
    'pub fn decide(first:bool,second:bool)->bool{if !first==second&&!second{true}else{false}}',
  ]) assert.equal(simplifyFamilyLibraryNegatedConjunctions(source), source);
  assert.equal(simplifyFamilyLibraryNegatedConjunctions(after), after);
  const vector = 'pub fn decide(letter:bool,character:Text,index:f64)->bool{if !letter&&!((index>0f64)&&Text::from_units(vec![48,49]).contains(&character)){true}else{false}}';
  const projected = simplifyFamilyLibraryNegatedConjunctions(vector);
  assert.notEqual(projected, vector);
  assert.equal(projected.split('Text::from_units(vec![48,49])').length, 2);
});

test('family numeric truthiness lifts the complete checked condition once with a fresh scoped binding', () => {
  const source = '/** @param {number} value @returns {number} */function decide(value){return value?1:2;}';
  const emitted = emitRust(checkProgram(parseJavaScript(source)), {arrayConsumption: true}).definitions.join('\n\n');
  const after = liftFamilyLibraryTruthinessConditions(emitted);
  assert.notEqual(after, emitted, 'the actual checked Number truthiness condition is lifted');
  const condition = /if\s*(\{\s*let ml_value = [\s\S]*?ml_value\.is_nan\(\)\s*\})/u.exec(emitted)?.[1];
  assert.ok(condition);
  assert.equal(after.split(condition).length, 2, 'the whole original Number computation remains once');
  assert.match(after, /let ml_library_condition_1 =/u);
  assert.match(after, /if ml_library_condition_1/u);
  assert.equal(liftFamilyLibraryTruthinessConditions(after), after);
  const collision = emitted.replace('value: f64', 'value: f64, ml_library_condition_1: bool');
  assert.match(liftFamilyLibraryTruthinessConditions(collision), /let ml_library_condition_2 =/u);
  const unsafe = 'pub fn decide(value:bool)->bool{if {let ml_value=value;ml_value}{true}else{false}}';
  assert.equal(liftFamilyLibraryTruthinessConditions(unsafe), unsafe);
});
