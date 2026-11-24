import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/string-array-sort-rules.js',moduleName:'string_array_sort_rules',model:'dense primitive string copying sort 1',programAsset:'string-array-sort-program'});
