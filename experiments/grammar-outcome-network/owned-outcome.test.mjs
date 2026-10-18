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
import {insertOwnedGrammarOutcome,readOwnedGrammarOutcome,concreteLinksWithOwnedGrammarOutcomes} from './owned-outcome.mjs';

function composedGrammar(candidate){
 const grammar=parseGrammarLinks(nativeGrammarText('native-json'));
 return decorateGrammar(grammar,new DecoratorSet([{id:'owned-composition',level:'grammar-rule',when:[['name',grammar.startRule().name]],actions:[{op:'set',field:'composition',value:renderLinksExpression({kind:'any-grammar',item:{kind:'empty'},items:[candidate]})}]}]));
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
