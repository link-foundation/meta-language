import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-optional-chain-rules.js',moduleName:'javascript_optional_chain_rules',sourceDependencies:true,model:'continuous optional chain receiver and suffix scope construction 1',programAsset:'javascript-optional-chain-program'});
