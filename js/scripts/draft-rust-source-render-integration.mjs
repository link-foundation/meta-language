// Source construction only: never import/evaluate the deferred controls.
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
for(const path of ['js/src/translation/rust-record-rules.js','js/src/translation/emit-rust.js','js/scripts/draft-native-source-interfaces.mjs',
 'js/scripts/draft-rust-source-render-interfaces.mjs','js/scripts/draft-rust-source-render-fixtures.mjs','js/scripts/draft-rust-source-render-integration.mjs',
 'experiments/rust-source-render/oracle.mjs','js/tests/translation-rust-source-render-interfaces.test.js']){
 parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();
}
for(const path of ['generate-rust-record-rules.mjs','draft-rust-source-render-interfaces.mjs','draft-rust-source-render-fixtures.mjs']){
 await import(new URL(path,import.meta.url));globalThis.gc?.();
}
const file=new URL('parity/language-features.lino',root),inventory=readProjectData(file);
const common=['parity/fixtures/rust-source-render-interfaces.lino','parity/translation/rust-source-render-interface.lino','experiments/rust-source-render/oracle.mjs'];
const row={id:'rust-source-render-interfaces',required:true,
 javascript:{status:'implemented',evidence:['js/src/translation/rust-record-rules.js','js/src/translation/emit-rust.js','js/tests/translation-rust-source-render-interfaces.test.js',...common]},
 rust:{status:'implemented',evidence:['rust/src/translation/rust_record_rules.rs','rust/src/translation/rust_source_render_interfaces.rs','rust/src/translation/emit_rust/expressions.rs','rust/tests/unit/translation_rust_source_render_interfaces.rs',...common]}};
const at=inventory.features.findIndex(entry=>entry.id===row.id);if(at<0)inventory.features.push(row);else inventory.features[at]=row;writeProjectData(file,inventory);
const native=new URL('rust/tests/unit/mod.rs',root),declaration='mod translation_rust_source_render_interfaces;',source=readFileSync(native,'utf8');
if(!source.includes(declaration))writeFileSync(native,source+'\n'+declaration+'\n');
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
