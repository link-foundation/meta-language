import {draftTranslationProfileInventory} from './draft-translation-profile-inventory.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftTranslationProfileInventory({id:'javascript-closed-classes',fixture:'javascript-closed-classes',programAsset:'javascript-class-source-program',
 nativeTest:'translation_javascript_classes',experimentPath:'experiments/javascript-classes/oracle.mjs',
 javascript:['js/src/translation/javascript-class-source-rules.js','js/src/translation/javascript-classes.js','js/tests/translation-javascript-classes.test.js','js/src/self-translation.js'],
 rust:['rust/src/translation/javascript_class_source_rules.rs','rust/src/translation/javascript/classes.rs','rust/src/self_translation/javascript_classes.rs','rust/tests/unit/translation_javascript_classes.rs']});
