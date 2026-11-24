// Source/fixture metadata authors only. No represented source or control runs.
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {readProjectData,writeProjectData} from './project-data.mjs';
const reuse=process.argv.slice(2).join(' ')==='--draft --reuse-policy';
if(!reuse&&process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft, optionally --reuse-policy after exact source generation');
const root=new URL('../../',import.meta.url);
const javascript=['js/src/unsigned-source-document-rules.js','js/src/unsigned-graph-command-rules.js','js/src/unsigned-graph-cli.js','js/src/network.js','js/tests/unsigned-graph-command.test.js','js/src/unsigned-link-network.js','js/src/unsigned-typed-program-links.js','js/src/unsigned-source-command-rules.js','js/src/unsigned-source-cli.js','js/src/cli.js','js/tests/unsigned-source-command.test.js','js/src/index.js','js/tests/unsigned-source-documents.test.js'];
const rust=['rust/src/unsigned_source_document_rules.rs','rust/src/unsigned_graph_command_rules.rs','rust/src/unsigned_graph_cli.rs','rust/src/link_network.rs','rust/tests/unit/unsigned_graph_command.rs','rust/tests/unsigned_graph_cli.rs','rust/src/unsigned_link_network.rs','rust/src/unsigned_typed_program_links.rs','rust/src/unsigned_source_cli.rs','rust/src/unsigned_source_command_rules.rs','rust/src/main.rs','rust/tests/unit/unsigned_source_command.rs','rust/tests/unsigned_source_cli.rs','rust/src/lib.rs','rust/tests/unit/unsigned_source_documents.rs'];
for(const path of [...javascript,'js/scripts/draft-unsigned-source-documents.mjs','js/scripts/draft-unsigned-source-document-rules.mjs','js/scripts/draft-unsigned-source-document-fixtures.mjs','experiments/unsigned-source-documents/produce.mjs','experiments/unsigned-source-command/produce.mjs','js/scripts/draft-unsigned-source-command-rules.mjs','js/scripts/draft-unsigned-source-command-fixtures.mjs','js/scripts/draft-unsigned-graph-fixtures.mjs','js/scripts/draft-unsigned-graph-command-rules.mjs','experiments/unsigned-graph/fixture-input.mjs','experiments/unsigned-graph/produce.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
process.argv.splice(2,process.argv.length-2,'--draft');
for(const policy of ['unsigned-source-document','unsigned-source-command','unsigned-graph-command']){
if(!reuse){await import('./draft-'+policy+'-rules.mjs');globalThis.gc?.();}
else{
 const recipe=readProjectData(new URL('parity/self-translation/'+policy+'-rules-native.lino',root));
 for(const input of recipe.sourceClosure.inputs){const actual=readFileSync(new URL(input.path,root));if(actual.length!==input.bytes||createHash('sha256').update(actual).digest('hex')!==input.sourceSha256)throw new TypeError('preserve stale source for regeneration: '+input.path);}
 const asset=recipe.programAsset,actual=readFileSync(new URL(asset.path,root)),twin=readFileSync(new URL(asset.path.replace('js/src/','rust/src/'),root));if(actual.length!==asset.bytes||createHash('sha256').update(actual).digest('hex')!==asset.sha256||!actual.equals(twin))throw new TypeError('preserve stale source program asset: '+asset.path);
}
}
await import('./draft-unsigned-source-document-fixtures.mjs');globalThis.gc?.();
await import('./draft-unsigned-source-command-fixtures.mjs');globalThis.gc?.();
await import('./draft-unsigned-graph-fixtures.mjs');globalThis.gc?.();
const inventoryFile=new URL('parity/language-features.lino',root),inventory=readProjectData(inventoryFile),id='exact-unsigned-source-document-identities',common=['parity/fixtures/unsigned-source-documents.lino','experiments/unsigned-source-documents/produce.mjs','parity/fixtures/unsigned-source-command.lino','experiments/unsigned-source-command/produce.mjs','parity/fixtures/unsigned-graph-command.lino','experiments/unsigned-graph/fixture-input.mjs','experiments/unsigned-graph/produce.mjs'];
const feature={id,required:true,javascript:{status:'implemented',evidence:[...javascript,...common]},rust:{status:'implemented',evidence:[...rust,...common]}};
const prior=inventory.features.findIndex(row=>row.id===id);if(prior<0)inventory.features.push(feature);else inventory.features[prior]=feature;writeProjectData(inventoryFile,inventory);
const modulesFile=new URL('rust/tests/unit/mod.rs',root);let modules=readFileSync(modulesFile,'utf8');for(const name of ['unsigned_source_documents','unsigned_source_command','unsigned_graph_command']){const declaration='mod '+name+';';if(!modules.includes(declaration))modules+='\n'+declaration+'\n';}writeFileSync(modulesFile,modules);
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
