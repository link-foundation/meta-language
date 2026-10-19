import assert from 'node:assert/strict';
import test from 'node:test';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {parseProgrammingLanguage} from '../../js/src/programming-language-parser.js';
import {LinkNetwork} from '../../js/src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../js/src/link-network-binary.js';
import {structuredDataGraphCost} from '../../js/src/structured-data-graph.js';
import {lowerRestrictedCst,archiveRestrictedCLowering,readRestrictedCLowering} from './restricted-c-cst.mjs';
const source='typedef enum { OPEN, CLOSE } Token; int stored = 5; int inspect(int parameter) { int local = parameter; local = local + stored; return local + CLOSE; }';
const lower=()=>lowerRestrictedCst(parseProgrammingLanguage(source,'C'),source,{abi:{intBits:32,sizeBits:64,charSigned:true}});

test('C archive replay keeps default traversal refusal and accepts an explicit exact caller budget',()=>{
 let shared={value:'authored'};
 for(let depth=0;depth<18;depth++)shared={left:shared,right:shared};
 const unit=lowerRestrictedCst(parseProgrammingLanguage(source,'C'),source,{
  abi:{intBits:32,sizeBits:64,charSigned:true},sourceDenominators:shared,
 });
 const packet=archiveRestrictedCLowering(unit),cost=structuredDataGraphCost(packet.network,packet.root);
 assert.ok(packet.network.links().length<1_000_000&&cost.nodes>1_000_000,'a small physical graph can have a larger expanded traversal');
 assert.throws(()=>readRestrictedCLowering(packet.network,packet.root),/traversal resource limit|expanded data resource limit/u);
 assert.throws(()=>readRestrictedCLowering(packet.network,packet.root,{nodeLimit:cost.nodes-1}),/traversal resource limit|expanded data resource limit/u);
 const restored=readRestrictedCLowering(packet.network,packet.root,{nodeLimit:cost.nodes});
 assertOwners(restored);
 assert.equal(restored.sourceDenominators,restored.loweringOptions.sourceDenominators);
 assert.equal(restored.sourceDenominators.left,restored.sourceDenominators.right);
});

function assertOwners(unit){
 const body=unit.functions[0].body,enumerator=unit.enums[0].entries[1].bindingOwner;
 assert.equal(body.find(item=>item.sourceEnumerator==='enum:CLOSE').sourceEnumeratorOwner,enumerator);
 const owners=[unit.globalInitializers.find(item=>item.operation==='declare C binding').binding,unit.functions[0].parameters[0],body.find(item=>item.operation==='declare C binding').binding];
 for(const owner of owners){
  const references=[...unit.globalInitializers,...body].filter(item=>item.operation==='address C binding'&&item.binding===owner.identity);
  assert.ok(references.length>0);for(const reference of references)assert.equal(reference.bindingOwner,owner);
 }
 assert.equal(unit.abi,unit.loweringOptions.abi);
 assert.ok(unit.obligations.some(item=>item.category==='C syntax derivation'));
 assert.equal(unit.checkedPortableProgram,null);
}

test('C instruction references retain actual enum global parameter and local owners through Links reload',()=>{
 const unit=lower();assertOwners(unit);const packet=archiveRestrictedCLowering(unit);assertOwners(readRestrictedCLowering(packet.network,packet.root));
});

test('equal cloned C owners cannot replace the actual declaration relationships',()=>{
 for(const kind of['enum','parameter','local','global']){
  const unit=lower(),body=unit.functions[0].body;
  if(kind==='enum'){
   const reference=body.find(item=>item.sourceEnumerator==='enum:CLOSE');reference.sourceEnumeratorOwner=structuredClone(reference.sourceEnumeratorOwner);
  }else{
   const owner=kind==='parameter'?unit.functions[0].parameters[0]:kind==='local'?body.find(item=>item.operation==='declare C binding').binding:unit.globalInitializers.find(item=>item.operation==='declare C binding').binding;
   const reference=body.find(item=>item.operation==='address C binding'&&item.binding===owner.identity);assert.ok(reference);reference.bindingOwner=structuredClone(owner);
  }
  const packet=archiveRestrictedCLowering(unit);assert.throws(()=>readRestrictedCLowering(packet.network,packet.root),/owner relationships/u);
 }
});

test('C source data retains shared input records and safe prototype spellings',()=>{
 const shared={marker:'shared'},extra=JSON.parse('{"__proto__":{"marker":"literal"},"constructor":"literal"}');
 const options={abi:{intBits:32,sizeBits:64,charSigned:true},extra,left:shared,right:shared};
 const unit=lowerRestrictedCst(parseProgrammingLanguage(source,'C'),source,options);
 assert.equal(unit.loweringOptions.left,unit.loweringOptions.right);
 assert.equal(Object.getPrototypeOf(unit.loweringOptions.extra),Object.prototype);
 assert.ok(Object.hasOwn(unit.loweringOptions.extra,'__proto__'));
 assert.equal(unit.loweringOptions.extra.__proto__.marker,'literal');
 const packet=archiveRestrictedCLowering(unit),restored=readRestrictedCLowering(packet.network,packet.root);
 assert.equal(restored.loweringOptions.left,restored.loweringOptions.right);
 assert.equal(restored.loweringOptions.extra.__proto__.marker,'literal');
});

test('C source data refuses hidden host values sparse arrays and structural cycles',()=>{
 let reads=0;
 const accessor={};Object.defineProperty(accessor,'hidden',{enumerable:true,get(){reads+=1;return 1;}});
 const hidden={};Object.defineProperty(hidden,'hidden',{value:1});
 const symbol={};symbol[Symbol('hidden')]=1;
 const cyclic={};cyclic.self=cyclic;
 const sparse=new Array(2),arrayField=[];arrayField.extra=1;
 for(const extra of[accessor,hidden,symbol,cyclic,sparse,arrayField,()=>1,undefined,1n,NaN]){
  assert.throws(()=>lowerRestrictedCst(parseProgrammingLanguage(source,'C'),source,{abi:{intBits:32,sizeBits:64,charSigned:true},extra}),TypeError);
 }
 assert.equal(reads,0,'no opaque getter supplies source facts');
});

test('complete C lowering text and binary reloads retain original source data and declaration relationships',()=>{
 const unit=lower(),packet=archiveRestrictedCLowering(unit);
 const text=packet.network.toLino(),network=LinkNetwork.fromLino(text);
 const binary=encodeLinkNetworkBinary(network);
 for(const restored of[network,decodeLinkNetworkBinary(binary)]){
  const actual=readRestrictedCLowering(restored,packet.root);
  assert.deepEqual(actual,unit);
  assertOwners(actual);
  assert.equal(actual.sourceCst.tokens.map(token=>token.text).join(''),source);
  assert.equal(actual.checkedPortableProgram,null);
 }
});

test('upstream C scanner lowering refuses local use before archive discovery or filesystem output',()=>{
 const result=spawnSync(process.execPath,[fileURLToPath(new URL('./capture-markdown.mjs',import.meta.url)),'--cargo-cache','--receipt','absent-capture.lino','--out','absent-lowering.lino'],{
  env:{...process.env,CI:'false',GITHUB_ACTIONS:'false'},encoding:'utf8',timeout:10_000,maxBuffer:16*1024,
 });
 assert.equal(result.error,undefined);
 assert.equal(result.status,1);
 assert.match(result.stderr,/complete upstream scanner CST lowering is CI-only; run authored controls locally/u);
 assert.doesNotMatch(result.stderr,/Pinned Markdown crate archive is absent|ENOENT/u);
});

test('C lowering refuses cyclic source trees and host accessors before observing any source facts',()=>{
 const parsed=parseProgrammingLanguage(source,'C');
 let reads=0;
 const opaque={};Object.defineProperty(opaque,'canonical',{enumerable:true,get(){reads++;return 'C';}});
 const options={};Object.defineProperty(options,'abi',{enumerable:true,get(){reads++;return {intBits:32};}});
 const cyclic=structuredClone(parsed);cyclic.tree.children.push(cyclic.tree);
 for(const [tree,input]of[[opaque,{}],[parsed,options],[cyclic,{}]]){
  assert.throws(()=>lowerRestrictedCst(tree,source,input),TypeError);
 }
 for(const end of[.5,NaN,Infinity,Buffer.byteLength(source)+1]){
  const changed=structuredClone(parsed);changed.tokens[0].span.byteRange.end=end;
  assert.throws(()=>lowerRestrictedCst(changed,source,{abi:{intBits:32,sizeBits:64,charSigned:true}}),Number.isFinite(end)?/source byte/u:/supported data values/u);
 }
 assert.equal(reads,0);
 assert.throws(()=>lowerRestrictedCst(parsed,{toString(){reads++;return source;}}),/source text/u);
 assert.equal(reads,0);
});
