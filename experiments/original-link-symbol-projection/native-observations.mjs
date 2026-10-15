// Observe the complete unchanged generated artifact; planning never starts a native process.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import vm from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary,readStructuredDataLino} from '../../js/src/index.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {tokenize} from '../../js/src/translation/lexer.js';
import {createSourceNativeObservationRunner} from '../source-native-observations.mjs';
import {originalLinkSymbolPolicy} from './policy.mjs';
import {projectOriginalLinkSymbolValues} from './rules.mjs';

export const ORIGINAL_LINK_SYMBOL_MODEL_COMMIT='ad70aa52e2a0ad5755d2a90ed0bf04cb784f5e2d';
const digests={
 'policy.mjs':'b88a1c3b2f51d2f28411ab43559b22cae585267192d70790a54457bbc21c4428',
 'rules.mjs':'dea761345d761f6d58ecb564893134ccdc149b19eaf89d51ef7dcbe78e4a9448',
 'generate.mjs':'fe0f6221edc8fd9d9c6215a2fd97e0dd677052f32d6fdd3c2e368fc1b926d100',
 'data/policy.lino':'ece8ece9be52b5917944ebd53d604d956ad4e973880ee7f719e8b5c620db152f',
 'data/native-definitions.lino':'c7b2c9340cff7f1d54a2a9852be022385b1f6783bc5dc900346290b778df99a0',
};
const checkedPlans=new WeakSet(),sha256=value=>createHash('sha256').update(value).digest('hex');
function textBytes(value){if(typeof value!=='string'||!value.isWellFormed())throw new TypeError('the oracle source must be primitive Unicode scalar text');return Buffer.from(value,'utf8');}
function freeze(value){if(value&&typeof value==='object'){Object.values(value).forEach(freeze);Object.freeze(value);}return value;}
function numberBits(value){assert.equal(typeof value,'number');const bytes=Buffer.alloc(8);bytes.writeDoubleBE(value);return bytes.toString('hex');}
function bitsNumber(value){assert.match(value,/^[0-9a-f]{16}$/u);return Buffer.from(value,'hex').readDoubleBE();}

export function verifyOriginalLinkSymbolNativeModel(read=readFileSync){
 return Object.entries(digests).map(([file,expected])=>{const actual=sha256(read(new URL(file,import.meta.url)));assert.equal(actual,expected,'the frozen symbol projection source changed: '+file);return{file,sha256:actual};});
}
export function originalLinkSymbolNativeMode(environment=process.env){
 if(environment.ORIGINAL_LINK_SYMBOL_NATIVE!=='1')return false;
 if(environment.CI!=='true')throw new TypeError('ORIGINAL_LINK_SYMBOL_NATIVE=1 requires CI=true; native compilation is forbidden locally');return true;
}
function nativeInterface(source){
 const tokens=tokenize(source,'Rust').tokens,found=[];let depth=0;
 for(let index=0;index<tokens.length;index++){
  if(depth===0&&tokens[index].value==='pub'&&tokens[index+1]?.value==='fn'&&tokens[index+2]?.value==='project_original_link_symbol_values')found.push(index);
  if(tokens[index].kind==='punct'&&tokens[index].value==='{')depth++;if(tokens[index].kind==='punct'&&tokens[index].value==='}')depth--;
 }
 assert.equal(depth,0);assert.equal(found.length,1,'the actual kernel exports one complete symbol projection function');
 const parameterOrder=['fixed_starts','fixed_lengths','fixed_units','source_starts','source_lengths','source_units'],vector=['Vec','<','f64','>'];
 const expected=['pub','fn','project_original_link_symbol_values','('];
 for(const[index,name]of parameterOrder.entries()){if(index)expected.push(',');expected.push(name,':',...vector);}expected.push(')','->',...vector,'{');
 assert.deepEqual(tokens.slice(found[0],found[0]+expected.length).map(token=>token.value),expected,'the actual generated value-vector ABI must remain exact');
 return{functionName:'project_original_link_symbol_values',parameterOrder,parameterModes:Array(6).fill('Vec<f64>'),resultMode:'Vec<f64>',referenceSemantics:false,interfaceSha256:sha256(expected.join('\0')),tokenCount:tokens.length};
}
// Independent host specification: ordinary String equality and Map insertion order.
function transport(values){
 const starts=[],lengths=[],units=[];
 for(const value of values){starts.push(units.length);lengths.push(typeof value==='string'?value.length:-1);if(typeof value==='string')for(let at=0;at<value.length;at++)units.push(value.charCodeAt(at));}
 return[starts,lengths,units];
}
function reference(fixed,source){
 const strings=[null,...fixed],indices=new Map(fixed.map((value,index)=>[value,index+1]));
 const symbols=source.map(value=>{if(typeof value!=='string')return-1;if(!indices.has(value)){indices.set(value,strings.length);strings.push(value);}return indices.get(value);});
 const names=[],units=[];
 for(const value of strings){names.push(units.length,value===null?0:value.length);if(value!==null){for(let at=0;at<value.length;at++)units.push(value.charCodeAt(at));units.push(0);}}
 return[1,source.length,strings.length,units.length,...symbols,...names,...units];
}
function cases(){
 const rows=[],argsFor=(fixed,source)=>[...transport(fixed),...transport(source)];
 const add=(id,args,expected,kind='complete ordered String value projection')=>rows.push({id,args,expected,kind});
 const exact=(id,fixed,source)=>add(id,argsFor(fixed,source),reference(fixed,source));
 exact('empty',[],[]);exact('absent non-String values',[],[null,undefined,false,0]);
 exact('empty Strings retain positive symbols',[],['','','alpha','beta','alpha']);
 exact('fixed dictionary precedes dynamic order',['','Rust','String'],['String','dynamic','Rust','dynamic',null,'']);
 exact('prefix values remain distinct',['prefix','pre'],['pre','prefix','prefixes','prefixes']);
 exact('NUL pairs and isolated UTF16 units',['','\0','a\0b','Ω😀'],['\0','\0\0','a\0','a\0b','\ud800','\udc00','😀','Ω😀','\ud800']);
 exact('normalization remains distinct',[],['é','e\u0301','é','e\u0301']);
 exact('null is distinct from its spelling',['null'],[null,'null','',undefined,'null']);
 const edgeUnits=[0,1,127,128,255,256,55295,55296,56319,57344,65535];
 exact('numeric UTF16 edges',[],edgeUnits.map(value=>String.fromCharCode(value)));
 const dictionary=readProjectData(new URL('../rust-text-certificate/data/policy.lino',import.meta.url)).symbols;assert.equal(dictionary.length,107);
 exact('actual fixed107 dictionary',dictionary,[dictionary[106],'native dynamic value',dictionary[0],'native dynamic value',null,'']);
 const base=()=>argsFor(['base'],['query',null]);
 for(const[label,value]of[['negative zero',-0],['negative',-2],['fraction',0.5],['NaN',NaN],['positive infinity',Infinity],['negative infinity',-Infinity],['above uint16',65536],['above declared bound',100001],['subnormal',Number.MIN_VALUE],['maximum Number',Number.MAX_VALUE]]){
  for(const slot of[0,1,2,3,4,5]){const args=base();args[slot][0]=value;add(label+' vector '+slot,args,slot<3?[-1]:[-2],'invalid numeric transport');}
 }
 for(const slot of[0,1,3,4]){const args=base();args[slot].pop();add('vector length mismatch '+slot,args,slot<3?[-1]:[-2],'malformed transport');}
 for(const slot of[2,5]){const args=base();args[slot].push(0);add('unconsumed unit '+slot,args,slot===2?[-1]:[-2],'malformed transport');}
 for(const slot of[0,3]){const args=base();args[slot][0]=1;add('nonconsecutive offset '+slot,args,slot===0?[-1]:[-2],'malformed transport');}
 add('absent fixed String',argsFor([null],[]),[-1],'fixed absent refusal');
 add('duplicate fixed String',argsFor(['same','other','same'],[]),[-3,2],'duplicate dictionary refusal');
 add('duplicate fixed empty String',argsFor(['',''],['other']),[-3,1],'duplicate dictionary refusal');
 const sourceFirst=argsFor(['same','same'],['query']);sourceFirst[5][0]=NaN;add('source invalid before duplicate fixed',sourceFirst,[-2],'refusal precedence');
 const fixedFirst=structuredClone(sourceFirst);fixedFirst[2][0]=NaN;add('fixed invalid before source invalid',fixedFirst,[-1],'refusal precedence');
 const absent=argsFor([],['',null]);absent[4][1]=-2;add('only minus one means absent',absent,[-2],'absent marker refusal');
 assert.equal(rows.length,84);return rows;
}
function consumerSource(observations,header){
 const blocks=observations.map((row,index)=>{
  const inputs=row.inputBits.map((bits,slot)=>`let input_${slot}:Vec<f64>=vec![${bits.map(value=>'f64::from_bits(0x'+value+'u64)').join(',')}];`);
  const checks=row.inputBits.map((bits,slot)=>`let expected_${slot}:Vec<u64>=vec![${bits.map(value=>'0x'+value+'u64').join(',')}];\nassert_eq!(input_${slot}.iter().map(|value|value.to_bits()).collect::<Vec<_>>(),expected_${slot},"original input vector ${slot} changed");`);
  return`{\n${inputs.join('\n')}\nlet observed=oracle_source::${header.functionName}(${row.inputBits.map((_,slot)=>'input_'+slot+'.clone()').join(',')});\n${checks.join('\n')}\nlet bits=observed.iter().map(|value|value.to_bits()).collect::<Vec<_>>();\nlet expected:Vec<u64>=vec![${row.resultBits.map(value=>'0x'+value+'u64').join(',')}];\nassert_eq!(bits,expected,"actual complete symbol projection differs from JavaScript");\nwrite!(&mut line,"${index}").expect("observation index");\nfor value in bits{write!(&mut line,":{value:016x}").expect("projection bits");}\nline.push('\\n');\n}`;
 });
 return`extern crate oracle_source;\nuse std::fmt::Write as _;\nfn main(){let mut line=String::new();\n${blocks.join('\n')}\nprint!("{line}");}\n`;
}
/** Check the complete original source and JavaScript, without a native process. */
export function planOriginalLinkSymbolNative(){
 const modelSources=verifyOriginalLinkSymbolNativeModel(),source=readFileSync(new URL('./rules.mjs',import.meta.url),'utf8'),artifact=readProjectData(new URL('./data/native-definitions.lino',import.meta.url));
 assert.deepEqual(readProjectData(new URL('./data/policy.lino',import.meta.url)),originalLinkSymbolPolicy);assert.equal(originalLinkSymbolPolicy.translation.referenceSemantics,false);
 const saved=LinkNetwork.fromLino(readFileSync(new URL('./data/native-definitions.lino',import.meta.url),'utf8'));
 assert.deepEqual(readStructuredDataLino(saved.toLino()),artifact);assert.deepEqual(readStructuredDataLino(decodeLinkNetworkBinary(encodeLinkNetworkBinary(saved)).toLino()),artifact);
 const program=checkProgram(parseJavaScript(source,originalLinkSymbolPolicy.translation)),rust=emitRust(program),javascript=emitJavaScript(program),context={};vm.runInNewContext(javascript.text,context,{timeout:2000});
 assert.equal(sha256(source),artifact.sourceSha256);assert.deepEqual(rust.definitions,artifact.declarations);assert.equal(rust.text,artifact.completeRust);assert.equal(rust.definitions.length,22);assert.equal(program.items.length,22);assert.deepEqual(program.items.map(row=>row.fullName),artifact.declarationNames);
 const callee=program.declarations.get('projectOriginalLinkSymbolValues'),vector={kind:'array',element:{kind:'float'}};
 assert.deepEqual(callee.params.map(row=>row.name),['fixedStarts','fixedLengths','fixedUnits','sourceStarts','sourceLengths','sourceUnits']);for(const row of callee.params){assert.deepEqual(row.type,vector);assert.equal(row.guard,null);assert.equal(row.default,null);}assert.deepEqual(callee.ret,vector);
 const target=javascript.mappings.find(row=>row.kind==='function'&&row.source===callee.fullName)?.target;assert.equal(typeof context[target],'function');
 const observations=cases().map(row=>{
  const inputBits=row.args.map(values=>values.map(numberBits)),direct=projectOriginalLinkSymbolValues(...row.args),observed=Array.from(context[target](...row.args));
  assert.deepEqual(direct,row.expected,row.id);assert.deepEqual(observed,row.expected,row.id);assert.deepEqual(row.args.map(values=>values.map(numberBits)),inputBits,'symbol projection preserves every original input bit');
  return{id:row.id,kind:row.kind,inputBits,resultBits:observed.map(numberBits),decision:observed};
 });
 const native=rust.text;assert.equal(textBytes(native).length,28991);assert.equal(sha256(native),'8e59fc8954cf258a7707984590f00ee7db1598365eb4a99702b5fb4d2b6a662b');
 const archive=LinkNetwork.parse(native,'Rust'),textReplay=LinkNetwork.fromLino(archive.toLino()).reconstructText(),binaryReplay=decodeLinkNetworkBinary(encodeLinkNetworkBinary(archive)).reconstructText();assert.deepEqual(textBytes(textReplay),textBytes(native));assert.deepEqual(textBytes(binaryReplay),textBytes(native));
 const header=nativeInterface(native),consumer=consumerSource(observations,header),sourceSha256=sha256(native),declarations=program.items.map((row,index)=>({kind:row.k,name:row.fullName,sha256:sha256(artifact.declarations[index])}));
 const entry={id:'complete original twenty-two-declaration symbol projection',kind:'complete unchanged generated String value projection',fixtureId:'experiments/original-link-symbol-projection/data/native-definitions.lino',source:native,sourceBytes:textBytes(native).length,sourceSha256,
  replaySha256:sha256(binaryReplay),textReplaySha256:sha256(textReplay),originalReceipt:{sourceBytes:textBytes(native).length,sourceSha256},...header,declarations,declarationSha256:sha256(JSON.stringify(artifact.declarations)),
  sourceModelSha256:digests['data/policy.lino'],kernelSha256:artifact.sourceSha256,invocationCount:observations.length,maximumObservedInputCells:Math.max(...observations.flatMap(row=>row.inputBits.map(bits=>bits.length))),maximumObservedOutputCells:Math.max(...observations.map(row=>row.resultBits.length)),
  observationScope:'Complete ordered String value symbols, name pairs and UTF16 units or exact refusal vectors over bounded dense primitive Number transports only. Equal values prove no original LinkId identity, scalar ownership, semantic equality, complete dynamic certificate tables or native100000-row resource behavior.',
  archiveFormats:['original complete declaration artifact Links text and binary','complete generated Rust CST Links text and binary'],consumerSource:consumer,consumerSha256:sha256(consumer),javaScriptSha256:sha256(javascript.text),observations};
 const plan=freeze({model:artifact.model,modelCommit:ORIGINAL_LINK_SYMBOL_MODEL_COMMIT,modelSources,originalCount:1,entries:[entry]});checkedPlans.add(plan);return plan;
}
function parseObservations(output,expected){
 const lines=output.split('\n');assert.equal(lines.pop(),'','the native projection stream ends with one newline');assert.equal(lines.length,expected.length,'every actual projection invocation is reported exactly once');
 return lines.map((line,index)=>{
  const parts=line.split(':');assert.equal(parts[0],String(index),'native invocation order is exact');assert.equal(parts.length,expected[index].resultBits.length+1,'the complete variable projection length is exact');
  const actual=parts.slice(1);actual.forEach(value=>assert.match(value,/^[0-9a-f]{16}$/u));assert.deepEqual(actual,expected[index].resultBits,'all native projection cells must match actual JavaScript');
  return{id:expected[index].id,kind:expected[index].kind,inputVectors:expected[index].inputBits.map(bits=>({cells:bits.length,sha256:sha256(bits.join(''))})),resultBits:actual,decision:actual.map(bitsNumber)};
 });
}
export const runOriginalLinkSymbolNative=createSourceNativeObservationRunner({
 nativeMode:originalLinkSymbolNativeMode,checkedPlan:plan=>checkedPlans.has(plan),textBytes,parseObservations,
 entryReceipt:entry=>({functionName:entry.functionName,parameterOrder:entry.parameterOrder,parameterModes:entry.parameterModes,resultMode:entry.resultMode,referenceSemantics:entry.referenceSemantics,interfaceSha256:entry.interfaceSha256,tokenCount:entry.tokenCount,
  declarations:entry.declarations,declarationSha256:entry.declarationSha256,sourceModelSha256:entry.sourceModelSha256,kernelSha256:entry.kernelSha256,textReplaySha256:entry.textReplaySha256,invocationCount:entry.invocationCount,
  maximumObservedInputCells:entry.maximumObservedInputCells,maximumObservedOutputCells:entry.maximumObservedOutputCells,observationScope:entry.observationScope,archiveFormats:entry.archiveFormats}),
 temporaryPrefix:'original-link-symbol-native-',failureLabel:'Original complete link symbol projection observations failed',
});
