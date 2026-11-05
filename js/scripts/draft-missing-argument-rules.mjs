import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/missing-argument-rules.js',moduleName:'missing_argument_rules',sourceDependencies:true,model:'actual implicit JavaScript undefined inputs for inferred callable parameters 1',programAsset:'missing-argument-program'});
