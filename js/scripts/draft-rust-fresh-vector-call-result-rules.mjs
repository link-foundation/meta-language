import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-fresh-vector-call-result-rules.js',moduleName:'rust_fresh_vector_call_result_rules',sourceDependencies:true,model:'closed fresh Rust vector result forwarding 1',programAsset:'rust-fresh-vector-call-result-program'});
