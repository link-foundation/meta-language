// Source declarations only, with no execution observations or product imports.
import {writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {renderRustTest,renderRustItemImports} from './generate-export-parity.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const file=new URL('../../parity/language-features.lino',import.meta.url),value=readProjectData(file),id='intrinsic-module-initializer-source-scope';
const common=['parity/fixtures/intrinsic-module-initializers.lino','parity/self-translation/intrinsic-initializer-rules-native.lino'];
const row={id,required:true,javascript:{status:'implemented',evidence:['js/src/translation/intrinsic-initializer-rules.js','js/src/translation/intrinsic-initializers.js','js/src/self-translation.js','js/tests/translation-intrinsic-initializers.test.js',...common]},
 rust:{status:'implemented',evidence:['rust/src/translation/intrinsic_initializer_rules.rs','rust/src/self_translation/binding.rs','rust/src/self_translation/binding/deferred_scope.rs','rust/src/self_translation/binding/recursive_groups.rs','rust/tests/intrinsic_module_initializers.rs',...common]}};
const position=value.features.findIndex(row=>row.id===id);if(position<0)value.features.push(row);else value.features[position]=row;
value.javascriptExports.readIntrinsicModuleInitializerBinding={rust:'read_intrinsic_module_initializer_binding',kind:'function'};
value.javascriptExports.acceptIntrinsicModuleInitializerScope={rust:'accept_intrinsic_module_initializer_scope',kind:'function'};
value.javascriptExports=Object.fromEntries(Object.entries(value.javascriptExports).sort(([first],[second])=>first<second?-1:first>second?1:0));
writeProjectData(file,value);
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity.rs',import.meta.url),renderRustTest(value.javascriptExports));
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity/items.rs',import.meta.url),renderRustItemImports(value.javascriptExports));
