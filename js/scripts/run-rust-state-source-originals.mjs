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
const repository=fileURLToPath(new URL('../../',import.meta.url));
function requireNativeCI(){if(process.env.CI!=='true'||process.env.META_LANGUAGE_NATIVE_RUST_STATE!=='1')throw new Error('native Rust state source controls run only in explicitly selected CI');}
const digest=text=>createHash('sha256').update(text).digest('hex');
function command(executable,args,directory){
 requireNativeCI();const environment={...process.env};delete environment.CARGO_TARGET_DIR;delete environment.CARGO_BUILD_TARGET_DIR;
 const result=spawnSync(executable,args,{cwd:repository,encoding:'utf8',timeout:60000,maxBuffer:2*1024*1024,detached:process.platform!=='win32',env:environment});
 if(result.error&&result.pid>0&&process.platform!=='win32'){try{process.kill(-result.pid,'SIGKILL');}catch{ /* This task-owned group may already have exited. */ }}
 const measured={executable,args,exitCode:result.status,signal:result.signal,stdout:result.stdout??'',stderr:result.stderr??'',...(result.error?{error:String(result.error.message)}:{})};if(directory)writeProjectData(join(directory,'command-'+digest(executable+'\0'+args.join('\0')).slice(0,16)+'.lino'),measured);return measured;
}
function successful(measured){assert.equal(measured.error,undefined);assert.equal(measured.signal,null);assert.equal(measured.exitCode,0,measured.stderr);return measured;}
function compilerProfile(){
 const output=successful(command('rustc',['--version'])).stdout,match=/^rustc (1\.(\d+)\.(\d+)) /u.exec(output);assert.ok(match&&Number(match[2])>=90&&Number(match[2])<100,'uncovered Rust '+output.trim());
 const configuration=successful(command('rustc',['--print','cfg'])).stdout;assert.match(configuration,/^target_pointer_width="64"$/mu,'uncovered source frontend pointer width; the kernel 32-bit matrix remains separate');
 const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered Node '+process.versions.node+'; requires >=22.15 <23 or >=24 <25');return {rust:match[1],rustOutput:output,node:process.versions.node,pointerWidth:64,edition:2024};
}
function javaScriptOutput(text){const lines=[];let thrown;try{vm.runInNewContext(text,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}},{timeout:5000});}catch(error){thrown={name:String(error?.name??'Error'),message:String(error?.message??error)};}return {stdout:lines.length?lines.join('\n')+'\n':'',...(thrown?{thrown}:{})};}
export function runRustStateSourceOriginals(){
 requireNativeCI();const commit=successful(command('git',['rev-parse','HEAD'])).stdout.trim();assert.match(commit,/^[a-f0-9]{40}$/u);
 const directory=join(repository,'issue-195-artifacts','rust-state-source',commit),work=join(directory,'work');mkdirSync(work,{recursive:true});
 const header=createControlDiagnostics(),profile=header.attempt('covered actual compiler and target versions',compilerProfile),library=header.attempt('complete shipped collection library bodies',()=>loadRustStandardCollectionSourceLibrary({invalid:()=>({k:'abort',message:'invalid collection source packet'}),resource:()=>({k:'abort',message:'collection source resource bound'})}));
 const records=[];let failures=header.failures.length;
 for(const group of rustStateSourceOriginalGroups)for(const [index,fixture]of group.rows.entries()){
  const diagnostics=createControlDiagnostics(),caseDirectory=join(directory,group.id+'-'+String(index).padStart(4,'0')),temporary=mkdtempSync(join(work,'case-'));mkdirSync(caseDirectory,{recursive:true});writeProjectData(join(temporary,'owner.lino'),{model:'task-owned Rust source control scratch 1',commit,group:group.id,index});
  const originalPath=join(caseDirectory,'original.rs'),productPath=join(caseDirectory,'product.rs'),originalBinary=join(temporary,'original'),productBinary=join(temporary,'product');writeFileSync(originalPath,fixture.source);
  try{
   const active=diagnostics.attempt('active source version coverage',()=>requireControlProduct(profile,'actual compiler profile'));
   const original=diagnostics.attempt('independent original Rust compilation',()=>{requireControlProduct(active,'source version');return successful(command('rustc',['--edition=2024','--crate-name','meta_language_original',originalPath,'-o',originalBinary],caseDirectory));});
   const originalRun=diagnostics.attempt('independent original Rust stdout',()=>{requireControlProduct(original,'original executable');const output=successful(command(originalBinary,[],caseDirectory));assert.equal(output.stdout,fixture.stdout);return output;});
   const checked=diagnostics.attempt('actual source frontend and checked current program',()=>{const version=requireControlProduct(active,'source version'),options={sourceVersion:version.rust,exclusiveSourceModel:'Rust exclusive source loans 1',...(group.collections?{standardCollectionSourceModel:'Rust closed standard collections 1',standardCollectionSourceLibrary:requireControlProduct(library,'complete actual library'),standardCollectionPointerWidth:64}:{})};return checkProgram(parseRust(fixture.source,options));});
   const javascript=diagnostics.attempt('JavaScript emission from the checked current program',()=>{const product=emitJavaScript(requireControlProduct(checked,'checked program'));writeFileSync(join(caseDirectory,'product.mjs'),product.text);return product;});
   diagnostics.attempt('emitted JavaScript stdout equals the original and literal oracle',()=>{const actual=javaScriptOutput(requireControlProduct(javascript,'JavaScript product').text);writeProjectData(join(caseDirectory,'javascript-output.lino'),actual);assert.equal(actual.thrown,undefined);assert.equal(actual.stdout,fixture.stdout);assert.equal(actual.stdout,requireControlProduct(originalRun,'original stdout').stdout);});
   const rust=diagnostics.attempt('Rust emission from the checked current program',()=>{const product=emitRust(requireControlProduct(checked,'checked program'));writeFileSync(productPath,product.text);return product;});
   const native=diagnostics.attempt('actual emitted Rust compilation',()=>{requireControlProduct(rust,'emitted Rust product');return successful(command('rustc',['--edition=2024','--crate-name','meta_language_product',productPath,'-o',productBinary],caseDirectory));});
   diagnostics.attempt('emitted Rust stdout equals the original and literal oracle',()=>{requireControlProduct(native,'emitted executable');const output=successful(command(productBinary,[],caseDirectory));assert.equal(output.stdout,fixture.stdout);assert.equal(output.stdout,requireControlProduct(originalRun,'original stdout').stdout);});
   const concrete=diagnostics.attempt('default complete CST reconstructs original bytes',()=>{const graph=LinkNetwork.parse(fixture.source,'Rust');assert.equal(graph.reconstructText(),fixture.source);return graph;});
   diagnostics.attempt('complete CST survives Links text transport',()=>assert.equal(LinkNetwork.fromLino(requireControlProduct(concrete,'concrete graph').toLino()).reconstructText(),fixture.source));
   diagnostics.attempt('complete CST survives binary Links transport',()=>assert.equal(decodeLinkNetworkBinary(encodeLinkNetworkBinary(requireControlProduct(concrete,'concrete graph'))).reconstructText(),fixture.source));
  }finally{rmSync(temporary,{recursive:true,force:true});}
  const record={group:group.id,index,name:fixture.name,sourceSha256:digest(fixture.source),assertions:diagnostics.assertions,failures:diagnostics.failures};writeProjectData(join(caseDirectory,'assertions.lino'),{commit,...record});records.push(record);failures+=record.failures.length;globalThis.gc?.();
 }
 const report={model:'complete Rust state original and emitted product controls 1',commit,profile:profile??null,header:{assertions:header.assertions,failures:header.failures},records};writeProjectData(join(directory,'controls.lino'),report);return {report,failures,directory};
}
if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){if(process.argv.slice(2).join(' ')!=='--execute')throw new Error('use --execute only in explicit native CI');const result=runRustStateSourceOriginals();process.stdout.write(result.report.records.length+' complete originals; '+result.failures+' failed assertions; '+result.directory+'\n');if(result.failures)process.exitCode=1;}
