// Author real source, literal data and source manifests. No future control is
// imported or evaluated. No represented heap operation or native build runs.
import {readFileSync,writeFileSync,readdirSync,lstatSync,existsSync,mkdirSync,renameSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
const syntax=['js/src/translation/javascript-ordinary-heap.js','js/src/translation/javascript-data-field-rules.js','js/src/translation/javascript-infer.js','js/src/translation/javascript-lower.js',
 'js/scripts/draft-javascript-ordinary-heap.mjs','js/scripts/draft-javascript-ordinary-heap-fixtures.mjs','js/scripts/draft-javascript-ordinary-heap-integration.mjs','js/scripts/draft-javascript-data-fields-fixtures.mjs',
 'js/tests/translation-javascript-ordinary-heap.test.js','js/tests/translation-javascript-data-fields.test.js','js/tests/source-library-staging-paths.test.js','js/scripts/stage-source-library-files.mjs','experiments/javascript-properties/heap-controls.mjs','experiments/javascript-properties/heap-oracle.mjs','experiments/javascript-properties/data-field-oracle.mjs'];
for(const path of syntax){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
for(const path of ['draft-javascript-data-field-rules.mjs','draft-javascript-ordinary-heap.mjs','draft-javascript-data-fields-fixtures.mjs','draft-javascript-ordinary-heap-fixtures.mjs']){await import(new URL(path,import.meta.url));globalThis.gc?.();}
// Preserve obsolete task-generated structural-layout source as exact Links
// records, and relocate its original files into task-owned /private/tmp.
// Nothing is deleted or overwritten; tracked or differently authored files
// refuse relocation. Package inventories must describe the selected layout.
const recipe=readProjectData(new URL('parity/self-translation/javascript-ordinary-heap-native.lino',root));
const selected=new Set(recipe.files.map(row=>'rust/src/translation/'+row.path));
function sources(directory){const files=[];for(const entry of readdirSync(new URL(directory,root),{withFileTypes:true})){
 const path=directory+'/'+entry.name;if(entry.isSymbolicLink())throw new TypeError('generated source directory refuses symbolic links');if(entry.isDirectory())files.push(...sources(path));else if(entry.isFile()&&entry.name.endsWith('.rs'))files.push(path);
}return files;}
const superseded=[];
for(const path of sources('rust/src/translation/javascript_ordinary_heap'))if(!selected.has(path)){
 const file=new URL(path,root);if(!lstatSync(file).isFile())throw new TypeError('superseded source must be regular');
 if(execFileSync('git',['ls-files','--',path],{cwd:root,encoding:'utf8'}))throw new TypeError('preserve tracked source before changing its layout: '+path);
 const bytes=readFileSync(file);if(bytes.length>4*1024*1024||!bytes.toString('utf8').startsWith('// Generated from complete js/src/translation/javascript-ordinary-heap.js. Do not edit by hand.\n'))throw new TypeError('source is not owned by this task author: '+path);
 superseded.push({path,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),source:bytes.toString('utf8')});
}
if(superseded.length){
 const archiveFile=new URL('parity/fixtures/history/javascript-ordinary-heap-superseded-source-files.lino',root);
 const archive=existsSync(archiveFile)?readProjectData(archiveFile):{schemaVersion:1,model:'exact superseded ordinary heap source authoring layouts 1',files:[]};
 for(const row of superseded)if(!archive.files.some(prior=>prior.path===row.path&&prior.sha256===row.sha256))archive.files.push(row);
 writeProjectData(archiveFile,archive);
 for(const row of superseded){
  const target=new URL(row.sha256+'/'+row.path,'file:///private/tmp/meta-language-bulk4622-superseded-native/');mkdirSync(new URL('.',target),{recursive:true});
  if(existsSync(target))throw new TypeError('preserve both source copies; relocation target already exists');
  renameSync(new URL(row.path,root),target);
 }
}
const inventoryFile=new URL('parity/language-features.lino',root),inventory=readProjectData(inventoryFile);
const common=['parity/fixtures/javascript-ordinary-heap.lino','experiments/javascript-properties/heap-controls.mjs','experiments/javascript-properties/heap-oracle.mjs'];
const entry={id:'javascript-ordinary-heap',required:true,javascript:{status:'implemented',evidence:['js/src/translation/javascript-ordinary-heap.js','js/src/data/javascript-ordinary-heap-program.lino','js/tests/translation-javascript-ordinary-heap.test.js',...common]},rust:{status:'implemented',evidence:['rust/src/translation/javascript_ordinary_heap.rs','rust/src/data/javascript-ordinary-heap-program.lino','rust/tests/unit/translation_javascript_ordinary_heap.rs',...common]}};
const at=inventory.features.findIndex(row=>row.id===entry.id);if(at<0)inventory.features.push(entry);else inventory.features[at]=entry;writeProjectData(inventoryFile,inventory);
for(const [path,declaration]of [['rust/tests/unit/mod.rs','mod translation_javascript_ordinary_heap;'],['rust/src/translation/mod.rs','pub(crate) mod javascript_ordinary_heap;']]){
 const file=new URL(path,root),source=readFileSync(file,'utf8');if(!source.includes(declaration))writeFileSync(file,source+'\n'+declaration+'\n');
}
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
