import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-copy-closure-parameter-rules.js',moduleName:'rust_copy_closure_parameter_rules',model:'actual immutable scalar Rust closure parameters 1',programAsset:'rust-copy-closure-parameter-program'});
