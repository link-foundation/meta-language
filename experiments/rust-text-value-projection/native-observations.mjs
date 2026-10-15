// CI-only external Rust oracle. The default module never starts a process.
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {existsSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import vm from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../js/src/index.js';
import {acceptRustOwnedLocalTextInput} from '../../js/src/translation/rust-owned-local-state-rules.js';
import {createRustTextValueProjectionLinks,decodeRustTextValueProjection,emitRustTextValueProjection} from './model.mjs';

export const RUST_TEXT_VALUE_PROJECTION_MODEL_COMMIT='0e78a59417dd5d7861abee9dd3b46c4ee7539ae4';
const modelDigests={
 'model.mjs':'d2be6a924798984c3393adf1050ebbb74b8159be79e8392e38e84b5bbcf7db56',
 'source.mjs':'6d64305d0a5b65de68e36d6e62db25b5f6bf6822b7ed3d0ca47ba7790a6565f5',
 'rules.mjs':'890472bec11e6095ffcca50fff363ae0d5538bf1cd639f61f645051d970f952b',
 'data/policy.lino':'0287e8b72b3b73fd82c5deac38f78a191e9426b3f5f7ed47c043f15c68da7272',
 'data/fixtures.lino':'6fd937e41d61f0fcd2bae6fb9b2fd9d0d358e775234408cf434843e853f8bce4',
};
const sha256=value=>createHash('sha256').update(value).digest('hex');
const checkedPlans=new WeakSet();
const textBytes=value=>{if(typeof value!=='string'||!acceptRustOwnedLocalTextInput(value))throw new TypeError('oracle text must be primitive Unicode scalar text');return Buffer.from(value,'utf8');};
function freeze(value){if(value&&typeof value==='object'){Object.values(value).forEach(freeze);Object.freeze(value);}return value;}
export function verifyRustTextValueProjectionModel(read=readFileSync){
 return Object.entries(modelDigests).map(([file,expected])=>{
  const actual=sha256(read(new URL(file,import.meta.url)));assert.equal(actual,expected,'the independently reviewed model changed: '+file);
  return{file,sha256:actual};
 });
}
export function rustTextNativeMode(environment=process.env){
 if(environment.RUST_TEXT_VALUE_PROJECTION_NATIVE!=='1')return false;
 if(environment.CI!=='true')throw new TypeError('RUST_TEXT_VALUE_PROJECTION_NATIVE=1 requires CI=true; native compilation is forbidden locally');
 return true;
}
function archiveSource(source){
 const packet=createRustTextValueProjectionLinks(source),text=LinkNetwork.fromLino(packet.network.toLino());
 const binary=decodeLinkNetworkBinary(encodeLinkNetworkBinary(text));
 return decodeRustTextValueProjection(binary,packet.root);
}
function parameterMode(position){
 assert.equal(position.type.kind,'string');assert.deepEqual(position.owner.arguments,[]);
 const standard=position.owner.kind==='library'&&position.owner.path.join('::')==='std::string::String';
 if(standard&&position.wrappers.length===0&&position.lifetimes.length===0)return'owned String';
 if(standard&&position.wrappers.length===1&&position.wrappers[0]==='reference'&&position.lifetimes.length===1&&position.lifetimes[0]===null)return'shared String';
 if(position.owner.kind==='primitive'&&position.owner.path.join('::')==='str'&&position.wrappers.length===1&&position.wrappers[0]==='reference'&&position.lifetimes.length===1&&position.lifetimes[0]===null)return'borrowed str';
 throw new TypeError('the external oracle has no argument construction for this actual source position');
}
function publicTarget(program,proof){
 const entry=program.declarations.get('choose');assert.ok(entry&&entry.k==='fn'&&entry.fullName==='choose'&&entry.name==='choose');
 const row=proof.functions.find(row=>row.declaration===entry);assert.ok(row&&row.signature===entry.sourceSignature);
 const header=proof.operations.parts.find(node=>node.kind==='source operation'&&node.op==='text value function'&&node.parts.some(token=>token.declaration===entry));
 assert.equal(header?.parts[0]?.value,'pub','the external consumer requires the actual public declaration header');
 assert.equal(parameterMode(entry.sourceSignature.result),'owned String','the actual return is an owned standard String');
 return{entry,parameters:entry.sourceSignature.parameters.map(parameterMode)};
}
function consumerSource(name,modes,observations){
 assert.match(name,/^[A-Za-z_][A-Za-z0-9_]*$/u);
 const blocks=observations.map(({args},index)=>{
  assert.equal(args.length,modes.length,'fixture arguments must match the actual source signature');
  const variables=args.map((arg,slot)=>`let argument_${slot}=String::from_utf8(vec![${[...textBytes(arg)].map(byte=>byte+'u8').join(',')}]).expect("scalar fixture input");`);
  const actual=modes.map((mode,slot)=>mode==='owned String'?`argument_${slot}`:mode==='shared String'?`&argument_${slot}`:`argument_${slot}.as_str()`);
  return`{\n${variables.join('\n')}\nlet observed:String=oracle_source::${name}(${actual.join(',')});\nline.push_str("${index}:");\nfor byte in observed.as_bytes(){write!(&mut line,"{byte:02x}").expect("format observed bytes");}\nline.push('\\n');\n}`;
 });
 return`extern crate oracle_source;\nuse std::fmt::Write as _;\nfn main(){\nlet mut line=String::new();\n${blocks.join('\n')}\nprint!("{line}");\n}\n`;
}
function preparedEntry(id,kind,fixtureId,source,observations,originalReceipt){
 const value=archiveSource(source),replay=emitRustTextValueProjection(value.program,value.proof,'Rust').text;
 assert.deepEqual(textBytes(replay),textBytes(source),'the entire original Rust module must survive text and binary Links replay byte for byte');
 const target=publicTarget(value.program,value.proof),javascript=emitRustTextValueProjection(value.program,value.proof,'JavaScript'),context={};
 vm.runInNewContext(javascript.text,context,{timeout:1000});const name=javascript.mappings.find(row=>row.kind==='function'&&row.source===target.entry.fullName)?.target;assert.ok(name&&typeof context[name]==='function');
 const expected=observations.map(({args,value})=>{
  args.forEach(textBytes);textBytes(value);assert.equal(context[name](...args),value,'the actual whole JavaScript model must agree with the independent fixture value');
  return{args:[...args],value,utf8Hex:textBytes(value).toString('hex')};
 });
 const consumer=consumerSource(target.entry.name,target.parameters,expected);
 return{id,kind,fixtureId,source,sourceBytes:textBytes(source).length,sourceSha256:sha256(textBytes(source)),originalReceipt,
  replaySha256:sha256(textBytes(replay)),archiveFormats:['Links Notation text','binary links'],functionName:target.entry.name,parameterModes:target.parameters,
  consumerSource:consumer,consumerSha256:sha256(consumer),javaScriptSha256:sha256(javascript.text),observations:expected};
}
function editedSources(fixtures){
 const row=fixtures.cases.find(row=>row.id==='valid-clone-retarget'),value=archiveSource(row.source),entry=value.program.declarations.get('choose');
 const fact=value.proof.functions.find(fact=>fact.declaration===entry),target=entry.params[1],position=entry.sourceSignature.parameters[1];
 const token=value.proof.tokens.find(token=>token.binding===fact.input&&token.span.start===fact.coreValue.span.start);assert.ok(token);
 token.binding=target;entry.body.name=target.name;fact.input=target;fact.position=position;
 for(const condition of[...value.proof.inputAssumptions,...value.proof.valueAssumptions])if(condition.declaration===entry){condition.input=target;condition.position=position;}
 const retarget=emitRustTextValueProjection(value.program,value.proof,'Rust').text;
 const result=[preparedEntry('valid-clone-retarget-edited','coherent clone retarget',row.id,retarget,row.observations.map(observation=>({args:observation.args,value:observation.args[1]})),{sourceBytes:row.sourceBytes,sourceSha256:row.sourceSha256})];
 for(const[id,edited]of[['raw-literal-construction','raw edited "\n🌊'],['literal-owned',String.fromCharCode(...Array.from({length:32},(_,index)=>index))+'\u007fΩ😀']]){
  const original=fixtures.cases.find(row=>row.id===id),current=archiveSource(original.source),literal=current.proof.functions[0].literal;assert.ok(literal);
  literal.value=edited;const source=emitRustTextValueProjection(current.program,current.proof,'Rust').text;
  result.push(preparedEntry(id+'-edited','coherent literal edit',id,source,[{args:[],value:edited}],{sourceBytes:original.sourceBytes,sourceSha256:original.sourceSha256}));
 }
 return result;
}
/** Planning executes only JavaScript; it never probes or invokes a compiler. */
export function planRustTextValueProjectionNative({fixtures=readProjectData(new URL('./data/fixtures.lino',import.meta.url)),includeEdits=true}={}){
 const modelSources=verifyRustTextValueProjectionModel();assert.deepEqual(fixtures,readProjectData(new URL('./data/fixtures.lino',import.meta.url)),'the oracle requires the unchanged independently frozen fixture records');assert.equal(fixtures.cases.length,13,'all thirteen complete original fixture modules are required');
 const entries=fixtures.cases.map(row=>{
  const bytes=textBytes(row.source);assert.equal(bytes.length,row.sourceBytes,row.id+' original byte length');assert.equal(sha256(bytes),row.sourceSha256,row.id+' original source hash');
  return preparedEntry(row.id,'complete unmodified original',row.id,row.source,row.observations,{sourceBytes:row.sourceBytes,sourceSha256:row.sourceSha256});
 });
 if(includeEdits)entries.push(...editedSources(fixtures));
 const plan=freeze({model:fixtures.model,modelCommit:RUST_TEXT_VALUE_PROJECTION_MODEL_COMMIT,modelSources,originalCount:13,entries});checkedPlans.add(plan);return plan;
}
function checkFiles(entry,files){
 assert.deepEqual(readFileSync(files.source),textBytes(entry.source),entry.id+' original module bytes changed');
 assert.equal(sha256(readFileSync(files.source)),entry.sourceSha256,entry.id+' source receipt changed');
 assert.equal(sha256(readFileSync(files.consumer)),entry.consumerSha256,entry.id+' generated external consumer changed');
}
function runProcess(spawn,command,args,options,label){
 const result=spawn(command,args,{...options,shell:false,encoding:'utf8',maxBuffer:1024*1024});
 if(result.error)throw new Error(label+': '+(result.error.code==='ETIMEDOUT'?'timed out after '+options.timeout+'ms':result.error.message),{cause:result.error});
 if(result.signal)throw new Error(label+': terminated by '+result.signal);
 if(result.status!==0)throw new Error(label+': exit '+result.status+'; '+String(result.stderr??'').slice(0,4096));
 if(typeof result.stdout!=='string')throw new TypeError(label+': stdout must be actual UTF-8 text');
 return result.stdout;
}
function parseObservations(output,expected){
 const lines=output.split('\n');assert.equal(lines.pop(),'','the native hex stream must end with a newline');assert.equal(lines.length,expected.length,'the native stream must report every actual call exactly once');
 return lines.map((line,index)=>{
  const match=/^(0|[1-9][0-9]*):((?:[0-9a-f]{2})*)$/u.exec(line);assert.ok(match,'the native stream has malformed index or UTF-8 hex');assert.equal(Number(match[1]),index,'native observations must retain their actual order');
  const bytes=Buffer.from(match[2],'hex'),value=new TextDecoder('utf-8',{fatal:true,ignoreBOM:true}).decode(bytes);
  assert.equal(bytes.toString('hex'),expected[index].utf8Hex,'actual original Rust bytes differ from the JavaScript model and fixture');assert.equal(value,expected[index].value);
  return{argumentsUtf8Hex:expected[index].args.map(arg=>textBytes(arg).toString('hex')),utf8Hex:match[2],value};
 });
}
/** Both flags are mandatory, even when an injected process is used by contracts. */
export function runRustTextValueProjectionNative(plan,{environment=process.env,spawn=spawnSync,compiler='rustc',temporaryRoot=tmpdir(),compilerTimeout=30000,executionTimeout=10000}={}){
 if(!rustTextNativeMode(environment))throw new TypeError('native observations require the explicit CI-only switch');
 if(!checkedPlans.has(plan))throw new TypeError('native execution requires the exact checked in-memory observation plan');
 if(typeof spawn!=='function'||typeof compiler!=='string'||!compiler)throw new TypeError('the native runner requires an actual process function and compiler path');
 const observations=[],errors=[],processCounts={libraryCompilations:0,consumerCompilations:0,executions:0};
 for(const entry of plan.entries){
  let directory;
  try{
   assert.equal(textBytes(entry.source).length,entry.sourceBytes);assert.equal(sha256(textBytes(entry.source)),entry.sourceSha256);assert.equal(sha256(entry.consumerSource),entry.consumerSha256);
   directory=mkdtempSync(join(temporaryRoot,'rust-text-original-'));
   const files={source:join(directory,'original.rs'),consumer:join(directory,'consumer.rs'),library:join(directory,'liboracle_source.rlib'),executable:join(directory,'consumer')};
   writeFileSync(files.source,textBytes(entry.source));writeFileSync(files.consumer,entry.consumerSource);checkFiles(entry,files);
   const options={cwd:directory,env:environment,timeout:compilerTimeout};
   const libraryArgs=['--edition=2024','--crate-type=rlib','--crate-name','oracle_source',files.source,'-o',files.library];
   processCounts.libraryCompilations++;runProcess(spawn,compiler,libraryArgs,options,entry.id+' original library compiler');checkFiles(entry,files);assert.ok(existsSync(files.library),'the compiler did not create the original library');
   const consumerArgs=['--edition=2024',files.consumer,'--extern','oracle_source='+files.library,'-o',files.executable];
   processCounts.consumerCompilations++;runProcess(spawn,compiler,consumerArgs,options,entry.id+' external consumer compiler');checkFiles(entry,files);assert.ok(existsSync(files.executable),'the compiler did not create the external consumer');
   processCounts.executions++;const stdout=runProcess(spawn,files.executable,[],{...options,timeout:executionTimeout},entry.id+' external original function execution');checkFiles(entry,files);
   observations.push({id:entry.id,kind:entry.kind,fixtureId:entry.fixtureId,sourceBytes:entry.sourceBytes,sourceSha256:entry.sourceSha256,replaySha256:entry.replaySha256,
    originalReceipt:entry.originalReceipt,functionName:entry.functionName,parameterModes:entry.parameterModes,consumerSha256:entry.consumerSha256,javaScriptSha256:entry.javaScriptSha256,
    compiler,compilerArguments:[libraryArgs,consumerArgs],observations:parseObservations(stdout,entry.observations)});
  }catch(error){errors.push(new Error(entry.id+': '+error.message,{cause:error}));}
  finally{if(directory)rmSync(directory,{recursive:true,force:true});}
 }
 if(errors.length)throw new AggregateError(errors,'Original Rust text observations failed:\n'+errors.map(error=>error.message).join('\n'));
 return{model:plan.model,modelCommit:plan.modelCommit,modelSources:plan.modelSources,repositoryCommit:environment.GITHUB_SHA??'',originalCount:plan.originalCount,processCounts,observations};
}
