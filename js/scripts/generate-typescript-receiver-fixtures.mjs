// Authored receiver observations; generated artifacts are parity inputs only.
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {eraseTypeScript} from '../src/typescript-erasure.js';
import {projectTypeScriptTyping} from '../src/typescript-portable-typing.js';
import {selfTranslate} from '../src/self-translation.js';

/** Prepare complete finite receiver sources and independent expected values. */
export function generateTypeScriptReceiverFixtures({check=false}={}) {
  const cases=[
    {id:'void-receiver',source:'function add(this: void, a: number, b: number): number { return a + b; }\n',
      invocation:'[add(2, 3), add.length]',expectedValue:[5,2],portable:true},
    {id:'sole-receiver',source:'function answer(this: void): number { return 7; }\n',
      invocation:'[answer(), answer.length]',expectedValue:[7,0],portable:true},
    {id:'trailing-comma',source:'function answer(this: void,): number { return 9; }\n',
      invocation:'[answer(), answer.length]',expectedValue:[9,0],portable:true},
    {id:'receiver-comments-and-unicode',source:'// λ\r\nfunction add( /* before */ this: void /* middle */, /* after */ 汉字: number): number { return 汉字 + 1; }\r\n',
      invocation:'[add(4), add.length]',expectedValue:[5,1],portableRefusal:'@param needs a type and a name'},
    {id:'receiver-unicode-comment-portable',source:'// λ\r\nfunction add( /* before */ this: void /* middle */, /* after */ value: number): number { return value + 1; }\r\n',
      invocation:'[add(4), add.length]',expectedValue:[5,1],portable:true},
    {id:'receiver-default-runtime-parameter',source:'function read(this: void, value: number = 3): number { return value; }\n',
      invocation:'[read(), read(8), read.length]',expectedValue:[3,8,0],portable:true},
    {id:'receiver-rest-runtime-parameter',source:'function count(this: void, ...values: number[]): number { return values.length; }\n',
      invocation:'[count(2, 3, 4), count.length]',expectedValue:[3,0]},
    {id:'receiver-destructuring-runtime-parameter',source:'function read(this: void, {value}: {value: number}): number { return value; }\n',
      invocation:'[read({value: 6}), read.length]',expectedValue:[6,1]},
    {id:'actual-receiver',source:'function read(this: {value: number}, increment: number): number { return this.value + increment; }\n',
      invocation:'[read.call({value: 5}, 2), read.length]',expectedValue:[7,1],receiverTypeRefusal:true},
    {id:'function-expression',source:'const read = function(this: {value: number}, increment: number): number { return this.value + increment; };\n',
      invocation:'[read.call({value: 5}, 3), read.length]',expectedValue:[8,1]},
    {id:'generator-function',source:'function* read(this: void, value: number) { yield value; }\n',
      invocation:'[read(5).next().value, read.length]',expectedValue:[5,1]},
    {id:'generator-expression',source:'const read = function*(this: void, value: number) { yield value; };\n',
      invocation:'[read(6).next().value, read.length]',expectedValue:[6,1]},
    {id:'runtime-this-spelling',source:'function read(thisValue: number): number { return thisValue; }\n',
      invocation:'[read(8), read.length]',expectedValue:[8,1],portable:true},
  ].map(row=>{
    const erased=eraseTypeScript(row.source);
    let typing,typingRefusal;
    try{typing=projectTypeScriptTyping(row.source).code;}catch(error){typingRefusal={reason:error.reason,span:error.span};}
    return{...row,erased,...(typing===undefined?{typingRefusal}:{typing}),
      javascript:selfTranslate(row.source,'TypeScript','JavaScript').code,
      ...(row.portable||row.portableRefusal?{rust:selfTranslate(row.source,'TypeScript','Rust').code}:{})};
  });
  const refusals=[
    'function invalid(value: number, this: void): number { return value; }',
    'function invalid(this?: void): number { return 1; }',
    'function invalid(this: void = undefined): number { return 1; }',
    'function invalid(this): number { return 1; }',
    'function invalid(this: void, this: void): number { return 1; }',
    'const invalid = (this: void, value: number): number => value;',
    'class Entry { constructor(this: void) {} }',
    'const entry = { read(this: void): number { return 1; } };',
    'function invalid(this: , value: number): number { return value; }',
  ].map(source=>{
    let error;try{eraseTypeScript(source);}catch(failure){error={reason:failure.reason,span:failure.span};}
    assert.ok(error,source);return{source,error};
  });
  const fixture={schemaVersion:1,id:'typescript-receiver-parameters',
    basis:'https://www.typescriptlang.org/docs/handbook/2/functions.html#declaring-this-in-a-function',
    oracleRanges:['>=22.15.0 <23.0.0','>=24.0.0 <25.0.0'],
    boundary:'Finite source erasure for first required annotated receivers of supported function and generator declarations/expressions. Complete concrete source restoration remains required. Methods, constructors, arrows and malformed receivers refuse in this profile. Portable receiver typing admits only void; actual receiver semantics remain separate. Generated artifacts are parity inputs, not execution observations.',
    cases,refusals};
  const file=new URL('../../parity/fixtures/typescript-receiver-parameters.lino',import.meta.url);
  if(check)checkProjectData(file,fixture);else writeProjectData(file,fixture);
  return fixture;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateTypeScriptReceiverFixtures({check:process.argv.includes('--check')});
}
