// Source/model/literal-data construction only; controls are parsed, never run.
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {readProjectData,writeProjectData} from './project-data.mjs';
const mode=process.argv.slice(2).join(' '),reuse=mode==='--draft --reuse-libraries';
if(mode!=='--draft'&&!reuse)throw new TypeError('use --draft [--reuse-libraries]');
process.argv.splice(2,process.argv.length-2,'--draft');
const root=new URL('../../',import.meta.url);
for(const path of ['js/src/translation/javascript-call-code.js','js/src/translation/javascript-call-primitives.js','js/src/translation/javascript-call-integers.js','js/src/translation/javascript-call-number-conversion.js','js/src/translation/javascript-number-conversion-protocol.js','js/src/translation/javascript-text-conversion-protocol.js','js/src/translation/javascript-call-machine.js','js/src/translation/javascript-call-source.js','js/src/translation/javascript-call-lowering.js',
 'js/src/translation/javascript-scalar-conversion-protocol.js','js/scripts/draft-javascript-call-source.mjs','js/scripts/draft-javascript-call-source-fixtures.mjs','js/scripts/draft-javascript-call-source-integration.mjs','js/scripts/source-boolean-match-sharing.mjs','js/scripts/source-constructor-projections.mjs','js/scripts/source-lexical-aliases.mjs','js/src/translation/javascript-string-argument-rules.js','js/scripts/draft-javascript-string-argument-fixtures.mjs','js/tests/translation-javascript-string-arguments.test.js','js/src/translation/boolean-constructor-match-rules.js','js/scripts/draft-boolean-constructor-match-fixtures.mjs','js/tests/boolean-constructor-match.test.js',
 'experiments/javascript-properties/call-source-controls.mjs','experiments/javascript-properties/call-source-oracle.mjs','experiments/javascript-properties/call-source-exception-controls.mjs','js/scripts/draft-javascript-call-source-exceptions.mjs','experiments/javascript-properties/call-machine-controls.mjs','js/scripts/draft-javascript-call-machine-fixtures.mjs','js/tests/translation-javascript-call-source.test.js']){
 parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();
}
if(reuse){
 // Bounded authoring jobs may emit the two libraries separately. Admit only
 // exact current source inputs and the complete original generated files.
 for(const stem of ['javascript-call-source','javascript-call-machine']){
  const recipe=readProjectData(new URL('parity/self-translation/'+stem+'-native.lino',root));
  if(recipe.schemaVersion!==1||!recipe.sourceClosure?.inputs?.length||!recipe.files?.length||!recipe.programProjection?.entryPoints?.length||!recipe.programAsset)throw new TypeError('reuse needs complete actual source and linked-model receipts; native-only construction is insufficient');
  for(const input of recipe.sourceClosure.inputs){
   if(!/^js\/src\/(?:[a-z0-9-]+\/)*[a-z0-9-]+\.js$/u.test(input.path)||readFileSync(new URL(input.path,root),'utf8')!==input.source)throw new TypeError('source-library input changed; regenerate '+stem);
  }
  for(const file of recipe.files){
   if(!new RegExp('^'+stem.replaceAll('-','_')+'(?:/part_[0-9]+)?\\.rs$','u').test(file.path)||readFileSync(new URL('rust/src/translation/'+file.path,root),'utf8')!==file.source)throw new TypeError('source-library output changed; regenerate '+stem);
  }
  const asset=recipe.programAsset;
  if(asset.path!=='js/src/data/'+stem+'-program.lino')throw new TypeError('reuse needs its actual qualified source program asset');
  const bytes=readFileSync(new URL(asset.path,root)),twin=readFileSync(new URL(asset.path.replace('js/src/','rust/src/'),root));
  if(bytes.length!==asset.bytes||createHash('sha256').update(bytes).digest('hex')!==asset.sha256||!bytes.equals(twin))throw new TypeError('source program asset changed; regenerate '+stem);
  globalThis.gc?.();
 }
}else for(const path of ['draft-javascript-call-source.mjs','draft-javascript-call-machine.mjs']){await import(new URL(path,import.meta.url));globalThis.gc?.();}
for(const path of ['draft-javascript-call-source-fixtures.mjs','draft-javascript-call-source-exceptions.mjs','draft-javascript-string-argument-fixtures.mjs','draft-javascript-call-machine-fixtures.mjs','draft-boolean-constructor-match-fixtures.mjs']){await import(new URL(path,import.meta.url));globalThis.gc?.();}
for(const path of ['experiments/javascript-properties/call-source-codecs.mjs','experiments/javascript-properties/call-machine-codecs.mjs'])parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);
for(const [path,declarations]of [['rust/src/translation/mod.rs',['pub(crate) mod javascript_call_source;']],['rust/tests/unit/mod.rs',['mod translation_javascript_call_source;','mod translation_javascript_call_source_machine;','mod boolean_constructor_match;']]]){
 const file=new URL(path,root);let source=readFileSync(file,'utf8');for(const declaration of declarations)if(!source.includes(declaration))source+='\n'+declaration+'\n';writeFileSync(file,source);
}
const file=new URL('parity/language-features.lino',root),inventory=readProjectData(file),common=['parity/fixtures/javascript-call-source.lino','parity/fixtures/javascript-call-source-exceptions.lino','parity/fixtures/javascript-string-arguments.lino','experiments/javascript-properties/call-source-controls.mjs','experiments/javascript-properties/call-source-exception-controls.mjs','experiments/javascript-properties/call-source-oracle.mjs'];
const row={id:'javascript-call-source',required:true,javascript:{status:'implemented',evidence:['js/src/translation/javascript-call-source.js','js/src/translation/javascript-call-lowering.js','js/src/translation/javascript-string-argument-rules.js','js/tests/translation-javascript-string-arguments.test.js','js/tests/translation-javascript-call-source.test.js',...common]},
 rust:{status:'implemented',evidence:['rust/src/translation/javascript_call_source.rs','rust/src/translation/javascript_string_argument_rules.rs','rust/tests/unit/translation_javascript_call_source.rs','rust/tests/unit/translation_javascript_call_source_machine.rs','rust/tests/unit/translation_javascript_string_arguments.rs',...common]}};
const at=inventory.features.findIndex(entry=>entry.id===row.id);if(at<0)inventory.features.push(row);else inventory.features[at]=row;writeProjectData(file,inventory);
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
