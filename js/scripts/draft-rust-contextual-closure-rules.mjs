import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-contextual-closure-rules.js',moduleName:'rust_contextual_closure_rules',sourceDependencies:true,model:'ordinary Rust contextual immutable Copy closures 1',programAsset:'rust-contextual-closure-program'});
