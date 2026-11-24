import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-string-range-rules.js',moduleName:'javascript_string_range_rules',
 model:'JavaScript slice substring and Annex B substr source windows 1',programAsset:'javascript-string-range-program'});
