import {draftTranslationProfileInventory} from './draft-translation-profile-inventory.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftTranslationProfileInventory({id:'javascript-ordinary-scalar-closure-defaults',fixture:'scalar-closure-defaults',programAsset:'scalar-closure-default-program',nativeTest:'translation_javascript_scalar_closure_defaults',
 javascript:['js/src/translation/scalar-closure-default-rules.js','js/src/translation/whole-parameter-defaults.js','js/src/translation/array-formal-parameters.js','js/src/translation/whole-parameter-default-rules.js','js/tests/translation-scalar-closure-defaults.test.js'],
 rust:['rust/src/translation/scalar_closure_default_rules.rs','rust/src/translation/javascript/whole_parameter_defaults.rs','rust/src/translation/javascript/array_formal_parameters.rs','rust/src/translation/whole_parameter_default_rules.rs','rust/tests/unit/translation_javascript_scalar_closure_defaults.rs']});
