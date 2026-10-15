import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary,encodeTypedProgramLinks,emitTypedProgramLinks} from '../../js/src/index.js';
import {TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {createPortableRustTextCertificate,encodePortableRustTextCertificate,evaluateRustTextCertificate} from '../rust-text-certificate/bridge.mjs';
import {checkRustTextCertificate} from '../rust-text-certificate/rules.mjs';
import {emitRustTextValueProjection} from '../rust-text-value-projection/model.mjs';
import {projectOriginalLinkCertificateTables as project} from './rules.mjs';
import {projectOriginalLinkFixedSymbolTables,ORIGINAL_LINK_TABLE_POLICY as policy} from './transport.mjs';
import {originalLinkTableRulesSource} from './policy.mjs';
import {generateOriginalLinkTableProjection} from './generate.mjs';

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
function compare(packet,result){const expected=expectedTables(packet,result.transport.role);for(const name of policy.output)assert.deepEqual(result.tables[name],expected[name],name);const merged={...expected,...result.tables};assert.deepEqual(evaluateRustTextCertificate(merged),result.transport.role.transport.checked.decision);return merged;}
function tableRows(result,args){const output=project(...args);assert.equal(output.length,policy.output.length);return Object.fromEntries(policy.output.map((name,index)=>[name,output[index]]));}
function ordinal(result,id){const value=result.transport.role.transport.anchors.findIndex(anchor=>anchor.equals(id))+1;assert.ok(value);return value;}
function scalar(result,packet,parent,path){let node=parent;for(const name of path)node=field(packet.network,node,name).child;return ordinal(result,node)-1;}

test('all thirteen actual text/binary archives match every whole projected table and complete kernel decision',()=>{
 assert.equal(fixtures.cases.length,13);assert.deepEqual(policy.remaining,['tokens','declarations','parameters','cores','names','units']);
 for(const fixture of fixtures.cases)for(const load of loads){const packet=load(create(fixture)),before=packet.network.toLino(),result=projectOriginalLinkFixedSymbolTables(packet.network,packet.root);compare(packet,result);assert.equal(packet.network.toLino(),before);assert.equal(result.tables.kinds.includes(9),false);}
});

test('the new kernel reads actual Number and symbol payloads rather than fixture observations',()=>{
 const packet=create(fixtures.cases.find(row=>row.id==='owned-clone')),result=projectOriginalLinkFixedSymbolTables(packet.network,packet.root),expected=compare(packet,result),base=result.transport.arguments;
 const operations=field(packet.network,proof(packet),'operations').child,start=scalar(result,packet,operations,['span','start']);
 const numbers=copy(base);numbers[8][start]=numbers[8][start]+1;const changed=tableRows(result,numbers);assert.notDeepEqual(changed.operations,result.tables.operations);assert.notEqual(evaluateRustTextCertificate({...expected,...changed})[0],1);
 const unknown=copy(base),meaning=scalar(result,packet,rows(packet,'valueAssumptions')[0],['meaning']);unknown[9][meaning]=0;const renamed=tableRows(result,unknown);assert.equal(renamed.allocations[1],-1);assert.notEqual(evaluateRustTextCertificate({...expected,...renamed})[0],1);
});

test('signed zero and nonfinite Number payloads are retained then refused by the complete certificate',()=>{
 const packet=create(fixtures.cases[0]),result=projectOriginalLinkFixedSymbolTables(packet.network,packet.root),expected=compare(packet,result),slot=scalar(result,packet,proof(packet),['span','start']);
 for(const value of [-0,NaN,Infinity,-Infinity]){const args=copy(result.transport.arguments);args[8][slot]=value;const tables=tableRows(result,args);assert.ok(Object.is(tables.context[11],value));const merged={...expected,...tables};assert.notEqual(checkRustTextCertificate(...['context','kinds','tokens','declarations','parameters','positions','paths','layers','cores','functions','operations','parts','inputs','allocations','names','units','items'].map(name=>merged[name]))[0],1);if(Number.isFinite(value))assert.notEqual(evaluateRustTextCertificate(merged)[0],1);else assert.throws(()=>evaluateRustTextCertificate(merged),/non-data scalar/u);}
});

test('actual null and unknown symbols remain distinct from absent and nonregistered owners',()=>{
 const packet=create(fixtures.cases.find(row=>row.id==='cooked-literal-construction')),result=projectOriginalLinkFixedSymbolTables(packet.network,packet.root);compare(packet,result);assert.equal(result.tables.functions[5],0);assert.equal(result.tables.functions[6],0);
 const args=copy(result.transport.arguments),nullSlot=scalar(result,packet,rows(packet,'functions')[0],['input']);args[7][nullSlot]=0;const tables=tableRows(result,args);assert.equal(tables.functions[5],-1);
 const unknown=copy(result.transport.arguments),owner=field(packet.network,rows(packet,'functions')[0],'signature').child,ret=field(packet.network,owner,'result').child;unknown[9][scalar(result,packet,ret,['type','kind'])]=0;assert.equal(tableRows(result,unknown).positions.at(-8),-1);
});

test('operation markers require their actual String scalar tag as well as the marker code',()=>{
 const packet=create(fixtures.cases[0]),result=projectOriginalLinkFixedSymbolTables(packet.network,packet.root),expected=compare(packet,result),operation=field(packet.network,proof(packet),'operations').child,marker=field(packet.network,operation,'kind').child,slot=ordinal(result,marker)-1;
 for(const kind of [0,1,3,4]){const args=copy(result.transport.arguments);assert.equal(args[7][slot],2);args[7][slot]=kind;assert.deepEqual(project(...args),[[-22]]);}
 const parts=refs(packet.network,field(packet.network,operation,'parts').child),child=parts.find(id=>packet.network.fieldRelations().some(row=>row.parent.equals(id)&&row.label===label('kind')&&packet.network.link(row.child).metadata().definition==='source operation'));assert.ok(child);const childSlot=ordinal(result,field(packet.network,child,'kind').child)-1;assert.notEqual(childSlot,slot);
 for(const kind of [0,1,3,4]){const args=copy(result.transport.arguments);args[7][childSlot]=kind;const changed=tableRows(result,args);assert.notDeepEqual(changed.operations,result.tables.operations);assert.notEqual(evaluateRustTextCertificate({...expected,...changed})[0],1);}
 const link=packet.network.link(marker);link.setMetadata(link.metadata().clone().withTerm(schema.scalars.number).withDefinition('0'));
 for(const load of loads){const saved=load(packet);assert.throws(()=>projectOriginalLinkFixedSymbolTables(saved.network,saved.root));}
});

test('all seven raw cross-owner and cloned-owner forgeries still fail before host projection',()=>{
 const fixture=fixtures.cases.find(row=>row.id==='equal-valued-distinct-owners');
 for(const role of ['declaration','signature','resultPosition','operationNode','coreValue','input','position'])for(const load of loads){
  const packet=create(fixture),facts=rows(packet,'functions');replace(field(packet.network,facts[0],role),field(packet.network,facts[1],role).child);const saved=load(packet);assert.throws(()=>projectOriginalLinkFixedSymbolTables(saved.network,saved.root));
  const cloned=create(fixtures.cases.find(row=>row.id==='owned-clone')),target=field(cloned.network,rows(cloned,'functions')[0],role),source=cloned.network.link(target.child),copy=cloned.network.insertLink(source.references(),source.metadata().clone());for(const row of cloned.network.fieldRelations().filter(row=>row.parent.equals(target.child)))cloned.network.insertFieldRelation(copy,row.label,row.child);replace(target,copy);const savedClone=load(cloned);assert.throws(()=>projectOriginalLinkFixedSymbolTables(savedClone.network,savedClone.root));
 }
});

test('direct primitive cross-owner references change projected facts and the unchanged certificate refuses',()=>{
 const packet=create(fixtures.cases.find(row=>row.id==='equal-valued-distinct-owners')),result=projectOriginalLinkFixedSymbolTables(packet.network,packet.root),expected=compare(packet,result),facts=rows(packet,'functions');
 for(const name of ['declaration','signature','resultPosition','operationNode','coreValue','input','position']){
  const args=copy(result.transport.arguments),relation=field(packet.network,facts[0],name),index=result.transport.role.transport.relations.findIndex(row=>row.relation.equals(relation.link.id()));assert.ok(index>=0);args[4][index]=ordinal(result,field(packet.network,facts[1],name).child);
  const changed=tableRows(result,args);assert.notDeepEqual(changed.functions,result.tables.functions,name);assert.notEqual(evaluateRustTextCertificate({...expected,...changed})[0],1,name);
 }
});

test('coherent saved parameter and source interval edits affect real tables and executable replay',()=>{
 const value=createPortableRustTextCertificate(fixtures.cases.find(row=>row.id==='valid-clone-retarget').source),entry=value.program.declarations.get('choose'),fact=value.proof.functions[0],old=fact.input,target=entry.params[1],token=value.proof.tokens.find(token=>token.binding===old&&token.span.start===fact.coreValue.span.start);assert.ok(token);token.binding=target;entry.body.name=target.name;fact.input=target;fact.position=entry.sourceSignature.parameters[1];
 for(const condition of [...value.proof.inputAssumptions,...value.proof.valueAssumptions])if(condition.declaration===entry){condition.input=target;condition.position=fact.position;}
 for(const load of loads){const packet=load(attach(encodePortableRustTextCertificate(value.program,value.proof))),result=projectOriginalLinkFixedSymbolTables(packet.network,packet.root);compare(packet,result);const checked=result.transport.role.transport.checked,context={};vm.runInNewContext(emitRustTextValueProjection(checked.program,checked.proof,'JavaScript').text,context,{timeout:1000});assert.equal(context.choose('first','second'),'second');const handle=result.transport.role.anchors.findIndex(id=>id.equals(field(packet.network,rows(packet,'functions')[0],'input').child))+1;assert.equal(result.tables.functions[5],handle);assert.equal(result.tables.inputs[5],handle);assert.equal(result.tables.allocations[5],handle);}
 const original=fixtures.cases[0],shifted={...original,source:'\n'+original.source},first=create(original),second=create(shifted),a=projectOriginalLinkFixedSymbolTables(first.network,first.root),b=projectOriginalLinkFixedSymbolTables(second.network,second.root);compare(second,b);assert.equal(b.tables.context[12],a.tables.context[12]+1);assert.notDeepEqual(b.tables.operations,a.tables.operations);
});

test('early primitive shape, root, ordinal, scalar and dictionary bounds refuse malformed or holey values',()=>{
 const packet=create(fixtures.cases[0]),result=projectOriginalLinkFixedSymbolTables(packet.network,packet.root),base=result.transport.arguments;
 const wrong=copy(base),old=wrong[1];wrong[1]=wrong[0]+1;wrong[5]=wrong[5].map(value=>value===old?wrong[1]:value);assert.deepEqual(project(...wrong),[[-10]]);
 for(const vector of [2,4,5,6])for(const value of [0,-1,base[0]+1,0.5,NaN,Infinity,'1',null,undefined]){const args=copy(base);args[vector][0]=value;assert.deepEqual(project(...args),[[-21]]);}
 for(const[vector,maximum]of[[3,policy.fields.length],[7,4],[9,policy.symbols.length]])for(const value of [-1,maximum+1,0.5,NaN,Infinity,'0',null,undefined]){const args=copy(base);args[vector][0]=value;assert.deepEqual(project(...args),[[-21]]);}
 for(const vector of [2,3,4,5,6,7,8,9]){const args=copy(base);delete args[vector][0];assert.deepEqual(project(...args),[[-21]]);}
 for(const value of [true,false,'1',null,undefined,{},0n]){const args=copy(base);args[8][0]=value;assert.deepEqual(project(...args),[[-21]]);}
 const shape=copy(base);shape[1]=base[0]+1;shape[8].pop();assert.deepEqual(project(...shape),[[-21]]);
});

test('all eleven whole tables execute after checked text/binary reload without the original rules source',()=>{
 const checked=checkProgram(parseJavaScript(originalLinkTableRulesSource(),{referenceSemantics:false})),packet=encodeTypedProgramLinks(checked),fixture=create(fixtures.cases[0]),result=projectOriginalLinkFixedSymbolTables(fixture.network,fixture.root),base=result.transport.arguments;
 for(const load of loads){const saved=load(packet),context={};vm.runInNewContext(emitTypedProgramLinks(saved.network,saved.root,'JavaScript').code,context,{timeout:2000});assert.deepEqual(Array.from(context.projectOriginalLinkCertificateTables(...base),row=>Array.from(row)),project(...base));const wrong=copy(base);wrong[1]=base[0]+1;assert.deepEqual(Array.from(context.projectOriginalLinkCertificateTables(...wrong),row=>Array.from(row)),[[-10]]);const retag=copy(base),operation=field(fixture.network,proof(fixture),'operations').child,marker=scalar(result,fixture,operation,['kind']);retag[7][marker]=3;assert.deepEqual(Array.from(context.projectOriginalLinkCertificateTables(...retag),row=>Array.from(row)),[[-22]]);}
});

test('the shared column policy and every complete Rust definition regenerate automatically',()=>{
 const artifact=generateOriginalLinkTableProjection(true);assert.equal(artifact.declarations.length,29);assert.ok(artifact.completeRust.includes('pub fn project_original_link_certificate_tables('));assert.ok(artifact.completeRust.includes('fn main('));assert.ok(artifact.declarationNames.includes('projectOriginalLinkCertificateTables'));assert.equal(policy.output.length,11);
});
