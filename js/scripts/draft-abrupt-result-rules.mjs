import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/abrupt-result-rules.js',moduleName:'abrupt_result_rules',sourceDependencies:true,model:'nonreturning typed computation result roles 1',programAsset:'abrupt-result-program'});
