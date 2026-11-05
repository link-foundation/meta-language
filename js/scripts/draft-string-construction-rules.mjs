import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/string-construction-rules.js',moduleName:'string_construction_rules',
 model:'JavaScript source string construction intrinsics 1',programAsset:'string-construction-program'});
