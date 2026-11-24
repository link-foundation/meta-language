// Actual source construction only; never replay controls.
import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/utf16-text-expansion-rules.js',moduleName:'utf16_text_expansion_rules',
 model:'bounded UTF16 repetition and padding 1',programAsset:'utf16-text-expansion-program'});
