import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/utf16-method-emission-rules.js',moduleName:'utf16_method_emission_rules',model:'UTF16 typed string method source transport 1',programAsset:'utf16-method-emission-program'});
