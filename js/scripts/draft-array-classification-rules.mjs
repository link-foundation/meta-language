import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/array-classification-rules.js',moduleName:'array_classification_rules',
 model:'JavaScript actual array classification and evaluated builtin arguments 2',programAsset:'array-classification-program'});
