import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/named-default-callable-rules.js',moduleName:'named_default_callable_rules',sourceDependencies:true,model:'named callable value defaults through existing initialization 1',programAsset:'named-default-callable-program'});
