// Deferred controls. Do not import this module during the all-requirements draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {selfTranslate} from '../src/self-translation.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {lowerJavaScriptClosedClassSource,acceptJavaScriptClosedClassSourceSize,findJavaScriptClosedClassFieldEnd,readJavaScriptClosedClassPrefixEnd,renderJavaScriptClosedClassFieldDocumentation} from '../src/translation/javascript-class-source-rules.js';
import {tokenize} from '../src/translation/lexer.js';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
const fixture=readProjectData(new URL('../../parity/fixtures/javascript-closed-classes.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
const execute=(source,entry='compute')=>String(runInNewContext('"use strict";\n'+source.replace(/^export /gmu,'')+'\n'+entry+'();',{}, {timeout:1000}));
for(const row of fixture.cases)test(row.name,()=>{
  assert.equal(execute(row.source),row.value,'independent original JavaScript');
  const surface=parseJavaScript(row.source),program=checkProgram(surface),output=emitJavaScript(program),native=emitRust(program),packet=encodeTypedProgramLinks(program);
  const name=output.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
  assert.equal(execute(output.text,name),row.value);
  const pending=[surface];while(pending.length){const node=pending.pop();if(!node||typeof node!=='object')continue;
    if(node.span){assert.ok(node.span.start>=0);assert.ok(node.span.end<=row.source.length);assert.ok(node.span.start<=node.span.end);}
    for(const [key,value]of Object.entries(node))if(key!=='span'&&value&&typeof value==='object')pending.push(value);
  }
  for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
    const restored=decodeTypedProgramLinks(network,packet.root);
    assert.deepEqual(emitRust(restored),native);
    assert.equal(execute(emitJavaScript(restored).text,name),row.value);
    // Formal targets must eventually retain shared state too; unsupported
    // targets remain visible failures, never silently skipped coverage.
    for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
  }
  const translated=selfTranslate(row.source,'JavaScript','Rust',{semanticTranslation:true});
  assert.ok(translated.items.every(item=>['translated','comment','provenance'].includes(item.status)),'the whole actual class scope must translate');
  assert.ok(translated.code.includes('fn compute('));
});
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseJavaScript(row.source)),error=>error.kind===row.kind));
for(const row of fixture.malformed)test(row.name,()=>assert.throws(()=>parseJavaScript(row.source)));
for(const row of fixture.errors)test(row.name,()=>{
  const observe=source=>{
    try{runInNewContext('"use strict";\n'+source.replace(/^export /gmu,'')+'\ncompute();',{}, {timeout:1000});assert.fail('the original constructor must throw');}
    catch(error){return {errorClass:error.name,messageUnits:Array.from({length:error.message.length},(_,index)=>error.message.charCodeAt(index)),messagePresent:Object.hasOwn(error,'message')};}
  };
  const expected={errorClass:row.errorClass,messageUnits:row.messageUnits,messagePresent:row.messagePresent};
  assert.deepEqual(observe(row.source),expected);
  const program=checkProgram(parseJavaScript(row.source)),packet=encodeTypedProgramLinks(program);
  for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
    const restored=decodeTypedProgramLinks(network,packet.root);
    assert.deepEqual(observe(emitJavaScript(restored).text),expected);
    for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
  }
});
test('closed class source budgets stay explicit at every boundary',()=>{
  for(const [units,tokens,expected]of [[0,0,true],[131072,8192,true],[131073,8192,false],[131072,8193,false],[-1,0,false],[0,-1,false],[0.5,1,false],[1,0.5,false]])assert.equal(acceptJavaScriptClosedClassSourceSize(units,tokens),expected);
});
test('mismatched actual token columns cannot produce a provisional class source',()=>{
  const source='class Empty {}',tokens=tokenize(source,'JavaScript').tokens;
  const plan=lowerJavaScriptClosedClassSource(source,tokens.map(token=>token.value),[],tokens.map(token=>token.start),tokens.map(token=>token.end),'ml_');
  assert.equal(plan.changed,false);assert.equal(plan.text,source);assert.notEqual(plan.error,'');assert.deepEqual(plan.intervals,[]);
});
test('unproved external class consumers retain a precise composition obligation',()=>{
  for(const context of [{referenceSemantics:false},{moduleDirectory:[]},{literalBindings:new Map([['outer',{k:'num',value:'1'}]])},{moduleBindingNames:['outer']},{moduleScopeKnown:false}])
    assert.throws(()=>parseJavaScript(fixture.cases[0].source,context),error=>error.kind==='unsupported'&&error.reason.startsWith('JavaScript closed class context:'));
});
test('every complete preceding source graph native recipe and fixture stays exact',()=>{
  assert.equal(fixture.predecessors.filter(prior=>prior.revision==='eae050126c3df59af868552f7d1c764d968c5550').length,3);
  assert.equal(fixture.sourceModelVersion,4,'uncovered closed class fixture source model');
  assert.equal(fixture.predecessors.length,9);
  for(const prior of fixture.predecessors){
    assert.match(prior.revision,/^[a-f0-9]{40}$/u);
    const bytes=readFileSync(new URL('../../'+prior.retained,import.meta.url));
    assert.equal(bytes.length,prior.bytes);
    assert.equal(createHash('sha256').update(bytes).digest('hex'),prior.sha256);
  }
});

for(const [index,row]of fixture.fieldEnds.entries())test('public class field boundary '+index,()=>assert.equal(findJavaScriptClosedClassFieldEnd(row.values,row.first,row.last),row.expected));
for(const [index,row]of fixture.fieldPrefixEnds.entries())test('public fields retain actual constructor continuation '+index,()=>assert.equal(readJavaScriptClosedClassPrefixEnd(row.row),row.expected));

for(const [index,row]of fixture.fieldDocumentation.entries())test('public class field comment contract '+index,()=>assert.equal(renderJavaScriptClosedClassFieldDocumentation(row.value),row.expected));
