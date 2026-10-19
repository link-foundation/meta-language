// Lazy shared graph construction only. Importing this module runs no planner,
// source parser, compiler, process or observation recorder.
import assert from 'node:assert/strict';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkMetadata} from '../../js/src/primitives.js';
import {TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {createPortableRustTextCertificate} from '../rust-text-certificate/bridge.mjs';
import {encodeRustTextValueProjection} from '../rust-text-value-projection/model.mjs';
import {createRawTextCertificateFixture} from './raw-fixtures.mjs';
const label=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
const field=(packet,parent,name)=>{const rows=packet.network.fieldRelations().filter(row=>row.parent.equals(parent)&&row.label===label(name));assert.equal(rows.length,1,name);return rows[0];};
const sourceFixtures=()=>readProjectData(new URL('../rust-text-value-projection/data/fixtures.lino',import.meta.url)).cases;
const key=id=>id.asU64().toString();
function attachEditedPacket(value){const packet=encodeRustTextValueProjection(value.program,value.proof),body=packet.network.link(packet.root).references()[0],wrapper=field(packet,packet.root,'provenance').child,proof=field(packet,wrapper,'sourceInterpretation').child;packet.network.insertFieldRelation(body,label('sourceTextValueProjection'),proof);return packet;}

export function createOriginalLinkRawEditedFixture(edit){
 assert.ok(['retarget','declaration rename','parameter rename','literal value'].includes(edit));
 const fixtures=sourceFixtures(),sourceFixtureId=edit==='literal value'?'raw-literal-construction':'valid-clone-retarget',value=createPortableRustTextCertificate(fixtures.find(row=>row.id===sourceFixtureId).source),entry=value.program.declarations.get('choose');let name='choose',args=['first','second Ω😀'],expected='first';
 if(edit==='retarget'){const fact=value.proof.functions[0],old=fact.input,target=entry.params[1],token=value.proof.tokens.find(token=>token.binding===old&&token.span.start===fact.coreValue.span.start);assert.ok(token);token.binding=target;entry.body.name=target.name;fact.input=target;fact.position=entry.sourceSignature.parameters[1];for(const row of[...value.proof.inputAssumptions,...value.proof.valueAssumptions])if(row.declaration===entry){row.input=target;row.position=fact.position;}expected=args[1];}
 if(edit==='declaration rename'){name='choose_changed';entry.name=name;entry.fullName=name;value.program.declarations=new Map([[name,entry]]);}
 if(edit==='parameter rename'){entry.params[0].name='changed_input';entry.body.name='changed_input';args=['renamed Ω😀','other'];expected=args[0];}
 if(edit==='literal value'){expected='\0\u0001Ω😀\u{10ffff}';value.proof.functions[0].literal.value=expected;args=[];}
 return{...attachEditedPacket(value),witness:{edit,sourceFixtureId,execution:{name,args,expected}}};
}

export function createOriginalLinkRawOwnerForgery(role,cloning=false){
 assert.ok(['declaration','signature','resultPosition','operationNode','coreValue','input','position'].includes(role));
 const sourceFixtureId=cloning?'owned-clone':'equal-valued-distinct-owners',packet=createRawTextCertificateFixture(sourceFixtureId),body=packet.network.link(packet.root).references()[0],proof=field(packet,body,'sourceTextValueProjection').child,rows=packet.network.link(field(packet,proof,'functions').child).references(),target=field(packet,rows[0],role);let replacement;
 if(cloning){const original=packet.network.link(target.child);replacement=packet.network.insertLink(original.references(),original.metadata().clone());for(const row of packet.network.fieldRelations().filter(row=>row.parent.equals(target.child)))packet.network.insertFieldRelation(replacement,row.label,row.child);}else replacement=field(packet,rows[1],role).child;
 assert.ok(!target.child.equals(replacement));target.link.setReferences([target.parent,target.link.references()[1],replacement]);return{...packet,witness:{role,cloning,sourceFixtureId,original:key(target.child),replacement:key(replacement)}};
}

export function originalLinkRawCertificateCorpus(){
 const fixtures=sourceFixtures();assert.equal(fixtures.length,13);
 const accepted={rawShape:'accepted',selectedGraph:'accepted',oldDecoder:'accepted'},refused={rawShape:'refused',selectedGraph:'refused',oldDecoder:'refused'};
 return[
  ...fixtures.map(fixture=>({id:'original-source-'+fixture.id,sourceFixtureId:fixture.id,sourceSha256:fixture.sourceSha256,formats:['text','binary'],expected:accepted,contract:'The unchanged complete original source is captured into actual Links before raw preflight and the independent old certificate.',create:()=>createRawTextCertificateFixture(fixture.id)})),
  ...['retarget','declaration rename','parameter rename','literal value'].map(edit=>({id:'coherent-edit-'+edit,sourceFixtureId:edit==='literal value'?'raw-literal-construction':'valid-clone-retarget',formats:['text','binary'],expected:accepted,contract:'Coherent actual source-free owner edits remain executable; their declared observation is retained in witness.execution.',create:()=>createOriginalLinkRawEditedFixture(edit)})),
  ...['declaration','signature','resultPosition','operationNode','coreValue','input','position'].flatMap(role=>[false,true].map(cloning=>({id:(cloning?'cloned-owner-':'foreign-owner-')+role,sourceFixtureId:cloning?'owned-clone':'equal-valued-distinct-owners',formats:['text','binary'],expected:refused,contract:'A different actual owner LinkId cannot replace the referenced owner, including equal-looking cloned records.',create:()=>createOriginalLinkRawOwnerForgery(role,cloning)}))),
 ];
}

export function originalLinkRawNumericMetadataCorpus(){
 return[['NaN',NaN],['positive infinity',Infinity],['negative infinity',-Infinity],['negative zero',-0]].map(([name,value])=>({id:'actual-direct-number-metadata-'+name,sourceFixtureId:'owned-clone',formats:['direct'],expected:{rawShape:'accepted',selectedGraph:'accepted',oldDecoder:'accepted'},contract:'An actual ignored Number metadata payload is captured as exact Number bits; this is a direct graph domain, not binary String metadata or native high-ID capture.',create:()=>{const packet=createRawTextCertificateFixture(),metadata=packet.network.insertLink([],LinkMetadata.new().withDefinition(value));return{...packet,witness:{metadata:key(metadata),numberKind:name}};}}));
}
