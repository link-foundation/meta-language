import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/recursive-binding-rules.js',moduleName:'recursive_binding_rules',sourceDependencies:true,model:'actual recursive mixed binding ownership and iterable domains 1',programAsset:'recursive-binding-program'});
