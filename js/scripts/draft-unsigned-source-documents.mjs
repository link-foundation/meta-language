// Source/fixture metadata authors only. No represented source or control runs.
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {readProjectData,writeProjectData} from './project-data.mjs';
const reuse=process.argv.slice(2).join(' ')==='--draft --reuse-policy';
if(!reuse&&process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft, optionally --reuse-policy after exact source generation');
const root=new URL('../../',import.meta.url);
const javascript=['js/src/unsigned-source-document-rules.js','js/src/unsigned-link-network.js','js/src/unsigned-typed-program-links.js','js/src/index.js','js/tests/unsigned-source-documents.test.js'];
const rust=['rust/src/unsigned_source_document_rules.rs','rust/src/unsigned_link_network.rs','rust/src/unsigned_typed_program_links.rs','rust/src/lib.rs','rust/tests/unit/unsigned_source_documents.rs'];
for(const path of [...javascript,'js/scripts/draft-unsigned-source-documents.mjs','js/scripts/draft-unsigned-source-document-rules.mjs','js/scripts/draft-unsigned-source-document-fixtures.mjs','experiments/unsigned-source-documents/produce.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
process.argv.splice(2,process.argv.length-2,'--draft');
if(!reuse){await import('./draft-unsigned-source-document-rules.mjs');globalThis.gc?.();}
else{
 const recipe=readProjectData(new URL('parity/self-translation/unsigned-source-document-rules-native.lino',root));
 for(const input of recipe.sourceClosure.inputs){const actual=readFileSync(new URL(input.path,root));if(actual.length!==input.bytes||createHash('sha256').update(actual).digest('hex')!==input.sourceSha256)throw new TypeError('preserve stale source for regeneration: '+input.path);}
 const asset=recipe.programAsset,actual=readFileSync(new URL(asset.path,root)),twin=readFileSync(new URL(asset.path.replace('js/src/','rust/src/'),root));if(actual.length!==asset.bytes||createHash('sha256').update(actual).digest('hex')!==asset.sha256||!actual.equals(twin))throw new TypeError('preserve stale source program asset: '+asset.path);
}
await import('./draft-unsigned-source-document-fixtures.mjs');globalThis.gc?.();
const inventoryFile=new URL('parity/language-features.lino',root),inventory=readProjectData(inventoryFile),id='exact-unsigned-source-document-identities',common=['parity/fixtures/unsigned-source-documents.lino','experiments/unsigned-source-documents/produce.mjs'];
const feature={id,required:true,javascript:{status:'implemented',evidence:[...javascript,...common]},rust:{status:'implemented',evidence:[...rust,...common]}};
const prior=inventory.features.findIndex(row=>row.id===id);if(prior<0)inventory.features.push(feature);else inventory.features[prior]=feature;writeProjectData(inventoryFile,inventory);
const modulesFile=new URL('rust/tests/unit/mod.rs',root),modules=readFileSync(modulesFile,'utf8'),declaration='mod unsigned_source_documents;';if(!modules.includes(declaration))writeFileSync(modulesFile,modules+'\n'+declaration+'\n');
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
