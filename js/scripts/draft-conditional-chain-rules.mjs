import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/conditional-chain-rules.js',moduleName:'conditional_chain_rules',sourceDependencies:true,model:'ordered conditional continuation packets 1',programAsset:'conditional-chain-program'});
