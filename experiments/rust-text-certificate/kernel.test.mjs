import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../js/src/index.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {emitRustTextValueProjection} from '../rust-text-value-projection/model.mjs';
import {bundledTextCertificateSource} from './generate.mjs';
import {RUST_TEXT_CERTIFICATE_POLICY as policy,projectRustTextCertificate,evaluateRustTextCertificate,checkPortableRustTextCertificate,createPortableRustTextCertificate,encodePortableRustTextCertificate,decodePortableRustTextCertificate} from './bridge.mjs';

const fixtures=readProjectData(new URL('../rust-text-value-projection/data/fixtures.lino',import.meta.url));
const keys=['context','kinds','tokens','declarations','parameters','positions','paths','layers','cores','functions','operations','parts','inputs','allocations','names','units','items'];
const source=id=>fixtures.cases.find(row=>row.id===id).source;
const width=layout=>policy.layouts[layout].length;
const column=(layout,name)=>policy.layouts[layout].indexOf(name);
const symbol=value=>policy.symbols.indexOf(value)+1;
const text=packet=>({...packet,network:LinkNetwork.fromLino(packet.network.toLino())});
const binary=packet=>({...packet,network:decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))});
const reload=packet=>binary(text(packet));
const decode=packet=>decodePortableRustTextCertificate(packet.network,packet.root);
const create=id=>createPortableRustTextCertificate(source(id));
function accepted(table){
 const decision=evaluateRustTextCertificate(table);
 assert.deepEqual(decision,[1,table.tokens.length/width('token'),table.declarations.length/width('declaration'),table.inputs.length/width('condition'),table.allocations.length/width('condition')]);
 return decision;
}
function refused(table,label){
 let decision;try{decision=evaluateRustTextCertificate(table);}catch(error){assert.ok(error instanceof TypeError,label);return;}
 assert.notEqual(decision[0],1,label);
}
function retarget(value,index){
 const entry=value.program.declarations.get('choose'),row=value.proof.functions[0],target=entry.params[index];
 const token=value.proof.tokens.find(token=>token.binding===row.input&&token.span.start===row.coreValue.span.start);assert.ok(token);
 token.binding=target;entry.body.name=target.name;row.input=target;row.position=entry.sourceSignature.parameters[index];
 for(const condition of[...value.proof.inputAssumptions,...value.proof.valueAssumptions])if(condition.declaration===entry){condition.input=row.input;condition.position=row.position;}
}
function runValue(value,observations){
 const emitted=emitRustTextValueProjection(value.program,value.proof,'JavaScript'),context={};vm.runInNewContext(emitted.text,context,{timeout:1000});
 for(const row of observations)assert.equal(context.choose(...row.args),row.value);
}
const label=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
function field(network,parent,name){const row=network.fieldRelations().find(row=>row.parent.equals(parent)&&row.label===label(name));assert.ok(row,name);return row;}
function replaceField(row,child){row.link.setReferences([row.parent,row.link.references()[1],child]);}
function rawProof(packet){return field(packet.network,field(packet.network,packet.root,'provenance').child,'sourceInterpretation').child;}
function rows(packet,name){return packet.network.link(field(packet.network,rawProof(packet),name).child).references();}

test('all thirteen complete originals retain source bytes and actual semantic owners through the portable kernel and both Links codecs',()=>{
 assert.equal(fixtures.cases.length,13);
 for(const entry of policy.protectedSources)assert.equal(createHash('sha256').update(readFileSync(new URL(entry.path,import.meta.url))).digest('hex'),entry.sha256);
 for(const entry of fixtures.cases){
  assert.equal(Buffer.byteLength(entry.source),entry.sourceBytes);assert.equal(createHash('sha256').update(entry.source).digest('hex'),entry.sourceSha256);
  const value=createPortableRustTextCertificate(entry.source);accepted(value.table);assert.deepEqual(projectRustTextCertificate(value.program,value.proof),value.table);
  const packet=encodePortableRustTextCertificate(value.program,value.proof),restored=decode(reload(packet));
  assert.deepEqual(restored.table,value.table,entry.id);assert.deepEqual(restored.decision,value.decision,entry.id);
  assert.equal(emitRustTextValueProjection(restored.program,restored.proof,'Rust').text,entry.source,entry.id);runValue(restored,entry.observations);
  for(const row of restored.proof.functions){assert.equal(row.declaration,restored.program.declarations.get(row.declaration.fullName));assert.equal(row.signature,row.declaration.sourceSignature);assert.equal(row.resultPosition,row.signature.result);assert.equal(row.coreValue,row.declaration.body);}
 }
});

test('coherent actual clone retargets accept String and shared String while rejecting the distinct str clone semantics',()=>{
 let value=create('valid-clone-retarget');retarget(value,1);accepted(checkPortableRustTextCertificate(value.program,value.proof).table);
 value=decode(reload(encodePortableRustTextCertificate(value.program,value.proof)));runValue(value,[{args:['first','second Ω😀'],value:'second Ω😀'}]);
 retarget(value,0);value=decode(reload(encodePortableRustTextCertificate(value.program,value.proof)));runValue(value,[{args:['first Ω😀','second'],value:'first Ω😀'}]);
 const invalid=create('invalid-slice-retarget');retarget(invalid,1);refused(projectRustTextCertificate(invalid.program,invalid.proof));
 assert.throws(()=>checkPortableRustTextCertificate(invalid.program,invalid.proof),/generated certificate kernel refused/u);
 assert.throws(()=>encodePortableRustTextCertificate(invalid.program,invalid.proof),/generated certificate kernel refused/u);
});

test('direct numeric certificates consume all rows and reject omissions, duplicates, scope and owner counterfeits',()=>{
 const original=create('equal-valued-distinct-owners').table,mutations=[];
 for(const[key,layout]of[['tokens','token'],['declarations','declaration'],['parameters','parameter'],['positions','position'],['cores','core'],['functions','function'],['operations','operation'],['inputs','condition'],['allocations','condition']]){
  mutations.push([key+' omitted',table=>{table[key].splice(0,width(layout));}]);
  mutations.push([key+' duplicated',table=>{table[key].push(...table[key].slice(0,width(layout)));}]);
 }
 for(const[key,layout,name]of[['parameters','parameter','declaration'],['positions','position','signature'],['cores','core','declaration'],['functions','function','declaration'],['functions','function','signature'],['functions','function','resultPosition'],['functions','function','operation'],['functions','function','core'],['functions','function','input'],['functions','function','position'],['inputs','condition','declaration'],['inputs','condition','operation'],['inputs','condition','core'],['inputs','condition','input'],['inputs','condition','position'],['allocations','condition','operation']]){
  mutations.push([key+'.'+name+' other owner',table=>{const at=column(layout,name),other=table[key].findIndex((value,index)=>index>=width(layout)&&index%width(layout)===at&&value!==table[key][at]);assert.ok(other>=0,key+'.'+name);table[key][at]=table[key][other];}]);
 }
 mutations.push(['worker is detached',table=>{table.context[column('context','proofWorker')]=table.declarations[0];}],['main effects are present',table=>{table.context[column('context','mainEffects')]=1;}],['wrong root operation',table=>{table.context[column('context','rootOperation')]=table.functions[column('function','operation')];}],['wrong observation',table=>{table.context[column('context','observation')]=symbol('Rust');}],['declaration map owner differs',table=>{table.declarations[column('declaration','mapKey')]=table.declarations[width('declaration')+column('declaration','name')];}],['items omitted',table=>{table.items.pop();}],['items duplicated',table=>{table.items.push(table.items[0]);}],['items reordered',table=>{table.items.reverse();}],['operation child points to a declaration',table=>{table.parts[0]=table.declarations[0];}],['condition meaning is another symbol',table=>{table.inputs[column('condition','meaning')]=symbol('Rust');}],['allocation condition is an input condition',table=>{table.allocations.splice(0,width('condition'),...table.inputs.slice(0,width('condition')));}]);
 for(const[name,mutate]of mutations){const table=structuredClone(original);mutate(table);refused(table,name);}
});

test('distinct owners cannot counterfeit a duplicate semantic declaration name using separate lexical aliases',()=>{
 const table=structuredClone(create('equal-valued-distinct-owners').table),w=width('declaration'),same=table.declarations[column('declaration','name')];
 for(const name of['name','fullName','mapKey'])table.declarations[w+column('declaration',name)]=same;
 for(let index=0;index<table.tokens.length;index+=width('token'))if(table.tokens[index+column('token','role')]===policy.roles.declaration&&table.tokens[index+column('token','target')]===table.declarations[w])table.tokens[index+column('token','value')]=same;
 refused(table,'two actual declarations cannot share one semantic name');
});

test('numeric transport refuses absent and unknown identities, scalar domain violations and counterfeit symbol storage',()=>{
 const original=create('owned-clone').table;
 for(const scalar of['1',false,null,undefined,NaN,Infinity,-2,9007199254740992]){const table=structuredClone(original);table.tokens[0]=scalar;refused(table,String(scalar));}
 for(const mutate of[table=>{table.kinds[table.tokens[0]]=policy.identityKinds.unregistered;},table=>{table.kinds.push(policy.identityKinds.parameter);},table=>{table.units[table.names[symbol('Rust')*2]]=88;},table=>{table.names[3]++;},table=>{table.units.push(0);},table=>{table.paths.push(symbol('Rust'));},table=>{table.layers.push(0,0);},table=>{table.parts.push(table.parts[0]);}]){const table=structuredClone(original);mutate(table);refused(table);}
 const negativeZero=structuredClone(original);assert.equal(negativeZero.context[column('context','sourceStart')],0);negativeZero.context[column('context','sourceStart')]=-0;assert.equal(evaluateRustTextCertificate(negativeZero)[0],-1);
});

test('an invalid unused parameter domain cannot turn the negative type selector into an accepted punctuation token',()=>{
 const value=createPortableRustTextCertificate(source('valid-clone-retarget').replace('right:&String','right:String')),table=structuredClone(value.table),entry=value.program.declarations.get('choose');
 const binder=value.proof.tokens.findIndex(token=>token.binding===entry.params[1]);assert.ok(binder>=0);assert.equal(value.proof.tokens[binder+2].raw,'String');
 table.positions[width('position')+column('position','ownerKind')]=symbol('bool');
 table.tokens[(binder+2)*width('token')+column('token','syntax')]=-1;table.tokens[(binder+2)*width('token')+column('token','kind')]=symbol('punct');
 refused(table);
});

test('transport records and arrays reject hidden fields, getters and sparse or non-array data before observing a getter',()=>{
 const original=create('owned-clone').table;
 for(const mutate of[table=>{table.extra=false;},table=>{delete table.context;},table=>{table.tokens.extra=false;},table=>{delete table.tokens[0];},table=>{table.tokens={...table.tokens};}]){const table=structuredClone(original);mutate(table);refused(table);}
 for(const key of['context','tokens']){let calls=0;const table=structuredClone(original);Object.defineProperty(table,key,{get(){calls++;return original[key];},enumerable:true});assert.throws(()=>evaluateRustTextCertificate(table));assert.equal(calls,0);}
 let calls=0;const table=structuredClone(original);Object.defineProperty(table.tokens,'0',{get(){calls++;return 1;},enumerable:true});assert.throws(()=>evaluateRustTextCertificate(table));assert.equal(calls,0);
});

test('actual source-free graph ownership and conditions are checked independently of equal record contents',()=>{
 const mutations=[value=>{value.proof.functions[0].signature=structuredClone(value.proof.functions[0].signature);},value=>{value.proof.functions[0].resultPosition=structuredClone(value.proof.functions[0].resultPosition);},value=>{value.proof.functions[0].operationNode=structuredClone(value.proof.functions[0].operationNode);},value=>{value.proof.functions[0].coreValue=structuredClone(value.proof.functions[0].coreValue);},value=>{value.proof.functions[0].input=structuredClone(value.proof.functions[0].input);},value=>{value.proof.functions[0].position=structuredClone(value.proof.functions[0].position);},value=>{value.proof.worker={...value.proof.worker};},value=>{value.proof.valueAssumptions=[];},value=>{value.proof.inputAssumptions[0].position=null;},value=>{value.proof.operations.parts[0].parts.at(-1).parts[1]=value.proof.operations;}];
 for(const mutate of mutations){const value=create('owned-clone');mutate(value);assert.throws(()=>checkPortableRustTextCertificate(value.program,value.proof));assert.throws(()=>encodePortableRustTextCertificate(value.program,value.proof));}
});

test('genuine saved text and binary Links reject other declarations and header positions with equal values',()=>{
 for(const load of[text,binary])for(const role of['declaration','signature','resultPosition','operationNode','coreValue','input','position']){
  const value=create('equal-valued-distinct-owners'),packet=encodePortableRustTextCertificate(value.program,value.proof),ids=rows(packet,'functions'),row=field(packet.network,ids[0],role),other=field(packet.network,ids[1],role);assert.ok(!row.child.equals(other.child));replaceField(row,other.child);assert.throws(()=>decode(load(packet)),role);
 }
 for(const load of[text,binary]){
  const value=create('valid-clone-retarget'),packet=encodePortableRustTextCertificate(value.program,value.proof),functionIds=rows(packet,'functions'),signature=field(packet.network,functionIds[0],'signature').child,positions=packet.network.link(field(packet.network,signature,'parameters').child).references();
  replaceField(field(packet.network,functionIds[0],'position'),positions[1]);assert.throws(()=>decode(load(packet)));
  const other=create('equal-valued-distinct-owners'),otherPacket=encodePortableRustTextCertificate(other.program,other.proof),conditions=rows(otherPacket,'valueAssumptions'),functions=rows(otherPacket,'functions');replaceField(field(otherPacket.network,conditions[0],'operationNode'),field(otherPacket.network,functions[1],'operationNode').child);assert.throws(()=>decode(load(otherPacket)));
 }
});

test('the entire untrimmed generated certificate kernel executes all thirteen actual transports after complete text and binary Links replay',{timeout:180000},()=>{
 const sourceText=bundledTextCertificateSource();
 let checked=checkProgram(parseJavaScript(sourceText));
 const artifact=readProjectData(new URL('./data/translation.lino',import.meta.url));assert.deepEqual(checked.items.map(row=>row.fullName),artifact.declarations);assert.ok(checked.items.length>=90);assert.ok(checked.declarations.has('checkRustTextCertificate'));
 let packet=encodeTypedProgramLinks(checked);const root=packet.root;checked=null;
 let serialized=packet.network.toLino();packet=null;
 let network=LinkNetwork.fromLino(serialized);serialized=null;
 let bytes=encodeLinkNetworkBinary(network);network=null;
 let restored=decodeLinkNetworkBinary(bytes);bytes=null;
 const program=decodeTypedProgramLinks(restored,root);restored=null;
 const emitted=emitJavaScript(program),context={};vm.runInNewContext(emitted.text,context,{timeout:10000});
 const target=emitted.mappings.find(row=>row.source==='checkRustTextCertificate')?.target;assert.equal(typeof context[target],'function');
 const evaluate=table=>Array.from(context[target](...keys.map(name=>table[name])));
 for(const entry of fixtures.cases){const{table}=createPortableRustTextCertificate(entry.source);assert.deepEqual(evaluate(table),accepted(table),entry.id);}
 const value=create('valid-clone-retarget');retarget(value,1);let table=projectRustTextCertificate(value.program,value.proof);assert.deepEqual(evaluate(table),accepted(table));
 const invalid=create('invalid-slice-retarget');retarget(invalid,1);table=projectRustTextCertificate(invalid.program,invalid.proof);assert.deepEqual(evaluate(table),evaluateRustTextCertificate(table));assert.notEqual(evaluate(table)[0],1);
 for(const mutate of[table=>{table.functions[column('function','position')]=-1;},table=>{table.tokens.splice(0,width('token'));},table=>{table.inputs.length=0;},table=>{table.allocations.push(...table.allocations.slice(0,width('condition')));},table=>{table.parts[0]=table.declarations[0];}]){table=structuredClone(create('owned-clone').table);mutate(table);assert.deepEqual(evaluate(table),evaluateRustTextCertificate(table));assert.notEqual(evaluate(table)[0],1);}
});
