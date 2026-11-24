import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-scalar-vector-copy-rules.js',moduleName:'rust_scalar_vector_copy_rules',model:'ordinary Rust scalar vector copies 1',programAsset:'rust-scalar-vector-copy-program'});
