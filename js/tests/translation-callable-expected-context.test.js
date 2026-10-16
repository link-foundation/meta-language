import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {format} from 'node:util';
import vm from 'node:vm';
import test from 'node:test';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {requirePartialCallableResultInference} from '../src/translation/partial-callable-rules.js';
import {decodeTypedProgramLinks,encodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {decodeLinkNetworkBinary,encodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {callableExpectedContextCases,callableExpectedContextSummary,callableExpectedContextSurface} from './fixtures/callable-expected-context.mjs';

const observe = source => {
  const lines=[];
  vm.runInNewContext(source,{console:{log:(...values)=>lines.push(format(...values))}},{timeout:1000});
  return lines;
};

test('direct Surface callback contexts retain contextual parameters and infer actual BigInt returns through every result wrapper',()=>{
  for(const entry of callableExpectedContextCases()){
    const program=checkProgram(entry.surface),summary=callableExpectedContextSummary(program);
    assert.deepEqual(summary.type,{kind:'array',element:{kind:'int'}},entry.id+' never invents a Float return');
    assert.ok(summary.callables.length,entry.id);
    for(const callable of summary.callables){
      assert.deepEqual(callable.type,{kind:'function',params:[{kind:'float'}],ret:{kind:'int'}},entry.id);
      if(callable.k==='lambda')assert.deepEqual(callable.params[0].span,{start:30,end:37});
    }
    assert.deepEqual(observe(emitJavaScript(program).text),entry.lines,entry.id);
  }
});

test('the original incomplete Match1 callback counterexample survives strict source-free text and binary rechecks',()=>{
  const entry=callableExpectedContextCases().find(entry=>entry.id==='match1');
  const program=checkProgram(entry.surface),expected=callableExpectedContextSummary(program);
  const saved=encodeTypedProgramLinks(program);
  for(const network of[LinkNetwork.fromLino(saved.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(saved.network))]){
    const decoded=decodeTypedProgramLinks(network,saved.root);
    recheckLinkedTypedProgram(decoded,{wholeProgram:true});
    assert.deepEqual(callableExpectedContextSummary(decoded),expected);
    assert.deepEqual(observe(emitJavaScript(decoded).text),entry.lines);
  }
});

test('partial Match1 result inference still refuses incompatible actual branch returns',()=>{
  const callback={k:'match1',scrutinee:{k:'num',value:'0',type:{kind:'nat'}},cases:[
    {pattern:{k:'natZero'},body:{k:'lambda',params:[{name:'element'}],captures:[],body:{k:'num',value:'7'}}},
    {pattern:{k:'natSucc',name:'predecessor'},body:{k:'lambda',params:[{name:'element'}],captures:[],body:{k:'str',value:'different'}}},
  ]};
  assert.throws(()=>checkProgram(callableExpectedContextSurface(callback)),error=>error.kind==='type'&&/int|string/u.test(error.reason));
});

test('the complete partial callable selector translates and preserves absent versus present return contexts',()=>{
  for(const kind of['','function','array','float'])for(const present of[false,true]){
    assert.equal(requirePartialCallableResultInference(kind,present),kind==='function'&&!present);
  }
  const source=readFileSync(new URL('../src/translation/partial-callable-rules.js',import.meta.url),'utf8');
  const program=checkProgram(parseJavaScript(source,{referenceSemantics:false}));
  const emitted=emitRust(program,{arrayConsumption:true});
  assert.equal(emitted.definitions.filter(definition=>definition.includes('pub fn require_partial_callable_result_inference(')).length,1);
  const context={};
  vm.runInNewContext(emitJavaScript(program).text.replace(/^export /gmu,''),context,{timeout:1000});
  for(const kind of['function','array'])for(const present of[false,true]){
    assert.equal(context.requirePartialCallableResultInference(kind,present),requirePartialCallableResultInference(kind,present));
  }
});
