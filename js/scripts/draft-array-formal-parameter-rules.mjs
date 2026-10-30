import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/array-formal-parameter-rules.js',moduleName:'array_formal_parameter_rules',model:'actual ordinary array formal parameters 1',sourceDependencies:true,programAsset:'array-formal-parameter-program'});
