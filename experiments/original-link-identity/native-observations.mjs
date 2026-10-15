// Observe the unchanged generated kernel only; no native process runs during planning.
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
import {readOriginalLinkIdentityOrdinals} from './rules.mjs';
import {originalLinkIdentityWords} from './policy.mjs';

export const ORIGINAL_LINK_IDENTITY_MODEL_COMMIT='9d578197fa32541d146d78c86c0a0d721ff540e4';
const digests={
 'policy.mjs':'080d379c8c97696fb611f841b1d7dba220864e4cd82984febb5c550e7bdd46e5',
 'rules.mjs':'ac1d9dae9835c1fe5b089d96a163ad3aaf0ac89ef6f29faccf0fc19c700727ba',
 'generate.mjs':'a257b6c6a16824bc7bb4f239c9afec5f78bec76adc472b19b5af7d63a1a6c4d1',
 'data/policy.lino':'c287a0ef8c8eba5a8ee077924fc0f05b306c8e03e44ad0033d4c6f437eb4ed89',
 'data/native-definitions.lino':'07c73971deb4891740b535028c5eda2a5519b19237ba074561f76ebe878ca975',
};
const mode='std::rc::Rc<std::cell::RefCell<Vec<f64>>>',checkedPlans=new WeakSet();
const sha256=value=>createHash('sha256').update(value).digest('hex');
function textBytes(value){if(typeof value!=='string'||!value.isWellFormed())throw new TypeError('the oracle source must be primitive Unicode scalar text');return Buffer.from(value,'utf8');}
function freeze(value){if(value&&typeof value==='object'){Object.values(value).forEach(freeze);Object.freeze(value);}return value;}
function numberBits(value){assert.equal(typeof value,'number');const bytes=Buffer.alloc(8);bytes.writeDoubleBE(value);return bytes.toString('hex');}
function bitsNumber(value){assert.match(value,/^[0-9a-f]{16}$/u);return Buffer.from(value,'hex').readDoubleBE();}

export function verifyOriginalLinkIdentityNativeModel(read=readFileSync){
 return Object.entries(digests).map(([file,expected])=>{const actual=sha256(read(new URL(file,import.meta.url)));assert.equal(actual,expected,'the frozen identity source changed: '+file);return{file,sha256:actual};});
}
export function originalLinkIdentityNativeMode(environment=process.env){
 if(environment.ORIGINAL_LINK_IDENTITY_NATIVE!=='1')return false;
 if(environment.CI!=='true')throw new TypeError('ORIGINAL_LINK_IDENTITY_NATIVE=1 requires CI=true; native compilation is forbidden locally');return true;
}
function nativeInterface(source){
 const tokens=tokenize(source,'Rust').tokens,found=[];let depth=0;
 for(let index=0;index<tokens.length;index++){
  if(depth===0&&tokens[index].value==='pub'&&tokens[index+1]?.value==='fn'&&tokens[index+2]?.value==='read_original_link_identity_ordinals')found.push(index);
  if(tokens[index].kind==='punct'&&tokens[index].value==='{')depth++;if(tokens[index].kind==='punct'&&tokens[index].value==='}')depth--;
 }
 assert.equal(depth,0);assert.equal(found.length,1,'the actual kernel exports one complete ordinal lookup function');
 const vector=['std','::','rc','::','Rc','<','std','::','cell','::','RefCell','<','Vec','<','f64','>','>','>'];
 const expected=['pub','fn','read_original_link_identity_ordinals','('];
 for(const[index,name]of['high','low','query_high','query_low'].entries()){if(index)expected.push(',');expected.push(name,':',...vector);}expected.push(')','->',...vector,'{');
 assert.deepEqual(tokens.slice(found[0],found[0]+expected.length).map(token=>token.value),expected,'the real generated reference-vector ABI must remain exact');
 return {functionName:'read_original_link_identity_ordinals',parameterOrder:['high','low','query_high','query_low'],parameterModes:Array(4).fill(mode),resultMode:mode,interfaceSha256:sha256(expected.join('\0')),tokenCount:tokens.length};
}
function cases(){
 const rows=[],add=(id,args,expected,kind='exact identity membership')=>rows.push({id,args,expected,kind});
 const exact=(id,index,queries)=>{
  const table=[...index].sort((a,b)=>a<b?-1:a>b?1:0),parts=table.map(originalLinkIdentityWords),wanted=queries.map(originalLinkIdentityWords);
  add(id,[parts.map(row=>row[0]),parts.map(row=>row[1]),wanted.map(row=>row[0]),wanted.map(row=>row[1])],queries.map(query=>table.findIndex(key=>key===query)+1));
 };
 exact('empty',[],[]);exact('unused index with empty queries',[1n],[]);exact('zero key',[0n],[0n]);exact('maximum unsigned64',[18446744073709551615n],[18446744073709551615n]);
 exact('adjacent above safe Number',[9007199254740992n,9007199254740993n],[9007199254740993n,9007199254740992n,9007199254740993n]);
 exact('uint32 word carry',[4294967295n,4294967296n],[4294967296n,4294967295n]);
 exact('ordered repeated references',[0n,1n,9007199254740992n,9007199254740993n,18446744073709551614n,18446744073709551615n],[18446744073709551615n,9007199254740993n,1n,9007199254740993n,0n]);
 add('signed zero words',[[0],[0],[-0],[-0]],[1]);
 add('index vector length',[[0],[],[],[]],[-1],'malformed index');add('duplicate index',[[0,0],[1,1],[0],[1]],[-1],'malformed index');
 add('descending high words',[[1,0],[0,1],[0],[1]],[-1],'malformed index');add('descending low words',[[0,0],[2,1],[0],[1]],[-1],'malformed index');
 add('query vector length',[[0],[1],[0],[]],[-2],'malformed queries');add('missing empty index',[[],[],[0],[0]],[-4,0],'missing identity');add('missing second identity',[[0],[1],[0,0],[1,2]],[-4,1],'missing identity');
 for(const[label,value]of[['negative',-1],['fraction',0.5],['above uint32',4294967296],['NaN',NaN],['positive infinity',Infinity],['negative infinity',-Infinity],['subnormal',Number.MIN_VALUE],['maximum Number',Number.MAX_VALUE]]){
  for(const slot of[0,1,2,3]){const args=[[0],[1],[0],[1]];args[slot][0]=value;add(label+' vector '+slot,args,slot<2?[-1]:[-3,0],'invalid numeric word');}
 }
 assert.equal(rows.length,47);return rows;
}
function consumerSource(observations,header){
 const blocks=observations.map((row,index)=>{
  const inputs=row.inputBits.map((bits,slot)=>`let input_${slot}=std::rc::Rc::new(std::cell::RefCell::new(vec![${bits.map(value=>'f64::from_bits(0x'+value+'u64)').join(',')} ]));`);
  const checks=row.inputBits.map((bits,slot)=>`assert_eq!(input_${slot}.borrow().iter().map(|value|value.to_bits()).collect::<Vec<_>>(),vec![${bits.map(value=>'0x'+value+'u64').join(',')}],"original input vector ${slot} changed");`);
  return`{\n${inputs.join('\n')}\nlet observed=oracle_source::${header.functionName}(input_0.clone(),input_1.clone(),input_2.clone(),input_3.clone());\n${checks.join('\n')}\nlet bits=observed.borrow().iter().map(|value|value.to_bits()).collect::<Vec<_>>();\nassert_eq!(bits,vec![${row.resultBits.map(value=>'0x'+value+'u64').join(',')}],"actual ordinal vector differs from JavaScript");\nwrite!(&mut line,"${index}").expect("observation index");\nfor value in bits{write!(&mut line,":{value:016x}").expect("ordinal bits");}\nline.push('\\n');\n}`;
 });
 return`extern crate oracle_source;\nuse std::fmt::Write as _;\nfn main(){let mut line=String::new();\n${blocks.join('\n')}\nprint!("{line}");}\n`;
}
/** Planning checks actual JS and archives; its only native output is source text. */
export function planOriginalLinkIdentityNative(){
 const modelSources=verifyOriginalLinkIdentityNativeModel(),source=readFileSync(new URL('./rules.mjs',import.meta.url),'utf8'),artifact=readProjectData(new URL('./data/native-definitions.lino',import.meta.url));
 const artifactNetwork=decodeLinkNetworkBinary(encodeLinkNetworkBinary(LinkNetwork.fromLino(readFileSync(new URL('./data/native-definitions.lino',import.meta.url),'utf8'))));
 assert.deepEqual(readStructuredDataLino(artifactNetwork.toLino()),artifact,'all original declaration data survive text and binary Links');
 const program=checkProgram(parseJavaScript(source)),rust=emitRust(program),javascript=emitJavaScript(program),context={};vm.runInNewContext(javascript.text,context,{timeout:1000});
 assert.equal(sha256(source),artifact.sourceSha256);assert.deepEqual(rust.definitions,artifact.declarations);assert.equal(rust.definitions.length,11);assert.equal(program.items.length,11);
 const callee=program.declarations.get('readOriginalLinkIdentityOrdinals'),vector={kind:'array',element:{kind:'float'},reference:true};
 assert.deepEqual(callee.params.map(row=>row.name),['high','low','queryHigh','queryLow']);for(const row of callee.params){assert.deepEqual(row.type,vector);assert.equal(row.guard,null);assert.equal(row.default,null);}assert.deepEqual(callee.ret,vector);
 const target=javascript.mappings.find(row=>row.kind==='function'&&row.source===callee.fullName)?.target;assert.equal(typeof context[target],'function');
 const observations=cases().map(row=>{
  const inputBits=row.args.map(values=>values.map(numberBits)),direct=readOriginalLinkIdentityOrdinals(...row.args),observed=Array.from(context[target](...row.args));
  assert.deepEqual(direct,row.expected,row.id);assert.deepEqual(observed,row.expected,row.id);assert.deepEqual(row.args.map(values=>values.map(numberBits)),inputBits,'identity lookup preserves every input bit');
  return{id:row.id,kind:row.kind,inputBits,resultBits:observed.map(numberBits),decision:observed};
 });
 const native=rust.text;assert.equal(textBytes(native).length,17378);assert.equal(sha256(native),'fe8afbe10f64da5198fcb1fba4354e13958e4201587047dedd4899e2ce784b2a');
 for(const definition of artifact.declarations)assert.ok(native.includes(definition));
 const archive=LinkNetwork.parse(native,'Rust'),restored=decodeLinkNetworkBinary(encodeLinkNetworkBinary(LinkNetwork.fromLino(archive.toLino()))).reconstructText();assert.deepEqual(textBytes(restored),textBytes(native));
 const header=nativeInterface(native),consumer=consumerSource(observations,header),sourceSha256=sha256(native),declarations=program.items.map((row,index)=>({kind:row.k,name:row.fullName,sha256:sha256(artifact.declarations[index])}));
 const entry={id:'complete original eleven-declaration identity kernel',kind:'complete unchanged generated identity artifact',fixtureId:'experiments/original-link-identity/data/native-definitions.lino',source:native,sourceBytes:textBytes(native).length,sourceSha256,
  replaySha256:sha256(restored),originalReceipt:{sourceBytes:textBytes(native).length,sourceSha256},...header,declarations,declarationSha256:sha256(JSON.stringify(artifact.declarations)),
  sourceModelSha256:digests['data/policy.lino'],kernelSha256:artifact.sourceSha256,invocationCount:observations.length,maximumObservedIndexRows:6,maximumObservedQueryRows:5,
  observationScope:'Exact unsigned64-word membership and ordered positive or negative ordinal vectors only. These bounded calls prove no source ownership, semantic equality, public high-u64 graph support, complete17-table integration or native100000-row resource behavior.',
  archiveFormats:['original complete declaration data Links text and binary','complete generated Rust CST Links text and binary'],consumerSource:consumer,consumerSha256:sha256(consumer),javaScriptSha256:sha256(javascript.text),observations};
 const plan=freeze({model:artifact.model,modelCommit:ORIGINAL_LINK_IDENTITY_MODEL_COMMIT,modelSources,originalCount:1,entries:[entry]});checkedPlans.add(plan);return plan;
}
function parseObservations(output,expected){
 const lines=output.split('\n');assert.equal(lines.pop(),'','the native ordinal stream ends with one newline');assert.equal(lines.length,expected.length,'every actual lookup invocation is reported exactly once');
 return lines.map((line,index)=>{
  const parts=line.split(':');assert.equal(parts[0],String(index),'native invocation order is exact');assert.equal(parts.length,expected[index].resultBits.length+1,'the complete ordinal vector length is exact');
  const actual=parts.slice(1);actual.forEach(value=>assert.match(value,/^[0-9a-f]{16}$/u));assert.deepEqual(actual,expected[index].resultBits,'native ordinal cells must match actual JavaScript');
  return{id:expected[index].id,kind:expected[index].kind,inputVectors:expected[index].inputBits.map(bits=>({cells:bits.length,sha256:sha256(bits.join(''))})),resultBits:actual,decision:actual.map(bitsNumber)};
 });
}
export const runOriginalLinkIdentityNative=createSourceNativeObservationRunner({
 nativeMode:originalLinkIdentityNativeMode,checkedPlan:plan=>checkedPlans.has(plan),textBytes,parseObservations,
 entryReceipt:entry=>({functionName:entry.functionName,parameterOrder:entry.parameterOrder,parameterModes:entry.parameterModes,resultMode:entry.resultMode,interfaceSha256:entry.interfaceSha256,tokenCount:entry.tokenCount,
  declarations:entry.declarations,declarationSha256:entry.declarationSha256,sourceModelSha256:entry.sourceModelSha256,kernelSha256:entry.kernelSha256,invocationCount:entry.invocationCount,
  maximumObservedIndexRows:entry.maximumObservedIndexRows,maximumObservedQueryRows:entry.maximumObservedQueryRows,observationScope:entry.observationScope,archiveFormats:entry.archiveFormats}),
 temporaryPrefix:'original-link-identity-native-',failureLabel:'Original complete link identity kernel observations failed',
});
