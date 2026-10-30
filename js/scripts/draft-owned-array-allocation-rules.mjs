import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/owned-array-allocation-rules.js',moduleName:'owned_array_allocation_rules',model:'typed standard array slice allocation ownership 1',programAsset:'owned-array-allocation-program'});
