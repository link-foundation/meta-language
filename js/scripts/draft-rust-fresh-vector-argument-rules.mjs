import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-fresh-vector-argument-rules.js',moduleName:'rust_fresh_vector_argument_rules',model:'ordinary fresh Rust scalar vector arguments 1',programAsset:'rust-fresh-vector-argument-program'});
