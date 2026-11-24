import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/whole-parameter-default-rules.js',moduleName:'whole_parameter_default_rules',sourceDependencies:true,model:'actual whole formal default intervals and represented contexts 1',programAsset:'whole-parameter-default-program'});
