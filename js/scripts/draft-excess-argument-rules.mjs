import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/excess-argument-rules.js',moduleName:'excess_argument_rules',model:'all supplied JavaScript arguments before callee initialization 1',programAsset:'excess-argument-program'});
