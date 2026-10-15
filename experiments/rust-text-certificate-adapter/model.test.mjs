import assert from 'node:assert/strict';
import test,{before,after} from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../js/src/index.js';
import {TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {RUST_TEXT_VALUE_PROJECTION_POLICY as sourcePolicy} from '../rust-text-value-projection/model.mjs';
import {captureOwnedRustTextCertificate,readOwnedRustTextCertificate} from './model.mjs';
import {createRustTextCertificatePublicOverlay} from './runtime.mjs';

const fixtures=readProjectData(new URL('../rust-text-value-projection/data/fixtures.lino',import.meta.url));
const code=id=>fixtures.cases.find(row=>row.id===id).source;
const facet='sourceTextValueProjection';
const label=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
const text=packet=>({...packet,network:LinkNetwork.fromLino(packet.network.toLino())});
const binary=packet=>({...packet,network:decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))});
let overlay;
before(async()=>{overlay=await createRustTextCertificatePublicOverlay();});
after(()=>overlay?.dispose());
const create=id=>captureOwnedRustTextCertificate(code(id));
const encode=program=>overlay.api.encodeTypedProgramLinks(program);
const decode=packet=>overlay.api.decodeTypedProgramLinks(packet.network,packet.root);
function field(network,parent,name){const row=network.fieldRelations().find(row=>row.parent.equals(parent)&&row.label===label(name));assert.ok(row,name);return row;}
function proofId(packet){return field(packet.network,packet.network.link(packet.root).references()[0],facet).child;}
function replaceField(row,child){row.link.setReferences([row.parent,row.link.references()[1],child]);}
function execute(packet,observations,name='choose'){
 const emitted=overlay.api.emitTypedProgramLinks(packet.network,packet.root,'JavaScript'),context={};vm.runInNewContext(emitted.code,context,{timeout:1000});
 for(const{args,value}of observations)assert.equal(context[name](...args),value);return{emitted,context};
}
function refuse(program){
 for(const[name,check]of[['owned facet',()=>readOwnedRustTextCertificate(program)],['validation',()=>overlay.validation.validateLinkedTypedProgram(program)],['recheck',()=>overlay.projection.recheckLinkedTypedProgram(program)],['Rust',()=>overlay.rust.emitRust(program)],['JavaScript',()=>overlay.javascript.emitJavaScript(program)],['encode',()=>encode(program)]])assert.throws(check,name);
}
function retarget(program,index){const entry=program.declarations.get('choose'),proof=program[facet],row=proof.functions[0],target=entry.params[index],token=proof.tokens.find(token=>token.binding===row.input&&token.span.start===row.coreValue.span.start);assert.ok(token);token.binding=target;entry.body.name=target.name;row.input=target;row.position=entry.sourceSignature.parameters[index];for(const condition of[...proof.inputAssumptions,...proof.valueAssumptions])if(condition.declaration===entry){condition.input=target;condition.position=row.position;}}

test('all thirteen complete modules use actual codec validation, both saved formats, source replay and executable public emission',()=>{
 assert.equal(fixtures.cases.length,13);
 for(const entry of fixtures.cases)for(const load of[text,binary]){
  const program=captureOwnedRustTextCertificate(entry.source),packet=load(encode(program)),before=packet.network.toLino(),restored=decode(packet),proof=restored[facet],owned=readOwnedRustTextCertificate(restored);
  assert.equal(owned.decision[0],1);assert.equal(proof.worker,restored.main);assert.equal(Object.hasOwn(owned.program,facet),false);assert.equal(owned.program.declarations,restored.declarations);
  for(const row of proof.functions){assert.equal(row.declaration,restored.declarations.get(row.declaration.fullName));assert.equal(row.signature,row.declaration.sourceSignature);assert.equal(row.resultPosition,row.signature.result);assert.equal(row.coreValue,row.declaration.body);if(row.input)assert.equal(row.position,row.signature.parameters[row.declaration.params.indexOf(row.input)]);}
  const emitted=overlay.api.emitTypedProgramLinks(packet.network,packet.root,'Rust');assert.equal(emitted.code,entry.source);assert.deepEqual(emitted.mappings,[...restored.declarations.values()].map(value=>({kind:'function',source:value.fullName,target:value.name,sourceSpan:value.span})));assert.equal(overlay.rust.emitRust(restored).text,entry.source);execute(packet,entry.observations);
  assert.equal(packet.network.toLino(),before);assert.equal(overlay.projection.recheckLinkedTypedProgram(restored),restored);
 }
});

test('actual saved clone retargets recompute receiver dispatch and executable parameter selection',()=>{
 for(const load of[text,binary]){
  const program=decode(load(encode(create('valid-clone-retarget'))));retarget(program,1);const packet=load(encode(program)),restored=decode(packet),row=restored[facet].functions[0];
  assert.equal(row.input,restored.declarations.get('choose').params[1]);assert.match(overlay.api.emitTypedProgramLinks(packet.network,packet.root,'Rust').code,/right\.clone\(\)/u);execute(packet,[{args:['first','second Ω😀'],value:'second Ω😀'}]);
  const invalid=create('invalid-slice-retarget');retarget(invalid,1);refuse(invalid);
 }
});

test('source-free names and control-code String edits preserve complete replay and rebind actual conditions',()=>{
 const program=create('raw-literal-construction'),entry=program.declarations.get('choose'),value=Array.from({length:32},(_,index)=>String.fromCharCode(index)).join('')+'\u007f"\\Ω😀\u{10ffff}';
 entry.name='choose_changed';entry.fullName='choose_changed';program.declarations=new Map([['choose_changed',entry]]);program[facet].functions[0].literal.value=value;
 const packet=binary(text(encode(program))),emitted=overlay.api.emitTypedProgramLinks(packet.network,packet.root,'Rust'),fresh=captureOwnedRustTextCertificate(emitted.code),next=text(encode(fresh));
 assert.match(emitted.code,/pub fn choose_changed/u);assert.ok(emitted.code.includes('\\u{8}'));assert.deepEqual(emitted.assumptions,overlay.rust.emitRust(decode(next)).assumptions);execute(packet,[{args:[],value}],'choose_changed');execute(next,[{args:[],value}],'choose_changed');
});

test('mandatory facade validation refuses omitted, copied and contradictory actual source owners at every host boundary',()=>{
 const mutations=[
  program=>{program[facet]=null;},program=>{program[facet]=false;},program=>{program[facet]=undefined;},
  program=>{program.main[facet]=program[facet];delete program[facet];},program=>{program.main[facet]=program[facet];},
  program=>{program.extra=false;},program=>{program[facet].worker={...program.main};},
  program=>{program[facet].functions[0].signature=structuredClone(program[facet].functions[0].signature);},
  program=>{program[facet].functions[0].coreValue=structuredClone(program[facet].functions[0].coreValue);},
  program=>{program[facet].functions[0].input=structuredClone(program[facet].functions[0].input);},
  program=>{program[facet].functions[0].position=null;},program=>{delete program[facet].functions[0].input;},
  program=>{program[facet].valueAssumptions=[];},program=>{program[facet].inputAssumptions[0].meaning='unconditional equivalence';},
  program=>{program[facet].tokens.extra=false;},program=>{program.declarations.extra=false;},
 ];
 for(const mutate of mutations){const program=create('owned-clone');mutate(program);refuse(program);}
 for(const target of['Rust','JavaScript']){const program=create('owned-clone');assert.throws(()=>target==='Rust'?overlay.rust.emitRust(program,{sourceHeaderModel:'unproved'}):overlay.javascript.emitJavaScript(program,{scopedBindingModel:'unproved'}));}
});

test('descriptor guards reject hidden source facts and getters before invoking them',()=>{
 for(const target of['root','proof','parameter']){
  const program=create('owned-clone'),value=target==='root'?program:target==='proof'?program[facet]:program.declarations.get('choose').params[0];let calls=0;
  Object.defineProperty(value,'hidden',{get(){calls++;return false;}});refuse(program);assert.equal(calls,0);
 }
 const program=create('owned-clone'),proof=program[facet];let calls=0;Object.defineProperty(program,facet,{get(){calls++;return proof;},enumerable:true});refuse(program);assert.equal(calls,0);
});

test('actual decoded selected programs cannot fall through either emitter via function or inherited roots',()=>{
 const program=decode(binary(encode(create('owned-clone'))));
 for(const root of[Object.assign(function host(){},program),Object.create(program),Object.assign(new Date(0),program)])refuse(root);
 for(const root of[Object.assign(function host(){},program),Object.create(program)]){let calls=0;Object.defineProperty(root,facet,{get(){calls++;return program[facet];},enumerable:true});refuse(root);assert.equal(calls,0);}
});

test('explicit falsey target profile selections refuse through actual linked and direct emitters',()=>{
 const program=create('owned-clone'),packet=encode(program);
 for(const selection of['',null,false,0]){
  assert.throws(()=>overlay.rust.emitRust(program,{sourceHeaderModel:selection}));assert.throws(()=>overlay.javascript.emitJavaScript(program,{scopedBindingModel:selection}));
  assert.throws(()=>overlay.api.emitTypedProgramLinks(packet.network,packet.root,'Rust',{rustSourceHeaderEmissionModel:selection}));assert.throws(()=>overlay.api.emitTypedProgramLinks(packet.network,packet.root,'JavaScript',{javaScriptScopedBindingEmissionModel:selection}));
 }
 assert.equal(overlay.rust.emitRust(program,{sourceHeaderModel:undefined}).text,code('owned-clone'));
});

test('malformed main containers cannot hide source facts or invoke accessors during an unselected fallback',()=>{
 for(const kind of['function','inherited','accessor']){
  const program=create('owned-clone'),main=program.main;delete program[facet];let calls=0;
  if(kind==='function')program.main=Object.assign(function host(){},main);
  if(kind==='inherited')program.main=Object.create(main);
  if(kind==='accessor')Object.defineProperty(program,'main',{get(){calls++;return main;},enumerable:true});
  refuse(program);assert.equal(calls,0);
 }
});

test('a misplaced source facet on a callback refuses without observing callback accessors',()=>{
 for(const accessor of[false,true]){
  const program=create('owned-clone'),proof=program[facet];delete program[facet];program.hidden=function host(){};let calls=0;
  if(accessor)Object.defineProperty(program.hidden,facet,{get(){calls++;return proof;},enumerable:true});else program.hidden[facet]=proof;
  refuse(program);assert.equal(calls,0);
 }
});

test('saved raw owner substitutions are mandatory before public decode and both linked emissions',()=>{
 for(const load of[text,binary])for(const role of['signature','coreValue','input','position']){
  const packet=encode(create('equal-valued-distinct-owners')),proof=proofId(packet),functions=packet.network.link(field(packet.network,proof,'functions').child).references(),first=field(packet.network,functions[0],role),other=field(packet.network,functions[1],role);assert.ok(!first.child.equals(other.child));replaceField(first,other.child);
  const saved=load(packet),before=saved.network.toLino();for(const check of[()=>decode(saved),()=>overlay.api.emitTypedProgramLinks(saved.network,saved.root,'Rust'),()=>overlay.api.emitTypedProgramLinks(saved.network,saved.root,'JavaScript')])assert.throws(check,role);assert.equal(saved.network.toLino(),before);
 }
});

test('all seven actual null-prototype core roles retain consistent direct output, metadata, conditions and coherent edits',()=>{
 const source=code('valid-clone-retarget');
 for(const role of['root','main','declaration','parameter','signature','position','body']){
  const program=captureOwnedRustTextCertificate(source),entry=program.declarations.get('choose'),value={root:program,main:program.main,declaration:entry,parameter:entry.params[0],signature:entry.sourceSignature,position:entry.sourceSignature.parameters[0],body:entry.body}[role];Object.setPrototypeOf(value,null);
  const checked=readOwnedRustTextCertificate(program);assert.equal(checked.program.declarations,program.declarations);assert.equal(checked.proof.functions[0].input,entry.params[0]);assert.equal(checked.proof.functions[0].coreValue,entry.body);
  for(const edited of[false,true]){
   if(edited)retarget(program,1);const packet=binary(text(encode(program))),restored=decode(packet);
   for(const target of['Rust','JavaScript']){
    const emit=target==='Rust'?overlay.rust.emitRust:overlay.javascript.emitJavaScript,direct=emit(program),saved=emit(restored);assert.equal(direct.text,saved.text,role);assert.deepEqual(direct.mappings,saved.mappings,role);assert.deepEqual(direct.assumptions,saved.assumptions,role);
   }
   assert.equal(overlay.rust.emitRust(program).text,edited?source.replace('{left.clone()}','{right.clone()}'):source);execute(packet,[{args:['first','second Ω😀'],value:edited?'second Ω😀':'first'}]);assert.equal(Object.getPrototypeOf(value),null,'the caller graph is never normalized in place');
  }
  const counterfeit=captureOwnedRustTextCertificate(source);Object.setPrototypeOf(counterfeit,null);counterfeit[facet].functions[0].input=structuredClone(counterfeit[facet].functions[0].input);
  for(const emit of[overlay.rust.emitRust,overlay.javascript.emitJavaScript])assert.throws(()=>emit(counterfeit),/Portable Rust text certificate/u);
 }
});

test('all twenty-six original complete-source refusals keep their exact reason and span',()=>{
 assert.equal(fixtures.invalid.length,26);for(const entry of fixtures.invalid)assert.throws(()=>captureOwnedRustTextCertificate(entry.source),error=>{assert.equal(error.name,entry.name,entry.id);assert.equal(error.reason??error.message,entry.reason,entry.id);assert.deepEqual(error.span??null,entry.span,entry.id);return true;});
});

test('owned moves retain input conditions without invented allocation while cloning keeps actual guarded conditions',()=>{
 const move=decode(text(encode(create('owned-move'))));assert.equal(move[facet].valueAssumptions.length,0);assert.equal(overlay.rust.emitRust(move).assumptions.length,1);
 const packet=binary(encode(create('owned-clone'))),{context,emitted}=execute(packet,[{args:['Ω😀\0'],value:'Ω😀\0'}]);assert.equal(emitted.assumptions.filter(row=>row.id===sourcePolicy.valueAssumption.id).length,1);
 for(const input of['\ud800','\udfff',new String('text'),1,null,{}])assert.throws(()=>context.choose(input),/input domain/u);assert.throws(()=>context.choose(),/input domain/u);assert.throws(()=>context.choose('one','two'),/input domain/u);
});
