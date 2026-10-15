import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../js/src/index.js';
import {LinkMetadata} from '../../js/src/primitives.js';
import {TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {decodeRustTextCertificateFacet} from '../rust-text-certificate-adapter/raw-identity.mjs';
import {createRawTextCertificateFixture,originalLinkRawAdversarialFixtures} from './raw-fixtures.mjs';
import {readOriginalLinkRawTransport,checkOriginalLinkRawSelectedGraph} from './raw-transport.mjs';
import {checkOriginalLinkRawShape,hasOriginalRawMember,readOriginalRawMemberPosition} from './raw-rules.mjs';
import {generateOriginalLinkRawJavaScript,generateOriginalLinkRawNative} from './generate-raw.mjs';
import {emitRustTextValueProjection} from '../rust-text-value-projection/model.mjs';
import {originalLinkRawCertificateCorpus,originalLinkRawNumericMetadataCorpus,createOriginalLinkRawEditedFixture,createOriginalLinkRawOwnerForgery} from './raw-corpus.mjs';
const archive=(packet,format)=>format==='direct'?packet:({...packet,network:format==='text'?LinkNetwork.fromLino(packet.network.toLino()):decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))});
const label=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
const field=(packet,parent,name)=>{const rows=packet.network.fieldRelations().filter(row=>row.parent.equals(parent)&&row.label===label(name));assert.equal(rows.length,1,name);return rows[0];};
const children=(packet,id)=>packet.network.link(id).references();
const programBody=packet=>children(packet,packet.root)[0];
const selectedProof=packet=>field(packet,programBody(packet),'sourceTextValueProjection').child;
const replace=(row,child)=>row.link.setReferences([row.parent,row.link.references()[1],child]);
const facts=packet=>children(packet,field(packet,selectedProof(packet),'functions').child);
function checkWhole(packet){const before=packet.network.toLino(),transport=readOriginalLinkRawTransport(packet.network,packet.root),decision=checkOriginalLinkRawShape(...transport.arguments);assert.equal(decision[0],1,JSON.stringify(decision));const result=checkOriginalLinkRawSelectedGraph(packet.network,packet.root);assert.deepEqual(result.tables.decision,result.checked.decision);assert.equal(Object.keys(result.tables.tables).length,17);assert.equal(packet.network.toLino(),before);return result;}
for(const fixture of originalLinkRawAdversarialFixtures())test('original raw graph contract: '+fixture.id,()=>{
 for(const format of['direct','text','binary']){
  const original=fixture.create(),override=fixture.expected.archives?.[format];
  if(override==='serialization-refused'){assert.equal(format,'binary');assert.throws(()=>archive(original,format),'the existing binary metadata invariant remains enforced');continue;}
  const expected={...fixture.expected,...(typeof override==='object'?override:{})},packet=archive(original,format),before=packet.network.toLino(),transport=readOriginalLinkRawTransport(packet.network,packet.root),columns=transport.columns.map(row=>[...row]),decision=checkOriginalLinkRawShape(...transport.arguments),actual=decision[0]===0?'unselected':decision[0]===1?'accepted':'refused';
  assert.equal(actual,expected.rawShape,fixture.id+' '+format+' '+JSON.stringify(decision));assert.deepEqual(transport.columns,columns,'the numeric preflight preserves every primitive input');assert.equal(packet.network.toLino(),before,'raw original links remain unchanged');assert.equal(transport.anchors.length,packet.network.len());
  if(expected.selectedGraph==='refused')assert.throws(()=>checkOriginalLinkRawSelectedGraph(packet.network,packet.root));else{const full=checkOriginalLinkRawSelectedGraph(packet.network,packet.root);assert.equal(full.decision[0],expected.selectedGraph==='unselected'?0:1);if(full.checked)assert.equal(full.checked.decision[0],1);}
  if(expected.oldDecoder==='accepted')assert.equal(decodeRustTextCertificateFacet(packet.network,packet.root).decision[0],1);else if(expected.oldDecoder==='refused')assert.throws(()=>decodeRustTextCertificateFacet(packet.network,packet.root));else if(expected.oldDecoder==='unselected')assert.equal(decodeRustTextCertificateFacet(packet.network,packet.root),null);
 }
});
test('separate raw instructions regenerate without touching any old kernel or generating native output',()=>{assert.ok(generateOriginalLinkRawJavaScript(true).includes('export function checkOriginalLinkRawShape'));});
test('portable dense Number membership preserves SameValueZero independently of strict positions',()=>{
 for(const values of[[],[NaN],[-0],[0],[NaN,-0,0,Infinity,-Infinity,7]])for(const query of[NaN,-0,0,Infinity,-Infinity,7,8]){
  const before=[...values];assert.equal(hasOriginalRawMember(values,query),values.includes(query));assert.equal(readOriginalRawMemberPosition(values,query),values.indexOf(query));assert.deepEqual(values,before);
 }
 assert.equal(hasOriginalRawMember([NaN],NaN),true);assert.equal(hasOriginalRawMember([-0],0),true);assert.equal(hasOriginalRawMember([0],-0),true);assert.equal(readOriginalRawMemberPosition([NaN],NaN),-1);
});
test('actual draft generation receipts retain complete source bytes without requiring historical failures',async()=>{
 const path=new URL('./data/raw-translation-attempts.lino',import.meta.url),before=readFileSync(path,'utf8'),data=readProjectData(path),hash=value=>createHash('sha256').update(value).digest('hex');assert.equal(data.version,1);assert.ok(data.attempts.length>0&&data.attempts.length<=32);let bytes=0;
 for(const attempt of data.attempts){assert.equal(hash(attempt.source),attempt.sourceSha256);assert.equal(Buffer.byteLength(attempt.source),attempt.sourceBytes);bytes+=attempt.sourceBytes;if(attempt.operation.kind==='authentic translation diagnostic'){assert.equal(typeof attempt.operation.reason,'string');if(attempt.operation.span){assert.ok(attempt.operation.span.start>=0&&attempt.operation.span.end<=attempt.source.length);}}else{assert.equal(attempt.operation.kind,'generated complete Rust text');assert.ok(attempt.operation.declarations>0);}}
 assert.ok(bytes<=1048576);await assert.rejects(generateOriginalLinkRawNative(true,{recordAttempt:true}),/freshness check cannot record/u);assert.equal(readFileSync(path,'utf8'),before);
 const artifact=readProjectData(new URL('./data/raw-native-definitions.lino',import.meta.url)),source=readFileSync(new URL('./raw-rules.mjs',import.meta.url),'utf8');assert.equal(artifact.sourceSha256,hash(source));assert.equal(artifact.declarations.length,106);assert.equal(artifact.declarationNames.length,106);assert.ok(artifact.completeRust.includes('pub fn check_original_link_raw_shape(root_high: f64, root_low: f64, columns: Vec<Vec<f64>>) -> Vec<f64>'));assert.ok(artifact.completeRust.includes('fn main('));
});
test('raw primitive columns reject non-Numbers and sparse cells without imposing finite metadata',()=>{
 const packet=createRawTextCertificateFixture(),transport=readOriginalLinkRawTransport(packet.network,packet.root);
 for(let column=0;column<transport.columns.length;column++){
  assert.ok(transport.columns[column].length>0,'the authentic transport exercises every column');
  for(const mutation of['string','hole']){
   const columns=transport.columns.map(row=>[...row]);
   if(mutation==='hole')delete columns[column][0];else columns[column][0]='false';
   assert.ok(checkOriginalLinkRawShape(transport.arguments[0],transport.arguments[1],columns)[0]<0,column+' '+mutation);
  }
 }
 for(const fixture of originalLinkRawNumericMetadataCorpus()){
  const input=fixture.create(),value=fixture.id.endsWith('NaN')?NaN:fixture.id.endsWith('positive infinity')?Infinity:fixture.id.endsWith('negative infinity')?-Infinity:-0,original=readOriginalLinkRawTransport(input.network,input.root),ordinal=original.anchors.findIndex(id=>id.asU64().toString()===input.witness.metadata);
  assert.ok(ordinal>=0);assert.equal(original.columns[6][ordinal*4+3],4);assert.ok(Object.is(original.columns[10][ordinal*4+3],value));
  assert.equal(checkOriginalLinkRawShape(...original.arguments)[0],1,'actual ignored Number metadata retains its original representation');
  checkWhole(input);assert.equal(decodeRustTextCertificateFacet(input.network,input.root).decision[0],1);
 }
 for(const column of[11,12,13])for(const value of[NaN,Infinity,-1,0.5,10]){
  const columns=transport.columns.map(row=>[...row]);columns[column][0]=value;
  assert.ok(checkOriginalLinkRawShape(transport.arguments[0],transport.arguments[1],columns)[0]<0,'metadata tag domain '+column+' '+value);
 }
});
test('raw Number lexical peeks stay inside the actual scalar definition slice',()=>{
 for(const adjacent of['e','E','.','+','-','7']){
  const packet=createRawTextCertificateFixture(),body=packet.network.link(packet.root).references()[0],version=packet.network.fieldRelations().find(row=>row.parent.equals(body)&&row.label===schema.fields.schemaVersion);
  assert.ok(version);
  const scalar=packet.network.insertLink([],LinkMetadata.new().withLanguage(schema.language).withTerm(schema.scalars.number).withDefinition('1'));
  packet.network.insertLink([],LinkMetadata.new().withTerm(adjacent));
  version.link.setReferences([version.parent,version.link.references()[1],scalar]);
  const transport=readOriginalLinkRawTransport(packet.network,packet.root);
  assert.equal(checkOriginalLinkRawShape(...transport.arguments)[0],1,'valid1 followed by unrelated '+adjacent);
  assert.equal(checkOriginalLinkRawSelectedGraph(packet.network,packet.root).checked.decision[0],1);
 }
 for(const definition of['-','1.','1e','1E','1e+','1e-']){
  const packet=createRawTextCertificateFixture(),body=packet.network.link(packet.root).references()[0],version=packet.network.fieldRelations().find(row=>row.parent.equals(body)&&row.label===schema.fields.schemaVersion);
  const scalar=packet.network.insertLink([],LinkMetadata.new().withLanguage(schema.language).withTerm(schema.scalars.number).withDefinition(definition));
  packet.network.insertLink([],LinkMetadata.new().withTerm('1'));
  version.link.setReferences([version.parent,version.link.references()[1],scalar]);
  const transport=readOriginalLinkRawTransport(packet.network,packet.root);
  assert.ok(checkOriginalLinkRawShape(...transport.arguments)[0]<0,'adjacent metadata cannot complete '+definition);
 }
});
test('all thirteen unchanged real graph archives pass raw roles before their independent complete certificate',()=>{
 const fixtures=readProjectData(new URL('../rust-text-value-projection/data/fixtures.lino',import.meta.url));assert.equal(fixtures.cases.length,13);
 const corpus=originalLinkRawCertificateCorpus(),originals=corpus.filter(row=>row.id.startsWith('original-source-'));assert.equal(corpus.length,31);assert.deepEqual(originals.map(row=>row.sourceFixtureId),fixtures.cases.map(row=>row.id));assert.deepEqual(originals.map(row=>row.sourceSha256),fixtures.cases.map(row=>row.sourceSha256));
 for(const fixture of originals){assert.deepEqual(fixture.formats,['text','binary']);for(const format of fixture.formats)checkWhole(archive(fixture.create(),format));}
});
test('four coherent source-free edits retain actual owners and affect executable results after both archives',()=>{
 for(const edit of['retarget','declaration rename','parameter rename','literal value']){
  const packet=createOriginalLinkRawEditedFixture(edit),{name,args,expected}=packet.witness.execution;
  for(const format of['text','binary']){const result=checkWhole(archive(packet,format)),context={};vm.runInNewContext(emitRustTextValueProjection(result.checked.program,result.checked.proof,'JavaScript').text,context,{timeout:1000});assert.equal(context[name](...args),expected,edit+' '+format);}
 }
});
test('cross-owner and equal-looking cloned raw proof anchors refuse before ordinary decode',()=>{
 for(const role of['declaration','signature','resultPosition','operationNode','coreValue','input','position'])for(const cloning of[false,true])for(const format of['text','binary']){
  const packet=createOriginalLinkRawOwnerForgery(role,cloning),saved=archive(packet,format),transport=readOriginalLinkRawTransport(saved.network,saved.root);assert.ok(checkOriginalLinkRawShape(...transport.arguments)[0]<0,role+' '+cloning+' '+format);assert.throws(()=>checkOriginalLinkRawSelectedGraph(saved.network,saved.root));assert.throws(()=>decodeRustTextCertificateFacet(saved.network,saved.root));
 }
});
test('legacy raw Var names use their own lexical parameter and conflict with semantic references',()=>{
 for(const conflict of[false,true])for(const format of['text','binary']){
  const packet=createRawTextCertificateFixture(),declaration=field(packet,facts(packet)[0],'declaration').child,core=field(packet,declaration,'body').child,reference=field(packet,core,'binding reference'),name=field(packet,reference.child,'name').child;
  if(!conflict)packet.network.deleteLink(reference.link.id());packet.network.insertFieldRelation(core,label('name'),name);const saved=archive(packet,format),transport=readOriginalLinkRawTransport(saved.network,saved.root);
  if(conflict){assert.ok(checkOriginalLinkRawShape(...transport.arguments)[0]<0);assert.throws(()=>checkOriginalLinkRawSelectedGraph(saved.network,saved.root));}else checkWhole(saved);
 }
});
test('raw declaration inventory checks identities and key values without inventing Map iteration order',()=>{
 const packet=createRawTextCertificateFixture('equal-valued-distinct-owners'),body=programBody(packet),map=packet.network.link(field(packet,body,'declarations').child),original=map.references();assert.equal(original.length,2);map.setReferences([...original].reverse());
 for(const format of['text','binary']){const saved=archive(packet,format),transport=readOriginalLinkRawTransport(saved.network,saved.root);assert.equal(checkOriginalLinkRawShape(...transport.arguments)[0],1);assert.throws(()=>checkOriginalLinkRawSelectedGraph(saved.network,saved.root),'the unchanged complete certificate retains its own canonical ordering obligation');}
 for(const mutation of['key mismatch','duplicate item','missing item']){
  const input=createRawTextCertificateFixture('equal-valued-distinct-owners'),root=programBody(input),pairs=children(input,field(input,root,'declarations').child),items=input.network.link(field(input,root,'items').child);
  if(mutation==='key mismatch'){const pair=input.network.link(pairs[0]),refs=pair.references(),key=input.network.insertLink([],input.network.link(refs[0]).metadata().withDefinition('wrong_actual_mapping_key'));pair.setReferences([key,refs[1]]);}else{const values=items.references();items.setReferences(mutation==='duplicate item'?[values[0],values[0]]:[values[0]]);}
  for(const format of['text','binary']){const saved=archive(input,format),transport=readOriginalLinkRawTransport(saved.network,saved.root);assert.ok(checkOriginalLinkRawShape(...transport.arguments)[0]<0,mutation);assert.throws(()=>checkOriginalLinkRawSelectedGraph(saved.network,saved.root));}
 }
});
test('raw owned recipes refuse normalized duplicates, unsafe labels and unrelated known fields',()=>{
 for(const mutation of['normalized duplicate','unsafe property','known foreign role','missing label term']){
  const packet=createRawTextCertificateFixture(),declaration=field(packet,facts(packet)[0],'declaration').child;
  if(mutation==='normalized duplicate')packet.network.insertFieldRelation(declaration,'params',field(packet,declaration,'params').child);
  if(mutation==='unsafe property')packet.network.insertFieldRelation(declaration,'__proto__',field(packet,declaration,'name').child);
  if(mutation==='known foreign role'){const type=field(packet,declaration,'ret').child;packet.network.insertFieldRelation(type,label('span'),field(packet,declaration,'span').child);}
  if(mutation==='missing label term'){const relation=field(packet,declaration,'name'),original=packet.network.link(relation.link.references()[1]),replacement=packet.network.insertLink(original.references(),original.metadata().withTerm(undefined));relation.link.setReferences([relation.parent,replacement,relation.child]);}
  for(const format of['text','binary']){const saved=archive(packet,format),transport=readOriginalLinkRawTransport(saved.network,saved.root);assert.ok(checkOriginalLinkRawShape(...transport.arguments)[0]<0,mutation);assert.throws(()=>checkOriginalLinkRawSelectedGraph(saved.network,saved.root));}
 }
});
test('raw transport rejects declared resource excess before materialization or element traversal',()=>{
 let materialized=false;assert.throws(()=>readOriginalLinkRawTransport({len:()=>100001,links:()=>{materialized=true;throw new Error('must not materialize');}},null),RangeError);assert.equal(materialized,false);
 const packet=createRawTextCertificateFixture(),transport=readOriginalLinkRawTransport(packet.network,packet.root);
 for(const vector of[0,2,9]){const columns=transport.columns.map(row=>[...row]),oversized=[];oversized.length=vector===0?100001:1000001;columns[vector]=oversized;assert.deepEqual(checkOriginalLinkRawShape(transport.arguments[0],transport.arguments[1],columns),[-1,0]);}
 const model=field(packet,selectedProof(packet),'model'),record=packet.network.link(selectedProof(packet)).metadata();let child=model.child;
 for(let depth=0;depth<257;depth++){const parent=packet.network.insertLink([],record.clone());packet.network.insertFieldRelation(parent,label('value'),child);child=parent;}
 replace(model,child);const nested=readOriginalLinkRawTransport(packet.network,packet.root);assert.equal(checkOriginalLinkRawShape(...nested.arguments)[0],-3,'active depth is bounded before role interpretation');
});
