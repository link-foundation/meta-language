import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/optional-callable-rules.js',moduleName:'optional_callable_rules',sourceDependencies:true,model:'actual optional callable arguments and explicit value bridges 1',programAsset:'optional-callable-program'});
