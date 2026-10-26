import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-fresh-vector-result-rules.js',moduleName:'rust_fresh_vector_result_rules',sourceDependencies:true,model:'ordinary fresh Rust scalar vector results 1',programAsset:'rust-fresh-vector-result-program'});
