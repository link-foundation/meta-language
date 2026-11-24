import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/sequential-parameter-default-rules.js',moduleName:'sequential_parameter_default_rules',sourceDependencies:true,model:'ordered actual JavaScript parameter initialization environments 1',programAsset:'sequential-parameter-default-program'});
