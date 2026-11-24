import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-array-index-rules.js',moduleName:'rust_array_index_rules',model:'ordinary Rust scalar array reads 1',programAsset:'rust-array-index-program'});
