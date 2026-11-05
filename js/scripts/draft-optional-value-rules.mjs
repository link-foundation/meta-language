// Generate the complete current source without executing codec/fixture gates.
import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/optional-value-rules.js',moduleName:'optional_value_rules',
 model:'JavaScript optional values and Boolean conversion source operations 2',programAsset:'optional-value-program'});
