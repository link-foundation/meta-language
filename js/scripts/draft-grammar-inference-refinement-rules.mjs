import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/grammar-inference-refinement-rules.js',moduleName:'grammar_inference_refinement_rules',directory:'',model:'labelled regular grammar refinement 1',sourceDependencies:true,programAsset:'grammar-inference-refinement-program'});
