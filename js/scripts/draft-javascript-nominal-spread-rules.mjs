import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-nominal-spread-rules.js',moduleName:'javascript_nominal_spread_rules',
 model:'owned nominal JavaScript complete field copies 1',programAsset:'javascript-nominal-spread-program'});
