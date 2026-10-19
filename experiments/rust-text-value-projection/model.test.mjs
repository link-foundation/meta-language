import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../js/src/index.js';
import {parseRust,parseRustTokenStream} from '../../js/src/translation/rust.js';
import {tokenize} from '../../js/src/translation/lexer.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {rustTextValueProjectionRules} from './generate-rules.mjs';
import {rustTextValueProjectionFixtures} from './generate-fixtures.mjs';
import {createRustTextValueProjection,checkRustTextValueProjection,encodeRustTextValueProjection,createRustTextValueProjectionLinks,decodeRustTextValueProjection,emitRustTextValueProjection,emitRustTextValueProjectionLinks,emitGenericRustTextValueProjection,RUST_TEXT_VALUE_PROJECTION_POLICY as policy} from './model.mjs';
const fixtures=readProjectData(new URL('./data/fixtures.lino',import.meta.url));
const source=id=>fixtures.cases.find(row=>row.id===id).source;
const text=packet=>({...packet,network:LinkNetwork.fromLino(packet.network.toLino())});
const binary=packet=>({...packet,network:decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))});
const reload=packet=>binary(text(packet));
const decode=packet=>decodeRustTextValueProjection(packet.network,packet.root);
const create=code=>reload(createRustTextValueProjectionLinks(code));
const label=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
function field(network,parent,name){const row=network.fieldRelations().find(row=>row.parent.equals(parent)&&row.label===label(name));assert.ok(row,name);return row;}
function replaceField(row,child){row.link.setReferences([row.parent,row.link.references()[1],child]);}
function rawProof(packet){return field(packet.network,field(packet.network,packet.root,'provenance').child,'sourceInterpretation').child;}
function rows(packet,name){return packet.network.link(field(packet.network,rawProof(packet),name).child).references();}
function execute(packet,observations,name='choose'){
 const emitted=emitRustTextValueProjectionLinks(packet.network,packet.root,'JavaScript'),context={};vm.runInNewContext(emitted.text,context,{timeout:1000});
 for(const{args,value}of observations)assert.equal(context[name](...args),value);
 return{emitted,context};
}
function reject(value){for(const check of[()=>checkRustTextValueProjection(value.program,value.proof),()=>encodeRustTextValueProjection(value.program,value.proof),()=>emitRustTextValueProjection(value.program,value.proof,'Rust'),()=>emitRustTextValueProjection(value.program,value.proof,'JavaScript')])assert.throws(check);}
function retarget(value,index){const entry=value.program.declarations.get('choose'),row=value.proof.functions[0],target=entry.params[index];
 const token=value.proof.tokens.find(token=>token.binding===row.input&&token.span.start===row.coreValue.span.start);assert.ok(token);token.binding=target;entry.body.name=target.name;
 row.input=target;row.position=entry.sourceSignature.parameters[index];for(const condition of[...value.proof.inputAssumptions,...value.proof.valueAssumptions])if(condition.declaration===entry){condition.input=row.input;condition.position=row.position;}
}
function verifyWhole(packet,observations,name='choose'){
 const value=decode(packet),rust=emitRustTextValueProjection(value.program,value.proof,'Rust'),restored=create(rust.text),fresh=decode(restored);
 assert.equal(fresh.proof.model,policy.model);assert.deepEqual(rust.assumptions,emitRustTextValueProjection(fresh.program,fresh.proof,'Rust').assumptions);execute(restored,observations,name);return{rust,restored,...fresh};
}

test('thirteen complete original modules preserve every byte, real source owners and checked values through text and binary',()=>{
 assert.deepEqual(rustTextValueProjectionFixtures(),fixtures);assert.equal(fixtures.cases.length,13);
 for(const entry of fixtures.protectedArtifacts)assert.equal(createHash('sha256').update(readFileSync(new URL(entry.path,import.meta.url))).digest('hex'),entry.sha256);
 for(const entry of fixtures.cases){assert.equal(Buffer.byteLength(entry.source),entry.sourceBytes);assert.equal(createHash('sha256').update(entry.source).digest('hex'),entry.sourceSha256);
  const packet=create(entry.source),{program,proof}=decode(packet);assert.equal(emitRustTextValueProjection(program,proof,'Rust').text,entry.source);execute(packet,entry.observations);verifyWhole(packet,entry.observations);
  assert.equal(proof.functions[0].operationNode.op,entry.operation);assert.ok(packet.network.links().every(link=>link.metadata().definition!==entry.source));
  for(const row of proof.functions){assert.equal(row.declaration,program.declarations.get(row.declaration.fullName));assert.equal(row.signature,row.declaration.sourceSignature);assert.equal(row.resultPosition,row.signature.result);assert.equal(row.coreValue,row.declaration.body);
   if(row.input){const index=row.declaration.params.indexOf(row.input);assert.ok(index>=0);assert.equal(row.position,row.signature.parameters[index]);assert.ok(proof.tokens.some(token=>token.binding===row.input&&token.span.start===row.coreValue.span.start));}else assert.equal(row.position,null);
   if(row.literal)assert.ok(proof.tokens.some(token=>token.literal===row.literal));
  }
 }
});

test('actual clone receiver dispatch is rechecked after coherent valid and invalid saved input retargets',()=>{
 let value=decode(create(source('valid-clone-retarget')));retarget(value,1);let packet=reload(encodeRustTextValueProjection(value.program,value.proof));execute(packet,[{args:['first','second Ω😀'],value:'second Ω😀'}]);let result=verifyWhole(packet,[{args:['first','second Ω😀'],value:'second Ω😀'}]);assert.match(result.rust.text,/right\.clone\(\)/u);
 value=decode(result.restored);retarget(value,0);packet=reload(encodeRustTextValueProjection(value.program,value.proof));verifyWhole(packet,[{args:['first','second'],value:'first'}]);
 value=decode(create(source('invalid-slice-retarget')));retarget(value,1);reject(value);
});

test('coherent source-free declaration, parameter and Unicode literal edits rebind whole source conditions at new spans',()=>{
 let value=decode(create('/* original Ω😀 */\n'+source('valid-clone-retarget'))),entry=value.program.declarations.get('choose');entry.name='choose_changed';entry.fullName='choose_changed';value.program.declarations=new Map([['choose_changed',entry]]);entry.params[0].name='changed_input';entry.body.name='changed_input';
 let packet=reload(encodeRustTextValueProjection(value.program,value.proof)),result=verifyWhole(packet,[{args:['renamed Ω😀','other'],value:'renamed Ω😀'}],'choose_changed');assert.match(result.rust.text,/pub fn choose_changed\(changed_input:String/u);
 value=decode(create(source('raw-literal-construction')));const literal=value.proof.functions[0].literal;literal.value=Array.from({length:32},(_,at)=>String.fromCharCode(at)).join('')+'\u007f"\\Ω😀\u{10ffff}';
 packet=reload(encodeRustTextValueProjection(value.program,value.proof));const originalStart=value.proof.functions[0].operationNode.span.end;result=verifyWhole(packet,[{args:[],value:literal.value}]);assert.ok(result.rust.text.includes('\\u{8}'));assert.notEqual(result.proof.functions[0].operationNode.span.end,originalStart);
 for(let cycle=0;cycle<2;cycle++){packet=result.restored;result=verifyWhole(packet,[{args:[],value:literal.value}]);}
});

test('input value and allocation-success conditions retain actual referenced owners and move creates no allocation premise',()=>{
 const move=decode(create(source('owned-move')));assert.equal(move.proof.inputAssumptions.length,1);assert.equal(move.proof.valueAssumptions.length,0);assert.equal(emitRustTextValueProjection(move.program,move.proof,'Rust').assumptions.length,1);
 for(const id of['owned-clone','shared-string-clone','borrowed-str-construction','borrowed-str-owned','borrowed-str-display']){
  const packet=create(source(id)),{proof}=decode(packet);assert.equal(proof.inputAssumptions.length,1);assert.equal(proof.valueAssumptions.length,1);const row=proof.functions[0];for(const condition of[...proof.inputAssumptions,...proof.valueAssumptions]){assert.equal(condition.operationNode,row.operationNode);assert.equal(condition.declaration,row.declaration);assert.equal(condition.coreValue,row.coreValue);assert.equal(condition.input,row.input);assert.equal(condition.position,row.position);}
  const{context,emitted}=execute(packet,[{args:['Ω😀\0'],value:'Ω😀\0'}]);for(const input of['\ud800','\udfff',new String('text'),1,null,{}])assert.throws(()=>context.choose(input),/input domain/u);assert.throws(()=>context.choose(),/input domain/u);assert.throws(()=>context.choose('one','two'),/input domain/u);assert.equal(emitted.assumptions.at(-1).id,'owned-local-text-input-realm');
 }
 const literal=decode(create(source('cooked-literal-construction')));assert.equal(literal.proof.inputAssumptions.length,0);assert.equal(literal.proof.valueAssumptions.length,1);
});

test('twenty-six exact complete-source refusals retain body, header, lifetime, owner and grammar boundaries',()=>{
 assert.equal(fixtures.invalid.length,26);for(const entry of fixtures.invalid)assert.throws(()=>createRustTextValueProjection(entry.source),error=>{assert.equal(error.name,entry.name,entry.id);assert.equal(error.reason??error.message,entry.reason,entry.id);assert.deepEqual(error.span??null,entry.span,entry.id);return true;});
 for(const code of[source('owned-clone').replace('text.clone()','text.clone(1i32)'),source('owned-clone').replace('text.clone()','text.other()'),source('owned-clone').replace('pub fn','pub\ufefffn'),source('owned-clone').replace('text.clone()','{text.clone()}')])assert.throws(()=>createRustTextValueProjection(code));
});

test('raw and cooked literal spellings and ordinary Rust trivia replay through the actual complete production lexer',()=>{
 for(const raw of['r"zero"','r###"quote "+ Ω😀"###','"\\0'+'123"','"\\x7f"','"\\u{1_f600}"','"line\\\n    folded"']){
  const code=source('cooked-literal-construction').replace('"Ω😀"',raw),packet=create(code);assert.equal(emitRustTextValueProjectionLinks(packet.network,packet.root,'Rust').text,code);verifyWhole(packet,[{args:[],value:tokenize(raw,'Rust').tokens[0].value}]);
 }
 for(const gap of['\u0085','\u200e','\u200f','/**/','/***/','//// ordinary\n','/* Ω😀 /* nested /** doc-looking */ */ */']){const code=source('owned-clone').replace('pub fn','pub'+gap+'fn'),packet=create(code);assert.equal(emitRustTextValueProjectionLinks(packet.network,packet.root,'Rust').text,code);execute(packet,[{args:['input'],value:'input'}]);}
 for(const raw of['"\\b"','"\\f"','"\\v"','"\\{"','"\\x80"','"\\u{d800}"'])assert.throws(()=>createRustTextValueProjection(source('cooked-literal-construction').replace('"Ω😀"',raw)));
});

test('malformed source-free core, source-leaf, signature, operation, assumption and descriptor edits refuse at every explicit boundary',()=>{
 const mutations=[
  value=>{value.proof.functions[0].position=null;},value=>{delete value.proof.functions[0].input;},value=>{value.proof.functions[0].input=null;},value=>{value.proof.functions[0].signature=structuredClone(value.proof.functions[0].signature);},
  value=>{value.proof.functions[0].resultPosition=structuredClone(value.proof.functions[0].resultPosition);},value=>{value.proof.functions[0].operationNode=structuredClone(value.proof.functions[0].operationNode);},value=>{value.proof.functions[0].coreValue=structuredClone(value.proof.functions[0].coreValue);},
  value=>{value.proof.inputAssumptions[0].position=null;},value=>{value.proof.valueAssumptions=[];},value=>{value.proof.valueAssumptions[0].meaning='unconditional equivalence';},value=>{value.proof.valueAssumptions.push(value.proof.valueAssumptions[0]);},
  value=>{value.proof.functions[0].signature.parameters[0].owner.path=['custom','String'];},value=>{value.proof.functions[0].signature.parameters[0].wrappers.push('reference');value.proof.functions[0].signature.parameters[0].lifetimes.push(null);},
  value=>{value.proof.functions[0].signature.result.wrappers=['reference'];value.proof.functions[0].signature.result.lifetimes=[null];},value=>{value.proof.worker={...value.proof.worker};},
  value=>{value.proof.gaps[1].units[0]='32';},value=>{value.proof.gaps[1].units[0]=-0;},value=>{value.proof.span.extra=false;},value=>{value.proof.tokens.extra=false;},
  value=>{value.proof.operations.parts[0].parts.at(-1).parts[1]=value.proof.operations;},value=>{value.program.declarations.get('choose').params[0].extra=false;},
 ];
 for(const mutate of mutations){const value=decode(create(source('owned-clone')));mutate(value);reject(value);}
 for(const scalar of[true,42,42n,new String('text'),'\ud800']){const value=decode(create(source('cooked-literal-construction')));value.proof.functions[0].literal.value=scalar;reject(value);}
 const moved=decode(create(source('owned-move')));moved.proof.valueAssumptions=[{...moved.proof.inputAssumptions[0],id:policy.valueAssumption.id,meaning:policy.valueAssumption.meaning}];reject(moved);
 let calls=0;const getter=decode(create(source('owned-clone')));Object.defineProperty(getter.proof.functions[0],'hidden',{get(){calls++;return false;}});reject(getter);assert.equal(calls,0);
});

test('genuine saved text and binary identity substitutions refuse before source projection or either emitter',()=>{
 for(const load of[text,binary])for(const role of['declaration','signature','resultPosition','operationNode','coreValue','input','position']){
  const packet=create(source('equal-valued-distinct-owners')),ids=rows(packet,'functions'),row=field(packet.network,ids[0],role),other=field(packet.network,ids[1],role);assert.ok(!row.child.equals(other.child));replaceField(row,other.child);const saved=load(packet);
  for(const check of[()=>decode(saved),()=>emitRustTextValueProjectionLinks(saved.network,saved.root,'Rust'),()=>emitRustTextValueProjectionLinks(saved.network,saved.root,'JavaScript')])assert.throws(check,role);
 }
 for(const load of[text,binary]){
  const packet=create(source('valid-clone-retarget')),functions=rows(packet,'functions'),signature=field(packet.network,functions[0],'signature').child,positions=packet.network.link(field(packet.network,signature,'parameters').child).references();
  replaceField(field(packet.network,functions[0],'position'),positions[1]);assert.throws(()=>decode(load(packet)));
  const other=create(source('equal-valued-distinct-owners')),conditions=rows(other,'valueAssumptions'),functionIds=rows(other,'functions');replaceField(field(other.network,conditions[0],'operationNode'),field(other.network,functionIds[1],'operationNode').child);assert.throws(()=>decode(load(other)));
 }
});

test('actual complete generic Rust artifacts retain exact receipts and explicit runtime composition refusals',()=>{
 for(const entry of fixtures.cases){const value=decode(create(entry.source)),emitted=emitGenericRustTextValueProjection(value.program,value.proof);assert.equal(emitted.text,entry.fullGenericRust);assert.equal(Buffer.byteLength(emitted.text),entry.genericBytes);assert.equal(createHash('sha256').update(emitted.text).digest('hex'),entry.genericSha256);
  assert.throws(()=>createRustTextValueProjection(emitted.text),error=>{assert.equal(error.name,entry.genericRefusal.name);assert.equal(error.reason??error.message,entry.genericRefusal.reason);assert.deepEqual(error.span??null,entry.genericRefusal.span);return true;});
 }
});

test('the internal complete-token seam preserves all twenty-six indirection fixtures and nineteen original source signatures',()=>{
 for(const file of['../../parity/fixtures/rust-source-indirection.lino','../../parity/fixtures/rust-source-signatures.lino']){
  const fixture=readProjectData(new URL(file,import.meta.url));for(const entry of fixture.cases){const options=fixture.sourceSignatureModel?{sourceSignatureModel:fixture.sourceSignatureModel}:{},surface=parseRust(entry.source,options);assert.deepEqual(surface,parseRustTokenStream(tokenize(entry.source,'Rust').tokens,options));if(entry.surface)assert.deepEqual(JSON.parse(JSON.stringify(surface)),entry.surface);}
 }
 const fixture=readProjectData(new URL('../../parity/fixtures/rust-source-indirection.lino',import.meta.url));assert.equal(fixture.cases.length,26);
 for(const entry of fixture.cases){const context={console:{log:line=>lines.push(String(line))}},lines=[];vm.runInNewContext(emitJavaScript(checkProgram(parseRust(entry.source))).text,context,{timeout:1000});assert.deepEqual(lines,entry.output,entry.id);}
});

test('pure linked policy kernels are automatically translated and executable after real text and binary Links replay',()=>{
 const checked=checkProgram(parseJavaScript(rustTextValueProjectionRules())),packet=encodeTypedProgramLinks(checked),restored=decodeTypedProgramLinks(reload(packet).network,packet.root),emitted=emitJavaScript(restored),context={};vm.runInNewContext(emitted.text,context);
 const rule=name=>context[emitted.mappings.find(row=>row.source===name).target];assert.equal(rule('acceptRustTextValueProjectionModel')(policy.model),true);assert.equal(rule('acceptRustTextValueProjectionModel')(''),false);
 for(const standard of[false,true])for(const slice of[false,true])for(const layers of[0,1,2])assert.equal(rule('readRustTextValueProjectionDomain')(standard,slice,layers),standard&&layers===0?policy.domains.owned:standard&&layers===1?policy.domains.reference:slice&&layers===1?policy.domains.slice:'');
 for(const operation of[...policy.operations.map(row=>row.name),'unproved operation'])for(const domain of[...Object.values(policy.domains),'unknown'])assert.equal(rule('acceptRustTextValueProjectionOperation')(operation,domain),policy.operations.some(row=>row.name===operation&&row.inputs.includes(domain)));
});
