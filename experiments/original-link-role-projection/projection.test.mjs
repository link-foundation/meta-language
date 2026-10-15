import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary,encodeTypedProgramLinks,emitTypedProgramLinks} from '../../js/src/index.js';
import {TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {createPortableRustTextCertificate,encodePortableRustTextCertificate,evaluateRustTextCertificate} from '../rust-text-certificate/bridge.mjs';
import {emitRustTextValueProjection} from '../rust-text-value-projection/model.mjs';
import {registerOriginalLinkRoles as register,projectOriginalLinkRoleRegistry as project} from './rules.mjs';
import {projectOriginalLinkCertificateRoles,ORIGINAL_LINK_ROLE_POLICY as policy} from './transport.mjs';
import {originalLinkRoleRulesSource} from './policy.mjs';
import {generateOriginalLinkRoleProjection} from './generate.mjs';

const fixtures=readProjectData(new URL('../rust-text-value-projection/data/fixtures.lino',import.meta.url));
const key=id=>id.asU64().toString();
const label=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
const field=(network,parent,name)=>{const matches=network.fieldRelations().filter(row=>row.parent.equals(parent)&&row.label===label(name));assert.equal(matches.length,1,name);return matches[0];};
const refs=(network,id)=>network.link(id).references();
const body=packet=>refs(packet.network,packet.root)[0];
function attach(packet){const proof=field(packet.network,field(packet.network,packet.root,'provenance').child,'sourceInterpretation').child;packet.network.insertFieldRelation(body(packet),label('sourceTextValueProjection'),proof);return packet;}
const create=fixture=>{const value=createPortableRustTextCertificate(fixture.source);return attach(encodePortableRustTextCertificate(value.program,value.proof));};
const loads=[packet=>({...packet,network:LinkNetwork.fromLino(packet.network.toLino())}),packet=>({...packet,network:decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))})];
const replace=(row,child)=>row.link.setReferences([row.parent,row.link.references()[1],child]);
const proof=packet=>field(packet.network,body(packet),'sourceTextValueProjection').child;
const rows=(packet,name)=>refs(packet.network,field(packet.network,proof(packet),name).child);
function unpack(result){const[decision,count,occurrences]=result;assert.equal(decision,1);return{anchors:result.slice(3,3+count),kinds:result.slice(3+count,4+2*count),handles:result.slice(4+2*count),occurrences};}

// Independent test correspondence reads actual raw fields/ordered references.
// It never infers same-owner identity from reconstructed object equality.
function remapExistingTables(packet,registration){
 const{network}=packet,table=registration.transport.checked.table,map=new Map(),owners=new Map();
 const handle=new Map(registration.anchors.map((id,index)=>[key(id),index+1]));
 function bind(id,old){assert.ok(Number.isInteger(old)&&old>0);const actual=handle.get(key(id));assert.ok(actual,'actual role is registered');if(map.has(old))assert.equal(map.get(old),actual);if(owners.has(actual))assert.equal(owners.get(actual),old);map.set(old,actual);owners.set(actual,old);assert.equal(registration.kinds[actual],table.kinds[old]);}
 const fld=(id,name)=>field(network,id,name).child,children=id=>refs(network,id);
 const tokens=children(fld(proof(packet),'tokens'));tokens.forEach((id,index)=>bind(id,table.tokens[index*9]));
 const declarations=children(fld(body(packet),'declarations')).map(pair=>children(pair)[1]);let parameter=0,position=0;
 for(const[index,id]of declarations.entries()){
  const offset=index*20,signature=fld(id,'sourceSignature');bind(id,table.declarations[offset]);bind(signature,table.declarations[offset+4]);bind(fld(id,'body'),table.declarations[offset+5]);
  for(const input of children(fld(id,'params')))bind(input,table.parameters[parameter++*8]);
  for(const item of [...children(fld(signature,'parameters')),fld(signature,'result')])bind(item,table.positions[position++*11]);
 }
 let operation=0;const seen=new Set();
 function walk(id){if(seen.has(key(id)))return;seen.add(key(id));bind(id,table.operations[operation++*6]);for(const child of children(fld(id,'parts'))){const kind=network.fieldRelations().find(row=>row.parent.equals(child)&&row.label===label('kind'));if(kind&&network.link(kind.child).metadata().definition==='source operation')walk(child);}}
 walk(fld(proof(packet),'operations'));bind(fld(body(packet),'main'),table.context[2]);
 assert.equal(map.size,table.kinds.length-1);assert.equal(map.size,registration.kinds.length-1);
 const identity=value=>value<=0?value:(assert.ok(map.has(value),'no orphan original role'),map.get(value));
 const output=Object.fromEntries(Object.entries(table).map(([name,values])=>[name,[...values]]));output.kinds=[...registration.kinds];
 const rewrite=(name,width,columns)=>{for(let row=0;row<output[name].length;row+=width)for(const column of columns)output[name][row+column]=identity(output[name][row+column]);};
 rewrite('context',15,[2,6,10]);rewrite('tokens',9,[0,4]);rewrite('declarations',20,[0,4,5,10,19]);rewrite('parameters',8,[0,1,5,6,7]);rewrite('positions',11,[0,1]);rewrite('cores',7,[0,1]);rewrite('functions',8,[0,1,2,3,4,5,6,7]);rewrite('operations',6,[0]);rewrite('parts',1,[0]);rewrite('inputs',7,[2,3,4,5,6]);rewrite('allocations',7,[2,3,4,5,6]);rewrite('items',1,[0]);
 assert.deepEqual(evaluateRustTextCertificate(output),registration.transport.checked.decision);
 return{table:output,map};
}

test('compact registration preserves aliases, first occurrence order and provisional role upgrades',()=>{
 const members=[8,2,8,5,2],roles=[9,2,6,1,9],before=JSON.stringify([members,roles]);Object.freeze(members);Object.freeze(roles);
 const out=unpack(register(100,members,roles));assert.deepEqual(out,{anchors:[8,2,5],kinds:[0,6,2,1],handles:[1,2,1,3,2],occurrences:5});assert.equal(JSON.stringify([members,roles]),before);
 assert.deepEqual(register(0,[],[]),[1,0,0,0]);assert.deepEqual(register(100000,[100000],[8]),[1,1,1,100000,0,8,1]);
});

test('invalid ordinals, scalar coercions, contradictory roles and orphan kind nine refuse exactly',()=>{
 for(const value of [-1,100001,0.5,NaN,Infinity,true,'1',null,undefined])assert.deepEqual(register(value,[],[]),[-1]);
 assert.deepEqual(register(2,[1],[]),[-2]);assert.deepEqual(register(1,Array(policy.maximumOccurrences+1).fill(1),Array(policy.maximumOccurrences+1).fill(1)),[-2]);
 for(const value of [0,-0,-1,3,0.5,NaN,Infinity,true,'1',null,undefined])assert.deepEqual(register(2,[value],[1]),[-3,0]);
 assert.deepEqual(register(1,Array(1),[1]),[-3,0]);
 for(const value of [0,-0,-1,10,0.5,NaN,Infinity,true,'1',null,undefined])assert.deepEqual(register(2,[1],[value]),[-4,0]);
 assert.deepEqual(register(2,[1,1],[9,9]),[-6,1]);assert.deepEqual(register(2,[1,1],[2,6]),[-5,1]);assert.deepEqual(register(2,[1,1,1],[9,2,6]),[-5,2]);
});

test('deterministic role instructions agree with an independent Map registration oracle',()=>{
 let seed=17;for(let trial=0;trial<40;trial++){
  const members=[],roles=[];for(let index=0;index<32;index++){seed=(seed*1664525+1013904223)>>>0;const member=seed%17+1;members.push(member);roles.push(member%8+1);}
  const expected=new Map();for(let index=0;index<members.length;index++)if(!expected.has(members[index]))expected.set(members[index],roles[index]);
  const out=unpack(register(17,members,roles));assert.deepEqual(out.anchors,[...expected.keys()]);assert.deepEqual(out.kinds,[0,...expected.values()]);assert.deepEqual(out.handles,members.map(member=>[...expected.keys()].indexOf(member)+1));
 }
});

test('all thirteen real text/binary archives replace the actual kinds table through original IDs',()=>{
 assert.equal(fixtures.cases.length,13);
 for(const fixture of fixtures.cases)for(const load of loads){
  const packet=load(create(fixture)),before=packet.network.toLino(),registration=projectOriginalLinkCertificateRoles(packet.network,packet.root);remapExistingTables(packet,registration);
  assert.ok(registration.transport.anchors.length>registration.anchors.length);assert.equal(registration.kinds.includes(9),false);assert.equal(packet.network.toLino(),before);
  const original=packet.network.fieldRelations();assert.deepEqual(registration.transport.relations.map(row=>[key(row.relation),key(row.parent),key(row.label),key(row.child)]),original.map(row=>[key(row.link.id()),key(row.parent),key(row.link.references()[1]),key(row.child)]));
  for(const id of registration.anchors)assert.equal(packet.network.link(id).id(),id);
 }
});

test('equal-valued original owners stay distinct and all seven forged cross-owner roles still refuse',()=>{
 const fixture=fixtures.cases.find(row=>row.id==='equal-valued-distinct-owners');
 for(const load of loads){
  const packet=load(create(fixture)),registration=projectOriginalLinkCertificateRoles(packet.network,packet.root),functions=rows(packet,'functions');
  for(const role of ['declaration','signature','resultPosition','operationNode','coreValue','input','position']){
   const first=field(packet.network,functions[0],role).child,second=field(packet.network,functions[1],role).child;assert.notEqual(key(first),key(second));assert.notEqual(registration.anchors.findIndex(id=>id.equals(first)),registration.anchors.findIndex(id=>id.equals(second)));
   const forged=load(create(fixture)),facts=rows(forged,'functions');replace(field(forged.network,facts[0],role),field(forged.network,facts[1],role).child);assert.throws(()=>projectOriginalLinkCertificateRoles(forged.network,forged.root));
  }
 }
});

test('equal-looking cloned owners cannot pass the existing checker by obtaining membership',()=>{
 const fixture=fixtures.cases.find(row=>row.id==='owned-clone');
 for(const role of ['declaration','signature','resultPosition','operationNode','coreValue','input','position']){
  const packet=create(fixture),target=field(packet.network,rows(packet,'functions')[0],role),source=packet.network.link(target.child),copy=packet.network.insertLink(source.references(),source.metadata().clone());
  for(const row of packet.network.fieldRelations().filter(row=>row.parent.equals(target.child)))packet.network.insertFieldRelation(copy,row.label,row.child);replace(target,copy);
  for(const load of loads){const saved=load(packet);assert.throws(()=>projectOriginalLinkCertificateRoles(saved.network,saved.root));}
 }
});

test('a coherent saved parameter retarget preserves the same registered original roles',()=>{
 const value=createPortableRustTextCertificate(fixtures.cases.find(row=>row.id==='valid-clone-retarget').source),entry=value.program.declarations.get('choose'),fact=value.proof.functions[0],old=fact.input,target=entry.params[1];
 const token=value.proof.tokens.find(token=>token.binding===old&&token.span.start===fact.coreValue.span.start);assert.ok(token);token.binding=target;entry.body.name=target.name;fact.input=target;fact.position=entry.sourceSignature.parameters[1];
 for(const condition of [...value.proof.inputAssumptions,...value.proof.valueAssumptions])if(condition.declaration===entry){condition.input=target;condition.position=fact.position;}
 for(const load of loads){const packet=load(attach(encodePortableRustTextCertificate(value.program,value.proof))),registration=projectOriginalLinkCertificateRoles(packet.network,packet.root);remapExistingTables(packet,registration);const checked=registration.transport.checked;assert.equal(checked.proof.functions[0].input,checked.program.declarations.get('choose').params[1]);const context={};vm.runInNewContext(emitRustTextValueProjection(checked.program,checked.proof,'JavaScript').text,context,{timeout:1000});assert.equal(context.choose('first','second'),'second');}
});

test('the complete registry executes from checked text/binary Links without original JS source',()=>{
 const program=checkProgram(parseJavaScript(originalLinkRoleRulesSource(),{referenceSemantics:false})),packet=encodeTypedProgramLinks(program),sample=create(fixtures.cases[0]),actual=projectOriginalLinkCertificateRoles(sample.network,sample.root);
 for(const load of loads){const saved=load(packet),context={};vm.runInNewContext(emitTypedProgramLinks(saved.network,saved.root,'JavaScript').code,context,{timeout:2000});
  assert.deepEqual(Array.from(context.registerOriginalLinkRoles(10,[8,2,8],[9,2,6])),register(10,[8,2,8],[9,2,6]));
  assert.deepEqual(Array.from(context.projectOriginalLinkRoleRegistry(...actual.transport.arguments)),project(...actual.transport.arguments));
 }
});

test('transport shape and root errors refuse before role registration',()=>{
 const packet=create(fixtures.cases[0]),actual=projectOriginalLinkCertificateRoles(packet.network,packet.root);
 const args=actual.transport.arguments.map(value=>Array.isArray(value)?[...value]:value);args[2].pop();assert.deepEqual(project(...args),[-11]);
 const wrong=actual.transport.arguments.map(value=>Array.isArray(value)?[...value]:value);wrong[1]=actual.members[0];assert.deepEqual(project(...wrong),[-10]);
});

test('shared instructions and the entire emitted Rust artifact regenerate without native compilation',()=>{
 const result=generateOriginalLinkRoleProjection(true);assert.equal(result.declarations.length,20);assert.ok(result.completeRust.includes('pub fn project_original_link_role_registry('));assert.ok(result.completeRust.includes('fn main('));assert.ok(result.declarationNames.includes('registerOriginalLinkRoles'));assert.equal(result.model,policy.model);
});
