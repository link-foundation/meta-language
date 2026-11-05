import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-source-constant-rules.js',moduleName:'rust_source_constant_rules',model:'ordinary Rust source constant values distinct from free function items 1',programAsset:'rust-source-constant-program'});
