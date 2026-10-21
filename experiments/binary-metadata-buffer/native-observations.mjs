// The original complete generated bundle is compiled only through the explicit CI selector.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import vm from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {readSourceTextRustBlock} from '../../js/scripts/source-text-byte-library-style.mjs';
import {LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary,readStructuredDataLino} from '../../js/src/index.js';
import {LinkMetadata} from '../../js/src/primitives.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {tokenize} from '../../js/src/translation/lexer.js';
import {createSourceNativeObservationRunner} from '../source-native-observations.mjs';
import {transcodeLinkNetworkBinary as previous} from './prior-rules.mjs';
import {transcodeLinkNetworkBinary as candidate} from './candidate.mjs';

export const BINARY_METADATA_BUFFER_MODEL_COMMIT='1bb41fae1cbe4c88592e3241d01563346c97fac0';
const root=new URL('../../',import.meta.url),sha256=value=>createHash('sha256').update(value).digest('hex'),checkedPlans=new WeakSet();
const digests={
 'experiments/binary-metadata-buffer/candidate.mjs':'d96f973eb3026166b16f74ce8884e4a3b39d65e6bfaf3469cd24e5aa3ddf82cb',
 'experiments/binary-metadata-buffer/prior-rules.mjs':'4f0debafedcc6e1985ccad8ed0dd673261e511e705d4d7535db008d446cf11a4',
 'experiments/binary-metadata-buffer/generation-gap.lino':'8c00283255bdeccf250a4b42da1088fe6fb8970c461b9cf94b1b521dc341a1ec',
 'experiments/binary-metadata-buffer/native-policy.lino':'c13f683ba190c70b76a3cb719dc587af0f73cf66b34c0b13d719988252d6f2ee',
 'experiments/binary-metadata-buffer/native-styles.lino':'2dfea9af1dfba01ae02adc8f83a81070a9126b136e01b40d78ad2ad89068db48',
 'experiments/binary-metadata-buffer/native-generation.lino':'f21ade80837caee29d07c283ec06e0f0f92de78c2b3f48886aeb2e4cf9b62535',
 'js/scripts/generate-link-network-binary-rules.mjs':'b69fe844f08f933f7cca47b3fc34139e606b7bd72cefb3fd3c6380d4301745c8',
 'js/scripts/binary-metadata-candidate-styles.mjs':'78c8fdb8fdfaaddf1d70cee614031371ad6457427da154b13b6eee2c5bf4bb11',
 'js/scripts/source-text-byte-library-style.mjs':'cc6d3ac83a88653d05c3ced8f66ed1784c918afab4e7b0e0872af35832765695',
 'js/scripts/product-rule-style-corrections.mjs':'9728ead881e57f16401321be9eb00e25416aab74cdb4d7164485cd0a3abbc216',
 'parity/self-translation/link-network-binary-rules-decorators.lino':'bebfe31e9c01424bb894fa80935df173a7814515bd51d7d8a6eea9f454d371cb',
 'rust/src/translation/frontend_rules.rs':'017c049132b271f5c219736086ac04b2cfb349d88de767478edcad46f05d597f',
 'parity/fixtures/link-network-binary.lino':'10e687951cb962341c77c7d6160c5e0a15bb6751f68cbe30c91755a8a7d23806',
};
const librarySource='pub mod translation {\n    pub mod frontend_rules {\n        include!("provider_array.rs");\n    }\n}\n#[path = "candidate/mod.rs"]\npub mod candidate;\n';
function textBytes(value){if(typeof value!=='string'||!value.isWellFormed())throw new TypeError('the oracle source must be primitive Unicode scalar text');return Buffer.from(value,'utf8');}
function freeze(value){if(value&&typeof value==='object'){Object.values(value).forEach(freeze);Object.freeze(value);}return value;}
function numberBits(value){assert.equal(typeof value,'number');const bytes=Buffer.alloc(8);bytes.writeDoubleBE(value);return bytes.toString('hex');}
function bitsNumber(value){assert.match(value,/^[0-9a-f]{16}$/u);return Buffer.from(value,'hex').readDoubleBE();}
export function verifyBinaryMetadataBufferNativeModel(read=readFileSync){
 return Object.entries(digests).map(([file,expected])=>{
  const physicalBytes=read(new URL(file,root));let actualBytes=physicalBytes,sourceSuccession;
  if(file==='rust/src/translation/frontend_rules.rs'&&sha256(physicalBytes)==='3c9be49f013c6ed1d3e8f2c5a9235236160f6eed4cc3e60b27b415ef5d7d55e7'){
   assert.equal(physicalBytes.length,43982);
   const physical=physicalBytes.toString('utf8'),current=readSourceTextRustBlock(physical,'fn','accept_java_script_optional_tuple_context');
   const receipt=read(new URL('experiments/binary-metadata-buffer/frontend-owner-string-construction-revision.lino',root));
   assert.equal(sha256(receipt),'434c61e21c16876f6ea5910a146fee3bfeaa019a79ba95005f2e83490bbcdcac');
   const retained=readStructuredDataLino(receipt.toString('utf8'));
   assert.equal(sha256(retained.current.source),'bbf032af0422fd7252da32e090c065d3e4c9c4c8d88a284a4a15c02c1884a71d');
   const previous=readSourceTextRustBlock(retained.current.source,'fn','accept_java_script_optional_tuple_context');
   assert.equal(physical.split(current).length,2);actualBytes=Buffer.from(physical.replace(current,()=>previous));
   assert.equal(sha256(actualBytes),'bbf032af0422fd7252da32e090c065d3e4c9c4c8d88a284a4a15c02c1884a71d');
   assert.equal(actualBytes.length,43944);
   assert.equal(readSourceTextRustBlock(physical,'mod','ml_array'),readSourceTextRustBlock(actualBytes.toString('utf8'),'mod','ml_array'),'the entire compiled provider is unchanged');
   assert.equal(actualBytes.toString('utf8').replace(previous,()=>current),physical,'the complete actual successor is recoverable');
   sourceSuccession={revision:'3fe3a9f9decfa6431b5b0718729a5d123b5f236b',actualSourceBytes:physicalBytes.length,actualSourceSha256:sha256(physicalBytes),historicalSourceSha256:sha256(actualBytes)};
  }
  const actualSha256=sha256(actualBytes);let modelBytes=actualBytes,ownerRevision;
  const refusal='the frozen binary candidate model changed: '+file;
  if(file==='rust/src/translation/frontend_rules.rs'&&actualSha256!==expected){
   assert.ok(actualSha256==='37033e27b97bc145b0d51b89016606e141d5962c0f423ee131202432a301dad3'||actualSha256==='bbf032af0422fd7252da32e090c065d3e4c9c4c8d88a284a4a15c02c1884a71d',refusal);
   let ownerBytes=actualBytes,successor;
   if(actualSha256==='bbf032af0422fd7252da32e090c065d3e4c9c4c8d88a284a4a15c02c1884a71d'){
    const successorFile='experiments/binary-metadata-buffer/frontend-owner-string-construction-revision.lino',successorBytes=read(new URL(successorFile,root));
    assert.equal(sha256(successorBytes),'434c61e21c16876f6ea5910a146fee3bfeaa019a79ba95005f2e83490bbcdcac',refusal+' successor receipt');
    successor=readStructuredDataLino(successorBytes.toString('utf8'));
    assert.equal(successor.schemaVersion,1,refusal);assert.deepEqual(successor.predecessor,{file:'experiments/binary-metadata-buffer/frontend-owner-revision.lino',sha256:'95f86b39d65d4b2faa57ceb942bc77feab2086184b239ed3f4210c8be6a4720c',commit:'5fd3fb6ec8d15331bd59c67dac55c6d8d258e10a',ownerPath:file,ownerSha256:'37033e27b97bc145b0d51b89016606e141d5962c0f423ee131202432a301dad3'},refusal);
    assert.equal(successor.current.commit,'2bd4d4f6ad6e2a06398199d50ab93171a5f4b799',refusal);assert.equal(successor.current.path,file,refusal);
    assert.equal(successor.current.sha256,actualSha256,refusal);assert.equal(successor.current.sourceBytes,actualBytes.length,refusal);assert.equal(successor.current.source,actualBytes.toString('utf8'),refusal);
    assert.equal(successor.change.kind,'replacement',refusal);assert.equal(successor.change.owner,'read_type_query_result',refusal);
    assert.equal(successor.change.current,readSourceTextRustBlock(actualBytes.toString('utf8'),'fn','read_type_query_result')+'\n',refusal);
    assert.equal(successor.change.original.split('String::from("")').length,2,refusal);
    assert.equal(successor.change.current,successor.change.original.replace('String::from("")','String::new()'),refusal);
    assert.equal(successor.current.source.split(successor.change.current).length,2,refusal);
    const restoredOwner=successor.current.source.replace(successor.change.current,()=>successor.change.original);
    assert.equal(sha256(restoredOwner),'37033e27b97bc145b0d51b89016606e141d5962c0f423ee131202432a301dad3',refusal);ownerBytes=Buffer.from(restoredOwner,'utf8');
    assert.equal(successor.change.original,readSourceTextRustBlock(restoredOwner,'fn','read_type_query_result')+'\n',refusal);
   }
   const receiptFile='experiments/binary-metadata-buffer/frontend-owner-revision.lino';
   const receiptBytes=read(new URL(receiptFile,root));assert.equal(sha256(receiptBytes),'95f86b39d65d4b2faa57ceb942bc77feab2086184b239ed3f4210c8be6a4720c',refusal+' owner receipt');
   const revision=readStructuredDataLino(receiptBytes.toString('utf8'));
   assert.equal(revision.schemaVersion,1,refusal);assert.equal(revision.original.commit,BINARY_METADATA_BUFFER_MODEL_COMMIT,refusal);
   assert.equal(revision.original.path,file,refusal);assert.equal(revision.original.sha256,expected,refusal);
   assert.equal(revision.current.commit,'5fd3fb6ec8d15331bd59c67dac55c6d8d258e10a',refusal);
   assert.equal(revision.current.path,file,refusal);assert.equal(revision.current.sha256,sha256(ownerBytes),refusal);
   assert.equal(revision.current.sourceBytes,ownerBytes.length,refusal);assert.equal(revision.current.source,ownerBytes.toString('utf8'),refusal);
   assert.equal(sha256(revision.original.source),expected,refusal);assert.equal(Buffer.byteLength(revision.original.source),revision.original.sourceBytes,refusal);
   assert.deepEqual(revision.changes.map(row=>[row.kind,row.owner]),[['replacement','read_type_query_result'],['addition','accept_java_script_optional_tuple_context']],refusal);
   assert.equal(revision.changes[0].current,readSourceTextRustBlock(revision.current.source,'fn','read_type_query_result')+'\n',refusal);
   assert.equal(revision.changes[0].original,readSourceTextRustBlock(revision.original.source,'fn','read_type_query_result')+'\n',refusal);
   assert.equal(revision.changes[1].current,'\n'+readSourceTextRustBlock(revision.current.source,'fn','accept_java_script_optional_tuple_context')+'\n',refusal);
   assert.equal(revision.changes[1].original,'',refusal);
   let restored=ownerBytes.toString('utf8');
   for(const change of revision.changes){assert.equal(restored.split(change.current).length,2,refusal);restored=restored.replace(change.current,()=>change.original);}
   assert.equal(restored,revision.original.source,refusal);assert.equal(sha256(restored),expected,refusal);
   const provider=readSourceTextRustBlock(actualBytes.toString('utf8'),'mod','ml_array');
   assert.equal(provider,readSourceTextRustBlock(restored,'mod','ml_array'),refusal);assert.equal(provider,revision.provider.source,refusal);
   assert.equal(Buffer.byteLength(provider),3648,refusal);assert.equal(sha256(provider),'c2a535a8fbc2ab09deca3f7c92cad278b0a08a526f5f87a54e2aebb822c8b76a',refusal);
   assert.deepEqual(revision.frozenBindings,Object.entries(digests).map(([file,sha256])=>({file,sha256})),refusal);
   modelBytes=Buffer.from(restored,'utf8');
   {
    const actualBytes=ownerBytes,actualSha256=sha256(ownerBytes);
   ownerRevision={file:receiptFile,sha256:sha256(receiptBytes),originalCommit:revision.original.commit,currentCommit:revision.current.commit,
    actualSourceSha256:actualSha256,actualSourceBytes:actualBytes.length,providerSha256:sha256(provider)};
   }
   if(successor){
    assert.deepEqual(successor.original,{commit:revision.original.commit,path:file,sourceBytes:revision.original.sourceBytes,sha256:expected},refusal);
    assert.deepEqual(successor.frozenBindings,revision.frozenBindings,refusal);assert.deepEqual(successor.compiledArtifacts,revision.compiledArtifacts,refusal);
    assert.equal(successor.provider.sha256,sha256(provider),refusal);assert.equal(successor.provider.sourceBytes,Buffer.byteLength(provider),refusal);
    ownerRevision={file:'experiments/binary-metadata-buffer/frontend-owner-string-construction-revision.lino',sha256:'434c61e21c16876f6ea5910a146fee3bfeaa019a79ba95005f2e83490bbcdcac',originalCommit:revision.original.commit,currentCommit:successor.current.commit,
     actualSourceSha256:actualSha256,actualSourceBytes:actualBytes.length,providerSha256:sha256(provider),predecessor:ownerRevision};
   }
  }
  const actual=sha256(modelBytes);assert.equal(actual,expected,refusal);
  return{file,sha256:actual,...(ownerRevision?{ownerRevision}:{}),...(sourceSuccession?{sourceSuccession}:{})};
 });
}
export function binaryMetadataBufferNativeMode(environment=process.env){if(environment.BINARY_METADATA_BUFFER_NATIVE!=='1')return false;if(environment.CI!=='true')throw new TypeError('BINARY_METADATA_BUFFER_NATIVE=1 requires CI=true; native compilation is forbidden locally');return true;}
function inventory(source,path){
 const tokens=tokenize(source,'Rust').tokens,rows=[];let depth=0;
 for(let index=0;index<tokens.length;index++){
  const token=tokens[index];
  if(depth===0&&token.value==='pub'){
   let kindAt=index+1;if(tokens[kindAt]?.value==='const')kindAt++;
   const kind=tokens[kindAt]?.value;
   if(['fn','mod','struct','enum','use'].includes(kind)){
    let end=kindAt+1;while(end<tokens.length&&!['{',';'].includes(tokens[end].value))end++;
    const name=kind==='use'?tokens[end-1].value:tokens[kindAt+1].value;
    if(tokens[end]?.value==='{'){let nested=1;for(end++;end<tokens.length;end++){if(tokens[end].kind==='punct'&&tokens[end].value==='{')nested++;if(tokens[end].kind==='punct'&&tokens[end].value==='}')nested--;if(nested===0)break;}}
    assert.ok(tokens[end],'every actual declaration is complete');
    rows.push({path,kind,name,start:token.start,end:tokens[end].end,sha256:sha256(source.slice(token.start,tokens[end].end))});
   }
  }
  if(token.kind==='punct'&&token.value==='{')depth++;if(token.kind==='punct'&&token.value==='}')depth--;
 }
 assert.equal(depth,0);return rows;
}
function nativeInterface(source){
 const tokens=tokenize(source,'Rust').tokens,indexes=[];let depth=0;
 for(let at=0;at<tokens.length;at++){if(depth===0&&tokens[at].value==='pub'&&tokens[at+1]?.value==='fn'&&tokens[at+2]?.value==='transcode_link_network_binary')indexes.push(at);if(tokens[at].kind==='punct'&&tokens[at].value==='{')depth++;if(tokens[at].kind==='punct'&&tokens[at].value==='}')depth--;}
 assert.equal(depth,0);assert.equal(indexes.length,1,'the actual complete candidate exports one transcoder');
 const expected=['pub','fn','transcode_link_network_binary','(','input',':','&','[','f64',']',',','binary',':','bool',',','record_limit',':','f64',',','byte_limit',':','f64',',',')','->','super','::','ml_string','::','Text','{'];
 assert.deepEqual(tokens.slice(indexes[0],indexes[0]+expected.length).map(token=>token.value),expected,'the actual borrowed input and UTF16 Text ABI must remain exact');
 return{functionName:'candidate::transcode_link_network_binary',parameterOrder:['input','binary','record_limit','byte_limit'],parameterModes:['&[f64]','bool','f64','f64'],resultMode:'candidate::ml_string::Text',interfaceSha256:sha256(expected.join('\0'))};
}
function archiveReceipt(source,path){
 const network=LinkNetwork.parse(source,'Rust'),text=LinkNetwork.fromLino(network.toLino()).reconstructText(),binary=decodeLinkNetworkBinary(encodeLinkNetworkBinary(network)).reconstructText();assert.deepEqual(textBytes(text),textBytes(source));assert.deepEqual(textBytes(binary),textBytes(source));
 return{path,sourceBytes:textBytes(source).length,sourceSha256:sha256(source),textReplaySha256:sha256(text),binaryReplaySha256:sha256(binary)};
}
const bytes=hex=>Array.from({length:hex.length/2},(_,index)=>Number.parseInt(hex.slice(index*2,index*2+2),16));
const fields=encoded=>encoded.slice(0,-1).split(',').map(Number);
function cases(){
 const rows=[],limit=1048576,record=[1,0,1,0,0,0,0,0,0,0,0,0],version1=[1,1,...record],version2=[2,2,1,...record,0,0],replace=(input,index,value)=>input.map((item,at)=>at===index?value:item);
 const add=(id,input,binary,recordLimit=10000,byteLimit=limit,expected=null,kind='exact profile framing')=>rows.push({id,input,binary,recordLimit,byteLimit,expected,kind});
 const fixture=readProjectData(new URL('../../parity/fixtures/link-network-binary.lino',import.meta.url));
 for(const[version,archive]of[[1,fixture.legacyVersion1Bytes],[2,fixture.version2Bytes]]){
  const flat=fields(previous(archive,true,10000,limit));add('golden v'+version+' decode',archive,true);add('golden v'+version+' encode',flat,false,10000,limit,Buffer.from(archive).toString('hex'));
 }
 for(const[version,flat]of[[1,[1,0]],[2,[2,1,0,0,0]]]){const encoded=previous(flat,false,10000,limit);add('empty v'+version+' encode',flat,false);add('empty v'+version+' decode',bytes(encoded),true,10000,limit,flat.join(',')+',');}
 const network=new LinkNetwork();network.insertLinkWithOptionalId(1,[2],LinkMetadata.new().withDefinition('first α😀').withLanguage('Rust'));network.insertLinkWithOptionalId(2,[1],LinkMetadata.new().withDefinition('second \0 Ω').withLanguage('JavaScript'));network.setTerm(1,'named first');network.setTerm(2,'different second');
 const current=[...encodeLinkNetworkBinary(network)],flatCurrent=fields(previous(current,true,10000,limit));
 const firstOnly=new LinkNetwork();firstOnly.insertLinkWithOptionalId(1,[2],LinkMetadata.new().withDefinition('first α😀').withLanguage('Rust'));firstOnly.setTerm(1,'named first');
 const firstRecord=fields(previous([...encodeLinkNetworkBinary(firstOnly)],true,10000,limit)).slice(3,-2);
 for(const version of[1,2]){
  const flat=version===2?flatCurrent:[1,...flatCurrent.slice(2,-2)],encoded=previous(flat,false,10000,limit),width=encoded.length/2;
  add('distinct metadata v'+version+' encode',flat,false);add('distinct metadata v'+version+' decode',bytes(encoded),true);
  for(const delta of[-1,0,1]){add('metadata v'+version+' encode byte bound '+delta,flat,false,10000,width+delta);add('metadata v'+version+' decode byte bound '+delta,bytes(encoded),true,10000,width+delta);}
  const metadataOffset=version===2?6:5,secondRecord=(version===2?3:2)+firstRecord.length;assert.equal(flat[secondRecord],2);
  add('malformed v'+version+' precedes output bound',replace(flat,metadataOffset,2),false,10000,width-1,'-13,'+metadataOffset+',','refusal precedence');
  add('duplicate v'+version+' precedes output bound',replace(flat,secondRecord,1),false,10000,width-1,'-8,'+secondRecord+',','refusal precedence');
 }
 const encoded=bytes(previous(version1,false,10000,limit)),refusals=[
  [version1,false,0,limit,'-1,0,'],[version1,false,1,15,'-1,0,'],[fixture.version2Bytes,true,10000,16,'-2,0,'],
  [replace(encoded,0,0),true,10000,limit,'-3,0,'],[replace(encoded,7,51),true,10000,limit,'-4,7,'],[replace(version2,1,0),false,10000,limit,'-5,1,'],[replace(version1,1,10001),false,10000,limit,'-6,1,'],
  [replace(version1,2,0),false,10000,limit,'-7,2,'],[replace(version1,2,Number.MAX_SAFE_INTEGER),false,10000,limit,'-7,2,'],[[1,2,...record,...record],false,10000,limit,'-8,14,'],[replace(version1,3,-1),false,10000,limit,'-9,3,'],[[1,1,1,1,0,...record.slice(2)],false,10000,limit,'-10,4,'],
  [replace(encoded,32,5),true,10000,limit,'-11,32,'],[replace(version1,4,256),false,10000,limit,'-12,4,'],[replace(version1,4,2),false,10000,limit,'-13,4,'],[replace(version1,10,2),false,10000,limit,'-14,10,'],
  [[...version1.slice(0,10),1,100,...version1.slice(11)],false,10000,limit,'-15,11,'],[[...version1.slice(0,10),1,1,256,...version1.slice(11)],false,10000,limit,'-16,12,'],[replace(version1,13,2),false,10000,limit,'-17,13,'],[[...version1.slice(0,13),1,-1,0,0,0,0,0],false,10000,limit,'-18,14,'],
  [replace(encoded,32,9),true,10000,limit,'-19,50,'],[replace(version2,15,-1),false,10000,limit,'-20,15,'],[[...version2.slice(0,15),1,0,2,0],false,10000,limit,'-21,17,'],[[...version1,0],false,10000,limit,'-22,14,'],
 ];
 for(const[index,row]of refusals.entries())add('exact refusal '+index,...row,'exact code and input offset');
 for(const[label,value]of[['NaN',NaN],['infinity',Infinity],['negative',-1],['fraction',0.5],['unsafe',9007199254740992]]){add('record limit '+label,version1,false,value,limit,'-1,0,','limit domain');add('byte limit '+label,version1,false,10000,value,'-1,0,','limit domain');}
 add('malformed metadata before final output limit',replace(version1,4,2),false,10000,16,'-13,4,','refusal precedence');
 add('invalid limits before malformed header',[0],true,0,0,'-1,0,','refusal precedence');add('input bound before malformed header',Array(17).fill(0),true,1,16,'-2,0,','refusal precedence');
 for(const end of[0,7,8,15,16,32,encoded.length-1])add('v1 binary truncation '+end,encoded.slice(0,end),true);
 add('v1 binary trailing byte',[...encoded,0],true);add('v2 profile trailing field',[...version2,0],false);
 assert.equal(rows.length,74);return rows;
}
function consumerSource(observations,header){
 const blocks=observations.map((row,index)=>{
  const input=row.inputBits.map(value=>'f64::from_bits(0x'+value+'u64)').join(','),expected=row.inputBits.map(value=>'0x'+value+'u64').join(',');
  return`{\nlet input:Vec<f64>=vec![${input}];\nlet observed=oracle_source::${header.functionName}(&input,${row.binary},f64::from_bits(0x${row.recordLimitBits}u64),f64::from_bits(0x${row.byteLimitBits}u64)).to_string();\nlet expected_input:Vec<u64>=vec![${expected}];\nassert_eq!(input.iter().map(|value|value.to_bits()).collect::<Vec<_>>(),expected_input,"original input bits changed");\nlet expected:Vec<u8>=vec![${Array.from(Buffer.from(row.result,'ascii')).join(',')}];\nassert_eq!(observed.as_bytes(),expected.as_slice(),"complete byte or error framing differs from JavaScript");\nwrite!(&mut line,"${index}:").expect("invocation index");\nfor byte in observed.as_bytes(){write!(&mut line,"{byte:02x}").expect("framing byte");}\nline.push('\\n');\n}`;
 });return`extern crate oracle_source;\nuse std::fmt::Write as _;\nfn main(){let mut line=String::new();\n${blocks.join('\n')}\nprint!("{line}");}\n`;
}
function emittedJavaScript(source,policy){const program=checkProgram(parseJavaScript(source,policy.parser)),emitted=emitJavaScript(program),context={};vm.runInNewContext(emitted.text,context,{timeout:2000});const target=emitted.mappings.find(row=>row.kind==='function'&&row.source==='transcodeLinkNetworkBinary')?.target;assert.equal(typeof context[target],'function');return{program,emitted,invoke:(...args)=>context[target](...args)};}
export function planBinaryMetadataBufferNative(){
 const modelSources=verifyBinaryMetadataBufferNativeModel(),receipt=readProjectData(new URL('./native-generation.lino',import.meta.url)),policy=readProjectData(new URL('./native-policy.lino',import.meta.url)),gap=readProjectData(new URL('./generation-gap.lino',import.meta.url));
 assert.deepEqual(policy,{schemaVersion:1,model:'binary metadata buffer native generation 1',scope:'native generation input only',parser:{referenceSemantics:false},checkedProgram:{utf16Strings:true}});assert.deepEqual(receipt.generationPolicy,policy);
 const hash=path=>digests[path];assert.equal(receipt.candidateSourceSha256,gap.candidate.sha256);assert.equal(receipt.baselineSourceSha256,gap.baseline.sha256);assert.equal(receipt.generatorSha256,hash('js/scripts/generate-link-network-binary-rules.mjs'));assert.equal(receipt.candidateStyleBuilderSha256,hash('js/scripts/binary-metadata-candidate-styles.mjs'));assert.equal(receipt.candidateStylesSha256,hash('experiments/binary-metadata-buffer/native-styles.lino'));assert.equal(receipt.generationPolicySha256,hash('experiments/binary-metadata-buffer/native-policy.lino'));assert.equal(receipt.baselineStylesSha256,hash('parity/self-translation/link-network-binary-rules-decorators.lino'));
 const old=emittedJavaScript(readFileSync(new URL('./prior-rules.mjs',import.meta.url),'utf8'),policy),next=emittedJavaScript(readFileSync(new URL('./candidate.mjs',import.meta.url),'utf8'),policy),raw=emitRust({...next.program,...policy.checkedProgram}),oldRaw=emitRust(old.program);
 assert.equal(next.program.items.length,53);assert.equal(old.program.items.length,54);assert.equal(raw.definitions.length,53);assert.equal(raw.preludes.length,4);assert.equal(oldRaw.definitions.length,54);assert.equal(oldRaw.preludes.length,4);
 const rawSource=[...raw.preludes,...raw.definitions].join('\n\n'),oldRawSource=[...oldRaw.preludes,...oldRaw.definitions].join('\n\n');assert.equal(sha256(rawSource),'0e4c7c907ac0789d898ca9080bff2e27c80d08802c29ef7016743d9b0ae56ce6');assert.equal(sha256(oldRawSource),'c09bb04cd012bac8347176de503814978dd4fda1a11db647b754bd12a90bc88f');
 const stringPrelude=raw.preludes.find(source=>source.includes('pub mod ml_string {'));assert.equal(sha256(stringPrelude),receipt.stringPreludeSha256);assert.ok(oldRaw.preludes.includes(stringPrelude),'the entire original UTF16 String prelude remains unchanged');
 const candidateEntry=next.program.declarations.get('transcodeLinkNetworkBinary');assert.deepEqual(candidateEntry.params.map(row=>row.type),[{kind:'array',element:{kind:'float'}},{kind:'bool'},{kind:'float'},{kind:'float'}]);assert.deepEqual(candidateEntry.ret,{kind:'string'});
 const additionalSources=[],archives=[],declarations=[];assert.deepEqual(receipt.nativeArtifacts.map(row=>row.path),['part_0.rs','part_1.rs','part_2.rs','part_3.rs','mod.rs']);
 for(const artifact of receipt.nativeArtifacts){const source=readFileSync(new URL('./native/'+artifact.path,import.meta.url),'utf8');assert.equal(sha256(source),artifact.sha256,'every complete native artifact must match the frozen generation receipt');const relativePath='candidate/'+artifact.path;additionalSources.push({relativePath,source,sourceBytes:textBytes(source).length,sourceSha256:sha256(source)});archives.push(archiveReceipt(source,relativePath));if(artifact.path!=='mod.rs')declarations.push(...inventory(source,relativePath));}
 assert.equal(declarations.length,57);assert.deepEqual(declarations.map(row=>row.name),receipt.declarations.map(row=>row.name));assert.deepEqual(inventory(rawSource,'whole raw emission').map(row=>row.name),declarations.map(row=>row.name));
 const provider=readSourceTextRustBlock(readFileSync(new URL('rust/src/translation/frontend_rules.rs',root),'utf8'),'mod','ml_array');assert.equal(sha256(provider),'c2a535a8fbc2ab09deca3f7c92cad278b0a08a526f5f87a54e2aebb822c8b76a');additionalSources.push({relativePath:'provider_array.rs',source:provider,sourceBytes:textBytes(provider).length,sourceSha256:sha256(provider)});archives.push(archiveReceipt(provider,'provider_array.rs'));
 const header=nativeInterface(additionalSources.find(row=>row.relativePath==='candidate/part_1.rs').source),wrapper=archiveReceipt(librarySource,'original.rs');
 const observations=cases().map(row=>{const inputBits=row.input.map(numberBits),args=[row.input,row.binary,row.recordLimit,row.byteLimit],result=previous(...args);if(row.expected!==null)assert.equal(result,row.expected,row.id);assert.equal(candidate(...args),result,row.id);assert.equal(old.invoke(...args),result,row.id);assert.equal(next.invoke(...args),result,row.id);assert.deepEqual(row.input.map(numberBits),inputBits,'both whole JavaScript programs preserve every input bit');assert.match(result,/^[\x00-\x7f]*$/u);return{id:row.id,kind:row.kind,inputBits,binary:row.binary,recordLimitBits:numberBits(row.recordLimit),byteLimitBits:numberBits(row.byteLimit),result,resultHex:Buffer.from(result,'ascii').toString('hex')};});
 const errorCodes=[...new Set(observations.filter(row=>row.result.startsWith('-')).map(row=>Number(row.result.split(',')[0])))].sort((a,b)=>b-a);assert.deepEqual(errorCodes,Array.from({length:22},(_,index)=>-index-1));
 const consumer=consumerSource(observations,header),entry={id:'complete binary metadata buffer candidate bundle',kind:'unchanged complete candidate native generation',fixtureId:'experiments/binary-metadata-buffer/native-generation.lino',source:librarySource,sourceBytes:wrapper.sourceBytes,sourceSha256:wrapper.sourceSha256,replaySha256:wrapper.binaryReplaySha256,originalReceipt:wrapper,additionalSources,archives,...header,declarations,
  declarationSha256:sha256(JSON.stringify(declarations)),sourceModelSha256:hash('experiments/binary-metadata-buffer/native-policy.lino'),generationReceiptSha256:hash('experiments/binary-metadata-buffer/native-generation.lino'),candidateRawSha256:sha256(rawSource),baselineRawSha256:sha256(oldRawSource),generationPolicy:policy,baselineDeclarations:54,candidateDeclarations:53,candidatePreludeCount:4,
  sourceBundleSha256:sha256(JSON.stringify(additionalSources.map(({relativePath,sourceBytes,sourceSha256})=>({relativePath,sourceBytes,sourceSha256})))),invocationCount:observations.length,errorCodes,maximumObservedInputCells:Math.max(...observations.map(row=>row.inputBits.length)),maximumObservedResultBytes:Math.max(...observations.map(row=>row.result.length)),
  observationScope:'Exact bounded v1/v2 bytes, flat fields, all twenty-two errors, original offsets, limits and precedence for this complete isolated candidate bundle. These observations establish no production selection, whole-crate parity, project completion or performance improvement.',consumerSource:consumer,consumerSha256:sha256(consumer),javaScriptSha256:sha256(next.emitted.text),baselineJavaScriptSha256:sha256(old.emitted.text),observations};
 const plan=freeze({model:policy.model,modelCommit:BINARY_METADATA_BUFFER_MODEL_COMMIT,modelSources,originalCount:1,entries:[entry]});checkedPlans.add(plan);return plan;
}
function parseObservations(output,expected){const lines=output.split('\n');assert.equal(lines.pop(),'','native framing output ends with one newline');assert.equal(lines.length,expected.length,'every complete actual invocation must be reported');return lines.map((line,index)=>{const parts=line.split(':');assert.equal(parts.length,2);assert.equal(parts[0],String(index),'actual invocation order is exact');assert.match(parts[1],/^(?:[0-9a-f]{2})*$/u);assert.equal(parts[1],expected[index].resultHex,'every actual result byte and error offset matches both JavaScript models');return{id:expected[index].id,kind:expected[index].kind,inputCells:expected[index].inputBits.length,inputSha256:sha256(expected[index].inputBits.join('')),binary:expected[index].binary,recordLimitBits:expected[index].recordLimitBits,byteLimitBits:expected[index].byteLimitBits,resultHex:parts[1],result:Buffer.from(parts[1],'hex').toString('ascii')};});}
export const runBinaryMetadataBufferNative=createSourceNativeObservationRunner({nativeMode:binaryMetadataBufferNativeMode,checkedPlan:plan=>checkedPlans.has(plan),textBytes,parseObservations,
 entryReceipt:entry=>({functionName:entry.functionName,parameterOrder:entry.parameterOrder,parameterModes:entry.parameterModes,resultMode:entry.resultMode,interfaceSha256:entry.interfaceSha256,declarations:entry.declarations,declarationSha256:entry.declarationSha256,archives:entry.archives,sourceModelSha256:entry.sourceModelSha256,generationReceiptSha256:entry.generationReceiptSha256,candidateRawSha256:entry.candidateRawSha256,baselineRawSha256:entry.baselineRawSha256,generationPolicy:entry.generationPolicy,baselineDeclarations:entry.baselineDeclarations,candidateDeclarations:entry.candidateDeclarations,candidatePreludeCount:entry.candidatePreludeCount,sourceBundleSha256:entry.sourceBundleSha256,invocationCount:entry.invocationCount,errorCodes:entry.errorCodes,maximumObservedInputCells:entry.maximumObservedInputCells,maximumObservedResultBytes:entry.maximumObservedResultBytes,baselineJavaScriptSha256:entry.baselineJavaScriptSha256,observationScope:entry.observationScope}),
 temporaryPrefix:'binary-metadata-buffer-native-',failureLabel:'Complete binary metadata buffer candidate observations failed'});
