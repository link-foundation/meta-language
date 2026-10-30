import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-owned-binding-rules.js',moduleName:'rust_owned_binding_rules',model:'ordinary Rust owned vector binding consumption 1',programAsset:'rust-owned-binding-program'});
