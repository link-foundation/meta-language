// CI-only complete-original controls. Do not invoke during bulk source drafting.
// Each assertion records actual outputs only after its operation has executed.
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {mkdirSync,mkdtempSync,writeFileSync,rmSync} from 'node:fs';
import {join,resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import vm from 'node:vm';
import {createHash} from 'node:crypto';
import {rustStateSourceOriginalGroups} from './rust-state-source-original-groups.mjs';
import {writeProjectData} from './project-data.mjs';
import {createControlDiagnostics,requireControlProduct} from '../tests/support/control-diagnostics.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {loadRustStandardCollectionSourceLibrary} from '../src/translation/rust-standard-collection-library.js';
import {readActiveRuntimeDataVersions} from '../src/translation/runtime-data-module.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {createActualRustTypeMapChildrenControlGroup,createActualRustTypeChildrenControlGroup,createActualRustTypeRegionCompositionControlGroup} from './rust-type-map-children-source-controls.mjs';
import {compileRustStateNativeOriginal} from './rust-state-native-original-compiler.mjs';
const repository=fileURLToPath(new URL('../../',import.meta.url));
function requireNativeCI(){if(process.env.CI!=='true'||process.env.META_LANGUAGE_NATIVE_RUST_STATE!=='1')throw new Error('native Rust state source controls run only in explicitly selected CI');}
const digest=text=>createHash('sha256').update(text).digest('hex');
function command(executable,args,directory,{timeoutMilliseconds=60000,maxOutputBytes=2*1024*1024,developmentDebug}={}){
 requireNativeCI();const environment={...process.env};delete environment.CARGO_TARGET_DIR;delete environment.CARGO_BUILD_TARGET_DIR;
 assert.ok(Number.isSafeInteger(timeoutMilliseconds)&&timeoutMilliseconds>0&&timeoutMilliseconds<=900000);assert.ok(Number.isSafeInteger(maxOutputBytes)&&maxOutputBytes>0&&maxOutputBytes<=8*1024*1024);if(developmentDebug!==undefined){assert.equal(developmentDebug,'0');environment.CARGO_PROFILE_DEV_DEBUG=developmentDebug;}
 const result=spawnSync(executable,args,{cwd:repository,encoding:'utf8',timeout:timeoutMilliseconds,maxBuffer:maxOutputBytes,detached:process.platform!=='win32',env:environment});
 if(result.error&&result.pid>0&&process.platform!=='win32'){try{process.kill(-result.pid,'SIGKILL');}catch{ /* This task-owned group may already have exited. */ }}
 const measured={executable,args,settings:{timeoutMilliseconds,maxOutputBytes,developmentDebug:developmentDebug??null},exitCode:result.status,signal:result.signal,stdout:result.stdout??'',stderr:result.stderr??'',...(result.error?{error:String(result.error.message)}:{})};if(directory)writeProjectData(join(directory,'command-'+digest(executable+'\0'+args.join('\0')).slice(0,16)+'.lino'),measured);return measured;
}
function successful(measured){assert.equal(measured.error,undefined);assert.equal(measured.signal,null);assert.equal(measured.exitCode,0,measured.stderr);return measured;}
function compilerProfile(){
 const output=successful(command('rustc',['--version'])).stdout,match=/^rustc (1\.(\d+)\.(\d+)) /u.exec(output);assert.ok(match&&Number(match[2])>=90&&Number(match[2])<100,'uncovered Rust '+output.trim());
 const verbose=successful(command('rustc',['-vV'])).stdout,host=/^host: ([A-Za-z0-9_-]+)$/mu.exec(verbose)?.[1];assert.ok(host,'an actual native host target is required');
 const cargoOutput=successful(command('cargo',['--version'])).stdout,cargo=/^cargo (1\.(\d+)\.(\d+)) /u.exec(cargoOutput);assert.ok(cargo&&Number(cargo[2])>=90&&Number(cargo[2])<100,'uncovered Cargo '+cargoOutput.trim());
 const configuration=successful(command('rustc',['--print','cfg'])).stdout;assert.match(configuration,/^target_pointer_width="64"$/mu,'uncovered source frontend pointer width; the kernel 32-bit matrix remains separate');
 const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered Node '+process.versions.node+'; requires >=22.15 <23 or >=24 <25');const runtime=readActiveRuntimeDataVersions();return {rust:match[1],rustOutput:output,rustVerboseOutput:verbose,host,cargo:cargo[1],cargoOutput,node:process.versions.node,linksVersion:runtime.linksVersion,unicode:process.versions.unicode,v8:process.versions.v8,platform:process.platform,architecture:process.arch,pointerWidth:64,edition:2024};
}
function javaScriptOutput(text){const lines=[];let thrown;try{vm.runInNewContext(text,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}},{timeout:5000});}catch(error){thrown={name:String(error?.name??'Error'),message:String(error?.message??error)};}return {stdout:lines.length?lines.join('\n')+'\n':'',...(thrown?{thrown}:{})};}
export function runRustStateSourceOriginals(){
 requireNativeCI();const commit=successful(command('git',['rev-parse','HEAD'])).stdout.trim();assert.match(commit,/^[a-f0-9]{40}$/u);
 const directory=join(repository,'issue-195-artifacts','rust-state-source',commit),work=join(directory,'work');mkdirSync(work,{recursive:true});
 const header=createControlDiagnostics(),profile=header.attempt('covered actual compiler and target versions',compilerProfile),library=header.attempt('complete shipped collection library bodies',()=>loadRustStandardCollectionSourceLibrary({invalid:()=>({k:'abort',message:'invalid collection source packet'}),resource:()=>({k:'abort',message:'collection source resource bound'})}));
 const dependencyTarget=mkdtempSync(join(work,'cargo-target-'));writeProjectData(join(dependencyTarget,'owner.lino'),{model:'task-owned Rust source control dependency target 1',commit});
 const nativeContext={dependencyTarget,command,successful,profile},groups=[...rustStateSourceOriginalGroups,createActualRustTypeMapChildrenControlGroup(header,directory,nativeContext),createActualRustTypeChildrenControlGroup(header,directory,nativeContext),createActualRustTypeRegionCompositionControlGroup(header,directory,nativeContext)];
 const records=[];let failures=header.failures.length;
 for(const group of groups)for(const [index,fixture]of group.rows.entries()){
  const diagnostics=createControlDiagnostics(),caseDirectory=join(directory,group.id+'-'+String(index).padStart(4,'0')),temporary=mkdtempSync(join(work,'case-'));mkdirSync(caseDirectory,{recursive:true});writeProjectData(join(temporary,'owner.lino'),{model:'task-owned Rust source control scratch 1',commit,group:group.id,index});
  const originalPath=join(caseDirectory,'original.rs'),frontendPath=join(caseDirectory,'frontend.rs'),productPath=join(caseDirectory,'product.rs'),originalBinary=join(temporary,'original'),projectionBinary=join(temporary,'projection'),productBinary=join(temporary,'product');let sourceInput;
  try{
   sourceInput=diagnostics.attempt('complete actual original and frontend source derivation',()=>{const value=fixture.sourceFactory?fixture.sourceFactory():fixture;assert.equal(value.name,fixture.name);assert.equal(value.stdout,fixture.stdout);assert.equal(typeof value.source,'string');writeFileSync(originalPath,value.nativeSource??value.source);writeFileSync(frontendPath,value.source);if(value.capture!==undefined)writeProjectData(join(caseDirectory,'source-capture.lino'),value.capture);return value;});
   const active=diagnostics.attempt('active source version coverage',()=>{assert.equal(fixture.sourceVersionRange??group.sourceVersionRange,'>=1.90.0 <1.100.0','uncovered per-original language range requires an explicit interpretation');return requireControlProduct(profile,'actual compiler profile');});
   const original=diagnostics.attempt('independent original Rust compilation',()=>{const version=requireControlProduct(active,'source version');return compileRustStateNativeOriginal(requireControlProduct(sourceInput,'actual original source'),{originalPath,originalBinary,temporary,dependencyTarget,caseDirectory,command,successful,target:version.host});});
   const originalRun=diagnostics.attempt('independent original Rust stdout',()=>{requireControlProduct(original,'original executable');const output=successful(command(originalBinary,[],caseDirectory));assert.equal(output.stdout,fixture.stdout);return output;});
   if(group.valueProjection)diagnostics.attempt('actual crate method stdout equals captured original and literal oracle',()=>{const reference=requireControlProduct(group.ownCrateReference,'actual locked crate reference');assert.equal(reference.cases[index].name,fixture.name);const output=successful(command(reference.binary,[String(index)],caseDirectory));assert.equal(output.stdout,fixture.stdout);assert.equal(output.stdout,requireControlProduct(originalRun,'captured full original stdout').stdout);});
   if(group.valueProjection){
    const projected=diagnostics.attempt('independent projected value declaration Rust compilation',()=>{const version=requireControlProduct(active,'source version');requireControlProduct(sourceInput,'actual projected source');return successful(command('rustc',['--edition=2024','--target',version.host,'--crate-name','meta_language_projection',frontendPath,'-o',projectionBinary],caseDirectory));});
    diagnostics.attempt('projected value declaration stdout equals full original and literal oracle',()=>{requireControlProduct(projected,'projected executable');const output=successful(command(projectionBinary,[],caseDirectory));assert.equal(output.stdout,fixture.stdout);assert.equal(output.stdout,requireControlProduct(originalRun,'full original stdout').stdout);});
   }
   const checked=diagnostics.attempt('actual source frontend and checked current program',()=>{const version=requireControlProduct(active,'source version'),options={sourceVersion:version.rust,...(group.exclusive===false?{}:{exclusiveSourceModel:'Rust exclusive source loans 1'}),...(group.sourceSignatureModel?{sourceSignatureModel:group.sourceSignatureModel}:{}),...(group.fnMut?{fnMutSourceModel:'Rust closed source FnMut specializations 1'}:{}),...(group.collections?{standardCollectionSourceModel:'Rust closed standard collections 1',standardCollectionSourceLibrary:requireControlProduct(library,'complete actual library'),standardCollectionPointerWidth:64}:{})};return checkProgram(parseRust(requireControlProduct(sourceInput,'actual frontend source').source,options));});
   const javascript=diagnostics.attempt('JavaScript emission from the checked current program',()=>{const product=emitJavaScript(requireControlProduct(checked,'checked program'));writeFileSync(join(caseDirectory,'product.mjs'),product.text);return product;});
   diagnostics.attempt('emitted JavaScript stdout equals the original and literal oracle',()=>{const actual=javaScriptOutput(requireControlProduct(javascript,'JavaScript product').text);writeProjectData(join(caseDirectory,'javascript-output.lino'),actual);assert.equal(actual.thrown,undefined);assert.equal(actual.stdout,fixture.stdout);assert.equal(actual.stdout,requireControlProduct(originalRun,'original stdout').stdout);});
   const rust=diagnostics.attempt('Rust emission from the checked current program',()=>{const product=emitRust(requireControlProduct(checked,'checked program'));writeFileSync(productPath,product.text);return product;});
   const typed=diagnostics.attempt('encode actual checked program as typed Links',()=>encodeTypedProgramLinks(requireControlProduct(checked,'checked program')));
   for(const format of ['text','binary']){
    const restored=diagnostics.attempt('typed Links '+format+' transport and independent current graph recheck',()=>{
     const packet=requireControlProduct(typed,'typed program'),bytes=format==='text'?packet.network.toLino():encodeLinkNetworkBinary(packet.network),network=format==='text'?LinkNetwork.fromLino(bytes):decodeLinkNetworkBinary(bytes),program=decodeTypedProgramLinks(network,packet.root);recheckLinkedTypedProgram(program,{wholeProgram:true});
     // Complete common libraries already belong to the recorded commit and
     // shipped assets. Retain transport identity without writing their full
     // duplicated graph once for every original.
     writeProjectData(join(caseDirectory,'typed-'+format+'-transport.lino'),{format,root:String(packet.root),sha256:digest(bytes),bytes:typeof bytes==='string'?Buffer.byteLength(bytes):bytes.byteLength,nodes:network.links().length});return program;
    });
    diagnostics.attempt('typed Links '+format+' JavaScript product and original stdout',()=>{const product=emitJavaScript(requireControlProduct(restored,'restored typed program'));assert.equal(product.text,requireControlProduct(javascript,'original JavaScript product').text);const output=javaScriptOutput(product.text);writeProjectData(join(caseDirectory,'typed-'+format+'-javascript-output.lino'),output);assert.equal(output.thrown,undefined);assert.equal(output.stdout,fixture.stdout);assert.equal(output.stdout,requireControlProduct(originalRun,'original stdout').stdout);});
    diagnostics.attempt('typed Links '+format+' Rust product equals the selected Rust product bytes',()=>assert.equal(emitRust(requireControlProduct(restored,'restored typed program')).text,requireControlProduct(rust,'original Rust product').text));
   }
   const native=diagnostics.attempt('actual emitted Rust compilation',()=>{const version=requireControlProduct(active,'source version');requireControlProduct(rust,'emitted Rust product');return successful(command('rustc',['--edition=2024','--target',version.host,'--crate-name','meta_language_product',productPath,'-o',productBinary],caseDirectory));});
   diagnostics.attempt('emitted Rust stdout equals the original and literal oracle',()=>{requireControlProduct(native,'emitted executable');const output=successful(command(productBinary,[],caseDirectory));assert.equal(output.stdout,fixture.stdout);assert.equal(output.stdout,requireControlProduct(originalRun,'original stdout').stdout);});
   const concrete=diagnostics.attempt('default complete CST reconstructs original bytes',()=>{const input=requireControlProduct(sourceInput,'actual original source'),source=input.nativeSource??input.source,graph=LinkNetwork.parse(source,'Rust');writeFileSync(join(caseDirectory,'concrete.lino'),graph.toLino());assert.equal(graph.reconstructText(),source);return graph;});
   diagnostics.attempt('complete CST accepts the independently compiled original without recovery',()=>assert.equal(requireControlProduct(concrete,'concrete graph').verifyFullMatch().isClean(),true));
   diagnostics.attempt('complete CST survives Links text transport',()=>{const input=requireControlProduct(sourceInput,'actual original source');assert.equal(LinkNetwork.fromLino(requireControlProduct(concrete,'concrete graph').toLino()).reconstructText(),input.nativeSource??input.source);});
   diagnostics.attempt('complete CST survives binary Links transport',()=>{const input=requireControlProduct(sourceInput,'actual original source');assert.equal(decodeLinkNetworkBinary(encodeLinkNetworkBinary(requireControlProduct(concrete,'concrete graph'))).reconstructText(),input.nativeSource??input.source);});
   if(group.valueProjection)diagnostics.attempt('projected value source retains its own clean complete CST and both transports',()=>{const source=requireControlProduct(sourceInput,'actual projected source').source,graph=LinkNetwork.parse(source,'Rust');writeFileSync(join(caseDirectory,'frontend-concrete.lino'),graph.toLino());assert.equal(graph.reconstructText(),source);assert.equal(graph.verifyFullMatch().isClean(),true);assert.equal(LinkNetwork.fromLino(graph.toLino()).reconstructText(),source);assert.equal(decodeLinkNetworkBinary(encodeLinkNetworkBinary(graph)).reconstructText(),source);});
  }finally{rmSync(temporary,{recursive:true,force:true});}
  const record={group:group.id,index,name:fixture.name,sourceSha256:sourceInput?digest(sourceInput.source):null,nativeSourceSha256:sourceInput?digest(sourceInput.nativeSource??sourceInput.source):null,assertions:diagnostics.assertions,failures:diagnostics.failures};writeProjectData(join(caseDirectory,'assertions.lino'),{commit,...record});records.push(record);failures+=record.failures.length;globalThis.gc?.();
 }
 rmSync(dependencyTarget,{recursive:true,force:true});
 const report={model:'complete Rust state original and emitted product controls 1',commit,profile:profile??null,header:{assertions:header.assertions,failures:header.failures},records};writeProjectData(join(directory,'controls.lino'),report);return {report,failures,directory};
}
if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){if(process.argv.slice(2).join(' ')!=='--execute')throw new Error('use --execute only in explicit native CI');const result=runRustStateSourceOriginals();process.stdout.write(result.report.records.length+' complete originals; '+result.failures+' failed assertions; '+result.directory+'\n');if(result.failures)process.exitCode=1;}
