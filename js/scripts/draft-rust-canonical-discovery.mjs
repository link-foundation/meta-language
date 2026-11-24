// Source/literal author only. No represented programs or test controls execute.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {draftTranslationProfileInventory} from './draft-translation-profile-inventory.mjs';
import {draftRustPackageSourceAssets} from './rust-package-source-assets.mjs';
const root=new URL('../../',import.meta.url);
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const families=['byte-ascii','checked-integer','integer-bits','integer-magnitudes','primitive-associated-calls','primitive-try-from','primitive-selections','standard-owned-calls','standard-string-views','wrapping-shifts'];
const javascript=families.map(name=>'js/src/translation/rust-'+name+'.js');
for(const path of [...javascript,'js/scripts/draft-rust-canonical-discovery.mjs','js/scripts/draft-rust-canonical-discovery-fixtures.mjs','js/tests/translation-rust-canonical-discovery.test.js','experiments/rust-records/canonical-discovery.mjs'])parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);
await import('./draft-rust-canonical-discovery-fixtures.mjs');globalThis.gc?.();
draftTranslationProfileInventory({id:'rust-canonical-discovery',fixture:'rust-canonical-discovery',programAsset:'rust-primitive-associated-call-program',sourceLanguage:'Rust',experimentPath:'experiments/rust-records/canonical-discovery.mjs',nativeTest:'translation_rust_canonical_discovery',
 javascript:[...javascript,'js/tests/translation-rust-canonical-discovery.test.js'],
 rust:[...families.map(name=>'rust/src/translation/rust/expressions/'+name.replaceAll('-','_')+'.rs'),'rust/tests/unit/translation_rust_canonical_discovery.rs']});
draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
