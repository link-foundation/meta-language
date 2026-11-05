// Source/context drafting only. Represented source, products and controls never run.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {draftTranslationProfileInventory} from './draft-translation-profile-inventory.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
for(const path of ['js/src/self-translation.js','js/src/cli.js','js/scripts/draft-project-translation.mjs','js/scripts/draft-rust-source-project.mjs','js/src/translation/rust-shared-array-handle-modules.js','js/tests/self-translation-source-versions.test.js','experiments/self-translation-source-versions/products.mjs','js/scripts/draft-self-translation-source-versions.mjs']){
 parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();
}
draftTranslationProfileInventory({id:'self-translation-explicit-rust-source-std-context',fixture:'rust-source-versions',programAsset:'rust-source-version-program',sourceLanguage:'Rust',nativeTest:'self_translation_source_versions',experimentPath:'experiments/self-translation-source-versions/products.mjs',javascript:['js/src/cli.js','js/scripts/draft-project-translation.mjs','js/scripts/draft-rust-source-project.mjs','js/src/self-translation.js','js/src/translation/rust-shared-array-handle-modules.js','js/src/translation/rust-source-versions.js','js/tests/self-translation-source-versions.test.js'],rust:['rust/src/main.rs','rust/src/project_translation/settings.rs','rust/tests/integration/cli.rs','rust/src/self_translation.rs','rust/src/self_translation/semantic_input.rs','rust/src/self_translation/emission.rs','rust/src/self_translation/shared_array_handles.rs','rust/src/translation/rust/source_profiles.rs','rust/tests/unit/self_translation_source_versions.rs']});
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
