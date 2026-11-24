import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-comparison-rules.js',moduleName:'javascript_comparison_rules',
 model:'JavaScript primitive equality and relational comparison 1',programAsset:'javascript-comparison-program'});
