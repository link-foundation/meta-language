import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/record-closure-parameter-rules.js',moduleName:'record_closure_parameter_rules',model:'actual flat required record closure parameter context 1',sourceDependencies:true,programAsset:'record-closure-parameter-program'});
