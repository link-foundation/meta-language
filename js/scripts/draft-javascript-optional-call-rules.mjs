import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-optional-call-rules.js',moduleName:'javascript_optional_call_rules',model:'represented JavaScript lexical and property optional callable construction 2',programAsset:'javascript-optional-call-program'});
