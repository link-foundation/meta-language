// Inventory existing authored Links source graphs, not their execution. Each
// file is parsed once as data, never decoded, evaluated, emitted or compared.
import {readFileSync,readdirSync,lstatSync,writeFileSync,mkdirSync,existsSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {LinkNetwork} from '../src/network.js';
import {writeProjectData,checkProjectData} from './project-data.mjs';
import {formatProductRuleSource} from './product-rule-style-corrections.mjs';
const mode=process.argv.slice(2).join(' '),check=mode==='--check';
if(!check&&mode!=='--draft')throw new TypeError('use --draft or --check');
const root=new URL('../../',import.meta.url),directory=new URL('js/src/data/',root),entries=[];
for(const asset of readdirSync(directory).filter(name=>/^[a-z][a-z0-9-]*-program\.lino$/u.test(name)).sort()){
 const file=new URL(asset,directory),nativeFile=new URL('rust/src/data/'+asset,root);
 if(!lstatSync(file).isFile()||lstatSync(file).isSymbolicLink())throw new TypeError('a source program must be a regular package asset: '+asset);
 const bytes=readFileSync(file);
 // Pair missing artifacts mechanically from the complete original graph.
 // Existing different artifacts must be reconciled, never silently replaced.
 if(!existsSync(nativeFile)&&!check)writeFileSync(nativeFile,bytes,{flag:'wx'});
 if(!lstatSync(nativeFile).isFile()||lstatSync(nativeFile).isSymbolicLink())throw new TypeError('a native source program must be a regular paired package asset: '+asset);
 const native=readFileSync(nativeFile);
 if(bytes.length>16*1024*1024||!bytes.equals(native))throw new TypeError('a source program exceeds the author bound or its package twin differs: '+asset);
 const network=LinkNetwork.fromLino(bytes.toString('utf8'));
 const roots=network.links().filter(link=>link.metadata().language==='meta-language typed program'&&link.metadata().term==='typed program'&&link.metadata().definition==='1');
 if(roots.length!==1)throw new TypeError('a source program needs one actual existing schema root: '+asset);
 entries.push({identifier:'meta-language/source-program/'+asset.slice(0,-5),asset,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),root:String(roots[0].id().asU64())});
 if(globalThis.gc)globalThis.gc();
}
const catalog={schemaVersion:1,model:'complete qualified typed source programs 1',entries};
for(const runtime of ['js','rust'])(check?checkProjectData:writeProjectData)(new URL(runtime+'/src/data/source-definition-catalog.lino',root),catalog);
const arms=entries.map(entry=>'        '+JSON.stringify(entry.asset)+' => Some(include_str!("../data/'+entry.asset+'")),');
const assetsFile=new URL('rust/src/source_definition_catalog/assets.rs',root),assetsSource=formatProductRuleSource('// Generated from actual paired source program assets. Do not edit.\n\npub(super) fn source_asset(asset: &str) -> Option<&\'static str> {\n    match asset {\n'+arms.join('\n')+'\n        _ => None,\n    }\n}\n');
if(check){if(readFileSync(assetsFile,'utf8')!==assetsSource)throw new TypeError('source definition catalog native assets are stale');}
else{mkdirSync(new URL('rust/src/source_definition_catalog/',root),{recursive:true});writeFileSync(assetsFile,assetsSource);}
