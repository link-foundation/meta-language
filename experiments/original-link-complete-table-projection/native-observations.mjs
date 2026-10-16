// CI-only observation planning; the complete generated artifact is never trimmed.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import vm from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary,readStructuredDataLino} from '../../js/src/index.js';
import {TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {tokenize} from '../../js/src/translation/lexer.js';
import {createSourceNativeObservationRunner} from '../source-native-observations.mjs';
import {SOURCE_NATIVE_RUNNER_ADDITIONS,SOURCE_NATIVE_RUNNER_VERIFIER_FILES,verifySourceNativeRunnerSourceClosure} from '../source-native-runner-source-closure.mjs';
import {createPortableRustTextCertificate,encodePortableRustTextCertificate,evaluateRustTextCertificate} from '../rust-text-certificate/bridge.mjs';
import {emitRustTextValueProjection,encodeRustTextValueProjection} from '../rust-text-value-projection/model.mjs';
import {projectOriginalLinkCompleteCertificateTables} from './rules.mjs';
import {projectOriginalLinkCompleteTables,ORIGINAL_LINK_COMPLETE_TABLE_POLICY as policy} from './transport.mjs';
import {originalLinkCompleteTablePolicy} from './policy.mjs';
import {createCompleteRustSourceArchives,replayCompleteRustSourceArchives,readCanonicalArchiveTransportSource,requireSuccessfulSourceArchive} from './archive-transport.mjs';

export const ORIGINAL_LINK_COMPLETE_TABLE_MODEL_COMMIT='9a835e469572341e6a8e5c6e9fe6f40b8655dfbe';
const digests={
 'policy.mjs':'e869b0164d92e28c4e34277ec0ac9a24e0476ee5e104042748f2f35a3e959970',
 'rules.mjs':'da3815634f0ec09afdefd432c817e0368c73749fd6ca88fbc86db82f8beb5952',
 'generate.mjs':'92afd9fb275c3e3c04a84728f99a224d0ea2226d658a1484efac0a9112856d4f',
 'transport.mjs':'6975fd4431fa0631197c9af72a5be1abef92a62c250e42d5dc20ea3d89f5d7b2',
 'data/policy.lino':'d5414508fe555a1f87a732fd2fd3b3e32b6fed5134ed30268b7e8c10df5ff212',
 'data/native-definitions.lino':'8a844644c5e0e1f4211a270e23245ffa0b5d36344cea4d26d29460fe50bde224',
 '../original-link-table-projection/policy.mjs':'cb0e58a2f878f753f36e7b9ac0755277c9634cb90aeb9d67269342c471e5ea20',
 '../original-link-table-projection/rules.mjs':'7cb4eef983d851a4fa9da784c931cb826c33aad992eb166462a8679b267802f7',
 '../original-link-table-projection/transport.mjs':'fbe742bd90f9e3a3ed0d550060b2bf156a978c632f6f3fe5f652623c6c8d6212',
 '../original-link-table-projection/data/policy.lino':'1babbc5bb563be80887013cd531e3d56d60daea5a53d307445edd2052033f508',
 '../original-link-role-projection/rules.mjs':'74db55cf5fff7d9268c16b6dbfe93504081a614f5308a2ad731619765ce74944',
 '../original-link-role-projection/transport.mjs':'c265f3d6c732369eb00632d95e5dafb1b02b484e7e68e87cfce81d02f20df22b',
 '../original-link-role-projection/data/policy.lino':'be7e8a265748909af5bd92f6272510ab81a8947df7e0d21c3217233fd6f39b01',
 '../original-link-identity/policy.mjs':'080d379c8c97696fb611f841b1d7dba220864e4cd82984febb5c550e7bdd46e5',
 '../original-link-identity/rules.mjs':'ac1d9dae9835c1fe5b089d96a163ad3aaf0ac89ef6f29faccf0fc19c700727ba',
 '../original-link-symbol-projection/policy.mjs':'b88a1c3b2f51d2f28411ab43559b22cae585267192d70790a54457bbc21c4428',
 '../original-link-symbol-projection/rules.mjs':'dea761345d761f6d58ecb564893134ccdc149b19eaf89d51ef7dcbe78e4a9448',
 '../original-link-symbol-projection/data/policy.lino':'ece8ece9be52b5917944ebd53d604d956ad4e973880ee7f719e8b5c620db152f',
 '../rust-text-certificate/bridge.mjs':'c3121d3206ec0c6fafa1ac4506acc73012e245509b76c14bb77d696d11808e4d',
 '../rust-text-certificate/rules.mjs':'30f2e26a2f2b8d9b30de14a9ffed93a72c2b1e1edd6a95e717dafa36226e1f19',
 '../rust-text-certificate/data/policy.lino':'ee66558495d3eea1c022b1ca68f4e0a3ebba9e7ebb501001f8dff5a432ce8e14',
 '../rust-text-certificate-adapter/raw-identity.mjs':'09da17255d14999d85eaae3abae4c9789a082d636a3d30d9493a9704557eed94',
 '../rust-text-value-projection/model.mjs':'d2be6a924798984c3393adf1050ebbb74b8159be79e8392e38e84b5bbcf7db56',
 '../rust-text-value-projection/source.mjs':'6d64305d0a5b65de68e36d6e62db25b5f6bf6822b7ed3d0ca47ba7790a6565f5',
 '../rust-text-value-projection/rules.mjs':'890472bec11e6095ffcca50fff363ae0d5538bf1cd639f61f645051d970f952b',
 '../rust-text-value-projection/data/policy.lino':'0287e8b72b3b73fd82c5deac38f78a191e9426b3f5f7ed47c043f15c68da7272',
 '../rust-text-value-projection/data/fixtures.lino':'6fd937e41d61f0fcd2bae6fb9b2fd9d0d358e775234408cf434843e853f8bce4',
 '../generated-owned-rust/model.mjs':'482a8ffecc9e17c3bf3ae0eb4ef883453e61391204d9e5310741e9a3c9389524',
 '../generated-owned-rust/codec.mjs':'b9364434b7d521f3bd2457d7e2313b63a94ef303c03fc88f1f3a2d9a02904413',
 '../generated-owned-rust/parser.mjs':'0b2c4d49209fb41e54c43ba4389c76d3aba14307bf56684c4f93e84f39ea9d3f',
 '../generated-owned-rust/rules.mjs':'422d5af1ba85942184297516a49612c015c7e78379a28f7a206ca4e875264ee2',
 '../generated-owned-rust/value-edit-rules.mjs':'863031615810d43f5a4ca91c434f65dbc8f8b739f19b4347948d9e1b7abf558f',
 '../generated-owned-rust/trivia.mjs':'4674afab71a49a5016415a0e9875e7bdd89c48e29f7b6f446d7ea2e32d745d09',
 '../generated-owned-rust/data/policy.lino':'7ba85399443765c98641ef9b570a91a42822622a8b038891dc89bc471e42c13c',
 '../source-native-observations.mjs':'7375572e830c9d7a686efe84fb57111cec7861a334851ecad8b5604cf86440a8',
};
const checkedPlans=new WeakSet(),sha256=value=>createHash('sha256').update(value).digest('hex');
const runnerClosureReceiptSha256='fd8a20c45dc95698beb3e5b0263c058f610cc6e5b7ece01d8a50e1d646a656b1';
const nativeReceipt={sourceBytes:193995,sourceSha256:'761b3611c682e4bbcc85c29a1b91ed58a565d89e47a295703488c69a7b64fe6b'};
const parameterOrder=['membershipCount','root','fieldParents','fieldLabels','fieldChildren','referenceParents','referenceChildren','scalarKinds','scalarNumbers','scalarSymbols','scalarBooleans','rawStarts','rawLengths','rawUnits','fixedStarts','fixedLengths','fixedUnits'];
const maximumProtocolBytes=1024*1024,maximumConsumerBytes=4*1024*1024;
function textBytes(value){if(typeof value!=='string'||!value.isWellFormed())throw new TypeError('the oracle source must be primitive Unicode scalar text');return Buffer.from(value,'utf8');}
function freeze(value){if(value&&typeof value==='object'){Object.values(value).forEach(freeze);Object.freeze(value);}return value;}
function numberBits(value){assert.equal(typeof value,'number');const bytes=Buffer.alloc(8);bytes.writeDoubleBE(value);return bytes.toString('hex');}
function bitsNumber(value){assert.match(value,/^[0-9a-f]{16}$/u);return Buffer.from(value,'hex').readDoubleBE();}
function argumentBits(args){return args.map(value=>Array.isArray(value)?{kind:'vector',bits:value.map(numberBits)}:{kind:'scalar',bits:numberBits(value)});}
const copyArguments=args=>args.map(value=>Array.isArray(value)?[...value]:value);
const key=id=>id.asU64().toString();
const fieldLabel=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
function fieldRelation(network,parent,name){const rows=network.fieldRelations().filter(row=>row.parent.equals(parent)&&row.label===fieldLabel(name));assert.equal(rows.length,1,name);return rows[0];}
const references=(packet,id)=>packet.network.link(id).references();
const selectedBody=packet=>references(packet,packet.root)[0];
const selectedProof=packet=>fieldRelation(packet.network,selectedBody(packet),'sourceTextValueProjection').child;
const proofRows=(packet,name)=>references(packet,fieldRelation(packet.network,selectedProof(packet),name).child);
function attachFixture(packet){const provenance=fieldRelation(packet.network,packet.root,'provenance').child,proof=fieldRelation(packet.network,provenance,'sourceInterpretation').child;packet.network.insertFieldRelation(selectedBody(packet),fieldLabel('sourceTextValueProjection'),proof);return packet;}
function fixturePacket(fixture){const value=createPortableRustTextCertificate(fixture.source);return attachFixture(encodePortableRustTextCertificate(value.program,value.proof));}
function archivePacket(packet,format){return{...packet,network:format==='text'?LinkNetwork.fromLino(packet.network.toLino()):decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))};}
function replaceField(row,child){row.link.setReferences([row.parent,row.link.references()[1],child]);}
function actualOrdinal(result,id){const value=result.transport.fixed.role.transport.anchors.findIndex(anchor=>anchor.equals(id))+1;assert.ok(value);return value;}
function scalarOrdinal(result,packet,parent,path){let node=parent;for(const name of path)node=fieldRelation(packet.network,node,name).child;return actualOrdinal(result,node)-1;}

export function verifyOriginalLinkCompleteTableNativeModel(read=readFileSync){return Object.entries(digests).map(([file,expected])=>{
 const bytes=read(new URL(file,import.meta.url)),actual=sha256(bytes);
 if(file==='../source-native-observations.mjs'){
  const receiptBytes=read(new URL('./data/runner-source-closure.lino',import.meta.url));
  assert.equal(sha256(receiptBytes),runnerClosureReceiptSha256,'the frozen complete-table runner source-closure receipt changed');
  const additionalSources=Object.fromEntries(SOURCE_NATIVE_RUNNER_ADDITIONS.map(row=>[row.file,read(new URL('../../'+row.file,import.meta.url))]));
  const verifierSources=Object.fromEntries(SOURCE_NATIVE_RUNNER_VERIFIER_FILES.map(file=>[file,read(new URL('../../'+file,import.meta.url))]));
  const closure=verifySourceNativeRunnerSourceClosure(readStructuredDataLino(receiptBytes.toString('utf8')),bytes,digests,{producerCommit:ORIGINAL_LINK_COMPLETE_TABLE_MODEL_COMMIT,additionalSources,verifierSources});
  assert.equal(closure.baselineSha256,expected);
  return{file,sha256:actual,producerSha256:expected,runnerSourceClosure:closure,sourceClosureReceiptSha256:runnerClosureReceiptSha256};
 }
 assert.equal(actual,expected,'the frozen complete-table source changed: '+file);return{file,sha256:actual};
});}
export function originalLinkCompleteTableNativeMode(environment=process.env){if(environment.ORIGINAL_LINK_COMPLETE_TABLE_NATIVE!=='1')return false;if(environment.CI!=='true')throw new TypeError('ORIGINAL_LINK_COMPLETE_TABLE_NATIVE=1 requires CI=true; native compilation is forbidden locally');return true;}
function nativeInterface(source){
 const tokens=tokenize(source,'Rust').tokens,found=[];let depth=0;
 for(let index=0;index<tokens.length;index++){if(depth===0&&tokens[index].value==='pub'&&tokens[index+1]?.value==='fn'&&tokens[index+2]?.value==='project_original_link_complete_certificate_tables')found.push(index);if(tokens[index].kind==='punct'&&tokens[index].value==='{')depth++;if(tokens[index].kind==='punct'&&tokens[index].value==='}')depth--;}
 assert.equal(depth,0);assert.equal(found.length,1,'the complete artifact exports exactly one table projector');
 const names=parameterOrder.map(name=>name.replace(/[A-Z]/gu,letter=>'_'+letter.toLowerCase())),vector=['Vec','<','f64','>'],expected=['pub','fn','project_original_link_complete_certificate_tables','('];
 for(const[index,name]of names.entries()){if(index)expected.push(',');expected.push(name,':',...(index<2?['f64']:vector));}expected.push(')','->','Vec','<',...vector,'>','{');
 assert.deepEqual(tokens.slice(found[0],found[0]+expected.length).map(token=>token.value),expected,'the actual mixed scalar/nested-vector ABI is exact');
 return{functionName:'project_original_link_complete_certificate_tables',parameterOrder:names,parameterModes:['f64','f64',...Array(15).fill('Vec<f64>')],resultMode:'Vec<Vec<f64>>',referenceSemantics:false,interfaceSha256:sha256(expected.join('\0')),tokenCount:tokens.length};
}
// Independent expected-table oracle: the old object projector supplies values;
// an explicit original-LinkId bijection remaps only its identity columns.
function expectedTables(packet,registration){
 const{network}=packet,table=registration.transport.checked.table,map=new Map(),reverse=new Map(),handle=new Map(registration.anchors.map((id,index)=>[key(id),index+1]));
 function bind(id,old){const actual=handle.get(key(id));assert.ok(actual);if(map.has(old))assert.equal(map.get(old),actual);if(reverse.has(actual))assert.equal(reverse.get(actual),old);map.set(old,actual);reverse.set(actual,old);assert.equal(registration.kinds[actual],table.kinds[old]);}
 const fld=(id,name)=>fieldRelation(network,id,name).child,children=id=>references(packet,id);
 children(fld(selectedProof(packet),'tokens')).forEach((id,index)=>bind(id,table.tokens[index*9]));let parameter=0,position=0;
 const declarations=children(fld(selectedBody(packet),'declarations')).map(pair=>children(pair)[1]);
 for(const[index,id]of declarations.entries()){const offset=index*20,signature=fld(id,'sourceSignature');bind(id,table.declarations[offset]);bind(signature,table.declarations[offset+4]);bind(fld(id,'body'),table.declarations[offset+5]);for(const input of children(fld(id,'params')))bind(input,table.parameters[parameter++*8]);for(const item of [...children(fld(signature,'parameters')),fld(signature,'result')])bind(item,table.positions[position++*11]);}
 let operation=0;const seen=new Set();function walk(id){if(seen.has(key(id)))return;seen.add(key(id));bind(id,table.operations[operation++*6]);for(const child of children(fld(id,'parts'))){const kind=network.fieldRelations().find(row=>row.parent.equals(child)&&row.label===fieldLabel('kind'));if(kind&&network.link(kind.child).metadata().definition==='source operation')walk(child);}}
 walk(fld(selectedProof(packet),'operations'));bind(fld(selectedBody(packet),'main'),table.context[2]);assert.equal(map.size,table.kinds.length-1);assert.equal(map.size,registration.kinds.length-1);
 const identity=value=>value<=0?value:(assert.ok(map.has(value)),map.get(value)),out=Object.fromEntries(Object.entries(table).map(([name,values])=>[name,[...values]]));out.kinds=[...registration.kinds];
 const rewrite=(name,width,columns)=>{for(let row=0;row<out[name].length;row+=width)for(const column of columns)out[name][row+column]=identity(out[name][row+column]);};
 rewrite('context',15,[2,6,10]);rewrite('tokens',9,[0,4]);rewrite('declarations',20,[0,4,5,10,19]);rewrite('parameters',8,[0,1,5,6,7]);rewrite('positions',11,[0,1]);rewrite('cores',7,[0,1]);rewrite('functions',8,[0,1,2,3,4,5,6,7]);rewrite('operations',6,[0]);rewrite('parts',1,[0]);rewrite('inputs',7,[2,3,4,5,6]);rewrite('allocations',7,[2,3,4,5,6]);rewrite('items',1,[0]);return out;
}
function certificateObservation(rows){if(rows.length!==17)return{referenceRuntime:'JavaScript',kind:'exact projection refusal',rows:rows.map(row=>row.map(numberBits))};const tables=Object.fromEntries(policy.output.map((name,index)=>[name,rows[index]]));try{return{referenceRuntime:'JavaScript',kind:'complete certificate decision',bits:evaluateRustTextCertificate(tables).map(numberBits)};}catch(error){assert.ok(error instanceof TypeError);return{referenceRuntime:'JavaScript',kind:'complete certificate scalar refusal',reason:error.message};}}
function actualExecution(result,name,observations){const checked=result.transport.fixed.role.transport.checked,context={};vm.runInNewContext(emitRustTextValueProjection(checked.program,checked.proof,'JavaScript').text,context,{timeout:2000});for(const row of observations)assert.equal(context[name](...row.args),row.value);}
function cases(context,target,fixtures){
 const rows=[],rawRefusals=[];
 const add=(id,args,kind,provenance=[],expected=null)=>{const inputs=argumentBits(args),direct=projectOriginalLinkCompleteCertificateTables(...args),emitted=Array.from(context[target](...args),row=>Array.from(row));assert.deepEqual(emitted,direct,id);assert.deepEqual(argumentBits(args),inputs,'the projector preserves every original scalar/vector bit');if(expected)assert.deepEqual(direct,expected,id);const certificate=certificateObservation(direct);rows.push({id,kind,inputArguments:inputs,resultRows:direct.map(row=>({bits:row.map(numberBits)})),certificate,provenance});return direct;};
 const graphCase=(id,packet,sourceReceipt,execution)=>{
  let first=null;
  for(const format of['text','binary']){const saved=archivePacket(packet,format),before=saved.network.toLino(),result=projectOriginalLinkCompleteTables(saved.network,saved.root),expected=expectedTables(saved,result.transport.fixed.role);for(const name of policy.output)assert.deepEqual(result.tables[name],expected[name],id+' '+name);assert.deepEqual(result.decision,result.transport.fixed.role.transport.checked.decision);assert.equal(saved.network.toLino(),before);actualExecution(result,execution.name,execution.observations);
   const input=argumentBits(result.transport.arguments),receipt={format,root:key(saved.root),body:key(selectedBody(saved)),proof:key(selectedProof(saved)),networkSha256:sha256(before),inputSha256:sha256(JSON.stringify(input)),...sourceReceipt};
   if(!first)first={result,expected,input,provenance:[receipt]};else{assert.deepEqual(input,first.input,'both archive transports have exactly identical scalar/vector bits before deduplication');assert.deepEqual(result.tables,first.result.tables);first.provenance.push(receipt);}
  }
  add(id,first.result.transport.arguments,'actual checked original-ID source graph',first.provenance,policy.output.map(name=>first.expected[name]));return first;
 };
 assert.equal(fixtures.cases.length,13);
 for(const fixture of fixtures.cases){assert.equal(textBytes(fixture.source).length,fixture.sourceBytes);assert.equal(sha256(fixture.source),fixture.sourceSha256);graphCase('fixture '+fixture.id,fixturePacket(fixture),{fixtureId:fixture.id,sourceBytes:fixture.sourceBytes,sourceSha256:fixture.sourceSha256},{name:'choose',observations:fixture.observations});}
 const original=fixtures.cases.find(row=>row.id==='valid-clone-retarget'),retarget=createPortableRustTextCertificate(original.source),entry=retarget.program.declarations.get('choose'),fact=retarget.proof.functions[0],old=fact.input,replacement=entry.params[1],token=retarget.proof.tokens.find(row=>row.binding===old&&row.span.start===fact.coreValue.span.start);assert.ok(token);token.binding=replacement;entry.body.name=replacement.name;fact.input=replacement;fact.position=entry.sourceSignature.parameters[1];for(const condition of[...retarget.proof.inputAssumptions,...retarget.proof.valueAssumptions])if(condition.declaration===entry){condition.input=replacement;condition.position=fact.position;}
 graphCase('coherent clone receiver retarget',attachFixture(encodePortableRustTextCertificate(retarget.program,retarget.proof)),{parentFixtureId:original.id,parentSourceSha256:original.sourceSha256,edit:'actual receiver/input/position and conditions retargeted to the second existing parameter'},{name:'choose',observations:[{args:['first','second Ω😀'],value:'second Ω😀'}]});
 const renamed=createPortableRustTextCertificate(original.source),declaration=renamed.program.declarations.get('choose');declaration.name='choose_changed';declaration.fullName='choose_changed';declaration.params[0].name='changed_input';declaration.body.name='changed_input';renamed.program.declarations=new Map([['choose_changed',declaration]]);
 graphCase('coherent declaration and parameter rename',attachFixture(encodeRustTextValueProjection(renamed.program,renamed.proof)),{parentFixtureId:original.id,parentSourceSha256:original.sourceSha256,edit:'actual declaration inventory, owned parameter and linked reads renamed coherently'},{name:'choose_changed',observations:[{args:['renamed Ω😀','other'],value:'renamed Ω😀'}]});
 const literalFixture=fixtures.cases.find(row=>row.id==='raw-literal-construction');for(const[id,value]of[['control and Unicode literal',Array.from({length:32},(_,index)=>String.fromCharCode(index)).join('')+'\u007f"\\Ω😀\u{10ffff}'],['empty String literal','']]){const literal=createPortableRustTextCertificate(literalFixture.source);literal.proof.functions[0].literal.value=value;graphCase('coherent '+id,attachFixture(encodeRustTextValueProjection(literal.program,literal.proof)),{parentFixtureId:literalFixture.id,parentSourceSha256:literalFixture.sourceSha256,edit:id},{name:'choose',observations:[{args:[],value}]});}
 const ownerFixture=fixtures.cases.find(row=>row.id==='equal-valued-distinct-owners'),baseline=fixturePacket(ownerFixture),result=projectOriginalLinkCompleteTables(baseline.network,baseline.root),base=result.transport.arguments,facts=proofRows(baseline,'functions');
 for(const name of['declaration','signature','resultPosition','operationNode','coreValue','input','position']){const args=copyArguments(base),relation=fieldRelation(baseline.network,facts[0],name),index=result.transport.fixed.role.transport.relations.findIndex(row=>row.relation.equals(relation.link.id()));assert.ok(index>=0);args[4][index]=actualOrdinal(result,fieldRelation(baseline.network,facts[1],name).child);const output=add('primitive foreign owner '+name,args,'primitive cross-owner contradiction retained for the unchanged certificate');assert.equal(output.length,17);assert.notEqual(evaluateRustTextCertificate(Object.fromEntries(policy.output.map((table,index)=>[table,output[index]])))[0],1);}
 const packet=fixturePacket(fixtures.cases[0]),checked=projectOriginalLinkCompleteTables(packet.network,packet.root),args=checked.transport.arguments,spanSlot=scalarOrdinal(checked,packet,selectedProof(packet),['span','start']);
 for(const[id,value]of[['negative zero',-0],['chosen NaN',NaN],['positive infinity',Infinity],['negative infinity',-Infinity]]){const changed=copyArguments(args);changed[8][spanSlot]=value;const output=add('Number payload '+id,changed,'chosen primitive Number bits retained before complete-certificate refusal');assert.equal(numberBits(output[0][11]),numberBits(value));const observed=certificateObservation(output);assert.ok(observed.kind==='complete certificate scalar refusal'||bitsNumber(observed.bits[0])!==1);}
 const mutation=(id,mutate,expected)=>{const changed=copyArguments(args);mutate(changed);add(id,changed,'exact malformed primitive transport refusal',[],expected);};
 mutation('alien root after reference rebound',a=>{const old=a[1];a[1]=a[0]+1;a[5]=a[5].map(value=>value===old?a[1]:value);},[[-10]]);mutation('fractional root',a=>{a[1]=0.5;},[[-10]]);mutation('zero field parent',a=>{a[2][0]=0;},[[-21]]);mutation('zero field child',a=>{a[4][0]=0;},[[-21]]);mutation('unknown label above policy',a=>{a[3][0]=policy.fields.length+1;},[[-30]]);mutation('shape failure before alien root',a=>{a[1]=a[0]+1;a[8].pop();},[[-30]]);
 const operation=fieldRelation(packet.network,selectedProof(packet),'operations').child,marker=scalarOrdinal(checked,packet,operation,['kind']),child=references(packet,fieldRelation(packet.network,operation,'parts').child).find(id=>packet.network.fieldRelations().some(row=>row.parent.equals(id)&&row.label===fieldLabel('kind')&&packet.network.link(row.child).metadata().definition==='source operation'));assert.ok(child);const childMarker=scalarOrdinal(checked,packet,child,['kind']);
 mutation('root marker payload/code contradiction',a=>{a[13][a[11][marker]]=120;},[[-32]]);mutation('child marker payload/code contradiction',a=>{a[13][a[11][childMarker]]=120;},[[-32]]);const dynamic=args[7].findIndex((kind,index)=>kind===2&&args[9][index]===0);assert.ok(dynamic>=0);mutation('dynamic String forged fixed code',a=>{a[9][dynamic]=1;},[[-32]]);
 for(const slot of[14,15,16])mutation('fixed prefix contradiction '+slot,a=>{a[slot][0]++;},[[-32]]);mutation('root marker wrong scalar tag precedence',a=>{a[7][marker]=3;},[[-22]]);mutation('UTF16 code above uint16',a=>{a[13][0]=65536;},[[-30]]);mutation('UTF16 NaN code',a=>{a[13][0]=NaN;},[[-30]]);mutation('fractional raw String length',a=>{a[12][0]=0.5;},[[-30]]);mutation('unknown Boolean code',a=>{a[10][0]=2;},[[-30]]);
 for(const role of['declaration','signature','resultPosition','operationNode','coreValue','input','position'])for(const format of['text','binary'])for(const kind of['foreign owner','equal-looking cloned owner']){const forged=fixturePacket(kind==='foreign owner'?ownerFixture:fixtures.cases.find(row=>row.id==='owned-clone')),functions=proofRows(forged,'functions'),relation=fieldRelation(forged.network,functions[0],role),before=key(relation.child);let target;
  if(kind==='foreign owner')target=fieldRelation(forged.network,functions[1],role).child;else{const source=forged.network.link(relation.child);target=forged.network.insertLink(source.references(),source.metadata().clone());for(const row of forged.network.fieldRelations().filter(row=>row.parent.equals(relation.child)))forged.network.insertFieldRelation(target,row.label,row.child);}replaceField(relation,target);const saved=archivePacket(forged,format);let refusal;assert.throws(()=>projectOriginalLinkCompleteTables(saved.network,saved.root),error=>{refusal={name:error.name,message:error.message};return error instanceof TypeError;});rawRefusals.push({role,kind,format,before,after:key(target),root:key(saved.root),networkSha256:sha256(saved.network.toLino()),refusal,scope:'mandatory actual raw certificate checker; no native graph-decoder claim'});
 }
 assert.equal(rows.length,45);assert.equal(rawRefusals.length,28);return{rows,rawRefusals};
}
// Mechanical lossless output transport only: first-observed u64 bit words,
// then exact row lengths and dictionary indices. This is not String interning.
function protocolStream(observations){const dictionary=[],indices=new Map(),lines=observations.map((row,index)=>{const fields=[String(index),String(row.resultRows.length)];for(const result of row.resultRows){fields.push(String(result.bits.length));for(const bits of result.bits){if(!indices.has(bits)){indices.set(bits,dictionary.length);dictionary.push(bits);}fields.push(indices.get(bits).toString(16));}}return fields.join(':');});return['dictionary:'+dictionary.length+(dictionary.length?':'+dictionary.join(':'):''),...lines].join('\n')+'\n';}
function consumerSource(observations,header){
 const vectors=[],indices=new Map(),pool=bits=>{const key=bits.join('');if(!indices.has(key)){indices.set(key,vectors.length);vectors.push(bits);}return indices.get(key);};
 const blocks=observations.map((row,index)=>{const inputIndices=row.inputArguments.map(value=>value.kind==='vector'?pool(value.bits):null),expectedIndices=row.resultRows.map(value=>pool(value.bits));
  const inputs=row.inputArguments.map((value,slot)=>value.kind==='scalar'?`let input_${slot}=f64::from_bits(0x${value.bits}u64);`:`let input_${slot}:Vec<f64>=vectors[${inputIndices[slot]}].iter().map(|bits|f64::from_bits(*bits)).collect();`);
  const checks=row.inputArguments.map((value,slot)=>value.kind==='scalar'?`assert_eq!(input_${slot}.to_bits(),0x${value.bits}u64,"original scalar ${slot} changed");`:`assert_eq!(input_${slot}.iter().map(|value|value.to_bits()).collect::<Vec<_>>(),vectors[${inputIndices[slot]}],"original vector ${slot} changed");`);
  return`{\n${inputs.join('\n')}\nlet observed=oracle_source::${header.functionName}(${row.inputArguments.map((value,slot)=>'input_'+slot+(value.kind==='vector'?'.clone()':'')).join(',')});\n${checks.join('\n')}\nlet expected_rows:&[usize]=&[${expectedIndices.join(',')}];\nassert_eq!(observed.len(),expected_rows.len(),"whole result row count differs from JavaScript");\nwrite!(&mut line,"${index}:{}",observed.len()).expect("observation row count");\nfor (row,expected_index) in observed.iter().zip(expected_rows){let bits=row.iter().map(|value|value.to_bits()).collect::<Vec<_>>();assert_eq!(bits,vectors[*expected_index],"whole result row differs from JavaScript/independent oracle");write!(&mut line,":{}",bits.len()).expect("row length");for value in bits{let index=if let Some(index)=dictionary.iter().position(|prior|*prior==value){index}else{dictionary.push(value);dictionary.len()-1};write!(&mut line,":{index:x}").expect("exact bit dictionary index");}}\nline.push('\\n');\n}`;
 });
 return`extern crate oracle_source;\nuse std::fmt::Write as _;\nfn main(){let vectors:Vec<Vec<u64>>=vec![${vectors.map(bits=>'vec!['+bits.map(value=>'0x'+value+'u64').join(',')+']').join(',')}];let mut line=String::new();let mut dictionary:Vec<u64>=Vec::new();\n${blocks.join('\n')}\nprint!("dictionary:{}",dictionary.len());for value in dictionary{print!(":{value:016x}");}println!();print!("{line}");}\n`;
}
// Only source text and compact source receipts escape preparation. Full CST
// replay then finishes in a separate scope before the numeric matrix is built.
function prepareOriginalLinkCompleteTableSources(){
 const transportSources=[readCanonicalArchiveTransportSource()];
 const modelSources=verifyOriginalLinkCompleteTableNativeModel(),source=readFileSync(new URL('./rules.mjs',import.meta.url),'utf8'),artifact=readProjectData(new URL('./data/native-definitions.lino',import.meta.url));assert.deepEqual(readProjectData(new URL('./data/policy.lino',import.meta.url)),originalLinkCompleteTablePolicy);assert.equal(originalLinkCompleteTablePolicy.translation.referenceSemantics,false);
 const saved=LinkNetwork.fromLino(readFileSync(new URL('./data/native-definitions.lino',import.meta.url),'utf8'));assert.deepEqual(readStructuredDataLino(saved.toLino()),artifact);assert.deepEqual(readStructuredDataLino(decodeLinkNetworkBinary(encodeLinkNetworkBinary(saved)).toLino()),artifact);
 const program=checkProgram(parseJavaScript(source,originalLinkCompleteTablePolicy.translation)),rust=emitRust(program),javascript=emitJavaScript(program);assert.equal(sha256(source),artifact.sourceSha256);assert.deepEqual(rust.definitions,artifact.declarations);assert.equal(rust.text,artifact.completeRust);assert.equal(program.items.length,66);assert.equal(rust.definitions.length,66);assert.deepEqual(program.items.map(row=>row.fullName),artifact.declarationNames);
 const callee=program.declarations.get('projectOriginalLinkCompleteCertificateTables'),vector={kind:'array',element:{kind:'float'}};assert.deepEqual(callee.params.map(row=>row.name),parameterOrder);for(const[index,row]of callee.params.entries()){assert.deepEqual(row.type,index<2?{kind:'float'}:vector);assert.equal(row.guard,null);assert.equal(row.default,null);}assert.deepEqual(callee.ret,{kind:'array',element:vector});const target=javascript.mappings.find(row=>row.kind==='function'&&row.source===callee.fullName)?.target;
 const native=artifact.completeRust;assert.equal(textBytes(native).length,nativeReceipt.sourceBytes);assert.equal(sha256(native),nativeReceipt.sourceSha256);for(const definition of artifact.declarations)assert.ok(native.includes(definition));
 return freeze({model:artifact.model,modelSources,transportSources,native,header:nativeInterface(native),target,javaScriptText:javascript.text,javaScriptSha256:sha256(javascript.text),
  kernelSha256:artifact.sourceSha256,
  declarations:program.items.map((row,index)=>({kind:row.k,name:row.fullName,sha256:sha256(artifact.declarations[index])})),declarationSha256:sha256(JSON.stringify(artifact.declarations))});
}
function archiveCompleteTableRustSource(native){
 const options={parseSource:source=>{const cst=LinkNetwork.parse(source,'Rust');requireSuccessfulSourceArchive(cst);return cst;},encodeBinary:encodeLinkNetworkBinary,decodeBinary:decodeLinkNetworkBinary,
  onPhase:(phase,count)=>process.stdout.write(`Complete-table source archive: ${phase}; records=${count??'unknown'}; heap=${process.memoryUsage().heapUsed}\n`)};
 const archives=createCompleteRustSourceArchives(native,options),{textReplay,binaryReplay}=replayCompleteRustSourceArchives(archives,options);assert.deepEqual(textBytes(textReplay),textBytes(native));assert.deepEqual(textBytes(binaryReplay),textBytes(native));
 return{replaySha256:sha256(binaryReplay),textReplaySha256:sha256(textReplay)};
}
/** CI planning authenticates full sources and actual JS; it never starts a process. */
export function planOriginalLinkCompleteTableNative(environment=process.env){
 if(environment.CI!=='true')throw new TypeError('complete-table native observation planning requires CI=true; heavy archive work is CI-only');
 return runOriginalLinkCompleteTablePlanningPhases({prepareSource:prepareOriginalLinkCompleteTableSources,replaySource:archiveCompleteTableRustSource,buildMatrix:buildOriginalLinkCompleteTablePlan});
}
/** Ordering alone supplies no checked-plan or native-process authorization. */
export function runOriginalLinkCompleteTablePlanningPhases({prepareSource,replaySource,buildMatrix}){
 const prepared=prepareSource(),replay=replaySource(prepared.native);
 return buildMatrix(prepared,replay);
}
function buildOriginalLinkCompleteTablePlan(prepared,replay){
 const{modelSources,native,header,target}=prepared,context={};
 vm.runInNewContext(prepared.javaScriptText,context,{timeout:5000});assert.equal(typeof context[target],'function');
 const fixtures=readProjectData(new URL('../rust-text-value-projection/data/fixtures.lino',import.meta.url)),observed=cases(context,target,fixtures);
 const consumer=consumerSource(observed.rows,header),protocol=protocolStream(observed.rows),protocolBytes=textBytes(protocol).length,consumerBytes=textBytes(consumer).length;assert.ok(protocolBytes<=maximumProtocolBytes,'whole result protocol exceeds the unchanged runner stdout budget');assert.ok(consumerBytes<=maximumConsumerBytes,'whole external consumer exceeds its declared source budget');
 const entry={id:'complete original sixty-six-declaration table projector',kind:'complete unchanged generated original-ID seventeen-table artifact',fixtureId:'experiments/original-link-complete-table-projection/data/native-definitions.lino',source:native,...nativeReceipt,replaySha256:replay.replaySha256,textReplaySha256:replay.textReplaySha256,originalReceipt:nativeReceipt,...header,
  declarations:prepared.declarations,declarationSha256:prepared.declarationSha256,sourceModelSha256:digests['data/policy.lino'],kernelSha256:prepared.kernelSha256,fixtureSha256:digests['../rust-text-value-projection/data/fixtures.lino'],
  tableOrder:policy.output,invocationCount:45,originalFixtureCount:13,coherentEditCount:4,archiveTransportCount:34,rawRefusalCount:28,rawRefusals:observed.rawRefusals,protocolBytes,maximumProtocolBytes,consumerBytes,maximumConsumerBytes,
  observationScope:'All seventeen original-ID numeric tables or exact projection refusal rows over bounded, raw-checker-authorized primitive transports. Source-free original/edit graphs are checked in both archive formats before exact-bit transport deduplication. Primitive owner contradictions retain refusal by the existing JavaScript certificate; raw owner forgeries are separate mandatory host checks. Chosen NaN bits are bounded observations, not a universal NaN payload promise. No native pre-Serde graph capture, production/public selector, package, resource or performance claim.',
  archiveFormats:['all thirteen original and four coherent-edit graphs in Links text and binary','complete sixty-six-definition artifact data in Links text and binary','unchanged complete generated Rust CST in Links text and binary'],consumerSource:consumer,consumerSha256:sha256(consumer),javaScriptSha256:prepared.javaScriptSha256,observations:observed.rows};
 const plan=freeze({model:prepared.model,modelCommit:ORIGINAL_LINK_COMPLETE_TABLE_MODEL_COMMIT,modelSources,transportSources:prepared.transportSources,originalCount:1,entries:[entry]});checkedPlans.add(plan);return plan;
}
function parseObservations(output,expected){
 assert.ok(textBytes(output).length<=maximumProtocolBytes,'the actual protocol exceeds its declared bound');const lines=output.split('\n');assert.equal(lines.pop(),'','the native stream ends with one newline');assert.equal(lines.length,expected.length+1,'every invocation plus one dictionary is present');
 const header=lines.shift().split(':');assert.equal(header[0],'dictionary');assert.match(header[1]??'',/^(?:0|[1-9][0-9]*)$/u);const count=Number(header[1]);assert.ok(Number.isSafeInteger(count));assert.equal(header.length,count+2);const dictionary=header.slice(2);dictionary.forEach(bits=>assert.match(bits,/^[0-9a-f]{16}$/u));assert.equal(new Set(dictionary).size,dictionary.length,'bit dictionary entries cannot repeat');const used=new Set();
 const rows=lines.map((line,index)=>{const fields=line.split(':');assert.equal(fields.shift(),String(index),'invocation order is exact');assert.equal(fields.shift(),String(expected[index].resultRows.length),'complete result row count is exact');const resultRows=expected[index].resultRows.map(expectedRow=>{assert.equal(fields.shift(),String(expectedRow.bits.length),'every whole row length is exact');const bits=expectedRow.bits.map(()=>{const encoded=fields.shift();assert.match(encoded??'',/^(?:0|[1-9a-f][0-9a-f]*)$/u);const at=Number.parseInt(encoded,16);assert.ok(Number.isSafeInteger(at)&&at<dictionary.length,'every bit index names an actual dictionary entry');if(!used.has(at)){assert.equal(at,used.size,'dictionary order is first observed, never reordered');used.add(at);}return dictionary[at];});assert.deepEqual(bits,expectedRow.bits,'every native cell must match complete JavaScript/independent expected rows');return{bits};});assert.equal(fields.length,0,'no extra result cells or rows');return{id:expected[index].id,kind:expected[index].kind,inputArguments:expected[index].inputArguments,resultRows,certificate:expected[index].certificate,provenance:expected[index].provenance};});assert.equal(used.size,dictionary.length,'no unused dictionary entries');return rows;
}
const runCheckedCompleteTableNative=createSourceNativeObservationRunner({nativeMode:originalLinkCompleteTableNativeMode,checkedPlan:plan=>{if(!checkedPlans.has(plan))return false;assert.deepEqual(plan.transportSources,[readCanonicalArchiveTransportSource()],'the actual archive transport source changed before execution');return true;},textBytes,parseObservations,
 entryReceipt:entry=>({functionName:entry.functionName,parameterOrder:entry.parameterOrder,parameterModes:entry.parameterModes,resultMode:entry.resultMode,referenceSemantics:entry.referenceSemantics,interfaceSha256:entry.interfaceSha256,tokenCount:entry.tokenCount,declarations:entry.declarations,declarationSha256:entry.declarationSha256,sourceModelSha256:entry.sourceModelSha256,kernelSha256:entry.kernelSha256,fixtureSha256:entry.fixtureSha256,textReplaySha256:entry.textReplaySha256,tableOrder:entry.tableOrder,invocationCount:entry.invocationCount,originalFixtureCount:entry.originalFixtureCount,coherentEditCount:entry.coherentEditCount,archiveTransportCount:entry.archiveTransportCount,rawRefusalCount:entry.rawRefusalCount,rawRefusals:entry.rawRefusals,protocolBytes:entry.protocolBytes,maximumProtocolBytes:entry.maximumProtocolBytes,consumerBytes:entry.consumerBytes,maximumConsumerBytes:entry.maximumConsumerBytes,observationScope:entry.observationScope,archiveFormats:entry.archiveFormats}),
 temporaryPrefix:'original-link-complete-table-native-',failureLabel:'Original complete link table observations failed',
});
export function runOriginalLinkCompleteTableNative(plan,options){
 const receipt=runCheckedCompleteTableNative(plan,options);
 assert.deepEqual(plan.transportSources,[readCanonicalArchiveTransportSource()],'the actual archive transport source changed during execution');
 return{...receipt,transportSources:plan.transportSources};
}
