// CI-only reference compilation. Cargo dependencies serve the full ORIGINAL
// serde declarations; projected declarations and emitted products use rustc.
import assert from 'node:assert/strict';
import {copyFileSync,existsSync,readFileSync,writeFileSync} from 'node:fs';
import {join} from 'node:path';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {tomlEntries} from '../src/program-project-tree.js';
import {writeProjectData} from './project-data.mjs';
export function compileRustStateNativeOriginal(fixture,{originalPath,originalBinary,temporary,dependencyTarget,caseDirectory,command,successful,target}){
 if(process.env.CI!=='true'||process.env.META_LANGUAGE_NATIVE_RUST_STATE!=='1')throw new Error('native original compilation requires explicit CI');
 assert.match(target,/^[A-Za-z0-9_-]+$/u);if(fixture.nativeDependencies===undefined)return successful(command('rustc',['--edition=2024','--target',target,'--crate-name','meta_language_original',originalPath,'-o',originalBinary],caseDirectory));
 assert.deepEqual(fixture.nativeDependencies,{serde:{version:'1',features:['derive']}},'each additional dependency needs its own explicit original compiler profile');assert.equal(typeof fixture.nativeSource,'string');
 const workspaceLock=readFileSync(fileURLToPath(new URL('../../rust/Cargo.lock',import.meta.url)),'utf8'),entries=tomlEntries(workspaceLock),owners=entries.filter(row=>row.table==='package'&&row.key==='name'&&row.value==='serde');assert.equal(owners.length,1,'one active actual serde dependency must be locked');const versions=entries.filter(row=>row.table==='package'&&row.index===owners[0].index&&row.key==='version');assert.equal(versions.length,1);const activeSerdeVersion=versions[0].value;assert.match(activeSerdeVersion,/^1\.[0-9]+\.[0-9]+$/u,'the active project serde dependency must be covered by >=1 <2');
 const manifest='[package]\nname = "meta_language_original"\nversion = "0.0.0"\nedition = "2024"\n\n[workspace]\n\n[[bin]]\nname = "meta_language_original"\npath = '+JSON.stringify(originalPath)+'\n\n[dependencies]\nserde = { version = "='+activeSerdeVersion+'", features = ["derive"] }\n',manifestPath=join(temporary,'Cargo.toml');writeFileSync(manifestPath,manifest);writeFileSync(join(caseDirectory,'original-Cargo.toml'),manifest);
 const compiled=command('cargo',['build','--quiet','--manifest-path',manifestPath,'--target-dir',dependencyTarget,'--target',target,'--bin','meta_language_original'],caseDirectory,{developmentDebug:'0'}),lock=join(temporary,'Cargo.lock');
 if(existsSync(lock))writeFileSync(join(caseDirectory,'original-Cargo.lock'),readFileSync(lock));successful(compiled);
 const metadata=successful(command('cargo',['metadata','--format-version','1','--manifest-path',manifestPath],caseDirectory)),resolved=JSON.parse(metadata.stdout),serde=resolved.packages.filter(row=>row.name==='serde');
 assert.equal(serde.length,1);assert.equal(serde[0].version,activeSerdeVersion,'the native reference must use the actual project locked serde version');
 writeProjectData(join(caseDirectory,'original-native-dependencies.lino'),{serdeRange:'>=1 <2',serdeVersion:serde[0].version,workspaceLockSha256:createHash('sha256').update(workspaceLock).digest('hex'),packages:resolved.packages.map(({name,version,id})=>({name,version,id}))});
 const executable=join(dependencyTarget,target,'debug','meta_language_original'+(process.platform==='win32'?'.exe':''));copyFileSync(executable,originalBinary);return compiled;
}
