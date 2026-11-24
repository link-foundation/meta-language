import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {parseRust,parseRustTokenStream} from '../src/translation/rust.js';
import {tokenize} from '../src/translation/lexer.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
const source='fn probe()->bool{true}',alternate='fn probe()->bool{false}';
const models=['standard Rust shared array identity handles 1','standard Rust shared array identity handles 2'];
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');
const observe=surface=>{const context={};vm.runInNewContext(emitJavaScript(checkProgram(surface)).text,context,{timeout:1000});return context.probe();};

test('caller token getters cannot replace the actual source after complete validation',()=>{
 for(const model of models){
  const actual=tokenize(source,'Rust').tokens,changed=tokenize(alternate,'Rust').tokens;
  let validated=false;
  const supplied=new Proxy(actual,{get(target,property,receiver){
   if(typeof property==='string'&&/^(0|[1-9][0-9]*)$/u.test(property)){
    const index=Number(property),value=(validated?changed:target)[index];
    if(index===target.length-1)validated=true;
    return value;
   }
   return Reflect.get(target,property,receiver);
  }});
  assert.equal(observe(parseRustTokenStream(supplied,{sharedArrayHandleSourceModel:model},source)),true,'the emitted value must come from the authenticated source');
  assert.equal(validated,true,'the complete supplied stream still undergoes validation');
 }
});

test('a later source-token read cannot mutate an earlier authenticated token occurrence',()=>{
 for(const model of models){
  const supplied=tokenize(source,'Rust').tokens,literal=supplied.findIndex(token=>token.value==='true'),last=supplied.at(-1);assert.ok(literal>=0);
  Object.defineProperty(supplied,String(supplied.length-1),{enumerable:true,configurable:true,get(){supplied[literal].value='false';return last;}});
  assert.equal(observe(parseRustTokenStream(supplied,{sharedArrayHandleSourceModel:model},source)),true);
  assert.equal(supplied[literal].value,'false','the external mutation actually occurred');
 }
});

test('owned parsing preserves ordinary complete inputs and still refuses originally forged tokens',()=>{
 for(const model of models){
  const supplied=tokenize(source,'Rust').tokens;for(const token of supplied)Object.freeze(token);Object.freeze(supplied);
  assert.deepEqual(parseRustTokenStream(supplied,{sharedArrayHandleSourceModel:model},source),parseRust(source,{sharedArrayHandleSourceModel:model}));
  assert.throws(()=>parseRustTokenStream(tokenize(alternate,'Rust').tokens,{sharedArrayHandleSourceModel:model},source),/source token stream differs|every original source token/u);
 }
});
