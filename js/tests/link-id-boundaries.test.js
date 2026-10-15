import assert from 'node:assert/strict';
import test from 'node:test';
import {LinkId,LinkMetadata,idKey} from '../src/primitives.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkStatement,LinoSerializationError} from '../src/lino-serialization.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';

const unsafe=[9007199254740992n,9007199254740993n,18446744073709551615n];
const canonical=(id,term='value',references=[])=>`(${id}:${references.length?' '+references.join(' '):''} (meta: (t: concept) (n: 1) (term: ${term})))`;

test('LinkId refuses unsafe original BigInt, decimal tokens and fractional strings without collapsing identities',()=>{
 for(const value of unsafe)for(const input of[value,String(value),Number(value)])for(const create of[value=>new LinkId(value),LinkId.from,LinkId.fromU64])assert.throws(()=>create(input),TypeError);
 for(const value of['9007199254740991.1','1.00000000000000001','1.5','1e2','0x10','',true,[1],{valueOf(){throw new Error('coercion must not run');}},0,-0,-1,NaN,Infinity])assert.throws(()=>new LinkId(value),TypeError);
 for(const value of[1,2,Number.MAX_SAFE_INTEGER,'1','0002',' 0002\t',String(Number.MAX_SAFE_INTEGER),1n,BigInt(Number.MAX_SAFE_INTEGER)])assert.equal(new LinkId(value).asU64(),Number(value));
 const original=LinkId.from(2);assert.equal(LinkId.from(original),original);assert.equal(new LinkId(original).equals(2),true);assert.equal(original.equals('0002'),true);
});

test('mutated unsafe LinkId instances cannot bypass validation at inbound identity boundaries',()=>{
 const network=new LinkNetwork(),stored=network.insertLink([],LinkMetadata.new().withTerm('original')),before=network.toLino(),mutated=LinkId.from(2);mutated.value=9007199254740992;
 for(const operation of[()=>new LinkId(mutated),()=>LinkId.from(mutated),()=>LinkId.fromU64(mutated),()=>mutated.asU64(),()=>mutated.equals(2),()=>LinkId.from(2).equals(mutated),()=>Number(mutated),()=>String(mutated),()=>JSON.stringify(mutated),()=>network.link(mutated),()=>network.insertLinkWithOptionalId(mutated),()=>network.insertLink([mutated]),()=>network.link(stored).setReferences([mutated]),()=>encodeLinkStatement(mutated,[],LinkMetadata.new())]){assert.throws(operation,TypeError);assert.equal(network.toLino(),before);}
});

test('existing LinkId instances require an actual Number for symmetric equality and consistent lookup keys',()=>{
 const network=new LinkNetwork(),normal=network.insertLink(),before=network.toLino();
 for(const value of['1',1n]){const mutated=LinkId.from(1);mutated.value=value;
  for(const operation of[()=>new LinkId(mutated),()=>LinkId.from(mutated),()=>mutated.asU64(),()=>mutated.equals(normal),()=>normal.equals(mutated),()=>idKey(mutated),()=>network.link(mutated),()=>network.insertLinkWithOptionalId(mutated)])assert.throws(operation,TypeError);
  assert.equal(network.toLino(),before);assert.equal(network._nextId,2);
 }
 assert.equal(LinkId.from('1').equals(normal),true);assert.equal(normal.equals(LinkId.from(1n)),true);assert.equal(idKey(LinkId.from('1')),idKey(normal));
});

test('rejected preferred and automatic insertion preserve allocator and later normal allocation',()=>{
 for(const preferred of[Number.MAX_SAFE_INTEGER,undefined]){
  const network=new LinkNetwork();network.insertLink([],LinkMetadata.new().withTerm('original'));const before=network.toLino(),bytes=encodeLinkNetworkBinary(network);
  for(let attempt=0;attempt<2;attempt++){
   assert.throws(()=>network.insertLinkWithOptionalId(preferred,[9007199254740993n]),TypeError);assert.equal(network._nextId,2);assert.equal(network.len(),1);assert.equal(network.toLino(),before);assert.deepEqual(encodeLinkNetworkBinary(network),bytes);
  }
  assert.equal(network.insertLink([],LinkMetadata.new().withTerm('next')).asU64(),2);assert.equal(network._nextId,3);assert.equal(network.len(),2);assert.equal(network.link(1).metadata().term,'original');
 }
});

test('canonical and plain LiNo reject both colliding original IDs and unsafe reference tokens',()=>{
 const pairs=[['9007199254740992','9007199254740993'],['9007199254740991','9007199254740993']];
 for(const[first,second]of pairs){assert.throws(()=>LinkNetwork.fromLino(canonical(first,'first')+'\n'+canonical(second,'second')),LinoSerializationError);assert.throws(()=>LinkNetwork.fromLino(`(${first}: 1)\n(${second}: 2)`),TypeError);}
 for(const value of unsafe){assert.throws(()=>LinkNetwork.fromLino(canonical(1,'reference',[value])),LinoSerializationError);assert.throws(()=>LinkNetwork.fromLino(`(1: ${value})`),TypeError);}
 assert.throws(()=>LinkNetwork.fromLino(canonical('9007199254740991.1')),LinoSerializationError);
});

test('direct network insertion and references fail without replacing an existing ordinary link',()=>{
 const network=new LinkNetwork(),id=network.insertLink([],LinkMetadata.new().withTerm('original')),before=network.toLino();
 for(const value of unsafe){assert.throws(()=>network.insertLinkWithOptionalId(value,[],LinkMetadata.new().withTerm('counterfeit')),TypeError);assert.equal(network.toLino(),before);assert.throws(()=>network.link(id).setReferences([value]),TypeError);assert.equal(network.toLino(),before);assert.throws(()=>network.link(value),TypeError);}
 assert.equal(network.link(id).metadata().term,'original');assert.equal(network.len(),1);
});

test('the maximum safe allocation is unique and exhaustion never overwrites it or advances the exhausted allocator',()=>{
 const network=new LinkNetwork();network.insertLinkWithOptionalId(Number.MAX_SAFE_INTEGER-1,[],LinkMetadata.new().withTerm('penultimate'));
 const last=network.insertLink([],LinkMetadata.new().withTerm('last'));assert.equal(last.asU64(),Number.MAX_SAFE_INTEGER);assert.equal(network._nextId,Number.MAX_SAFE_INTEGER+1);
 const before=network.toLino();for(let count=0;count<3;count++){assert.throws(()=>network.insertLink(),TypeError);assert.equal(network.toLino(),before);assert.equal(network._nextId,Number.MAX_SAFE_INTEGER+1);}
 assert.equal(network.len(),2);assert.equal(network.link(last).metadata().term,'last');
 const explicit=new LinkNetwork();explicit.insertLinkWithOptionalId(Number.MAX_SAFE_INTEGER,[],LinkMetadata.new().withTerm('explicit'));assert.throws(()=>explicit.insertLink(),TypeError);assert.equal(explicit.len(),1);assert.equal(explicit.link(Number.MAX_SAFE_INTEGER).metadata().term,'explicit');
 assert.throws(()=>encodeLinkNetworkBinary(explicit),/unsafe or invalid next identity/u);
});

test('text writers and link-cli numeric targets validate original tokens before coercion or matching',()=>{
 for(const value of unsafe){assert.throws(()=>encodeLinkStatement(value,[],LinkMetadata.new()),TypeError);assert.throws(()=>encodeLinkStatement(1,[value],LinkMetadata.new()),TypeError);}
 const network=new LinkNetwork();network.insertLinkWithOptionalId(Number.MAX_SAFE_INTEGER,[1]);const before=network.toLino();
 for(const value of[...unsafe,'9007199254740991.1']){assert.throws(()=>network.applyLinkCliSubstitutionText(`((${value}: 1)) ()`),TypeError);assert.equal(network.toLino(),before);assert.throws(()=>network.applyLinkCliSubstitutionText(`() ((${value}: 1))`),TypeError);assert.equal(network.toLino(),before);}
});

test('ordinary IDs, shared references and cycles retain text, binary and link-cli round trips',()=>{
 const network=LinkNetwork.fromLino(canonical('0001','first',[2,1])+'\n'+canonical(2,'second',[1]));assert.equal(network.len(),2);assert.deepEqual(network.link(1).references().map(Number),[2,1]);
 const text=network.toLino(),restored=LinkNetwork.fromLino(text);assert.equal(restored.toLino(),text);const binary=decodeLinkNetworkBinary(encodeLinkNetworkBinary(restored));assert.equal(binary.toLino(),text);assert.equal(binary.insertLink().asU64(),3);
 const cli=new LinkNetwork();assert.deepEqual(cli.applyLinkCliSubstitutionText('() ((1 1))').created().map(Number),[1]);assert.deepEqual(cli.applyLinkCliSubstitutionText('((1: 1 1)) ((1: 1 2))').updated().map(Number),[1]);assert.deepEqual(cli.link(1).references().map(Number),[1,2]);assert.deepEqual(cli.applyLinkCliSubstitutionText('((1 2)) ()').deleted().map(Number),[1]);
});
