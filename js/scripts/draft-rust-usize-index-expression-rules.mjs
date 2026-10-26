import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-usize-index-expression-rules.js',moduleName:'rust_usize_index_expression_rules',model:'contextual Rust usize index expressions 1',programAsset:'rust-usize-index-expression-program',sourceDependencies:true});
