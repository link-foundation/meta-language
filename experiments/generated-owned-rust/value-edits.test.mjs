import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../js/src/index.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {generatedOwnedRustValueEditFixtures} from './generate-value-edit-fixtures.mjs';
import {generatedOwnedRustValueEditRules} from './generate-value-edit-rules.mjs';
import {createGeneratedOwnedRust,createGeneratedOwnedRustLinks,decodeGeneratedOwnedRustLinks,encodeGeneratedOwnedRustLinks,checkGeneratedOwnedRust,emitGeneratedOwnedRust,emitGeneratedOwnedRustLinks,emitCanonicalGeneratedOwnedRust,GENERATED_OWNED_RUST_VALUE_EDIT_POLICY as policy} from './model.mjs';
const original=readProjectData(new URL('./data/fixtures.lino',import.meta.url)),fixtures=readProjectData(new URL('./data/value-edit-fixtures.lino',import.meta.url));
const source=original.cases[0].fullEmittedRust;
const reloadText=packet=>({...packet,network:LinkNetwork.fromLino(packet.network.toLino())});
const reloadBinary=packet=>({...packet,network:decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))});
const reload=packet=>reloadBinary(reloadText(packet));
const decoded=packet=>decodeGeneratedOwnedRustLinks(packet.network,packet.root);
const create=code=>createGeneratedOwnedRustLinks(code,{model:policy.model});
const literal=(proof,kind)=>{const row=proof.literalEdits.find(row=>row.literal.type.kind===kind);assert.ok(row);return row;};
function execute(packet,observations){const emitted=emitGeneratedOwnedRustLinks(packet.network,packet.root,'JavaScript'),context={};vm.runInNewContext(emitted.text,context,{timeout:1000});for(const[text,count,flag,value]of observations)assert.equal(context.choose(text,BigInt(count),flag),value);return emitted;}
function verifyRust(packet){const emitted=emitGeneratedOwnedRustLinks(packet.network,packet.root,'Rust'),restored=reload(create(emitted.text)),value=decoded(restored);assert.equal(value.proof.model,policy.model);assert.deepEqual(emitted.assumptions,emitGeneratedOwnedRust(value.program,value.proof,'Rust').assumptions);return{emitted,restored,...value};}

test('the additive model preserves all original artifacts and real literal identities through text and binary',()=>{
 assert.deepEqual(generatedOwnedRustValueEditFixtures(),fixtures);
 const sharedQuoter=readFileSync(new URL('../../js/src/translation/rust-string-literals.js',import.meta.url),'utf8').split('\n').slice(1).join('\n').replace('export function rustEscape','function rustEscape').replace('export function rustString','function rustString').trimEnd()+'\n\n';assert.equal(createHash('sha256').update(sharedQuoter).digest('hex'),fixtures.quoterSourceSha256);
 for(const entry of fixtures.protectedArtifacts)assert.equal(createHash('sha256').update(readFileSync(new URL(entry.path,import.meta.url))).digest('hex'),entry.sha256);
 for(const entry of original.cases){const packet=reload(create(entry.fullEmittedRust)),{program,proof}=decoded(packet);assert.equal(emitGeneratedOwnedRust(program,proof,'Rust').text,entry.fullEmittedRust);execute(packet,entry.observations);
  for(const row of proof.literalEdits){assert.ok(proof.tokens.some(token=>token.literal===row.literal));assert.ok(row.operationNode.parts.includes(proof.tokens.find(token=>token.literal===row.literal)));if(row.signToken!==null)assert.equal(row.operationNode.parts[0],row.signToken);}
 }
});

test('actual whole modules quote every C0 and DEL value and preserve the new model through canonical cycles',()=>{
 const baseline=reload(create(source));
 for(const entry of fixtures.strings){let{program,proof}=decoded(baseline);literal(proof,'string').literal.value=entry.value;let packet=reload(encodeGeneratedOwnedRustLinks(program,proof));execute(packet,[['input','0',false,entry.value],['Ω😀','1',true,'Ω😀']]);
  for(let cycle=0;cycle<2;cycle++){const value=verifyRust(packet);assert.equal(literal(value.proof,'string').literal.value,entry.value);assert.equal(value.emitted.assumptions.length,11);execute(value.restored,[['input','0',false,entry.value]]);
   const canonical=emitCanonicalGeneratedOwnedRust(value.program,value.proof);assert.equal(canonical.assumptions.length,11);packet=reload(create(canonical.text));assert.deepEqual(canonical.assumptions,emitGeneratedOwnedRustLinks(packet.network,packet.root,'Rust').assumptions);if([...entry.value].some(letter=>letter.codePointAt(0)<32&&![9,10,13].includes(letter.codePointAt(0))))assert.throws(()=>createGeneratedOwnedRust(canonical.text));
  }
 }
});

test('actual signed i32 edits preserve both limits, zero and once-owned sign trivia in complete sources',()=>{
 for(const entry of fixtures.integers){let packet=reload(create(source.replace('0i32',entry.source+'i32'))),{program,proof}=decoded(packet);const row=literal(proof,'fixed');assert.equal(row.signToken!==null,entry.source.startsWith('-'));row.literal.value=entry.value;
  packet=reload(encodeGeneratedOwnedRustLinks(program,proof));execute(packet,[['input',entry.value,false,'zero'],['other',entry.value==='17'?'18':'17',false,'other']]);const value=verifyRust(packet);assert.equal(literal(value.proof,'fixed').literal.value,entry.value);execute(value.restored,[['input',entry.value,false,'zero']]);
 }
 for(const entry of fixtures.signContexts){const value=decoded(reload(create(source.replace('if (n == (0i32))',entry.source))));literal(value.proof,'fixed').literal.value=entry.value;const packet=reload(encodeGeneratedOwnedRustLinks(value.program,value.proof)),result=verifyRust(packet);assert.ok(result.emitted.text.includes(entry.retained));execute(result.restored,[['input',entry.value,false,'zero']]);}
});

test('complete production cooked tokens preserve original escapes and refuse malformed literal syntax',()=>{
 for(const raw of['"\\0'+'123"','"\\x7f"','"\\u{0}\\u{8}\\u{c}"','"\\u{1f600}"','"\\u{1_f600}"']){const code=source.replace('"zero"',raw),packet=reload(create(code));assert.equal(emitGeneratedOwnedRustLinks(packet.network,packet.root,'Rust').text,code);verifyRust(packet);}
 for(const spelling of['00i32','-00i32','-2147483648i32']){const code=source.replace('0i32',spelling),packet=reload(create(code));assert.equal(emitGeneratedOwnedRustLinks(packet.network,packet.root,'Rust').text,code);}
 for(const raw of['"\\b"','"\\f"','"\\v"','"\\{"','"\\x80"','"\\x0"','"\\u{d800}"','"\\u{110000}"'])assert.throws(()=>create(source.replace('"zero"',raw)));
 for(const spelling of['--1i32','+1i32','1u32','1.0i32','2147483648i32','-2147483649i32'])assert.throws(()=>create(source.replace('0i32',spelling)));
});

test('source-free malformed scalar, type, sign, CST and literal-owner edits refuse before either target',()=>{
 const mutations=[
  value=>{literal(value.proof,'fixed').literal.value='-0';},value=>{literal(value.proof,'fixed').literal.value=1;},value=>{literal(value.proof,'fixed').literal.value=true;},value=>{literal(value.proof,'fixed').literal.value=1n;},value=>{literal(value.proof,'fixed').literal.value='2147483648';},value=>{literal(value.proof,'fixed').literal.value='-2147483649';},
  value=>{literal(value.proof,'string').literal.value='\ud800';},value=>{literal(value.proof,'string').literal.value=new String('text');},
  value=>{literal(value.proof,'fixed').literal.type={kind:'fixed',bits:32,signed:false};},value=>{literal(value.proof,'fixed').signToken=null;},value=>{literal(value.proof,'fixed').signToken={...literal(value.proof,'fixed').signToken};},
  value=>{literal(value.proof,'fixed').signToken=literal(value.proof,'string').operationNode.parts[0];},value=>{literal(value.proof,'fixed').operationNode=literal(value.proof,'string').operationNode;},value=>{literal(value.proof,'fixed').literal=value.program.declarations.get('seed').params[1];},
  value=>{value.proof.literalEdits.pop();},value=>{literal(value.proof,'fixed').extra=true;},value=>{literal(value.proof,'fixed').signToken.raw='+';},value=>{literal(value.proof,'fixed').signToken.span.start++;},
 ];
 const baseline=reload(create(source.replace('0i32','-1i32')));
 for(const mutate of mutations){const value=decoded(baseline);mutate(value);for(const check of[()=>checkGeneratedOwnedRust(value.program,value.proof),()=>encodeGeneratedOwnedRustLinks(value.program,value.proof),()=>emitGeneratedOwnedRust(value.program,value.proof,'Rust'),()=>emitGeneratedOwnedRust(value.program,value.proof,'JavaScript')])assert.throws(check);}
});

test('saved text and binary graph edits cannot replace an actual sign, literal or operation owner',()=>{
 const packet=create(source.replace('0i32','-1i32')),label=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
 const field=(network,parent,name)=>{const row=network.fieldRelations().find(row=>row.parent.equals(parent)&&row.label===label(name));assert.ok(row,name);return row;};
 for(const binary of[false,true])for(const role of['signToken','literal','operationNode']){const saved=reload(packet),network=saved.network,provenance=field(network,saved.root,'provenance').child,proof=field(network,provenance,'sourceInterpretation').child,edits=network.link(field(network,proof,'literalEdits').child).references();assert.equal(edits.length,2);const row=field(network,edits[0],role),other=field(network,edits[1],role);assert.ok(!row.child.equals(other.child));row.link.setReferences([row.parent,row.link.references()[1],other.child]);const reloaded=(binary?reloadBinary:reloadText)(saved);for(const check of[()=>decoded(reloaded),()=>emitGeneratedOwnedRustLinks(reloaded.network,reloaded.root,'Rust'),()=>emitGeneratedOwnedRustLinks(reloaded.network,reloaded.root,'JavaScript')])assert.throws(check);}
});

test('the previous interpretation retains exact refusals and the new decision kernels replay as checked Links',()=>{
 for(const[name,code,kind,value]of[['control',source,'string','\0'],['sign',source.replace('0i32','-1i32'),'fixed','1']]){const original=createGeneratedOwnedRust(code),token=original.proof.tokens.find(token=>token.literal?.type.kind===kind);token.literal.value=value;assert.throws(()=>checkGeneratedOwnedRust(original.program,original.proof),error=>{assert.equal(error.reason,fixtures.originalRefusals[name].reason);assert.deepEqual(error.span,fixtures.originalRefusals[name].span);return true;});}
 const checked=checkProgram(parseJavaScript(generatedOwnedRustValueEditRules())),packet=encodeTypedProgramLinks(checked),restored=decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(LinkNetwork.fromLino(packet.network.toLino()))),packet.root),emitted=emitJavaScript(restored),context={};vm.runInNewContext(emitted.text,context);
 const rule=name=>context[emitted.mappings.find(row=>row.source===name).target];assert.equal(rule('acceptGeneratedOwnedRustValueEditModel')(policy.model),true);assert.equal(rule('acceptGeneratedOwnedRustValueEditModel')(policy.baseModel),false);
 for(const kind of['string','fixed','bool','unknown'])for(const operation of[policy.string.operation,policy.integer.operation,'unknown'])assert.equal(rule('acceptGeneratedOwnedRustLiteralEditOwner')(kind,operation),kind==='string'&&operation===policy.string.operation||kind==='fixed'&&operation===policy.integer.operation);
 for(const negative of[false,true])for(const owned of[false,true])assert.equal(rule('acceptGeneratedOwnedRustSignOwner')(negative,owned),negative===owned);
});
