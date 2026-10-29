import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/source-module-request-rules.js',moduleName:'source_module_request_rules',model:'actual source module requests 1',programAsset:'source-module-request-program'});
