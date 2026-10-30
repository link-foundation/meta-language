// Source-only authors: never execute represented checked arithmetic or controls.
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData} from './project-data.mjs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {draftTranslationProfileInventory} from './draft-translation-profile-inventory.mjs';
const reuse=process.argv.slice(2).join(' ')==='--draft --reuse-policies';
if(!reuse&&process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft, optionally --reuse-policies after source generation');
const root=new URL('../../',import.meta.url);
const javascript=['js/src/translation/rust-source-constant-evaluation-rules.js','js/src/translation/rust-source-constant-evaluation.js','js/src/translation/rust-constant-integer-rules.js','js/src/translation/rust-integer-ranges.js','js/src/translation/rust.js','js/tests/translation-rust-source-constant-evaluation.test.js'];
for(const path of [...javascript,'js/scripts/draft-rust-source-constant-evaluation.mjs','js/scripts/draft-rust-source-constants-fixtures.mjs','experiments/rust-records/source-constant-evaluation.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
process.argv.splice(2,process.argv.length-2,'--draft');
for(const name of ['rust-constant-integer','rust-source-constant-evaluation']){
 if(!reuse){await import('./draft-'+name+'-rules.mjs');globalThis.gc?.();continue;}
 const recipe=readProjectData(new URL('parity/self-translation/'+name+'-rules-native.lino',root));
 for(const input of recipe.sourceClosure.inputs){const actual=readFileSync(new URL(input.path,root));if(actual.length!==input.bytes||createHash('sha256').update(actual).digest('hex')!==input.sourceSha256)throw new TypeError('preserve stale policy source for regeneration: '+input.path);}
 const asset=recipe.programAsset,actual=readFileSync(new URL(asset.path,root)),twin=readFileSync(new URL(asset.path.replace('js/src/','rust/src/'),root));if(actual.length!==asset.bytes||createHash('sha256').update(actual).digest('hex')!==asset.sha256||!actual.equals(twin))throw new TypeError('preserve stale policy asset for regeneration: '+asset.path);
}
await import('./draft-rust-constant-patterns-fixtures.mjs');globalThis.gc?.();
await import('./draft-rust-source-constants-fixtures.mjs');globalThis.gc?.();
draftTranslationProfileInventory({id:'ordinary-rust-source-constant-scalar-evaluation',fixture:'rust-source-constant-evaluation',programAsset:'rust-source-constant-evaluation-program',sourceLanguage:'Rust',experimentPath:'experiments/rust-records/source-constant-evaluation.mjs',nativeTest:'translation_rust_source_constant_evaluation',javascript,rust:['rust/src/translation/rust_source_constant_evaluation_rules.rs','rust/src/translation/rust/source_constant_evaluation.rs','rust/src/translation/rust/items/constants.rs','rust/src/translation/rust/expressions/control.rs','rust/tests/unit/translation_rust_source_constant_evaluation.rs']});
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
