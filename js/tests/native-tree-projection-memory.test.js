import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import vm from 'node:vm';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {compileGrammar} from '../src/grammar.js';
import {parseGrammarLinks} from '../src/grammar-links.js';
import {languageEntry, LANGUAGE_CATALOG} from '../src/language-catalog.js';
import {parseNative, nativeGrammarText} from '../src/native-grammar-parser.js';

const controls=readProjectData(new URL('../../parity/fixtures/native-tree-projection-controls.lino',import.meta.url));
const current=readFileSync(new URL('../src/native-grammar-parser.js',import.meta.url),'utf8');

function extractNativeProjection(source){
 const start=source.indexOf('function projectNativeTree('),end=source.indexOf('/** The adapter',start);
 assert.ok(start>=0&&end>start);return source.slice(start,end);
}
function readProjectionBaseline(value){
 assert.equal(value.version,1);assert.equal(value.baseline.revision,'d9f7984ebd39f043bc9e18922d667d12026cd10e');
 assert.equal(value.baseline.file,'js/src/native-grammar-parser.js');assert.equal(value.baseline.bytes,7621);
 assert.equal(value.baseline.sha256,'1b70fbbd032a6da6da16239e48b8f26fa38f44344efb9d7c29589907e3b9dec3');
 assert.equal(Buffer.byteLength(value.baseline.source),value.baseline.bytes);
 assert.equal(createHash('sha256').update(value.baseline.source).digest('hex'),value.baseline.sha256);
 return extractNativeProjection(value.baseline.source);
}
function observeNativeProjection(source,input,profile,length){
 const instrumented=source.replace('const hoist = (node) => {','const hoist = (node) => { hoistCalls++;')
  .replace('const span = (node) => {','const span = (node) => { spanCalls++;');
 const context=vm.createContext({input,profile,length,result:null,counts:null});
 vm.runInContext("'use strict';let hoistCalls=0,spanCalls=0;"+instrumented+'result=projectNativeTree(input,length,profile);counts={hoistCalls,spanCalls};',context,{timeout:2000});
 return {result:structuredClone(context.result),counts:structuredClone(context.counts)};
}
function createProjectionControlTree(depth){
 const leaf=(type,kind,start,end,extra={})=>({type,kind,start,end,text:kind??' ',trivia:false,...extra});
 const branch=(children,start=0,end=150,extra={})=>({type:'node',kind:'group',start,end,children,...extra});
 let nested=branch([leaf('token',null,1,2,{trivia:true}),leaf('token','value',2,4,{field:'actual'}),leaf('missing','hidden',4,4,{literal:true}),leaf('token','comment',4,5,{trivia:true})],1,5);
 for(let index=0;index<depth;index++)nested=branch([leaf('token',null,0,1,{trivia:true}),nested],0,5,{field:'nested'});
 return branch([leaf('token','hidden',0,1),leaf('token',null,1,2,{trivia:true}),nested,branch([],5,5,{kind:'comment'}),leaf('token','punctuation',5,6),leaf('error',null,6,7),leaf('missing','value',7,7,{literal:false}),leaf('token',"'aliased",7,8)],0,200,{kind:'native-root'});
}
function freezeProjectionControl(value){
 if(value&&typeof value==='object'){Object.values(value).forEach(freezeProjectionControl);Object.freeze(value);}return value;
}

test('native tree projection preserves frozen controls with bounded normalization and span visits',()=>{
 const baseline=readProjectionBaseline(controls),actual=extractNativeProjection(current);
 assert.deepEqual(controls.cases,[8,16,32,64].flatMap(depth=>[false,true].map(rootIncludesLeadingTrivia=>({depth,rootIncludesLeadingTrivia}))));
 for(const {depth,rootIncludesLeadingTrivia} of controls.cases){
  const input=freezeProjectionControl(createProjectionControlTree(depth)),before=JSON.stringify(input);
  const profile={hidden:['hidden'],anonymous:['punctuation'],extras:['comment'],oracleKinds:{'native-root':'root',group:'expression'},rootIncludesLeadingTrivia};
  const old=observeNativeProjection(baseline,input,profile,200),observed=observeNativeProjection(actual,input,profile,200);
  assert.deepEqual(observed.result,old.result,'all ordered children, fields, spans, names and error/trivia flags');assert.equal(JSON.stringify(input),before);
  const stack=[input];let nodes=0;while(stack.length){const node=stack.pop();nodes++;if(node.type==='node')stack.push(...node.children);}
  assert.equal(observed.counts.hoistCalls,nodes-1,'normalize each non-root input node once');
  assert.ok(observed.counts.spanCalls<=nodes*2,'span visits stay bounded by the actual control tree');
  assert.ok(old.counts.hoistCalls>observed.counts.hoistCalls);assert.ok(old.counts.spanCalls>observed.counts.spanCalls);
 }
});

test('native parsing retains frozen projected trees for small Rust JavaScript and JavaScript Object Notation sources',()=>{
 const baseline=readProjectionBaseline(controls);assert.equal(controls.productionCases.length,3);
 for(const {language,source} of controls.productionCases){
  assert.ok(Buffer.byteLength(source)<128);const id=languageEntry(language).grammars[0].id;
  const parser=compileGrammar(parseGrammarLinks(nativeGrammarText(id))),outcome=parser.parseTree(source,{errorRecovery:true,recovery:'accept'});
  assert.equal(outcome.ok,true,language);assert.equal(outcome.tree.type,'node');
  const profile={...LANGUAGE_CATALOG.nativeGrammars[id],rootIncludesLeadingTrivia:LANGUAGE_CATALOG.nativeGrammars[id].rootIncludesLeadingTrivia??false};
  const expected=observeNativeProjection(baseline,outcome.tree,profile,Buffer.byteLength(source)).result;
  assert.deepEqual(parseNative(id,source),expected,language+' actual complete projected tree');
 }
});

test('native projection controls reject changed baseline source and false provenance',()=>{
 readProjectionBaseline(controls);
 for(const edit of[value=>{value.baseline.source+='\n';},value=>{value.baseline.sha256='0'.repeat(64);},value=>{value.baseline.revision='0'.repeat(40);},value=>{value.baseline.bytes++;}]){
  const changed=structuredClone(controls);edit(changed);assert.throws(()=>readProjectionBaseline(changed));
 }
});
