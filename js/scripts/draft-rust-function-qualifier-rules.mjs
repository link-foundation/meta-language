import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-function-qualifier-rules.js',moduleName:'rust_function_qualifier_rules',model:'ordinary Rust const function runtime interpretation 1',programAsset:'rust-function-qualifier-program'});
