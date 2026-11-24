import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/grammar-composition-batch-rules.js',moduleName:'grammar_composition_batch_rules',directory:'',sourceDependencies:true,model:'source-qualified grammar composition batch 1',programAsset:'grammar-composition-batch-program'});
