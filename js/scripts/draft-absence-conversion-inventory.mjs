import {draftTranslationProfileInventory} from './draft-translation-profile-inventory.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftTranslationProfileInventory({id:'javascript-tag-preserving-absence-only-conversions',fixture:'absence-conversions',programAsset:'absence-conversion-program',nativeTest:'translation_javascript_absence_conversions',
 javascript:['js/src/translation/absence-conversion-rules.js','js/src/translation/absence-conversions.js','js/src/translation/absence-only-rules.js','js/src/translation/check.js','js/src/translation/javascript-infer.js','js/tests/translation-absence-conversions.test.js'],
 rust:['rust/src/translation/absence_conversion_rules.rs','rust/src/translation/check/absence_conversions.rs','rust/src/translation/check.rs','rust/src/translation/javascript/infer/records.rs','rust/tests/unit/translation_javascript_absence_conversions.rs']});
