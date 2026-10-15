import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkNetwork} from '../../js/src/index.js';
import {parseRust} from '../../js/src/translation/rust.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {generatedOwnedRustFixture} from './generate-fixtures.mjs';
import {generatedOwnedRustRules} from './generate-rules.mjs';
import {createGeneratedOwnedRust,createGeneratedOwnedRustLinks,decodeGeneratedOwnedRustLinks,encodeGeneratedOwnedRustLinks,checkGeneratedOwnedRust,emitGeneratedOwnedRust,emitGeneratedOwnedRustLinks,emitCanonicalGeneratedOwnedRust,GENERATED_OWNED_RUST_POLICY as policy} from './model.mjs';
const fixture=readProjectData(new URL('./data/fixtures.lino',import.meta.url));
const source=index=>fixture.cases[index].fullEmittedRust;
const reload=packet=>({...packet,network:LinkNetwork.fromLino(packet.network.toLino())});
const chosen=packet=>decodeGeneratedOwnedRustLinks(packet.network,packet.root);
const caller=program=>program.declarations.get('choose');
function call(entry){let node=entry.body;while(node.k==='let'){if(node.value.k==='call')return node.value;node=node.body;}throw new Error('the actual initializer call exists');}
function execute(emitted,observations,name='choose'){
 const context={};vm.runInNewContext(emitted.text,context,{timeout:1000});
 for(const[text,count,flag,value]of observations)assert.equal(context[name](text,BigInt(count),flag),value);
 return context;
}
function coherentTarget(program,proof,target){call(caller(program)).fn=target;const token=proof.tokens.find(token=>token.declaration?.name==='seed'&&token.raw==='seed'&&token.span.start>caller(program).span.start);assert.ok(token);token.declaration=program.declarations.get(target);}

test('all five entire actual artifacts preserve receipts, every code unit and complete checked values through three code cycles',()=>{
 assert.equal(fixture.cases.length,5);assert.deepEqual(generatedOwnedRustFixture(),fixture);
 assert.equal(fixture.cases[0].bytes,1419);assert.equal(fixture.cases[0].sha256,'be586151e33dbc40e596acdcf3205a094601bcfc11190546947b9c85090dc7d1');
 for(const entry of fixture.cases){let code=entry.fullEmittedRust;
  assert.equal(Buffer.byteLength(code),entry.bytes);assert.equal(createHash('sha256').update(code).digest('hex'),entry.sha256);
  for(let cycle=0;cycle<3;cycle++){
   const packet=reload(createGeneratedOwnedRustLinks(code)),{program,proof}=chosen(packet);
   assert.equal(emitGeneratedOwnedRustLinks(packet.network,packet.root,'Rust').text,code);
   assert.equal(proof.assumptions.length,4);assert.equal(proof.tokens.at(-1).span.end,code.length);assert.equal(proof.clones.length,entry.clones);
   assert.ok(packet.network.links().every(link=>link.metadata().definition!==code));
   const js=emitGeneratedOwnedRustLinks(packet.network,packet.root,'JavaScript');execute(js,entry.observations);assert.equal(js.assumptions.length,4+proof.valueAssumptions.length+1);
   assert.ok(proof.clones.every(row=>row.receiver.type.kind==='string'&&row.owner.join('::')==='std::string::String'));
   code=emitCanonicalGeneratedOwnedRust(program,proof).text;
  }
 }
});

test('actual standard String domain guards preserve astral input and reject unpaired UTF16 and nonprimitive interfaces',()=>{
 const packet=reload(createGeneratedOwnedRustLinks(source(0))),{program,proof}=chosen(packet),emitted=emitGeneratedOwnedRust(program,proof,'JavaScript');
 const context=execute(emitted,[['Ω😀','1',false,'Ω😀'],['\u0000\n😀','1',true,'\u0000\n😀']]);
 for(const text of['\ud800','\udfff','x\ud800y',new String('x'),1,null,{}])assert.throws(()=>context.choose(text,1n,false),/argument domain/u);
 for(const count of[1,1.5,NaN,-2147483649n,2147483648n])assert.throws(()=>context.choose('x',count,false),/argument domain/u);
 for(const flag of[0,1,'false',null,{}])assert.throws(()=>context.choose('x',1n,flag),/argument domain/u);
 assert.equal(emitted.assumptions.at(-1).id,policy.inputAssumption.id);assert.match(policy.valueAssumption.meaning,/allocation succeeds/u);
});

test('source-free edits retarget actual declaration links, rename actual owners and change actual owned literal values',()=>{
 let packet=reload(createGeneratedOwnedRustLinks(source(3)));let{program,proof}=chosen(packet);
 coherentTarget(program,proof,'alternate');packet=reload(encodeGeneratedOwnedRustLinks(program,proof));({program,proof}=chosen(packet));
 const callNode=call(caller(program));assert.equal(callNode.fn,'alternate');
 const rows=packet.network.fieldRelations(),semantic=rows.find(row=>row.label===schema.references.declaration);assert.ok(semantic);assert.equal(packet.network.link(rows.find(row=>row.parent.equals(semantic.child)&&row.label==='name').child).metadata().definition,'alternate');
 execute(emitGeneratedOwnedRust(program,proof,'JavaScript'),[['before','1',false,'second'],['before','1',true,'selected']]);
 const rewritten=emitGeneratedOwnedRust(program,proof,'Rust').text;assert.match(rewritten,/crate::alternate\(text\.clone\(\), count\)/u);const rewrittenPacket=reload(createGeneratedOwnedRustLinks(rewritten));execute(emitGeneratedOwnedRustLinks(rewrittenPacket.network,rewrittenPacket.root,'JavaScript'),[['before','1',false,'second']]);
 packet=reload(createGeneratedOwnedRustLinks(source(0)));({program,proof}=chosen(packet));
 const seed=program.declarations.get('seed');seed.name='seed_changed';seed.fullName='seed_changed';program.declarations=new Map([...program.declarations].map(([name,value])=>[value===seed?seed.fullName:name,value]));call(caller(program)).fn=seed.fullName;
 const literal=proof.tokens.find(token=>token.literal?.type.kind==='string');assert.ok(literal);literal.literal.value='edited Ω😀';
 packet=reload(encodeGeneratedOwnedRustLinks(program,proof));({program,proof}=chosen(packet));
 execute(emitGeneratedOwnedRust(program,proof,'JavaScript'),[['before','0',false,'edited Ω😀'],['before','1',true,'before']]);
 const changed=emitGeneratedOwnedRust(program,proof,'Rust').text;assert.match(changed,/pub fn seed_changed/u);assert.match(changed,/crate::seed_changed/u);
 const changedPacket=reload(createGeneratedOwnedRustLinks(changed));const emitted=emitGeneratedOwnedRustLinks(changedPacket.network,changedPacket.root,'JavaScript');execute(emitted,[['before','0',false,'edited Ω😀']]);
});

test('one-sided identity, clone-owner, literal-domain, type, scalar-domain and runtime-control edits refuse',()=>{
 const mutations=[
  ({program})=>{call(caller(program)).fn='alternate';},
  ({program,proof})=>{const token=proof.tokens.find(token=>token.declaration?.name==='seed'&&token.span.start>caller(program).span.start);token.declaration=program.declarations.get('alternate');},
  ({program,proof})=>{proof.clones[0].receiver=caller(program).params[0];},
  ({proof})=>{proof.clones[0].owner=['custom','String'];},
  ({proof})=>{proof.clones[0].method='to_owned';},
  ({proof})=>{proof.clones.pop();},
  ({proof})=>{proof.valueAssumptions=[];},
  ({proof})=>{proof.assumptions.pop();},
  ({proof})=>{proof.assumptions[0].operationNode=proof.assumptions[1].operationNode;},
  ({program})=>{program.utf16Strings=true;},
  ({program})=>{program.declarations.get('seed').sourceSignature.result.type.elements[1].bits='32';},
  ({proof})=>{proof.tokens.find(token=>token.literal?.type.kind==='string').literal.value='\ud800';},
  ({proof})=>{proof.tokens.find(token=>token.literal?.type.kind==='string').literal.type={kind:'float'};},
  ({program})=>{let node=caller(program).body;while(node.k==='let'){if(node.value.k==='if'){node.value.cond.domain=false;return;}node=node.body;}throw new Error('condition exists');},
 ];
 for(const mutate of mutations){const value=chosen(reload(createGeneratedOwnedRustLinks(source(3))));mutate(value);assert.throws(()=>checkGeneratedOwnedRust(value.program,value.proof));assert.throws(()=>encodeGeneratedOwnedRustLinks(value.program,value.proof));}
});

test('the full lexer-owned CST refuses active code hidden as trivia, swallowed tokens and copied ownership records',()=>{
 const mutations=[
  value=>{value.proof.gaps[0].units[0]=32;value.proof.gaps[0].units[1]=32;},
  value=>{const gap=value.proof.gaps.find(gap=>gap.units.length>8&&gap.units.every(unit=>unit===32||unit===10));const start=gap.units.indexOf(32);gap.units[start]=47;gap.units[start+1]=47;},
  value=>{value.proof.tokens[0].raw='@';},
  value=>{value.proof.operations.parts[1].parts[2]={...value.proof.operations.parts[1].parts[2]};},
  value=>{value.proof.tokens.find(token=>token.declaration).declaration=structuredClone(value.program.items[0]);},
  value=>{value.proof.gaps[0].units[0]=false;},
  value=>{value.proof.gaps[0].units[0]=String(value.proof.gaps[0].units[0]);},
  value=>{value.proof.span.start='0';},
  value=>{value.proof.tokens.find(token=>token.kind==='identifier').suffix='i32';},
  value=>{value.proof.operations.parts.push(value.proof.operations);},
  value=>{value.proof.tokens[0].span.start=-0;},
  value=>{Object.defineProperty(value.proof,'hidden',{value:true});},
 ];
 for(const mutate of mutations){const value=createGeneratedOwnedRust(source(0));mutate(value);assert.throws(()=>checkGeneratedOwnedRust(value.program,value.proof));}
});

test('whole-source controls reject borrowed values, foreign methods, changed thread outcomes and extra active definitions',()=>{
 const mutations=[code=>code.replace('text.clone()','text.to_owned()'),code=>code.replace('text.clone()','&text'),code=>code.replace('std::thread::Builder','foreign::thread::Builder'),code=>code.replace('.stack_size(1 << 28)','.stack_size(1 << 29)'),code=>code.replace('std::process::exit(101)','std::process::exit(0)'),code=>code.replace('crate::seed','foreign::seed'),code=>code+'\nfn extra(){}',code=>'// all operations are proven equivalent\n'+code.replace('text.clone()','text.foreign()')];
 for(const mutate of mutations)assert.throws(()=>createGeneratedOwnedRustLinks(mutate(source(0))));
 for(const escape of['\\b','\\f','\\v','\\{'])assert.throws(()=>createGeneratedOwnedRustLinks(source(0).replace('"zero"','"'+escape+'"')));
 assert.throws(()=>createGeneratedOwnedRustLinks(source(0).replace('n == (0i32)','flag == true == false')),/comparison requires/u);
 for(const expression of['flag == (true == false)','(flag == true) == false']){const packet=reload(createGeneratedOwnedRustLinks(source(0).replace('n == (0i32)',expression)));execute(emitGeneratedOwnedRustLinks(packet.network,packet.root,'JavaScript'),[['x','1',false,'zero'],['x','1',true,'x']]);}
 const trivia=source(0)+'\n/* harmless owned trivia */\n';const packet=reload(createGeneratedOwnedRustLinks(trivia));assert.equal(emitGeneratedOwnedRustLinks(packet.network,packet.root,'Rust').text,trivia);
 for(const trivia of['\u00a0','\ufeff'])assert.throws(()=>createGeneratedOwnedRustLinks(source(0).replace('pub fn','pub'+trivia+'fn')));
 for(const comment of['/// doc\n','//! doc\n','/*! doc */','/** doc */'])assert.throws(()=>createGeneratedOwnedRustLinks(source(0).replace('} else {','} '+comment+' else {')),/doc comment/u);
 for(const comment of['//// ordinary\n','/**/','/***/','/* outer /** nested doc-looking */ outer */','/* Ω😀 \u00a0\ufeff */']){const code=source(0).replace('} else {','} '+comment+' else {'),packet=reload(createGeneratedOwnedRustLinks(code));assert.equal(emitGeneratedOwnedRustLinks(packet.network,packet.root,'Rust').text,code);}
 for(const unit of[9,10,11,12,13,32,133,8206,8207,8232,8233]){const code=source(0).replace('pub fn','pub'+String.fromCharCode(unit)+'fn'),packet=reload(createGeneratedOwnedRustLinks(code));assert.equal(emitGeneratedOwnedRustLinks(packet.network,packet.root,'Rust').text,code);}
 const initialBom='\ufeff'+source(0),bomPacket=reload(createGeneratedOwnedRustLinks(initialBom));assert.equal(emitGeneratedOwnedRustLinks(bomPacket.network,bomPacket.root,'Rust').text,initialBom);
 for(const code of[source(0).replace('"zero"','"\\u{7a}ero"'),source(0).replace('0i32','00i32')]){const packet=reload(createGeneratedOwnedRustLinks(code));assert.equal(emitGeneratedOwnedRustLinks(packet.network,packet.root,'Rust').text,code);}
});

test('saved raw Links preserve actual target identity and refuse forged clone owners and scalar roles',()=>{
 const original=createGeneratedOwnedRustLinks(source(3));
 const label=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
 function field(network,parent,name){const row=network.fieldRelations().find(row=>row.parent.equals(parent)&&row.label===label(name));assert.ok(row,name);return row;}
 function change(row,target){row.link.setReferences([row.parent,row.link.references()[1],target]);}
 for(let edit=0;edit<8;edit++){
  const packet=reload(edit===7?createGeneratedOwnedRustLinks(source(3).replace('pub fn','/*a*/pub fn')):original),network=packet.network,body=network.link(packet.root).references()[0],mapping=field(network,body,'declarations').child;
  const declarations=new Map(network.link(mapping).references().map(id=>{const [key,value]=network.link(id).references();return[network.link(key).metadata().definition,value];}));
  const provenance=field(network,packet.root,'provenance').child,proof=field(network,provenance,'sourceInterpretation').child;
  if(edit===0){const row=network.fieldRelations().find(row=>row.label===schema.references.declaration);assert.ok(row.child.equals(declarations.get('seed')));change(row,declarations.get('alternate'));}
  else if(edit===1){const row=network.fieldRelations().filter(row=>row.label===label('declaration')&&row.child.equals(declarations.get('seed'))).at(-1);assert.ok(row);change(row,declarations.get('alternate'));}
  else if(edit===2){const clones=field(network,proof,'clones').child,clone=network.link(clones).references()[0],owner=field(network,clone,'owner').child,parts=network.link(owner).references(),replacement=network.insertLink([],network.link(parts[0]).metadata().withDefinition('custom'));network.link(owner).setReferences([replacement,...parts.slice(1)]);}
  else if(edit===3){const span=field(network,proof,'span').child,row=field(network,span,'start'),replacement=network.insertLink([],network.link(row.child).metadata().withTerm(schema.scalars.string).withDefinition('0'));change(row,replacement);}
  else if(edit===4){const gaps=field(network,proof,'gaps').child,gap=network.link(gaps).references()[0],units=field(network,gap,'units').child,parts=network.link(units).references(),replacement=network.insertLink([],network.link(parts[0]).metadata().withTerm(schema.scalars.string));network.link(units).setReferences([replacement,...parts.slice(1)]);}
  else{
   const gaps=network.link(field(network,proof,'gaps').child).references();let changed=false;
   for(const gap of gaps){const units=field(network,gap,'units').child,parts=network.link(units).references(),values=parts.map(id=>Number(network.link(id).metadata().definition));const at=edit===7?values.findIndex((value,index)=>value===97&&values[index-1]===42&&values[index-2]===47&&values[index+1]===42&&values[index+2]===47):values.length===1&&values[0]===32?0:-1;if(at<0)continue;const replacement=network.insertLink([],network.link(parts[at]).metadata().withDefinition(String(edit===5?160:edit===6?65279:33)));parts[at]=replacement;network.link(units).setReferences(parts);changed=true;break;}
   assert.ok(changed,'an actual saved trivia code unit was changed');
  }
  const restored=LinkNetwork.fromLino(network.toLino());assert.throws(()=>decodeGeneratedOwnedRustLinks(restored,packet.root));assert.throws(()=>emitGeneratedOwnedRustLinks(restored,packet.root,'JavaScript'));
 }
});

test('all old exact whole-artifact refusals and original paired fixtures remain unchanged',()=>{
 const original=readProjectData(new URL('../owned-tuple-call-fixture.lino',import.meta.url));
 for(const entry of fixture.cases)for(const reader of entry.refusals){const options=reader.model==='default'?{}:reader.model===policy.runtimeModel?{runtimeShellModel:reader.model}:{localValueStateModel:reader.model};assert.throws(()=>checkProgram(parseRust(entry.fullEmittedRust,options)),error=>{assert.equal(error.kind,reader.kind);assert.equal(error.reason,reader.reason);assert.deepEqual(error.span??null,reader.span);return true;});}
 assert.equal(original.emittedArtifact.sha256,fixture.cases[0].sha256);assert.equal(original.authentic.combined.bytes,132002);
 assert.equal(readFileSync(new URL('./rules.mjs',import.meta.url),'utf8'),generatedOwnedRustRules());
 const checked=checkProgram(parseJavaScript(generatedOwnedRustRules())),packet=encodeTypedProgramLinks(checked),actual=decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()),packet.root),emitted=emitJavaScript(actual),context={};vm.runInNewContext(emitted.text,context);
 const accept=context[emitted.mappings.find(row=>row.source==='acceptGeneratedOwnedRustClone').target];for(const standard of[false,true])for(const string of[false,true])for(const receiver of[false,true])for(const arity of[-1,0,1])assert.equal(accept(standard,string,receiver,arity),standard&&string&&receiver&&arity===0);
 const rule=name=>context[emitted.mappings.find(row=>row.source===name).target];
 assert.equal(rule('acceptGeneratedOwnedRustModel')(policy.model),true);for(const model of['','unknown',policy.ownedCopyModel])assert.equal(rule('acceptGeneratedOwnedRustModel')(model),false);
 for(const value of[-1,0,1.5,9,65535,65536,NaN,Infinity])for(const negative of[false,true])assert.equal(rule('acceptGeneratedOwnedRustCodeUnit')(value,negative),!negative&&Number.isInteger(value)&&value>=0&&value<=65535);
 for(const value of[0,8,9,10,11,13,31,32,127,0x1f600])assert.equal(rule('acceptGeneratedOwnedRustLiteralCodePoint')(value),value>=32||[9,10,13].includes(value));
 for(const comparison of[false,true])for(const grouped of[false,true])assert.equal(rule('acceptGeneratedOwnedRustEqualityGrouping')(comparison,grouped),!comparison||grouped);
 for(const line of[false,true])for(const third of[-1,33,42,47,97])for(const fourth of[-1,33,42,47,97])assert.equal(rule('acceptGeneratedOwnedRustOrdinaryComment')(line,third,fourth),third!==33&&(line?third!==47||fourth===47:third!==42||fourth===42||fourth===47));
});
