// The portable kernel consumes actual UTF-16 units; adapters keep realm access explicit.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../js/scripts/project-data.mjs';
import {acceptRustOwnedLocalTextInput,acceptRustOwnedLocalTextUnits,renderRustOwnedLocalTextInput} from '../js/src/translation/rust-owned-local-state-rules.js';
import {rustOwnedLocalStatePortableRules} from './generate-rust-owned-local-state-rules.mjs';
import {parseJavaScript} from '../js/src/translation/javascript.js';
import {checkProgram} from '../js/src/translation/check.js';
import {emitRust} from '../js/src/translation/emit-rust.js';
const fixture=readProjectData(new URL('./rust-owned-local-state-fixture.lino',import.meta.url));
const units=text=>Array.from({length:text.length},(_,index)=>text.charCodeAt(index));
const from=values=>String.fromCharCode(...values);
const vectors=[[],[0],[0xd7ff],[0xe000],[0xffff],[0xd800],[0xdbff],[0xdc00],[0xdfff],[0xd800,0xdc00],[0xdbff,0xdfff],[0xd800,0xe000],[0xd800,0xd800,0xdc00],[0xdc00,0xd800],[0x41,0xd800],[0xd800,0xdc00,0x42],[0xd800,0xdc00,0xdfff],[0xd800,0xdc00,0xdbff,0xdfff]];
test('one portable UTF-16 unit predicate matches the frozen rendered helper and independent scalar oracle',()=>{
 const context={};vm.runInNewContext(renderRustOwnedLocalTextInput(),context,{timeout:1000});
 const texts=[...fixture.inputValues.validText,...fixture.inputValues.invalidText,...vectors.map(from)];
 for(const text of texts){
  const expected=Array.from(text).every(character=>{const point=character.codePointAt(0);return point<0xd800||point>0xdfff;});
  assert.equal(acceptRustOwnedLocalTextUnits(units(text)),expected);
  assert.equal(acceptRustOwnedLocalTextInput(text),expected);
  assert.equal(context.ml_ownedTextInput(text),expected);
 }
 for(const value of [null,undefined,1,{},new String('boxed')])assert.equal(acceptRustOwnedLocalTextInput(value),false);
});
test('the complete portable policy kernel and renderer self-translate through the actual checked frontend',()=>{
 const translated=emitRust(checkProgram(parseJavaScript(rustOwnedLocalStatePortableRules())));
 assert.ok(translated.definitions.some(source=>source.includes('fn accept_rust_owned_local_text_units(')));
 assert.ok(translated.definitions.some(source=>source.includes('fn ml_accept_rust_owned_local_text_units_loop1(')));
 assert.ok(translated.definitions.some(source=>source.includes('pub fn render_rust_owned_local_text_input(')));
 assert.ok(translated.definitions.every(source=>!source.includes('fn main(')&&!source.includes('fn ml_main(')));
});
