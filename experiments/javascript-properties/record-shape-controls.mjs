// Deferred layout, ownership, source editing and formal-basis observations.
import assert from 'node:assert/strict';
import {runInNewContext} from 'node:vm';
import {createHash} from 'node:crypto';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {assertVersions} from './controls.mjs';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {LinkNetwork} from '../../js/src/network.js';
import {LinkId,LinkType} from '../../js/src/primitives.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../js/src/link-network-binary.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks,readTypedProgramBindingIdentities,TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {expandTypedProgramRecordShapes,readTypedProgramRecordShapeProfile} from '../../js/src/typed-program-record-shapes.js';
import {encodeFormalConceptDefinition,proveFormalConceptEquivalence,verifyFormalConceptEquivalence} from '../../js/src/formal-concept-definition.js';
import {encodeFormalFunctionDefinition,encodeFormalFunctionDeclarationValue} from '../../js/src/formal-function-links.js';
export {assertVersions};
export const recordShapeFixture=readProjectData(new URL('../../parity/fixtures/typed-program-record-shapes.lino',import.meta.url));
const key=id=>id.asU64(),layout=readTypedProgramRecordShapeProfile();
const shapes=packet=>packet.network.links().filter(link=>link.metadata().language===schema.language&&link.metadata().term===schema.containers.record&&link.references().length);
function field(network,record,name){const refs=record.references(),roles=network.link(refs[0]).references(),label=schema.fields[name]??name,index=roles.findIndex(id=>network.link(id).metadata().term===label);assert.ok(index>=0);return {index:index+1,child:network.link(refs[index+1])};}
function malformed(packet,name,change,reason){const network=LinkNetwork.fromLino(packet.network.toLino()),record=network.link(network.link(packet.root).references()[0]),shape=network.link(record.references()[0]);change(network,record,shape);assert.throws(()=>decodeTypedProgramLinks(network,packet.root),error=>!reason||error.message.includes(reason));return {name,links:network.toLino(),root:String(key(packet.root)),...(reason?{reason}:{})};}
export function observeRecordShape(row){
 const program=checkProgram(parseJavaScript(row.source)),legacy=encodeTypedProgramLinks(program),compact=encodeTypedProgramLinks(program,{recordShapes:recordShapeFixture.model});
 assert.ok(compact.network.links().length<legacy.network.links().length,'complete source graph must actually reduce physical Links');assert.equal(String(key(compact.root)),String(key(legacy.root)));
 const owners=packet=>readTypedProgramBindingIdentities(packet.network,packet.root).map(binding=>String(key(binding))).sort();
 assert.deepEqual(owners(compact),owners(legacy),'all original binder identities remain distinct');
 const ownerIds=owners(compact);assert.equal(ownerIds.length,row.declarations.length);assert.equal(new Set(ownerIds).size,ownerIds.length);
 for(const network of [compact.network,LinkNetwork.fromLino(compact.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(compact.network))]){
  assert.deepEqual(decodeTypedProgramLinks(network,compact.root),decodeTypedProgramLinks(legacy.network,legacy.root));
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,compact.root,target),emitTypedProgramLinks(legacy.network,legacy.root,target));
 }
 const expanded=expandTypedProgramRecordShapes(compact.network,compact.root,schema.language,schema.containers.record);
 assert.deepEqual(decodeTypedProgramLinks(expanded.network,expanded.root),decodeTypedProgramLinks(legacy.network,legacy.root));assert.deepEqual(owners(expanded),owners(compact));
 const one=encodeFormalConceptDefinition(legacy),two=encodeFormalConceptDefinition(compact),proof=proveFormalConceptEquivalence(one,two);assert.ok(proof);assert.equal(verifyFormalConceptEquivalence(one,two,proof),true);
 for(const name of row.declarations){const first=encodeFormalFunctionDefinition(legacy,name),second=encodeFormalFunctionDefinition(compact,name),certificate=proveFormalConceptEquivalence(first,second);assert.ok(certificate);assert.equal(verifyFormalConceptEquivalence(first,second,certificate),true);
  if(row.boolean){const a=encodeFormalFunctionDeclarationValue(legacy,name),b=encodeFormalFunctionDeclarationValue(compact,name),same=proveFormalConceptEquivalence(a,b);assert.ok(same);assert.equal(verifyFormalConceptEquivalence(a,b,same),true);}}
 let edited;
 if(row.edit){const editPacket=encodeTypedProgramLinks(program,{recordShapes:recordShapeFixture.model,valueSharing:'exact scalars, types and source spans 1'}),network=LinkNetwork.fromLino(editPacket.network.toLino()),literals=shapes({network,root:editPacket.root}).filter(record=>{try{return field(network,record,'k').child.metadata().definition==='lit'&&field(network,record,'value').child.metadata().definition==='1';}catch{return false;}});assert.equal(literals.length,2);const literal=literals[0];assert.notEqual(key(literal.id()),key(literals[1].id()));assert.equal(key(field(network,literal,'value').child.id()),key(field(network,literals[1],'value').child.id()));
  const value=field(network,literal,'value'),replacement=network.insertLink([],value.child.metadata().clone().withDefinition('9')),references=literal.references();references[value.index]=replacement;literal.setReferences(references);
  const emission=emitTypedProgramLinks(network,editPacket.root,'JavaScript'),entry=name=>emission.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===name).target;
  assert.equal(runInNewContext(emission.code+'\n'+entry('first')+'(2);'),11);assert.equal(runInNewContext(emission.code+'\n'+entry('second')+'(2);'),3,'an occurrence edit must not mutate another shared value user');edited={links:network.toLino(),root:String(key(editPacket.root))};
 }
 const invalid=[
  malformed(compact,'missing actual layout selection',network=>{for(const relation of network.fieldRelations())if(key(relation.parent)===key(compact.root)&&relation.label===layout.rootField)network.deleteLink(relation.link.id());}),
  malformed(compact,'counterfeit profile identity',network=>{const relation=network.fieldRelations().find(relation=>key(relation.parent)===key(compact.root)&&relation.label===layout.rootField),link=network.link(relation.child);link.setMetadata(link.metadata().clone().withDefinition('unsupported record layout 9'));}),
  malformed(compact,'duplicate role in a shared shape',(_network,_record,shape)=>{const roles=shape.references();assert.ok(roles.length>1);roles[1]=roles[0];shape.setReferences(roles);}),
  malformed(compact,'a role is not its self-referencing field point',(network,_record,shape)=>network.link(shape.references()[0]).setReferences([])),
  malformed(compact,'mixed positional and named record fields',(network,record)=>network.insertFieldRelation(record.id(),'extra',record.references()[1])),
  malformed(compact,'a shape also carries named fields',(network,record,shape)=>network.insertFieldRelation(shape.id(),'extra',record.references()[1])),
  malformed(compact,'a role also carries named fields',(network,record,shape)=>network.insertFieldRelation(shape.references()[0],'extra',record.references()[1])),
  malformed(compact,'a foreign shape language',(network,_record,shape)=>shape.setMetadata(shape.metadata().clone().withLanguage('foreign shape model'))),
  malformed(compact,'a field role belongs to a foreign language',(network,_record,shape)=>{const role=network.link(shape.references()[0]);role.setMetadata(role.metadata().clone().withLanguage('foreign role namespace'));}),
  malformed(compact,'a shape is a named foreign entity',(_network,_record,shape)=>shape.setMetadata(shape.metadata().clone().withNamed(true))),
  malformed(compact,'a selected profile has the wrong link type',network=>{const relation=network.fieldRelations().find(relation=>key(relation.parent)===key(compact.root)&&relation.label===layout.rootField),link=network.link(relation.child);link.setMetadata(link.metadata().clone().withLinkType(LinkType.Object));}),
  malformed(compact,'bounded role projection rejects repeated long labels before owned copies',(network,record,shape)=>{const role=network.insertTypedPoint(LinkType.Field,'a'.repeat(1000000),'A field label names a relation between links.');shape.setReferences(Array(17).fill(role));record.setReferences([shape.id(),...Array(17).fill(record.references()[1])]);},'ordered program record role text exceeds its projection resource budget'),
  malformed(compact,'dangling actual value edge',(_network,record)=>{const refs=record.references();refs[1]=new LinkId(9999999);record.setReferences(refs);}),
  malformed(compact,'structural self cycle is not a binding reference',(_network,record)=>{const refs=record.references();refs[1]=record.id();record.setReferences(refs);}),
 ];
 return {name:row.name,legacy:legacy.network.toLino(),legacyRoot:String(key(legacy.root)),compact:compact.network.toLino(),compactRoot:String(key(compact.root)),owners:ownerIds,declarations:row.declarations,boolean:row.boolean??false,invalid,...(edited?{edited}:{})};
}
export function observeRecordShapePredecessors(){assert.equal(recordShapeFixture.predecessors.length,13);for(const row of recordShapeFixture.predecessors){const data=readProjectData(new URL('../../'+row.retained,import.meta.url));assert.equal(data.path,row.path);assert.equal(data.revision,row.revision);assert.equal(Buffer.byteLength(data.source),row.bytes);assert.equal(createHash('sha256').update(data.source).digest('hex'),row.sha256);}}
