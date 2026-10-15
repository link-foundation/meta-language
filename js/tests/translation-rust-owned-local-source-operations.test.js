import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import {rustOwnedLocalSourceOperationFixture} from '../../experiments/generate-rust-owned-local-source-operation-fixture.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {createTypedProgramLinks} from '../src/typed-program-links.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';

const file=new URL('../../experiments/rust-owned-local-source-operation-fixture.lino',import.meta.url);
const fixture=readProjectData(file);
const digest=value=>createHash('sha256').update(value).digest('hex');

test('all six original source-operation cases retain their values and actual ordered graph identities',()=>{
 assert.equal(fixture.cases.length,6);
 assert.equal(digest(JSON.stringify(fixture.cases)),'9c32354b8acee61e0efdd4f3d7ae65a64ce90ab6b4b97ef71c6697f556e01328');
 const rows=readFileSync(file,'utf8').split('\n').filter(line=>{
  const match=/^\((\d+):/u.exec(line);return match&&Number(match[1])>=2&&Number(match[1])<=104;
 });
 assert.equal(digest(rows.join('\n')+'\n'),'123e45f1ead9564aabffffdca8c03e1f3a29bd3ecbf82db0df3a40235e7fb362');
 assert.match(rows.find(line=>line.startsWith('(6: ')),/^\(6: 7 32 47 62 77 91 /u);
});

test('the generated nonempty method corpus records actual complete sources and exact original diagnostics',()=>{
 assert.deepEqual(rustOwnedLocalSourceOperationFixture(),fixture);
 assert.equal(fixture.borrowedMethodRefusals.length,7);
 const sources=new Set(fixture.cases.map(entry=>entry.source));
 for(const entry of [...fixture.cases,...fixture.borrowedMethodRefusals]){
  if(fixture.borrowedMethodRefusals.includes(entry)){assert.ok(!sources.has(entry.source));sources.add(entry.source);}
  assert.match(entry.source,/^fn bad\(text:String,count:i32,flag:bool\)->String\{.*\}fn main\(\)\{\}$/u);
  assert.throws(()=>checkProgram(parseRust(entry.source,{localValueStateModel:entry.model})),error=>{
   assert.equal(error.kind,entry.kind,entry.name);assert.equal(error.reason,entry.reason,entry.name);assert.deepEqual(error.span??null,entry.span,entry.name);return true;
  });
 }
});

test('actual public source import preserves each selected method receiver refusal and its span',()=>{
 for(const entry of fixture.borrowedMethodRefusals)assert.throws(()=>createTypedProgramLinks(entry.source,'Rust',{rustOwnedLocalStateModel:entry.model}),error=>{
  assert.equal(error.kind,entry.kind,entry.name);assert.equal(error.reason,entry.reason,entry.name);assert.deepEqual(error.span??null,entry.span,entry.name);return true;
 });
});

test('actual owned String clones and literal constructors retain the canonical copy interpretation',()=>{
 const model=fixture.borrowedMethodRefusals.find(entry=>entry.name==='canonical string view').model;
 for(const [body,expected]of [['text.clone()','Ω😀'],['String::from("literal")','literal']]){
  const program=checkProgram(parseRust(`fn keep(text:String,count:i32,flag:bool)->String{${body}}fn main(){}`,{localValueStateModel:model}));
  const context={};vm.runInNewContext(emitJavaScript(program).text,context,{timeout:1000});assert.equal(context.keep('Ω😀',1n,false),expected);
 }
});

test('the unselected actual string slice method keeps its prior value projection',()=>{
 const program=checkProgram(parseRust('fn view(text:&String)->&str{text.as_str()}fn main(){}'));
 const context={};vm.runInNewContext(emitJavaScript(program).text,context,{timeout:1000});
 assert.equal(context.view('Ω😀'),'Ω😀');
 assert.equal(Object.hasOwn(program.items[0],'sourceLocalState'),false);
});
