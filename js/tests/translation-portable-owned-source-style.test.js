import assert from 'node:assert/strict';
import {test} from 'node:test';
import {runInNewContext} from 'node:vm';
import {DecoratorSet} from '../src/decorators.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {STRING_PRELUDE} from '../src/translation/string-runtime.js';
import {readPortableOwnedSourceStyles} from '../scripts/portable-owned-source-style.mjs';
import {readPortableSourceParameterStyles, readPortableSourceLintStyles} from '../scripts/portable-source-library-style.mjs';

const copies = source => (source.match(/\.clone\(\)/gu) ?? []).length;
function review(source, options) {
  const result = readPortableOwnedSourceStyles(source, options);
  const styles = new DecoratorSet(result.actions.length ? [{id: 'owned-source-control', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: result.actions}] : []);
  assert.equal(styles.decorate('emitter', {format: 'Rust', source}).source, result.source);
  assert.deepEqual(readPortableOwnedSourceStyles(result.source, options), {source: result.source, actions: []});
  return result.source;
}

test('declared owned String results lose only their dead branch copies and replay exact source actions', () => {
  const source = 'pub fn selected() -> String { String::from("x") }\n'
    + 'pub fn choose(flag: bool) -> String { let first = self::selected(); if first == "" { String::new() } else if flag { first.clone() } else { format!("{}", first) } }';
  const styled = review(source);
  assert.equal(copies(styled), 0);
  assert.ok(styled.includes('else if flag { first }'));
  assert.ok(styled.includes('format!("{}", first)'));
  const joined = 'pub fn joined(flag: bool) -> String { let first = String::new(); let result = if flag { first.clone() } else { String::new() }; format!("{}{}", result, first) }';
  assert.equal(review(joined), joined, 'a read after the branch join keeps the original clone');
  const compound = 'pub fn compound(flag: bool, value: String) -> String { let copied = value.clone(); if flag && { self::condition() } { return copied; } self::observe(value); String::new() }';
  assert.equal(review(compound), compound, 'an unsupported condition block cannot hide the false continuation');
  const priorCapture = 'pub fn prior_capture(value: String) -> String { let displayed = format!("{value}"); self::observe(displayed); value.clone() }';
  assert.equal(copies(review(priorCapture)), 0, 'a completed plain capture does not keep a later terminal transfer live');
  const captureBranch = 'pub fn capture_branch(value: String, flag: bool) -> String { if flag { value.clone() } else { format!("{{{value}}}") } }';
  assert.equal(copies(review(captureBranch)), 0, 'an escaped-brace plain capture in another complete branch cannot read this branch transfer');
  const standardCapture = 'pub fn standard_capture(value: String) -> String { let displayed = std::format!("{value}"); self::observe(displayed); value.clone() }';
  assert.equal(copies(review(standardCapture)), 0, 'an unshadowed standard format macro retains the same explicit capture reads');
});

test('terminal function returns inside loops keep tuple order while loop backedges and breaks retain copies', () => {
  const terminal = 'pub fn finish(mut value: String, flag: bool) -> (String, String) { loop { return if flag { (value.clone(), value.clone()) } else { value = String::new(); continue; }; } }';
  const styled = review(terminal);
  assert.ok(styled.includes('(value.clone(), value)'));
  assert.equal(copies(styled), 1, 'only the last evaluated tuple copy is dead');
  const later = 'pub fn later(value: String) -> String { return self::consume(value.clone(), value.len()); }';
  assert.equal(review(later), later, 'a later argument still reads the original value');
  const earlier = 'pub fn earlier(value: String) -> String { return self::consume(value.len(), value.clone()); }';
  assert.equal(copies(review(earlier)), 0, 'an earlier argument does not keep the later transfer live');
  const recreated = 'pub fn recreated(flag: bool) -> String { loop { let fresh = String::new(); if flag { self::consume(fresh.clone()); continue; } return fresh.clone(); } }';
  assert.equal(copies(review(recreated)), 0, 'a unique let definition creates new storage on the next iteration');
  const assigned = 'pub fn assigned(mut value: String, flag: bool) -> String { loop { if flag { return value; } let selected = if value.is_empty() { String::new() } else { value.clone() }; value = selected; continue; } }';
  assert.equal(copies(review(assigned)), 0, 'a completed bare assignment kills the old loop state before the backedge');
  const tuple = 'pub fn assigned_tuple(mut index: f64, mut value: String, flag: bool) -> String { loop { if flag { return value; } let selected = if value.is_empty() { String::new() } else { value.clone() }; let next = index + 1f64; (index, value) = (next, selected); continue; } }';
  assert.equal(copies(review(tuple)), 0, 'a completed tuple assignment kills only its actual target bindings');
  const ordered = 'pub fn ordered(mut first: String, mut second: String) -> String { (first, second) = (first.clone(), first.clone()); second }';
  assert.ok(review(ordered).includes('(first, second) = (first.clone(), first);'), 'all tuple RHS reads precede every target kill');
  const laterStore = 'pub fn later_store(mut value: String) -> String { value = self::consume(value.clone(), value.len()); value }';
  assert.equal(review(laterStore), laterStore, 'a later RHS argument keeps the earlier clone before a bare store');
  const earlierStore = 'pub fn earlier_store(mut value: String) -> String { value = self::consume(value.len(), value.clone()); value }';
  assert.equal(copies(review(earlierStore)), 0, 'a last RHS argument can transfer before the completed bare store');
  for (const source of [
    'pub fn carried(value: String) -> String { loop { self::observe(value.clone()); continue; } }',
    'pub fn broken(value: String, flag: bool) -> String { loop { if flag { self::observe(value.clone()); break; } } value }',
    'pub fn condition(value: String, flag: bool) -> String { while flag { self::observe(value.clone()); } value }',
    'pub fn continued_rhs(mut value: String, flag: bool) -> String { loop { value = if flag { self::observe(value.clone()); continue; } else { String::new() }; } }',
    'pub fn compound_store(mut value: String) -> String { loop { value += self::read(value.clone()); continue; } }',
    'pub fn indexed_store(mut values: Vec<String>, value: String) -> String { loop { values[0] = value.clone(); continue; } }',
    'pub fn dereferenced_store(target: &mut String, value: String) -> String { loop { *target = value.clone(); continue; } }',
    'pub fn repeated_targets(mut value: String) -> String { loop { (value, value) = (value.clone(), value.clone()); continue; } }',
    'pub fn expression_store(mut value: String, flag: bool) -> String { loop { let selected = value.clone(); (if flag {} else {}, value = selected); continue; } }',
  ]) assert.equal(review(source), source);
});

test('actual declared enum field types prove owned pattern slots without crossing alternative arms', () => {
  const source = 'pub enum Selected { First(String), Second(String) }\n'
    + 'pub fn selected(value: self::Selected) -> String { match value { self::Selected::First(first) => { first.clone() } self::Selected::Second(second) => { second.clone() } } }';
  assert.equal(copies(review(source)), 0, 'both comma-free block arms consume their separate owned fields');
  const borrowed = source.replace('First(String)', 'First(&str)').replace('Second(String)', 'Second(&self::ml_string::Text)');
  assert.equal(review(borrowed), borrowed, 'declared borrowed enum fields are never treated as owned');
  assert.equal(review(source.replace('value: self::Selected', 'value: &self::Selected')),
    source.replace('value: self::Selected', 'value: &self::Selected'), 'borrowed scrutinees retain match-ergonomic reference fields');
  assert.equal(review(source.replace('match value', 'match &value')),
    source.replace('match value', 'match &value'), 'an explicit scrutinee loan never yields owned pattern fields');
  const loop = 'pub enum State { Next(String) }\n'
    + 'pub fn selected(value: String) -> self::State { self::State::Next(value) }\n'
    + 'pub fn repeated(mut value: String, flag: bool) -> String { loop { let state = if flag { self::State::Next(String::new()) } else { self::selected(value.clone()) }; match state { self::State::Next(next) => { if flag { return next.clone(); } value = next.clone(); continue; } } } }';
  const moved = review(loop);
  assert.equal(copies(moved), 0, 'each selected field and the completed reassignment kill the previous loop storage');
  assert.equal(moved.includes('value.clone()'), false);
  assert.equal(moved.includes('next.clone()'), false);
  const returned = 'pub enum State { Next(String) }\n'
    + 'pub fn returned(value: String, flag: bool) -> String { let state = if flag { let inspected = value.len(); self::observe(inspected); return value.clone(); } else { self::State::Next(value.clone()) }; match state { self::State::Next(next) => next.clone() } }';
  assert.equal(copies(review(returned)), 0, 'an actual return arm cannot supply the owned initializer result');
  const continued = 'pub enum State { Next(String) }\n'
    + 'pub fn continued(mut value: String, flag: bool) -> String { loop { let state = if flag { value = String::new(); continue; } else { self::State::Next(value.clone()) }; match state { self::State::Next(next) => return next.clone() } } }';
  assert.equal(copies(review(continued)), 0, 'an actual continue arm cannot supply the owned initializer result');
  const matched = 'pub enum Choice { First(String), Second(String) }\npub enum State { Next(String) }\n'
    + 'pub fn matched(choice: self::Choice) -> String { let state = match choice { self::Choice::First(first) => { self::State::Next(first.clone()) } self::Choice::Second(second) => { self::State::Next(second.clone()) } }; match state { self::State::Next(next) => next.clone() } }';
  assert.equal(copies(review(matched)), 0, 'all actual match result arms prove the same owned enum result');
  const borrowedResult = 'pub enum State { Next(String) }\n'
    + 'pub fn borrowed_result(value: &self::State, flag: bool) -> String { let state = if flag { return String::new(); } else { value }; match state { self::State::Next(next) => next.clone() } }';
  assert.equal(review(borrowedResult), borrowedResult, 'a divergent alternative never turns a borrowed surviving result into an owner');
  const conditionalReturn = 'pub enum State { Next(String) }\n'
    + 'pub fn conditional_return(value: &self::State, flag: bool, condition: bool) -> String { let state = if flag { if condition { return String::new(); } value } else { value }; match state { self::State::Next(next) => next.clone() } }';
  assert.equal(review(conditionalReturn), conditionalReturn, 'a conditional return with a fallthrough path proves no divergent branch');
  const brokenResult = 'pub enum State { Next(String) }\n'
    + 'pub fn broken_result(value: String) -> String { let state = loop { break self::State::Next(value); }; match state { self::State::Next(next) => next.clone() } }';
  assert.equal(review(brokenResult), brokenResult, 'a value-carrying break remains outside the bounded control proof');
  const shadow = 'pub fn shadow(value: String, borrowed: &String) -> String { match borrowed { value => value.clone() } }';
  assert.equal(review(shadow), shadow, 'an unrepresented pattern cannot borrow an outer binding type');
});

test('borrowed values, escaped loans, shadowed bindings, captures and unknown syntax keep their copies', () => {
  const unchanged = [
    'pub fn borrowed(value: &str) -> &str { value.clone() }',
    'pub fn borrowed(value: &self::ml_string::Text) -> self::ml_string::Text { value.clone() }',
    'pub fn borrowed(value: &[String]) -> &[String] { value.clone() }',
    'pub fn shadow(value: String) -> String { let value = String::new(); value.clone() }',
    'pub fn escaped(value: String) -> String { let alias = &value; let copied = value.clone(); self::observe(alias); copied }',
    'pub fn compound_loan(value: String) -> String { let alias = &{ &value }; let copied = value.clone(); self::observe(alias); copied }',
    'pub fn grouped_loan(value: String) -> String { let alias = &(self::borrow(value)); let copied = value.clone(); self::observe(alias); copied }',
    'pub fn escaped(value: String) -> String { let alias = value.as_str(); let copied = value.clone(); self::observe(alias); copied }',
    'pub fn borrowed_local() -> &str { let value = String::new().as_str(); value.clone() }',
    'pub fn captured(value: String) -> String { let read = || value.len(); value.clone() }',
    'pub fn opaque(value: String) -> String { unknown!(value); value.clone() }',
    'unknown_type!();\npub fn opaque_type(value: String) -> String { value.clone() }',
    '#[derive(Custom)]\npub struct Wrapper;\npub fn opaque_type(value: String) -> String { value.clone() }',
    'pub fn captured_format(value: String) -> String { let copied = value.clone(); let display = format!("{value}"); copied }',
    'pub fn captured_argument(value: String) -> String { format!("{value} {}", self::consume(value.clone())) }',
    'pub fn captured_tuple(value: String) -> String { format!("{value} {}", self::consume((value.clone(), value.len()))) }',
    'pub fn captured_width(value: String, width: usize) -> String { let copied = value.clone(); let display = format!("{:width$}", value); copied }',
    'pub fn captured_debug(value: String) -> String { let copied = value.clone(); let display = format!("{value:?}"); copied }',
    'pub fn unsupported(value: String) -> String { for index in 0..1 { self::observe(index); } value.clone() }',
    'struct String;\npub fn custom(value: String) -> String { value.clone() }',
    'struct Vec<T>(T);\npub fn custom(value: Vec<String>) -> Vec<String> { value.clone() }',
    'use custom::String;\npub fn custom(value: String) -> String { value.clone() }',
    'pub use crate::translation::frontend_rules::ml_array as String;\npub fn custom(value: String) -> String { value.clone() }',
    'pub use std::format;\npub fn imported_macro(value: String) -> String { value.clone() }',
    'pub use crate::translation::frontend_rules::*;\npub fn imported_glob(value: String) -> String { value.clone() }',
    'pub mod custom { pub use std::format; }\npub fn qualified_macro(value: String) -> String { let displayed = custom::format!("{value}"); value.clone() }',
    'pub mod std {}\npub fn shadowed_standard(value: String) -> String { std::format!("{}", value.clone()) }',
  ];
  for (const source of unchanged) assert.equal(review(source), source);
});

test('owned Text and primitive vectors retain literal and comment bytes and never edit opaque source text', () => {
  const source = 'pub fn text(value: self::ml_string::Text) -> self::ml_string::Text { value.clone() }\n'
    + 'pub fn rows(value: Vec<Vec<f64>>) -> Vec<Vec<f64>> { value.clone() }\n'
    + 'pub fn spelling(value: String) -> String { let literal = r#"pub fn hidden(value: String) -> String { value.clone() }"#; self::observe(literal); /* value.clone() */ value.clone() }\n'
    + 'pub fn commented(value: String) -> String { value./* keep this comment */clone() }';
  const styled = review(source);
  assert.ok(styled.includes('-> self::ml_string::Text { value.clone() }'), 'an unauthenticated Text name supplies no Clone behavior proof');
  assert.ok(styled.includes('-> Vec<Vec<f64>> { value }'));
  assert.ok(styled.includes('r#"pub fn hidden(value: String) -> String { value.clone() }"#'));
  assert.ok(styled.includes('/* value.clone() */ value }'));
  assert.ok(styled.includes('value./* keep this comment */clone()'));
  const complete = 'pub fn duplicate(value: String) -> String { value.clone() }';
  const collision = complete + '\npub fn literal() -> &\'static str { r#"' + complete + '"# }';
  assert.equal(review(collision), collision, 'a recorded whole-function replacement must not also match opaque literal bytes');
  const text = 'pub fn text(value: self::ml_string::Text) -> self::ml_string::Text { value.clone() }\n'
    + 'pub fn selected(value: &self::ml_string::Text, flag: bool) -> self::ml_string::Text { let local = value.clone(); if flag { local.slice(0f64, 1f64) } else { local.clone() } }\n'
    + 'pub fn split(value: &self::ml_string::Text) -> Vec<self::ml_string::Text> { let pieces = value.split(None, 1f64); pieces.clone() }';
  const authenticated = review(STRING_PRELUDE + '\n' + text);
  assert.ok(authenticated.includes('-> self::ml_string::Text { value }'));
  assert.ok(authenticated.includes('let local = value.clone();'), 'the borrowed parameter still produces its actual owned Text copy');
  assert.ok(authenticated.includes('else { local }'));
  assert.ok(authenticated.includes('let pieces = value.split(None, 1f64); pieces }'));
  const counterfeit = STRING_PRELUDE.replace('derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)', 'derive(Debug, PartialEq, Eq, PartialOrd, Ord)') + '\n' + text;
  assert.equal(review(counterfeit), counterfeit, 'a counterfeit provider cannot authorize owned Text clone removal');
  const overridden = STRING_PRELUDE + '\nimpl ml_string::Text { pub fn clone(&self) -> Self { panic!("counterfeit clone") } }\n' + text;
  assert.equal(review(overridden), overridden, 'an external inherent clone override is refused despite the authentic module');
  const spelling = STRING_PRELUDE.replace('pub fn length', '#[must_use]\n        pub fn length').replace('4294967296.0', '4_294_967_296.0');
  assert.ok(review(spelling + '\n' + text).includes('else { local }'), 'only known must-use and decimal grouping spelling differences are ignored');
});

test('actual checked JavaScript branches and changing string loops preserve values through source-owned styles', () => {
  const source = '/** @param {string} value @returns {string} */function identity(value){return value;}\n'
    + '/** @param {string} value @param {boolean} flag @returns {string} */function choose(value,flag){const local=identity(value);if(local==="")return "empty";return flag?local:"fallback";}\n'
    + '/** @param {number} count @returns {string} */function repeated(count){let cases="";while(count>0){cases=cases+"x";count-=1;}return cases;}';
  const program = checkProgram(parseJavaScript(source, {referenceSemantics: false}));
  const emitted = emitRust(program, {arrayConsumption: true});
  const loaned = readPortableSourceParameterStyles(emitted.definitions.join('\n\n')).source;
  const result = readPortableOwnedSourceStyles(loaned);
  assert.ok(result.actions.length > 0, 'actual source translation contains proven owned final copies');
  assert.ok(copies(result.source) < copies(loaned));
  review(loaned);
  const alias = 'pub use crate::translation::frontend_rules::ml_array;\n';
  const relocated = alias + loaned, composed = readPortableSourceLintStyles(relocated).source;
  assert.ok(copies(review(relocated)) < copies(relocated), 'the exact generated array-provider alias admits actual checked function ownership facts');
  assert.ok(composed.includes(alias), 'the complete composition retains the exact generated runtime relocation');
  assert.ok(copies(composed) < copies(relocated), 'the complete composer consumes final copies without adding any source body');
  const context = {};
  runInNewContext(emitJavaScript(program).text, context, {timeout: 1000});
  for (const [value, flag, expected] of [['', true, 'empty'], ['kept', true, 'kept'], ['kept', false, 'fallback']]) {
    assert.equal(context.choose(value, flag), expected);
  }
  assert.equal(context.repeated(0), '');
  assert.equal(context.repeated(4), 'xxxx');
});

test('explicitly selected owned functions transfer final copies only after authenticated immutable calls finish', () => {
  const options = {scopedCallBorrows: true, functionNames: ['selected']};
  const string = 'pub fn inspected(value: &String) -> bool { value.is_empty() }\n'
    + 'pub fn selected(value: String) -> Vec<String> { let empty = self::inspected(&(value)); if empty { Vec::<String>::new() } else { vec![value.clone()] } }';
  assert.equal(review(string), string, 'the historical default still refuses every explicit borrow');
  assert.equal(review(string, {scopedCallBorrows: true}), string, 'an explicit exact function-name selection is also required');
  assert.equal(review(string, {...options, functionNames: ['other']}), string);
  assert.equal(copies(review(string, options)), 0, 'a completed inspected immutable call cannot retain a String reference');
  const text = 'pub fn inspected(value: &self::ml_string::Text) -> bool { value.is_empty() }\n'
    + 'pub fn planned(value: &self::ml_string::Text, name: &self::ml_string::Text) -> Vec<self::ml_string::Text> { vec![value.clone(), name.clone()] }\n'
    + 'pub fn selected(name: &self::ml_string::Text, flag: bool) -> Vec<self::ml_string::Text> { '
    + 'let command = if flag { self::ml_string::Text::from_units(vec![97]) } else { self::ml_string::Text::from_units(vec![98]) }; '
    + 'let replayed = self::planned(&(command), name); if replayed.len() == 0f64 { Vec::<self::ml_string::Text>::new() } else { vec![command.clone()] } }\n'
    + 'pub fn selected_loop(mode: self::ml_string::Text) -> Vec<self::ml_string::Text> { '
    + 'let accepted = self::inspected(&(mode)); let argument = if accepted { mode.clone() } else { self::ml_string::Text::from_units(vec![]) }; '
    + 'let displayed = argument.length(); self::observe(displayed); vec![mode.clone()] }';
  const actual = STRING_PRELUDE + '\n' + text;
  const selected = {...options, functionNames: ['selected', 'selected_loop']};
  const styled = review(actual, selected);
  assert.ok(styled.includes('else { vec![command] }'), 'the replayed command call finishes before the final command transfer');
  assert.ok(styled.includes('if accepted { mode.clone() }'), 'the earlier mode copy stays because the final vector still needs mode');
  assert.ok(styled.includes('self::observe(displayed); vec![mode]'), 'the final mode copy becomes a transfer after the completed predicate call');
  assert.ok(styled.includes('vec![value.clone(), name.clone()]'), 'borrowed callee parameters retain their actual owned copies');
  const composed = readPortableSourceLintStyles(actual, selected).source;
  assert.ok(composed.includes('vec![command]'));
  assert.ok(composed.includes('vec![mode]'), 'the complete recorded composer carries the same narrow proof option');
  const constructed = 'pub fn planned(value: &self::ml_string::Text, name: &self::ml_string::Text) -> Vec<self::ml_string::Text> { vec![value.clone(), name.clone()] }\n'
    + 'pub fn selected(name: &self::ml_string::Text, pieces: &[self::ml_string::Text]) -> Vec<self::ml_string::Text> { '
    + 'let command = if (pieces.len() as f64) == 0f64 { self::ml_string::Text::from_units(vec![]) } else { '
    + '(self::ml_string::Text::from_units(vec![45, 62, 32])).concat(&self::ml_string::join(pieces, &self::ml_string::Text::from_units(vec![44, 32]))) }; '
    + 'let replayed = self::planned(&(command), name); if replayed.len() == 0f64 { Vec::<self::ml_string::Text>::new() } else { vec![command.clone()] } }';
  const constructedSource = STRING_PRELUDE + '\n' + constructed;
  assert.equal(review(constructedSource), constructedSource, 'constructor-method branch inference remains disabled by default');
  assert.ok(review(constructedSource, options).includes('else { vec![command] }'),
    'the actual emitted constructor-concat-join branch yields owned Text before replay and the final transfer');
  assert.ok(readPortableSourceLintStyles(constructedSource, options).source.includes('vec![command]'),
    'the exact emitted command constructor shape survives complete recorded style composition');
  const unknownMethod = STRING_PRELUDE + '\n' + constructed.replace(')).concat(', ')).unknown(');
  assert.equal(review(unknownMethod, options), unknownMethod, 'only authenticated provider method signatures determine branch ownership');
  const mismatched = constructedSource.replace('if (pieces.len() as f64) == 0f64 { self::ml_string::Text::from_units(vec![]) }',
    'if (pieces.len() as f64) == 0f64 { name }');
  assert.equal(review(mismatched, options), mismatched, 'a borrowed branch cannot join an owned temporary method result');
  assert.equal(review(text, selected), text, 'a Text spelling without its complete authenticated provider proves no ownership');
  const counterfeit = actual.replace('derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)', 'derive(Debug, PartialEq, Eq, PartialOrd, Ord)');
  assert.equal(review(counterfeit, selected), counterfeit, 'an altered Text clone provider remains refused');
  const refused = [
    'pub fn selected(value: String) -> String { self::unknown(&value); value.clone() }',
    'pub fn inspected(value: &String) -> bool { value.is_empty() }\npub fn selected(value: String) -> String { external::inspected(&value); value.clone() }',
    'pub fn inspected(value: &String) -> &String { value }\npub fn selected(value: String) -> String { let alias = self::inspected(&value); let copied = value.clone(); self::observe(alias); copied }',
    'pub fn inspected(value: &String) -> Custom { custom(value) }\npub fn selected(value: String) -> String { let alias = self::inspected(&value); value.clone() }',
    'pub fn inspected(value: &mut String) -> bool { value.is_empty() }\npub fn selected(mut value: String) -> String { self::inspected(&mut value); value.clone() }',
    'pub fn inspected(value: &String, target: &mut Vec<String>) -> bool { value.is_empty() }\npub fn selected(value: String, target: &mut Vec<String>) -> String { self::inspected(&value, target); value.clone() }',
    'pub fn inspected(value: &String) -> bool { value.is_empty() }\npub fn selected(value: String) -> String { let alias = &value; self::inspected(alias); value.clone() }',
    'pub fn inspected(value: &String) -> bool { value.is_empty() }\npub fn selected(value: String) -> String { self::inspected(&{ value }); value.clone() }',
    'pub fn inspected(value: &String) -> bool { value.is_empty() }\npub fn selected(value: String) -> String { self::inspected(&value); let copied = value.clone(); self::observe(value); copied }',
    'pub fn inspected(value: &String, next: String) -> bool { value.is_empty() }\npub fn selected(value: String) -> bool { self::inspected(&value, value.clone()) }',
    'pub fn inspected(value: &String) -> bool { unknown!(value) }\npub fn selected(value: String) -> String { self::inspected(&value); value.clone() }',
    'pub fn inspected(value: &String) -> bool { unsafe { custom(value) } }\npub fn selected(value: String) -> String { self::inspected(&value); value.clone() }',
    'pub fn inspected(value: &String) -> bool { value.is_empty() }\npub fn inspected(value: &String) -> bool { false }\npub fn selected(value: String) -> String { self::inspected(&value); value.clone() }',
    'pub fn inspected(value: &String) -> bool { value.is_empty() }\npub fn selected(value: String) -> String { let copied = value.clone(); self::inspected(&value); copied }',
  ];
  for (const source of refused) assert.equal(review(source, options), source);
});
