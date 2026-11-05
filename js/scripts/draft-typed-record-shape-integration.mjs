// Actual source parsing/generation and inventories only. No controls imported.
import {readFileSync,writeFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {resolveExports,renderRustTest,renderRustItemImports} from './generate-export-parity.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
for(const path of ['js/src/translation/typed-record-shape-rules.js','js/src/typed-program-record-shapes.js','js/src/typed-program-links.js','js/src/index.js',
 'js/src/formal-function-links.js','js/src/formal-concept-definition.js','js/scripts/draft-source-library.mjs','js/scripts/draft-typed-record-shape-rules.mjs',
 'js/scripts/draft-typed-record-shape-fixtures.mjs','js/scripts/draft-typed-record-shape-integration.mjs','experiments/javascript-properties/record-shape-controls.mjs',
 'experiments/javascript-properties/record-shape-oracle.mjs','experiments/javascript-properties/record-shape-native-reload.mjs','js/tests/typed-program-record-shapes.test.js']){
 parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();
}
for(const path of ['draft-typed-record-shape-rules.mjs','draft-typed-record-shape-fixtures.mjs']){await import(new URL(path,import.meta.url));globalThis.gc?.();}
for(const [path,declarations]of [['rust/src/translation/mod.rs',['pub(crate) mod typed_record_shape_rules;']],['rust/tests/unit/mod.rs',['mod typed_program_record_shapes;']]]){
 const file=new URL(path,root);let source=readFileSync(file,'utf8');for(const declaration of declarations)if(!source.includes(declaration))source+='\n'+declaration+'\n';writeFileSync(file,source);
}
const file=new URL('parity/language-features.lino',root),inventory=readProjectData(file),common=['parity/fixtures/typed-program-record-shapes.lino','experiments/javascript-properties/record-shape-controls.mjs','experiments/javascript-properties/record-shape-oracle.mjs','experiments/javascript-properties/record-shape-native-reload.mjs'];
const row={id:'typed-program-record-shapes',required:true,javascript:{status:'implemented',evidence:['js/src/typed-program-record-shapes.js','js/src/translation/typed-record-shape-rules.js','js/tests/typed-program-record-shapes.test.js',...common]},rust:{status:'implemented',evidence:['rust/src/typed_program_links/record_shapes.rs','rust/src/translation/typed_record_shape_rules.rs','rust/tests/unit/typed_program_record_shapes.rs',...common]}};
const at=inventory.features.findIndex(entry=>entry.id===row.id);if(at<0)inventory.features.push(row);else inventory.features[at]=row;
// Source-only addition of actual explicit public exports; importing the package
// to discover exports would evaluate its runtime setup during this draft.
const names=['encodeTypedProgramLinksWithValueSharing','encodeTypedProgramLinksWithRecordShapes'];
const nativeNames=['encode_typed_program_links_with_value_sharing','encode_typed_program_links_with_record_shapes'];
const definitions=readFileSync(new URL('rust/src/typed_program_links.rs',root),'utf8'),nativeRoot=readFileSync(new URL('rust/src/lib.rs',root),'utf8'),javascriptRoot=readFileSync(new URL('js/src/index.js',root),'utf8');
const nativeExports=nativeRoot.match(/pub use typed_program_links::\{([\s\S]*?)\};/u)?.[1],javascriptExports=javascriptRoot.match(/export \{([^}]*?)\} from '\.\/typed-program-links\.js';/u)?.[1];
const sourceFunctions=new Set([...definitions.matchAll(/^pub fn (\w+)\(/gmu)].map(match=>match[1]));
for(const [index,name]of nativeNames.entries())if(!sourceFunctions.has(name)||!nativeExports?.split(/[^A-Za-z0-9_]+/u).includes(name)||!javascriptExports?.split(/[^A-Za-z0-9_]+/u).includes(names[index]))throw new TypeError('the actual paired source export is required: '+name);
const mapped=resolveExports(names,nativeNames.map(name=>({name,path:name,kind:'function'})));if(mapped.missing.length)throw new TypeError('new source exports require actual Rust declarations: '+mapped.missing.join(', '));
inventory.javascriptExports=Object.fromEntries(Object.entries({...inventory.javascriptExports,...mapped.entries}).sort(([first],[second])=>first<second?-1:first>second?1:0));writeProjectData(file,inventory);
writeFileSync(new URL('rust/tests/unit/javascript_export_parity.rs',root),renderRustTest(inventory.javascriptExports));
writeFileSync(new URL('rust/tests/unit/javascript_export_parity/items.rs',root),renderRustItemImports(inventory.javascriptExports));globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
