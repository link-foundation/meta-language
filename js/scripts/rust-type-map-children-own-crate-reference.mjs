// Compile one native reference against the actual checkout/library, then invoke
// its unchanged method for every declared scenario. Never run this locally.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync,rmSync,mkdirSync} from 'node:fs';
import {basename,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {createHash,randomBytes} from 'node:crypto';
import {LinkNetwork} from '../src/network.js';
import {captureRustConcreteSourceItems} from '../src/translation/rust-concrete-source-items.js';
import {writeProjectData} from './project-data.mjs';
const repository=fileURLToPath(new URL('../../',import.meta.url)),digest=value=>createHash('sha256').update(value).digest('hex');
export function compileActualTypeOwnCrateReference(capture,{directory,dependencyTarget,command,successful,profile}){
 if(process.env.CI!=='true'||process.env.META_LANGUAGE_NATIVE_RUST_STATE!=='1')throw new Error('actual crate reference compilation requires explicit CI');
 const method=capture.originals[0]?.ownCrateDependency?.sourceOwner?.at(-1);assert.ok(['map_children','children'].includes(method),'each actual crate method needs its own declared source owner');
 const sourceOwners=[capture.types,capture.span];for(const owner of sourceOwners){assert.ok(['rust/src/translation/types.rs','rust/src/translation/mod.rs'].includes(owner.path));assert.equal(digest(readFileSync(join(repository,owner.path))),owner.sourceSha256,'the actual crate source must equal its captured original before compilation');}
 const ownCrate={package:'meta-language',library:'meta_language',repositoryPath:'rust',sourceOwner:['meta_language','translation','types','Type',method]},cases=[];
 for(const [index,row]of capture.originals.entries()){
  assert.deepEqual(row.ownCrateDependency,ownCrate);assert.equal(typeof row.ownCrateNativeSource,'string');
  const network=LinkNetwork.parse(row.ownCrateNativeSource,'Rust'),actual=captureRustConcreteSourceItems(network,row.ownCrateNativeSource,{path:'actual-crate-scenario-'+index+'.rs',selectors:[{kind:'function_item',name:'main'}]}).items[0];
  assert.equal(row.ownCrateNativeSource.slice(actual.range.end).trim(),'');assert.equal(typeof actual.bodyText,'string');
  cases.push({index,name:row.name,sourceSha256:digest(row.ownCrateNativeSource),bodySha256:digest(actual.bodyText),source:row.ownCrateNativeSource});
 }
 assert.ok(cases.length>0);const source=cases.map(row=>'mod actual_case_'+row.index+' {\n'+row.source+'\npub fn invoke(){main();}\n}').join('\n')+'\nfn main(){let index:usize=std::env::args().nth(1).expect("case index").parse().expect("numeric case index");match index{'+cases.map(row=>row.index+'=>actual_case_'+row.index+'::invoke(),').join('')+'_=>panic!("unknown case index"),}}\n';
 const artifact=join(directory,'actual-own-crate-reference');mkdirSync(artifact,{recursive:true});writeFileSync(join(artifact,'reference.rs'),source);
 const manifest=join(repository,'rust/Cargo.toml'),lock=readFileSync(join(repository,'rust/Cargo.lock'),'utf8'),example=join(repository,'rust/examples','issue_227_actual_type_'+randomBytes(12).toString('hex')),name=basename(example);mkdirSync(example);
 try{
  writeProjectData(join(example,'owner.lino'),{model:'task-owned actual Type native reference example 1',sourceSha256:digest(source)});writeFileSync(join(example,'main.rs'),source);
  const compilation=successful(command('cargo',['build','--quiet','--keep-going','--locked','--no-default-features','--manifest-path',manifest,'--target-dir',dependencyTarget,'--target',profile.host,'--example',name],artifact,{timeoutMilliseconds:900000,maxOutputBytes:8*1024*1024,developmentDebug:'0'}));
  assert.equal(readFileSync(join(repository,'rust/Cargo.lock'),'utf8'),lock,'the actual original must retain its active complete locked dependencies');
  for(const owner of sourceOwners)assert.equal(digest(readFileSync(join(repository,owner.path))),owner.sourceSha256,'the actual crate source must retain its captured original during compilation');
  writeProjectData(join(artifact,'reference.lino'),{owner:ownCrate,features:'no default features',locked:true,workspaceLockSha256:digest(lock),manifestSha256:digest(readFileSync(manifest)),sourceSha256:digest(source),cases:cases.map(({source,...row})=>row)});
  return {binary:join(dependencyTarget,profile.host,'debug','examples',name+(process.platform==='win32'?'.exe':'')),cases:cases.map(({source,...row})=>row),compilation};
 }finally{rmSync(example,{recursive:true,force:true});}
}
