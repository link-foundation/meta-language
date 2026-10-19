import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readFileSync} from 'node:fs';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary,encodeTypedProgramLinks,emitTypedProgramLinks} from '../../js/src/index.js';
import {TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {createPortableRustTextCertificate,encodePortableRustTextCertificate,evaluateRustTextCertificate} from '../rust-text-certificate/bridge.mjs';
import {checkRustTextCertificate} from '../rust-text-certificate/rules.mjs';
import {emitRustTextValueProjection,encodeRustTextValueProjection} from '../rust-text-value-projection/model.mjs';
import {projectOriginalLinkCompleteCertificateTables as project,readOriginalLinkCompleteStringQueries} from './rules.mjs';
import {projectOriginalLinkCompleteTables,ORIGINAL_LINK_COMPLETE_TABLE_POLICY as policy} from './transport.mjs';
import {originalLinkCompleteTableRulesSource} from './policy.mjs';
import {generateOriginalLinkCompleteTableProjection} from './generate.mjs';
const fixtures=readProjectData(new URL('../rust-text-value-projection/data/fixtures.lino',import.meta.url));
const key=id=>id.asU64().toString();
const label=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
const field=(network,parent,name)=>{const found=network.fieldRelations().filter(row=>row.parent.equals(parent)&&row.label===label(name));assert.equal(found.length,1,name);return found[0];};
const refs=(network,id)=>network.link(id).references();
const body=packet=>refs(packet.network,packet.root)[0];
const proof=packet=>field(packet.network,body(packet),'sourceTextValueProjection').child;
const rows=(packet,name)=>refs(packet.network,field(packet.network,proof(packet),name).child);
const replace=(row,child)=>row.link.setReferences([row.parent,row.link.references()[1],child]);
const loads=[packet=>({...packet,network:LinkNetwork.fromLino(packet.network.toLino())}),packet=>({...packet,network:decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))})];
function attach(packet){const source=field(packet.network,field(packet.network,packet.root,'provenance').child,'sourceInterpretation').child;packet.network.insertFieldRelation(body(packet),label('sourceTextValueProjection'),source);return packet;}
const create=fixture=>{const value=createPortableRustTextCertificate(fixture.source);return attach(encodePortableRustTextCertificate(value.program,value.proof));};
const copy=args=>args.map(value=>Array.isArray(value)?[...value]:value);

// Test-only independent raw-role correspondence. Expected values stay with the
// old projector; no reconstructed object equality establishes raw identity.
function expectedTables(packet,registration){
 const{network}=packet,table=registration.transport.checked.table,map=new Map(),reverse=new Map(),handle=new Map(registration.anchors.map((id,index)=>[key(id),index+1]));
 function bind(id,old){const actual=handle.get(key(id));assert.ok(actual);if(map.has(old))assert.equal(map.get(old),actual);if(reverse.has(actual))assert.equal(reverse.get(actual),old);map.set(old,actual);reverse.set(actual,old);assert.equal(registration.kinds[actual],table.kinds[old]);}
 const fld=(id,name)=>field(network,id,name).child,children=id=>refs(network,id);
 children(fld(proof(packet),'tokens')).forEach((id,index)=>bind(id,table.tokens[index*9]));let parameter=0,position=0;
 const declarations=children(fld(body(packet),'declarations')).map(pair=>children(pair)[1]);
 for(const[index,id]of declarations.entries()){
  const offset=index*20,signature=fld(id,'sourceSignature');bind(id,table.declarations[offset]);bind(signature,table.declarations[offset+4]);bind(fld(id,'body'),table.declarations[offset+5]);
  for(const input of children(fld(id,'params')))bind(input,table.parameters[parameter++*8]);
  for(const item of [...children(fld(signature,'parameters')),fld(signature,'result')])bind(item,table.positions[position++*11]);
 }
 let operation=0;const seen=new Set();function walk(id){if(seen.has(key(id)))return;seen.add(key(id));bind(id,table.operations[operation++*6]);for(const child of children(fld(id,'parts'))){const kind=network.fieldRelations().find(row=>row.parent.equals(child)&&row.label===label('kind'));if(kind&&network.link(kind.child).metadata().definition==='source operation')walk(child);}}
 walk(fld(proof(packet),'operations'));bind(fld(body(packet),'main'),table.context[2]);assert.equal(map.size,table.kinds.length-1);assert.equal(map.size,registration.kinds.length-1);
 const identity=value=>value<=0?value:(assert.ok(map.has(value)),map.get(value));
 const out=Object.fromEntries(Object.entries(table).map(([name,values])=>[name,[...values]]));out.kinds=[...registration.kinds];
 const rewrite=(name,width,columns)=>{for(let row=0;row<out[name].length;row+=width)for(const column of columns)out[name][row+column]=identity(out[name][row+column]);};
 rewrite('context',15,[2,6,10]);rewrite('tokens',9,[0,4]);rewrite('declarations',20,[0,4,5,10,19]);rewrite('parameters',8,[0,1,5,6,7]);rewrite('positions',11,[0,1]);rewrite('cores',7,[0,1]);rewrite('functions',8,[0,1,2,3,4,5,6,7]);rewrite('operations',6,[0]);rewrite('parts',1,[0]);rewrite('inputs',7,[2,3,4,5,6]);rewrite('allocations',7,[2,3,4,5,6]);rewrite('items',1,[0]);return out;
}
function compare(packet,result){const expected=expectedTables(packet,result.transport.fixed.role);assert.deepEqual(Object.keys(result.tables),policy.output);for(const name of policy.output)assert.deepEqual(result.tables[name],expected[name],name);assert.deepEqual(result.decision,result.transport.fixed.role.transport.checked.decision);return expected;}
function tableRows(args){const output=project(...args);assert.equal(output.length,policy.output.length);return Object.fromEntries(policy.output.map((name,index)=>[name,output[index]]));}
function ordinal(result,id){const value=result.transport.fixed.role.transport.anchors.findIndex(anchor=>anchor.equals(id))+1;assert.ok(value);return value;}
function scalar(result,packet,parent,path){let node=parent;for(const name of path)node=field(packet.network,node,name).child;return ordinal(result,node)-1;}
function execute(result,name,args,expected){const checked=result.transport.fixed.role.transport.checked,context={};vm.runInNewContext(emitRustTextValueProjection(checked.program,checked.proof,'JavaScript').text,context,{timeout:1000});assert.equal(context[name](...args),expected);}
// Independent value/order oracle for the old projector's symbol() calls. This
// oracle does not choose raw identities or provide any table input to the kernel.
function expectedQueries(program,proof){const out=[];for(const token of proof.tokens){const role=['declaration','binding','literal'].find(name=>Object.hasOwn(token,name)),target=role?token[role]:null;out.push(token.kind);if(!role)out.push(token.value);out.push(role==='declaration'||role==='binding'?target?.name:role==='literal'?target?.value:token.value,token.raw);}
 const position=value=>{out.push(...value.owner.path);for(let index=0;index<Math.max(value.wrappers.length,value.lifetimes.length);index++){out.push(value.wrappers[index]);if(value.lifetimes[index]!=null)out.push(value.lifetimes[index]);}out.push(value.type.kind,value.owner.kind);};
 for(const[key,entry]of program.declarations){for(const input of entry.params)out.push(input.name,input.type.kind);for(const value of entry.sourceSignature.parameters)position(value);position(entry.sourceSignature.result);out.push(entry.body.k,entry.body.type.kind,Object.hasOwn(entry.body,'name')?entry.body.name:entry.body.value,entry.name,entry.fullName,entry.ret.kind,key,entry.sourceSignature.model,entry.sourceSignature.language,entry.k);}
 for(const values of[proof.inputAssumptions,proof.valueAssumptions])for(const row of values)out.push(row.id,row.meaning);out.push(program.sourceLanguage,proof.model,proof.language,proof.observationModel);return out.map(value=>typeof value==='string'?value:undefined);
}

test('all thirteen real text/binary archives match all seventeen tables and the complete certificate',()=>{
 assert.equal(fixtures.cases.length,13);assert.equal(policy.output.length,17);
 for(const fixture of fixtures.cases)for(const load of loads){const packet=load(create(fixture)),before=packet.network.toLino(),result=projectOriginalLinkCompleteTables(packet.network,packet.root);compare(packet,result);assert.equal(packet.network.toLino(),before);assert.equal(result.tables.kinds.includes(9),false);}
});

test('pure original-ID String queries retain the actual old symbol traversal and first-observed order',()=>{
 for(const fixture of fixtures.cases){const packet=create(fixture),result=projectOriginalLinkCompleteTables(packet.network,packet.root),a=result.transport.arguments,checked=result.transport.fixed.role.transport.checked;const queries=readOriginalLinkCompleteStringQueries(a[1],a[2],a[3],a[4],a[5],a[6],a[7]);const values=queries.map(node=>node>0&&a[7][node-1]===2?String.fromCharCode(...a[13].slice(a[11][node-1],a[11][node-1]+a[12][node-1])):undefined);assert.deepEqual(values,expectedQueries(checked.program,checked.proof),fixture.id);}
});

test('coherent saved input retargets change actual owners and executable results in all seventeen tables',()=>{
 const value=createPortableRustTextCertificate(fixtures.cases.find(row=>row.id==='valid-clone-retarget').source),entry=value.program.declarations.get('choose'),fact=value.proof.functions[0],old=fact.input,target=entry.params[1],token=value.proof.tokens.find(token=>token.binding===old&&token.span.start===fact.coreValue.span.start);assert.ok(token);token.binding=target;entry.body.name=target.name;fact.input=target;fact.position=entry.sourceSignature.parameters[1];
 for(const condition of [...value.proof.inputAssumptions,...value.proof.valueAssumptions])if(condition.declaration===entry){condition.input=target;condition.position=fact.position;}
 for(const load of loads){const packet=load(attach(encodePortableRustTextCertificate(value.program,value.proof))),result=projectOriginalLinkCompleteTables(packet.network,packet.root);compare(packet,result);execute(result,'choose',['first','second Ω😀'],'second Ω😀');const input=ordinal(result,field(packet.network,rows(packet,'functions')[0],'input').child);assert.equal(result.tables.functions[5],input);assert.equal(result.tables.inputs[5],input);assert.equal(result.tables.allocations[5],input);}
});

test('source-free declaration/parameter renames and Unicode/control literal values feed new dynamic tables',()=>{
 const baseline=create(fixtures.cases.find(row=>row.id==='valid-clone-retarget')),before=projectOriginalLinkCompleteTables(baseline.network,baseline.root),value=createPortableRustTextCertificate(fixtures.cases.find(row=>row.id==='valid-clone-retarget').source),entry=value.program.declarations.get('choose');entry.name='choose_changed';entry.fullName='choose_changed';entry.params[0].name='changed_input';entry.body.name='changed_input';value.program.declarations=new Map([['choose_changed',entry]]);
 for(const load of loads){const packet=load(attach(encodeRustTextValueProjection(value.program,value.proof))),result=projectOriginalLinkCompleteTables(packet.network,packet.root);compare(packet,result);assert.notDeepEqual(result.tables.names,before.tables.names);assert.notDeepEqual(result.tables.units,before.tables.units);execute(result,'choose_changed',['renamed Ω😀','other'],'renamed Ω😀');}
 const literalValue=Array.from({length:32},(_,index)=>String.fromCharCode(index)).join('')+'\u007f"\\Ω😀\u{10ffff}',literal=createPortableRustTextCertificate(fixtures.cases.find(row=>row.id==='raw-literal-construction').source);literal.proof.functions[0].literal.value=literalValue;
 for(const load of loads){const packet=load(attach(encodeRustTextValueProjection(literal.program,literal.proof))),result=projectOriginalLinkCompleteTables(packet.network,packet.root);compare(packet,result);execute(result,'choose',[],literalValue);assert.ok(result.tables.units.includes(0));assert.ok(result.tables.units.includes(0xdbff));assert.ok(result.tables.units.includes(0xdfff));}
});

test('all seven cross-owner and equal-looking cloned-owner forgeries still fail before projection',()=>{
 const fixture=fixtures.cases.find(row=>row.id==='equal-valued-distinct-owners');
 for(const role of ['declaration','signature','resultPosition','operationNode','coreValue','input','position'])for(const load of loads){const packet=create(fixture),facts=rows(packet,'functions');replace(field(packet.network,facts[0],role),field(packet.network,facts[1],role).child);const saved=load(packet);assert.throws(()=>projectOriginalLinkCompleteTables(saved.network,saved.root));
 const cloned=create(fixtures.cases.find(row=>row.id==='owned-clone')),target=field(cloned.network,rows(cloned,'functions')[0],role),source=cloned.network.link(target.child),duplicate=cloned.network.insertLink(source.references(),source.metadata().clone());for(const row of cloned.network.fieldRelations().filter(row=>row.parent.equals(target.child)))cloned.network.insertFieldRelation(duplicate,row.label,row.child);replace(target,duplicate);const savedClone=load(cloned);assert.throws(()=>projectOriginalLinkCompleteTables(savedClone.network,savedClone.root));}
});

test('direct primitive foreign-owner references change tables and the unchanged complete certificate refuses',()=>{
 const packet=create(fixtures.cases.find(row=>row.id==='equal-valued-distinct-owners')),result=projectOriginalLinkCompleteTables(packet.network,packet.root),facts=rows(packet,'functions');
 for(const name of ['declaration','signature','resultPosition','operationNode','coreValue','input','position']){const args=copy(result.transport.arguments),relation=field(packet.network,facts[0],name),index=result.transport.fixed.role.transport.relations.findIndex(row=>row.relation.equals(relation.link.id()));assert.ok(index>=0);args[4][index]=ordinal(result,field(packet.network,facts[1],name).child);const changed=tableRows(args);assert.notDeepEqual(changed.functions,result.tables.functions,name);assert.notEqual(evaluateRustTextCertificate(changed)[0],1,name);}
});

test('signed zero and nonfinite primitive payloads are copied exactly before the certificate refuses',()=>{
 const packet=create(fixtures.cases[0]),result=projectOriginalLinkCompleteTables(packet.network,packet.root),slot=scalar(result,packet,proof(packet),['span','start']);
 for(const value of [-0,NaN,Infinity,-Infinity]){const args=copy(result.transport.arguments);args[8][slot]=value;const tables=tableRows(args);assert.ok(Object.is(tables.context[11],value));assert.notEqual(checkRustTextCertificate(...policy.output.map(name=>tables[name]))[0],1);if(Number.isFinite(value))assert.notEqual(evaluateRustTextCertificate(tables)[0],1);else assert.throws(()=>evaluateRustTextCertificate(tables),/non-data scalar/u);}
});

test('raw String absence, empty values, code-unit boundaries and malformed transports stay distinct',()=>{
 const packet=create(fixtures.cases[0]),result=projectOriginalLinkCompleteTables(packet.network,packet.root),base=result.transport.arguments;
 for(const value of [-1,0.5,NaN,Infinity,'1',null,undefined]){const args=copy(base);args[11][0]=value;assert.deepEqual(project(...args),[[-30]]);}
 for(const value of [-2,0.5,NaN,Infinity,'1',null,undefined]){const args=copy(base);args[12][0]=value;assert.deepEqual(project(...args),[[-30]]);}
 for(const value of [-1,65536,0.5,NaN,Infinity,'1',null,undefined]){const args=copy(base);args[13][0]=value;assert.deepEqual(project(...args),[[-30]]);}
 for(const value of [-2,2,0.5,NaN,Infinity,'1',null,undefined]){const args=copy(base);args[10][0]=value;assert.deepEqual(project(...args),[[-30]]);}
 for(const vector of[10,11,12,13]){const args=copy(base);delete args[vector][0];assert.deepEqual(project(...args),[[-30]]);}
 const tag=copy(base),stringIndex=tag[7].findIndex(value=>value===2);tag[7][stringIndex]=3;assert.deepEqual(project(...tag),[[-30]]);
 const nullIndex=base[7].findIndex(value=>value===1);assert.ok(nullIndex>=0);const missing=copy(base);missing[7][nullIndex]=0;const tables=tableRows(missing);assert.notDeepEqual(tables,result.tables);assert.notEqual(evaluateRustTextCertificate(tables)[0],1);
 const empty=createPortableRustTextCertificate(fixtures.cases.find(row=>row.id==='raw-literal-construction').source);empty.proof.functions[0].literal.value='';const saved=attach(encodeRustTextValueProjection(empty.program,empty.proof)),projected=projectOriginalLinkCompleteTables(saved.network,saved.root);compare(saved,projected);execute(projected,'choose',[],'');assert.equal(projected.tables.cores[4]>0,true);
 const prefix=copy(base);prefix[16][0]=prefix[16][0]+1;assert.deepEqual(project(...prefix),[[-32]]);
});

test('contradictory primitive String payloads and fixed-symbol codes refuse before interpretation',()=>{
 const packet=create(fixtures.cases[0]),result=projectOriginalLinkCompleteTables(packet.network,packet.root),base=result.transport.arguments,operation=field(packet.network,proof(packet),'operations').child,marker=scalar(result,packet,operation,['kind']);
 const root=copy(base);assert.equal(root[7][marker],2);assert.equal(root[9][marker],108);root[13][root[11][marker]]=120;assert.deepEqual(project(...root),[[-32]]);
 const child=refs(packet.network,field(packet.network,operation,'parts').child).find(id=>packet.network.fieldRelations().some(row=>row.parent.equals(id)&&row.label===label('kind')&&packet.network.link(row.child).metadata().definition==='source operation'));assert.ok(child);const childSlot=scalar(result,packet,child,['kind']);const nested=copy(base);nested[13][nested[11][childSlot]]=120;assert.deepEqual(project(...nested),[[-32]]);
 const dynamic=base[7].findIndex((kind,index)=>kind===2&&base[9][index]===0);assert.ok(dynamic>=0);const forged=copy(base);forged[9][dynamic]=1;assert.deepEqual(project(...forged),[[-32]]);
 for(const vector of[14,15,16]){const prefix=copy(base);prefix[vector][0]=prefix[vector][0]+1;assert.deepEqual(project(...prefix),[[-32]]);}
});

test('early bounds and original typed markers retain their prior failure precedence',()=>{
 const packet=create(fixtures.cases[0]),result=projectOriginalLinkCompleteTables(packet.network,packet.root),base=result.transport.arguments;
 const wrong=copy(base),old=wrong[1];wrong[1]=wrong[0]+1;wrong[5]=wrong[5].map(value=>value===old?wrong[1]:value);assert.deepEqual(project(...wrong),[[-10]]);
 for(const vector of[2,4,5,6])for(const value of[0,-1,base[0]+1,0.5,NaN,Infinity]){const args=copy(base);args[vector][0]=value;assert.deepEqual(project(...args),[[-21]]);}
 for(const value of[-1,policy.fields.length+1,0.5,NaN,Infinity]){const args=copy(base);args[3][0]=value;assert.deepEqual(project(...args),[[-30]]);}
 const operation=field(packet.network,proof(packet),'operations').child,marker=scalar(result,packet,operation,['kind']);for(const kind of[0,1,3,4]){const args=copy(base);args[7][marker]=kind;assert.deepEqual(project(...args),[[-22]]);}
 const shape=copy(base);shape[1]=base[0]+1;shape[8].pop();assert.deepEqual(project(...shape),[[-30]]);
});

test('all sixty-six definitions survive source-free checked text/binary reload and execute every new table',()=>{
 const checked=checkProgram(parseJavaScript(originalLinkCompleteTableRulesSource(),{referenceSemantics:false})),packet=encodeTypedProgramLinks(checked),fixture=create(fixtures.cases.find(row=>row.id==='equal-valued-distinct-owners')),result=projectOriginalLinkCompleteTables(fixture.network,fixture.root),base=result.transport.arguments;
 for(const load of loads){const saved=load(packet),context={};vm.runInNewContext(emitTypedProgramLinks(saved.network,saved.root,'JavaScript').code,context,{timeout:5000});assert.deepEqual(Array.from(context.projectOriginalLinkCompleteCertificateTables(...base),row=>Array.from(row)),project(...base));const wrong=copy(base);wrong[1]=base[0]+1;assert.deepEqual(Array.from(context.projectOriginalLinkCompleteCertificateTables(...wrong),row=>Array.from(row)),[[-10]]);}
});

test('all complete generated Rust definitions and both unchanged source kernels remain fresh',()=>{
 const artifact=generateOriginalLinkCompleteTableProjection(true);assert.equal(artifact.declarations.length,66);assert.equal(artifact.declarationNames.length,66);assert.ok(artifact.completeRust.includes('pub fn project_original_link_complete_certificate_tables('));assert.ok(artifact.completeRust.includes('fn main('));assert.ok(artifact.declarationNames.includes('projectOriginalLinkCompleteCertificateTables'));assert.ok(artifact.declarationNames.includes('readOriginalLinkCompleteStringQueries'));const source=originalLinkCompleteTableRulesSource();for(const path of['../original-link-table-projection/rules.mjs','../original-link-symbol-projection/rules.mjs'])assert.ok(source.includes(readFileSync(new URL(path,import.meta.url),'utf8')));assert.equal(policy.output.length,17);
});
