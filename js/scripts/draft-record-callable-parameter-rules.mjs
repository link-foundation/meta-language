import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/record-callable-parameter-rules.js',moduleName:'record_callable_parameter_rules',model:'actual ordinary record callable parameters 1',sourceDependencies:true,programAsset:'record-callable-parameter-program'});
