import assert from 'node:assert/strict';
import test from 'node:test';
import {LinkNetwork} from '../../js/src/network.js';
import {LinkType,idKey} from '../../js/src/primitives.js';
import {ParserRegistry} from '../../js/src/parser-registry.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../js/src/link-network-binary.js';
import {parseGrammarLinks} from '../../js/src/grammar-links.js';
import {nativeGrammarText} from '../../js/src/native-grammar-parser.js';
import {DecoratorSet} from '../../js/src/decorators.js';
import {decorateGrammar} from '../../js/src/grammar-decorators.js';
import {renderLinksExpression} from '../../js/src/grammar-links.js';
import {createGlobalConceptSpace,globalConceptSpaceToLinks,createGlobalConceptSpaceParser} from '../../js/src/global-concept-space.js';
import {compileGrammar} from '../../js/src/grammar.js';
import {encodeStructuredDataLinks} from '../../js/src/structured-data-encode.js';
import {insertOwnedGrammarOutcome,readOwnedGrammarOutcome,concreteLinksWithOwnedGrammarOutcomes} from './owned-outcome.mjs';

function composedGrammar(candidate){
 const grammar=parseGrammarLinks(nativeGrammarText('native-json'));
 return decorateGrammar(grammar,new DecoratorSet([{id:'owned-composition',level:'grammar-rule',when:[['name',grammar.startRule().name]],actions:[{op:'set',field:'composition',value:renderLinksExpression({kind:'any-grammar',item:{kind:'empty'},items:Array.isArray(candidate)?candidate:[candidate]})}]}]));
}

function archivedFixture(){
 const field=name=>({kind:'grammar-field-candidate',name,mapping:'field-ranges',selector:{kind:'source-field-selector',label:'value',mode:'include',first:0,count:1}});
 const grammars=[composedGrammar(field('Quoted')),composedGrammar({kind:'grammar-candidate',name:'Inner',mapping:'json-string'}),composedGrammar(field('Leaf')),parseGrammarLinks(nativeGrammarText('native-json'))];
 const names=['Host','Quoted','Inner','Leaf'],sources=grammars.map((grammar,index)=>({id:names[index],language:names[index],grammar}));
 const space=createGlobalConceptSpace(sources,{records:[]}),archive=globalConceptSpaceToLinks(space);
 for(const grammar of grammars)grammar.rules.clear();sources.length=0;space.sources.length=0;space.concepts.length=0;
 const source='{"k":"{\\"k\\":2}"}';
 const parser=createGlobalConceptSpaceParser(archive,'Host',{records:[]});
 const outcome=parser.parseTree(source);assert.equal(outcome.ok,true,JSON.stringify(outcome.rejection));
 return {source,outcome,parser};
}

function attachedFixture(){
 const {source,outcome,parser}=archivedFixture();let association,owner,tokens;
 const registry=new ParserRegistry().register('OwnedGrammar',(text)=>{
  const network=LinkNetwork.parse(text,'JSON');
  tokens=network.links().filter(link=>link.metadata().linkType===LinkType.SourceToken).map(link=>link.id());
  owner=network.links().find(link=>link.metadata().linkType===LinkType.Syntax&&link.metadata().span?.byteRange.start===0&&link.metadata().span?.byteRange.end===Buffer.byteLength(text)).id();
  association=insertOwnedGrammarOutcome(network,owner,tokens,parser.parseTree(text));
  return network;
 });
 const network=registry.parse(source,'OwnedGrammar');return {source,outcome,network,owner,tokens,association};
}

test('a public parser-registry extension owns the complete source-free mapped result without adding physical tokens',()=>{
 const {source,outcome,network,owner,tokens,association}=attachedFixture();
 assert.equal(network.reconstructText(),source);
 assert.equal(network.links().filter(link=>link.metadata().linkType===LinkType.SourceToken).length,tokens.length);
 const read=readOwnedGrammarOutcome(network,association);
 assert.equal(idKey(read.owner),idKey(owner));assert.deepEqual(read.tokens,tokens);assert.deepEqual(read.outcome,outcome);
 assert.deepEqual(read.outcome.sourcePlanes.map(plane=>plane.derivation?.operation??null),[null,'field-ranges','json-string','field-ranges']);
 for(const restored of [LinkNetwork.fromLino(network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))]){
  assert.equal(restored.reconstructText(),source);assert.deepEqual(readOwnedGrammarOutcome(restored,association).outcome,outcome);
  const concrete=new Set(concreteLinksWithOwnedGrammarOutcomes(restored).map(link=>idKey(link.id())));
  for(const member of read.members)assert.equal(concrete.has(idKey(member)),true);
 }
 // Current public projection still exposes the integration gap explicitly.
 const current=new Set(network.projectedLinks().map(link=>idKey(link.id())));
 assert.equal(read.members.some(member=>!current.has(idKey(member))),true);
});

test('owned payloads refuse foreign source bytes closed-graph edits and mismatched derivations',()=>{
 const {network,association,tokens}=attachedFixture();
 const stored=network.toLino(),original=readOwnedGrammarOutcome(network,association);
 const edited=LinkNetwork.fromLino(stored);edited.setTerm(tokens[0],'!');
 assert.throws(()=>readOwnedGrammarOutcome(edited,association),/physical token bytes|physical bytes differ/);
 for(const change of ['absent-member','duplicate-member','foreign-member','absent-owner','absent-source']){
  const variant=LinkNetwork.fromLino(stored),link=variant.link(association),refs=[...link.references()];
  if(change==='absent-member')refs.pop();
  if(change==='duplicate-member')refs.push(refs.at(-1));
  if(change==='foreign-member')refs.push(tokens[0]);
  if(change==='absent-owner')variant.deleteLink(refs[0]);
  if(change==='absent-source')variant.deleteLink(refs[1]);
  link.setReferences(refs);assert.throws(()=>readOwnedGrammarOutcome(variant,association),TypeError,change);
 }
 const forged=structuredClone(original.outcome);forged.sourcePlanes[3].derivation.input=0;
 const count=network.len();assert.throws(()=>insertOwnedGrammarOutcome(network,original.owner,tokens,forged),TypeError);assert.equal(network.len(),count);
 const getter={...original.outcome};let reads=0;Object.defineProperty(getter,'tree',{enumerable:true,get(){reads++;throw new Error('host getter executed');}});
 assert.throws(()=>insertOwnedGrammarOutcome(network,original.owner,tokens,getter),/non-data field/);assert.equal(reads,0);assert.equal(network.len(),count);
 const weak={...original.outcome,sourcePlanes:undefined};assert.throws(()=>insertOwnedGrammarOutcome(network,original.owner,tokens,weak),TypeError);assert.equal(network.len(),count);
});

test('owned transport preserves actual tree aliases signed zero and explicit resource refusal',()=>{
 const {network,association,tokens}=attachedFixture(),original=readOwnedGrammarOutcome(network,association);
 // An authored transport witness references the actual parser root twice;
 // it does not replace a production tree or claim extra grammar semantics.
 const twin={root:original.outcome.tree},outcome={...original.outcome,
  transportWitness:{first:twin,second:twin,independent:{root:structuredClone(original.outcome.tree)},negativeZero:-0}};
 const count=network.len();
 for(const options of [{codec:{nodeLimit:1}},{codec:{depthLimit:1}},{codec:{byteLimit:1}},{mapping:{stepLimit:0}},{mapping:{memoryLimit:0}},{mapping:{maxDepth:0}}]){
  assert.throws(()=>insertOwnedGrammarOutcome(network,original.owner,tokens,outcome,options));assert.equal(network.len(),count);
 }
 const attached=insertOwnedGrammarOutcome(network,original.owner,tokens,outcome);
 for(const restored of [network,LinkNetwork.fromLino(network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))]){
  const actual=readOwnedGrammarOutcome(restored,attached).outcome,witness=actual.transportWitness;
  assert.deepEqual(actual,outcome);assert.equal(witness.first,witness.second);assert.equal(witness.first.root,actual.tree);
  assert.notEqual(witness.first,witness.independent);assert.notEqual(witness.first.root,witness.independent.root);assert.ok(Object.is(witness.negativeZero,-0));
 }
});

test('owned transport retains actual ambiguous acceptance and rejection with every decoded source plane',()=>{
 const field=name=>({kind:'grammar-field-candidate',name,mapping:'field-ranges',selector:{kind:'source-field-selector',label:'value',mode:'include',first:0,count:1}});
 const grammars=[composedGrammar([field('Quoted'),field('QuotedTwin')]),composedGrammar({kind:'grammar-candidate',name:'Inner',mapping:'json-string'}),composedGrammar(field('Leaf')),parseGrammarLinks(nativeGrammarText('native-json'))];
 const names=['Host','Quoted','Inner','Leaf'];
 const sources=grammars.map((grammar,index)=>({id:names[index],language:names[index],grammar}));
 sources.push({id:'QuotedTwin',language:'QuotedTwin',grammar:grammars[1]});
 const archive=globalConceptSpaceToLinks(createGlobalConceptSpace(sources,{records:[]}));
 for(const grammar of grammars)grammar.rules.clear();sources.length=0;
 const parser=createGlobalConceptSpaceParser(archive,'Host',{records:[]});
 const source='{"k":"{\\"k\\":2}"}',network=LinkNetwork.parse(source,'JSON');
 const tokens=network.links().filter(link=>link.metadata().linkType===LinkType.SourceToken).map(link=>link.id());
 const owner=network.links().find(link=>link.metadata().linkType===LinkType.Syntax&&link.metadata().span?.byteRange.start===0&&link.metadata().span?.byteRange.end===Buffer.byteLength(source)).id();
 for(const [options,accepted]of [[{},true],[{ambiguity:'reject'},false]]){
  const outcome=parser.parseTree(source,options);
  assert.equal(outcome.ok,accepted);assert.ok(outcome.tree);assert.ok(outcome.ambiguities.length>0);
  assert.ok(outcome.ambiguities.some(item=>item.languages?.includes('QuotedTwin')));
  assert.ok(outcome.sourcePlanes.length>=4);
  if(accepted)assert.equal(outcome.rejection,null);
  else assert.equal(outcome.rejection.reason,'ambiguity');
  const association=insertOwnedGrammarOutcome(network,owner,tokens,outcome);
  for(const restored of [network,LinkNetwork.fromLino(network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))]){
   assert.equal(restored.reconstructText(),source);
   assert.deepEqual(readOwnedGrammarOutcome(restored,association).outcome,outcome);
  }
 }
});

test('equal bytes cannot substitute unrelated owners or foreign physical token identities',()=>{
 const {network,owner,tokens,association}=attachedFixture();
 const original=readOwnedGrammarOutcome(network,association);
 const unrelated=network.insertSyntaxNode('JSON','unrelated',[]);
 const donor=network.insertLink([],network.link(tokens[0]).metadata().clone());
 const foreign=[donor,...tokens.slice(1)],count=network.len();
 assert.throws(()=>insertOwnedGrammarOutcome(network,unrelated,tokens,original.outcome),/belong to the concrete owner/);
 assert.throws(()=>insertOwnedGrammarOutcome(network,owner,foreign,original.outcome),/belong to the concrete owner/);
 for(const edit of ['owner','physical-token']){
  const restored=LinkNetwork.fromLino(network.toLino()),link=restored.link(association),refs=[...link.references()];
  if(edit==='owner'){refs[0]=unrelated;link.setReferences(refs);}
  else {const manifest=restored.link(refs[1]);manifest.setReferences(foreign);}
  assert.throws(()=>readOwnedGrammarOutcome(restored,association),/belong to the concrete owner/);
 }
 const shifted=LinkNetwork.fromLino(network.toLino());
 for(const token of tokens){
  const span=shifted.link(token).metadata().span.clone();span.byteRange.start+=7;span.byteRange.end+=7;shifted.setSpan(token,span);
 }
 assert.throws(()=>readOwnedGrammarOutcome(shifted,association),/physical token intervals must be contiguous/);
 for(const changes of [{ok:'true'},{ok:false},{rejection:{reason:'forged'}},{ambiguities:null},{tree:null}]){
  assert.throws(()=>insertOwnedGrammarOutcome(network,owner,tokens,{...original.outcome,...changes}),/complete grammar outcome/);
 }
 assert.equal(network.len(),count);
 assert.deepEqual(readOwnedGrammarOutcome(network,association).outcome,original.outcome);
});

test('owned transport bounds physical expansion and refuses oversized membership before cloning payloads',()=>{
 const {network,association}=attachedFixture();
 const members=readOwnedGrammarOutcome(network,association).members;
 const withoutPlanes={...readOwnedGrammarOutcome(network,association).outcome};
 delete withoutPlanes.sourcePlanes;
 const originalCount=network.len();
 const original=readOwnedGrammarOutcome(network,association);
 assert.throws(()=>insertOwnedGrammarOutcome(network,original.owner,original.tokens,withoutPlanes),TypeError);
 assert.equal(network.len(),originalCount);
 let clones=0;
 const metadata=members.map(id=>network.link(id).metadata());
 for(const value of metadata)value.clone=()=>{clones++;throw new Error('payload clone executed before membership limit');};
 try{
  assert.throws(()=>readOwnedGrammarOutcome(network,association,{codec:{nodeLimit:1}}),/owned payload node resource limit/);
  assert.equal(clones,0);
 }finally{
  for(const value of metadata)delete value.clone;
 }
 const source='['.repeat(12)+JSON.stringify(Array(4).fill('x'.repeat(4096)))+']'.repeat(12);
 const outcome=compileGrammar(parseGrammarLinks(nativeGrammarText('native-json'))).parseTree(source);
 assert.equal(outcome.ok,true);
 assert.equal(Object.hasOwn(outcome,'sourcePlanes'),false);
 const codec={byteLimit:8192};
 assert.ok(Buffer.byteLength(source)>codec.byteLimit);
 // Actual repeated token strings fit the encoder's unique-scalar budget.
 // The existing decoder must still refuse their larger expanded value before
 // mapping or physical extraction; nested containers keep original bytes.
 assert.ok(encodeStructuredDataLinks(outcome,codec).network.links().length>0);
 const host=LinkNetwork.parse(source,'JSON');
 const tokens=host.links().filter(link=>link.metadata().linkType===LinkType.SourceToken).map(link=>link.id());
 const owner=host.links().find(link=>link.metadata().linkType===LinkType.Syntax&&link.metadata().span?.byteRange.start===0&&link.metadata().span?.byteRange.end===Buffer.byteLength(source)).id();
 const count=host.len();
 assert.throws(()=>insertOwnedGrammarOutcome(host,owner,tokens,outcome,{codec}),/expanded data resource limit/);
 assert.equal(host.len(),count);
 assert.equal(host.reconstructText(),source);
 const attached=insertOwnedGrammarOutcome(host,owner,tokens,outcome);
 assert.deepEqual(readOwnedGrammarOutcome(host,attached).outcome,outcome);
 for(const restored of [LinkNetwork.fromLino(host.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(host))]){
  assert.equal(restored.reconstructText(),source);
  assert.throws(()=>readOwnedGrammarOutcome(restored,attached,{codec}),/expanded data resource limit/);
  assert.deepEqual(readOwnedGrammarOutcome(restored,attached).outcome,outcome);
 }
});
