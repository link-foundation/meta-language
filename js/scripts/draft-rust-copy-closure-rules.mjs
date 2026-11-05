import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-copy-closure-rules.js',moduleName:'rust_copy_closure_rules',sourceDependencies:true,model:'ordinary explicitly typed immutable Rust Copy closures 1',programAsset:'rust-copy-closure-program'});
