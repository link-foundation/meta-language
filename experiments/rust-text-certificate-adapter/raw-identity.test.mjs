import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../js/src/index.js';
import {TYPED_PROGRAM_LINKS_SCHEMA as schema,decodeTypedProgramLinks} from '../../js/src/typed-program-links.js';
import {LinkMetadata,LinkType} from '../../js/src/primitives.js';
import {createPortableRustTextCertificate,encodePortableRustTextCertificate} from '../rust-text-certificate/bridge.mjs';
import {decodeRustTextCertificateFacet} from './raw-identity.mjs';

const fixtures=readProjectData(new URL('../rust-text-value-projection/data/fixtures.lino',import.meta.url));
const label=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
function field(network,parent,name){const row=network.fieldRelations().find(row=>row.parent.equals(parent)&&row.label===label(name));assert.ok(row,name);return row;}
function replaceField(row,child){row.link.setReferences([row.parent,row.link.references()[1],child]);}
function proofId(packet){return field(packet.network,packet.network.link(packet.root).references()[0],'sourceTextValueProjection').child;}
function rows(packet,name){return packet.network.link(field(packet.network,proofId(packet),name).child).references();}
function attach(packet){
 const wrapper=field(packet.network,packet.root,'provenance').child,proof=field(packet.network,wrapper,'sourceInterpretation').child;
 packet.network.insertFieldRelation(packet.network.link(packet.root).references()[0],label('sourceTextValueProjection'),proof);return packet;
}
function create(id){const source=fixtures.cases.find(row=>row.id===id).source,{program,proof}=createPortableRustTextCertificate(source);return attach(encodePortableRustTextCertificate(program,proof));}
const text=packet=>({...packet,network:LinkNetwork.fromLino(packet.network.toLino())});
const binary=packet=>({...packet,network:decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))});
const decode=packet=>decodeRustTextCertificateFacet(packet.network,packet.root);
function cloneRecord(network,id,omit=()=>false){
 const original=network.link(id),fields=network.fieldRelations().filter(row=>row.parent.equals(id)),copy=network.insertLink(original.references(),original.metadata().clone());
 for(const row of fields)if(!omit(row))network.insertFieldRelation(copy,row.label,row.child);return copy;
}
function scalar(network,kind,definition){return network.insertLink([],LinkMetadata.new().withLinkType(LinkType.Semantic).withLanguage(schema.language).withNamed(true).withTerm(schema.scalars[kind]).withDefinition(definition));}

test('all thirteen program-owned proof archives preserve actual core aliases through text and binary without changing stored bytes',()=>{
 assert.equal(fixtures.cases.length,13);
 for(const entry of fixtures.cases)for(const load of[text,binary]){
  const original=create(entry.id),packet=load(original),before=packet.network.toLino(),value=decode(packet);assert.equal(packet.network.toLino(),before);
  assert.equal(value.decision[0],1);assert.equal(value.proof.worker,value.program.main);assert.equal(Object.hasOwn(value.program,'sourceTextValueProjection'),false);
  for(const row of value.proof.functions){assert.equal(row.declaration,value.program.declarations.get(row.declaration.fullName));assert.equal(row.signature,row.declaration.sourceSignature);assert.equal(row.resultPosition,row.signature.result);assert.equal(row.coreValue,row.declaration.body);if(row.input){const index=row.declaration.params.indexOf(row.input);assert.ok(index>=0);assert.equal(row.position,row.signature.parameters[index]);}}
 }
});

test('a wholly unselected root returns null even when another program in the same network owns the facet',()=>{
 const entry=fixtures.cases[0],value=createPortableRustTextCertificate(entry.source),plain=encodePortableRustTextCertificate(value.program,value.proof);assert.equal(decode(plain),null);assert.equal(decodeTypedProgramLinks(plain.network,plain.root).sourceLanguage,'Rust');
 const selected=create('owned-clone'),body=selected.network.link(selected.root).references()[0],otherBody=cloneRecord(selected.network,body,row=>row.label===label('sourceTextValueProjection'));
 const other=selected.network.insertLink([otherBody],selected.network.link(selected.root).metadata().clone());
 assert.equal(decodeRustTextCertificateFacet(selected.network,other),null);assert.equal(decode(selected).decision[0],1);
});

test('user provenance never selects the facet and remains byte-preserved beside an independently selected body',()=>{
 for(const load of[text,binary]){
  let value=createPortableRustTextCertificate(fixtures.cases[0].source),packet=encodePortableRustTextCertificate(value.program,value.proof),metadata=field(packet.network,packet.root,'provenance').child;
  packet.network.insertFieldRelation(metadata,label('sourceTextValueProjection'),scalar(packet.network,'boolean','false'));packet=load(packet);let before=packet.network.toLino();assert.equal(decode(packet),null);assert.equal(packet.network.toLino(),before);assert.equal(decodeTypedProgramLinks(packet.network,packet.root).sourceLanguage,'Rust');
  packet=create('owned-clone');metadata=field(packet.network,packet.root,'provenance').child;packet.network.insertFieldRelation(metadata,label('sourceTextValueProjection'),scalar(packet.network,'boolean','false'));packet=load(packet);before=packet.network.toLino();assert.equal(decode(packet).decision[0],1);assert.equal(packet.network.toLino(),before);
 }
});

test('distinct equal-looking raw declarations, parameters, signatures and operations cannot become aliases after either reload',()=>{
 for(const load of[text,binary])for(const role of['declaration','signature','resultPosition','operationNode','coreValue','input','position']){
  const packet=create('equal-valued-distinct-owners'),ids=rows(packet,'functions'),target=field(packet.network,ids[0],role),other=field(packet.network,ids[1],role);assert.ok(!target.child.equals(other.child));replaceField(target,other.child);const saved=load(packet),before=saved.network.toLino();assert.throws(()=>decode(saved),role);assert.equal(saved.network.toLino(),before);
 }
});

test('fresh cloned records with identical fields stay distinct from the actual core and owned operation nodes',()=>{
 for(const load of[text,binary])for(const role of['declaration','signature','resultPosition','operationNode','coreValue','input','position']){
  const packet=create('owned-clone'),fact=rows(packet,'functions')[0],target=field(packet.network,fact,role),copy=cloneRecord(packet.network,target.child);assert.ok(!copy.equals(target.child));replaceField(target,copy);assert.throws(()=>decode(load(packet)),role);
 }
});

test('real condition and unused header pointers cannot be exchanged using equal semantic values',()=>{
 for(const load of[text,binary]){
  let packet=create('equal-valued-distinct-owners'),conditions=rows(packet,'valueAssumptions'),functions=rows(packet,'functions');replaceField(field(packet.network,conditions[0],'operationNode'),field(packet.network,functions[1],'operationNode').child);assert.throws(()=>decode(load(packet)));
  packet=create('valid-clone-retarget');const fact=rows(packet,'functions')[0],signature=field(packet.network,fact,'signature').child,positions=packet.network.link(field(packet.network,signature,'parameters').child).references();replaceField(field(packet.network,fact,'position'),positions[1]);assert.throws(()=>decode(load(packet)));
 }
});

test('coherent saved clone retargets preserve the actual parameter and signature positions while str remains refused',()=>{
 function retarget(value,index){const entry=value.program.declarations.get('choose'),row=value.proof.functions[0],target=entry.params[index],token=value.proof.tokens.find(token=>token.binding===row.input&&token.span.start===row.coreValue.span.start);assert.ok(token);token.binding=target;entry.body.name=target.name;row.input=target;row.position=entry.sourceSignature.parameters[index];for(const condition of[...value.proof.inputAssumptions,...value.proof.valueAssumptions])if(condition.declaration===entry){condition.input=row.input;condition.position=row.position;}}
 for(const load of[text,binary]){
  let value=decode(create('valid-clone-retarget'));retarget(value,1);let packet=load(attach(encodePortableRustTextCertificate(value.program,value.proof)));value=decode(packet);assert.equal(value.proof.functions[0].input,value.program.declarations.get('choose').params[1]);assert.equal(value.decision[0],1);
  retarget(value,0);packet=load(attach(encodePortableRustTextCertificate(value.program,value.proof)));assert.equal(decode(packet).decision[0],1);
  value=decode(create('invalid-slice-retarget'));retarget(value,1);assert.throws(()=>encodePortableRustTextCertificate(value.program,value.proof));
 }
});

test('misplaced, duplicated and nonrecord facet markers refuse before exposing a selected decoded program',()=>{
 const mutations=[
  packet=>{packet.network.insertFieldRelation(packet.network.link(packet.root).references()[0],label('sourceTextValueProjection'),proofId(packet));},
  packet=>{packet.network.insertFieldRelation(packet.root,label('sourceTextValueProjection'),proofId(packet));},
  packet=>{const body=packet.network.link(packet.root).references()[0];packet.network.insertFieldRelation(field(packet.network,body,'main').child,label('sourceTextValueProjection'),proofId(packet));},
  packet=>{packet.network.insertFieldRelation(packet.root,'unknown envelope field',proofId(packet));},
  packet=>{packet.network.insertFieldRelation(packet.network.link(packet.root).references()[0],'unknown program field',proofId(packet));},
  packet=>{const proof=proofId(packet);packet.network.insertFieldRelation(proof,'model',field(packet.network,proof,'model').child);},
  packet=>{packet.network.insertFieldRelation(proofId(packet),'unknown proof field',proofId(packet));},
 ];
 for(const load of[text,binary])for(const mutate of mutations){const packet=create('owned-clone');mutate(packet);assert.throws(()=>decode(load(packet)));}
 for(const load of[text,binary])for(const[kind,definition]of[['null',undefined],['boolean','false'],['undefined',undefined],['number','0'],['string','']]){const packet=create('owned-clone'),body=packet.network.link(packet.root).references()[0];replaceField(field(packet.network,body,'sourceTextValueProjection'),scalar(packet.network,kind,definition));assert.throws(()=>decode(load(packet)));}
});

test('a misplaced sole marker selects refusal while unsafe, duplicate and foreign proof fields retain the existing codec checks',()=>{
 for(const load of[text,binary]){
  let packet=create('owned-clone'),body=packet.network.link(packet.root).references()[0],marker=field(packet.network,body,'sourceTextValueProjection');packet.network.deleteLink(marker.link.id());packet.network.insertFieldRelation(field(packet.network,body,'main').child,label('sourceTextValueProjection'),marker.child);assert.throws(()=>decode(load(packet)));
  for(const mutation of['unsafe','duplicate','foreign']){packet=create('owned-clone');const proof=proofId(packet);if(mutation==='unsafe')packet.network.insertFieldRelation(proof,'__proto__',field(packet.network,proof,'model').child);if(mutation==='duplicate')packet.network.insertFieldRelation(proof,'language',field(packet.network,proof,'language').child);if(mutation==='foreign'){const row=field(packet.network,proof,'observationModel'),copy=cloneRecord(packet.network,row.child);packet.network.link(copy).setMetadata(packet.network.link(copy).metadata().withLanguage('foreign'));replaceField(row,copy);}assert.throws(()=>decode(load(packet)),mutation);}
 }
});
