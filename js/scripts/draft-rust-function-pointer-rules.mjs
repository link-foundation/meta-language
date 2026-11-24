import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-function-pointer-rules.js',moduleName:'rust_function_pointer_rules',sourceDependencies:true,model:'ordinary safe Rust function pointers over immutable Copy source domains 1',programAsset:'rust-function-pointer-program'});
