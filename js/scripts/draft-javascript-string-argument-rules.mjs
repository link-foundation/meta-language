// Construct the normative argument plan, never execute a represented call.
import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-string-argument-rules.js',moduleName:'javascript_string_argument_rules',model:'ordered JavaScript String argument capture 1',programAsset:'javascript-string-argument-program'});
