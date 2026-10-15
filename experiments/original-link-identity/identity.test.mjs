import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkNetwork,createTypedProgramLinks,emitTypedProgramLinks,encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../js/src/index.js';
import {LinkId} from '../../js/src/primitives.js';
import {TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {createPortableRustTextCertificate,encodePortableRustTextCertificate} from '../rust-text-certificate/bridge.mjs';
import {decodeRustTextCertificateFacet} from '../rust-text-certificate-adapter/raw-identity.mjs';
import {originalLinkIdentityPolicy as policy,originalLinkIdentityRulesSource,exactOriginalLinkIdentity,originalLinkIdentityWords,originalLinkIdentityIndex} from './policy.mjs';
import {acceptOriginalLinkIdentityWord,readOriginalLinkIdentityOrdinals as read} from './rules.mjs';
import {generateOriginalLinkIdentity} from './generate.mjs';

const words=values=>({high:values.map(value=>originalLinkIdentityWords(value)[0]),low:values.map(value=>originalLinkIdentityWords(value)[1])});
const run=(index,queries,operation=read)=>{const wanted=words(queries);return Array.from(operation(index.high,index.low,wanted.high,wanted.low));};
const oracle=(index,queries)=>queries.map(query=>index.anchors.findIndex(row=>row.key===query)+1);
const keyValues=[0n,1n,4294967295n,4294967296n,9007199254740991n,9007199254740992n,9007199254740993n,18446744073709551614n,18446744073709551615n];
const reloads=packet=>[{...packet,network:LinkNetwork.fromLino(packet.network.toLino())},{...packet,network:decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))}];

test('exact u64 words retain maxu64 and adjacent identities above the Number safe range',()=>{
 const index=originalLinkIdentityIndex([...keyValues].reverse()),queries=[keyValues.at(-1),keyValues[6],keyValues[5],0n,keyValues[6],keyValues.at(-2)];
 assert.deepEqual(run(index,queries),oracle(index,queries));
 assert.notEqual(run(index,[keyValues[5]])[0],run(index,[keyValues[6]])[0]);
 assert.deepEqual(originalLinkIdentityWords(keyValues.at(-1)),[4294967295,4294967295]);
 for(const id of keyValues){const[high,low]=originalLinkIdentityWords(id);assert.equal((BigInt(high)<<32n)|BigInt(low),id);}
 const before=JSON.stringify([index.high,index.low]);Object.freeze(index.high);Object.freeze(index.low);run(index,queries);assert.equal(JSON.stringify([index.high,index.low]),before);
 for(const value of [-1n,18446744073709551616n,-1,0.5,NaN,Infinity,'9007199254740993',null,undefined])assert.throws(()=>exactOriginalLinkIdentity(value));
});

test('the numeric word domain refuses coercion, holes, nonfinite values and out-of-range words',()=>{
 for(const value of [0,-0,1,4294967295])assert.equal(acceptOriginalLinkIdentityWord(value),true);
 for(const value of [-1,4294967296,0.5,NaN,Infinity,-Infinity,true,false,'0',null,undefined,{},0n]){
  assert.equal(acceptOriginalLinkIdentityWord(value),false);
  assert.deepEqual(read([value],[1],[0],[1]),[-1]);assert.deepEqual(read([0],[value],[0],[1]),[-1]);
  assert.deepEqual(read([0],[1],[value],[1]),[-3,0]);assert.deepEqual(read([0],[1],[0],[value]),[-3,0]);
 }
 assert.deepEqual(read(Array(1),[1],[0],[1]),[-1]);assert.deepEqual(read([0],[1],Array(1),[1]),[-3,0]);
});

test('the index is unique and ordered while query order and repeated aliases stay observable',()=>{
 assert.deepEqual(read([0,0],[1,1],[0],[1]),[-1]);assert.deepEqual(read([1,0],[0,1],[0],[1]),[-1]);
 assert.deepEqual(read([0,0],[2,1],[0],[1]),[-1]);assert.deepEqual(read([0],[],[],[]),[-1]);
 assert.deepEqual(read([0],[1],[0],[]),[-2]);assert.deepEqual(read([0],[1],[0,0],[1,2]),[-4,1]);
 const index=originalLinkIdentityIndex(keyValues),queries=[keyValues[6],keyValues[1],keyValues[6],keyValues[5],keyValues[1]];
 assert.deepEqual(run(index,queries),[7,2,7,6,2]);assert.deepEqual(run(index,[...queries].reverse()),[2,6,7,2,7]);
 assert.deepEqual(read([],[],[],[]),[]);assert.deepEqual(read([],[],[0],[0]),[-4,0]);assert.deepEqual(read([0],[0],[],[]),[]);
});

test('deterministic exact BigInt keys independently verify all sorted foreign-key handles',()=>{
 const values=new Set(keyValues);let value=1234567890123456789n;
 for(let index=0;index<2048;index++){value=(value*6364136223846793005n+1442695040888963407n)&18446744073709551615n;values.add(value);}
 const index=originalLinkIdentityIndex([...values]),queries=[...values].reverse().filter((_,position)=>position%3===0);
 queries.push(queries[0],queries[1],queries[0]);assert.deepEqual(run(index,queries),oracle(index,queries));
});

test('bounded fixed-count lookup covers the full row limit without mutating index or query vectors',()=>{
 const count=policy.maximumRows,high=Array(count).fill(0),low=Array.from({length:count},(_,index)=>index),queries=[...low].reverse();
 const output=read(high,low,high,queries);assert.equal(output.length,count);assert.equal(output[0],count);assert.equal(output.at(-1),1);
 assert.deepEqual(read([...high,0],[...low,count],[],[]),[-1]);assert.deepEqual(read([],[],Array(count+1).fill(0),Array(count+1).fill(0)),[-2]);
 assert.throws(()=>originalLinkIdentityIndex(Array(count+1).fill(0n)),/row bound/u);
 assert.equal(low[0],0);assert.equal(low.at(-1),count-1);assert.equal(queries[0],count-1);
});

test('actual raw certificate roles retain original LinkId anchors and distinct equal-valued owners',()=>{
 const fixtures=readProjectData(new URL('../rust-text-value-projection/data/fixtures.lino',import.meta.url));
 const fixture=fixtures.cases.find(row=>row.id==='equal-valued-distinct-owners');
 const value=createPortableRustTextCertificate(fixture.source),original=encodePortableRustTextCertificate(value.program,value.proof);
 const label=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
 const field=(network,parent,name)=>{const row=network.fieldRelations().find(row=>row.parent.equals(parent)&&row.label===label(name));assert.ok(row,name);return row;};
 const wrapper=field(original.network,original.root,'provenance').child,proof=field(original.network,wrapper,'sourceInterpretation').child;
 original.network.insertFieldRelation(original.network.link(original.root).references()[0],label('sourceTextValueProjection'),proof);
 for(const packet of reloads(original)){
  const before=packet.network.toLino(),proof=field(packet.network,field(packet.network,packet.root,'provenance').child,'sourceInterpretation').child;
  const functions=packet.network.link(field(packet.network,proof,'functions').child).references();
  const roles=['declaration','signature','resultPosition','operationNode','coreValue','input','position'];
  const first=roles.map(role=>field(packet.network,functions[0],role).child),second=roles.map(role=>field(packet.network,functions[1],role).child);
  const index=originalLinkIdentityIndex(packet.network.links().map(link=>link.id())),queries=[...first,...second,first[0],first[3]],result=run(index,queries);
  for(let position=0;position<queries.length;position++)assert.equal(index.anchors[result[position]-1].original,packet.network.link(queries[position]).id());
  for(let position=0;position<roles.length;position++)assert.notEqual(result[position],result[position+roles.length],roles[position]);
  assert.equal(result[0],result.at(-2));assert.equal(result[3],result.at(-1));assert.equal(decodeRustTextCertificateFacet(packet.network,packet.root).decision[0],1);
  assert.equal(packet.network.toLino(),before);
  // Exact membership is deliberately weaker than actual source ownership.
  const target=field(packet.network,functions[0],'input');target.link.setReferences([target.parent,target.link.references()[1],second[5]]);
  const changed=originalLinkIdentityIndex(packet.network.links().map(link=>link.id()));assert.ok(run(changed,[second[5]])[0]>0);
  assert.throws(()=>decodeRustTextCertificateFacet(packet.network,packet.root));
 }
});

test('the complete kernel executes after public text and binary Links reload with the same identities',()=>{
 const packet=createTypedProgramLinks(originalLinkIdentityRulesSource(),'JavaScript'),index=originalLinkIdentityIndex(keyValues),queries=[keyValues[6],keyValues[5],keyValues.at(-1),keyValues[6]];
 for(const archive of reloads(packet)){
  const context={};vm.runInNewContext(emitTypedProgramLinks(archive.network,archive.root,'JavaScript').code,context,{timeout:1000});
  assert.deepEqual(run(index,queries,context.readOriginalLinkIdentityOrdinals),run(index,queries));
  for(const args of [[[0,0],[1,1],[0],[1]],[[0],[1],[0],[2]],[[],[],[],[]],[[0],[1],[NaN],[1]]])assert.deepEqual(Array.from(context.readOriginalLinkIdentityOrdinals(...args)),read(...args));
 }
});

test('dynamic push, Array.from and unsafe public LinkId collisions remain explicit feasibility gaps',()=>{
 for(const control of [policy.feasibility.growth,policy.feasibility.arrayFrom])for(const options of [{},{referenceSemantics:true},{referenceSemantics:false}]){
  assert.throws(()=>checkProgram(parseJavaScript(control.source,options)),error=>error.kind===control.kind&&error.reason===control.reason&&error.span.start===control.span.start&&error.span.end===control.span.end);
 }
 for(const[first,second]of policy.feasibility.publicCollisions){const one=LinkId.fromU64(BigInt(first)),two=LinkId.fromU64(BigInt(second));assert.equal(one.equals(two),true);assert.throws(()=>originalLinkIdentityWords(one),/unsafe/u);assert.throws(()=>originalLinkIdentityWords(two),/unsafe/u);assert.notDeepEqual(originalLinkIdentityWords(BigInt(first)),originalLinkIdentityWords(BigInt(second)));}
 assert.equal(exactOriginalLinkIdentity(LinkId.fromU64(7)),7n);
});

test('links policy and all eleven complete native declarations regenerate from the actual checked source',()=>{
 const generated=generateOriginalLinkIdentity(true),stored=readProjectData(new URL('./data/native-definitions.lino',import.meta.url));
 assert.equal(generated.declarations.length,11);assert.deepEqual(stored,generated);
 assert.ok(generated.declarations.some(value=>value.includes('pub fn read_original_link_identity_ordinals(')));
 assert.ok(generated.declarations.some(value=>value.includes('pub fn find_original_link_identity_ordinal(')));
 assert.equal(generated.declarations.some(value=>value.includes('fn main(')),false);
});
