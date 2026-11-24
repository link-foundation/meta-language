// CI-only original/generated Rust std snapshot oracle. Never run while drafting.
import assert from 'node:assert/strict';
import {execFileSync,spawnSync} from 'node:child_process';
import {mkdirSync,writeFileSync,rmSync} from 'node:fs';
import {join} from 'node:path';
import {readProjectData,writeProjectData} from '../../js/scripts/project-data.mjs';
import {createRustSourceOwnerProgramLinks} from '../../js/src/rust-source-owner-program.js';
import {decodeTypedProgramLinks} from '../../js/src/typed-program-links.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
const [option,version]=process.argv.slice(2);if(option!=='--version'||!version)throw new TypeError('select --version with its actual installed compiler');
const fixture=readProjectData(new URL('../../parity/fixtures/rust-source-versions.lino',import.meta.url));
assert.ok(fixture.sourceVersions.includes(version),'uncovered source std '+version);
const actual=execFileSync('rustc',['--version'],{encoding:'utf8',timeout:30000}).trim().split(/\s+/u)[1];assert.equal(actual,version,'the original source must use its selected std compiler');
const directory=join(process.cwd(),'issue-195-artifacts','work','rust-source-versions',version+'-'+process.pid),receipt=join(process.cwd(),'issue-195-artifacts','rust-source-versions',version+'.lino');
mkdirSync(directory,{recursive:true});mkdirSync(join(process.cwd(),'issue-195-artifacts','rust-source-versions'),{recursive:true});
const packet=createRustSourceOwnerProgramLinks(fixture.source,{sourceVersion:version}),emission=emitRust(decodeTypedProgramLinks(packet.network,packet.root)),header=emission.text.split('\n').find(line=>line.startsWith('#![allow('));assert.ok(header,'complete emitted library attributes are required');
function driver(names){return 'fn main(){std::panic::set_hook(Box::new(|_|{}));'+names.map((name,index)=>'{match std::panic::catch_unwind(||'+name+'()){Ok(value)=>println!("'+index+':value:{}",value),Err(error)=>{let message=if let Some(message)=error.downcast_ref::<String>(){message.as_str()}else if let Some(message)=error.downcast_ref::<&str>(){*message}else{panic!("unrepresented panic payload")};let bytes=message.as_bytes().iter().map(|byte|format!("{:02x}",byte)).collect::<String>();println!("'+index+':abort:{}",bytes);}}}').join('')+'}';}
const original=fixture.cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\n'+driver(fixture.cases.map(row=>row.module+'::result'));
const generated=[header,...emission.preludes,...emission.definitions,driver(fixture.cases.map(row=>emission.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===row.module+'.result').target))].join('\n\n');
let canRemove=true;
function execute(name,source){const file=join(directory,name+'.rs'),binary=join(directory,name);writeFileSync(file,source);const compile=spawnSync('rustc',['--edition','2024','-C','overflow-checks=on',file,'-o',binary],{encoding:'utf8',timeout:120000,maxBuffer:4*1024*1024});if(compile.status===null)canRemove=false;assert.equal(compile.status,0,compile.stderr);const result=spawnSync(binary,[],{encoding:'utf8',timeout:30000,maxBuffer:1024*1024});assert.equal(result.status,0,result.stderr);return result.stdout.trimEnd().split('\n');}
try{const originalLines=execute('original',original),generatedLines=execute('generated',generated),expected=fixture.cases.map((row,index)=>{const value=row.expectedByVersion[version];return index+':'+value.kind+':'+(value.kind==='abort'?Buffer.from(value.message).toString('hex'):value.value);});assert.deepEqual(originalLines,expected);assert.deepEqual(generatedLines,expected);
writeProjectData(receipt,{schemaVersion:1,commit:execFileSync('git',['rev-parse','HEAD'],{encoding:'utf8'}).trim(),sourceVersion:version,compilerVersion:actual,original:originalLines,generated:generatedLines});
}finally{if(canRemove)rmSync(directory,{recursive:true,force:true});}
