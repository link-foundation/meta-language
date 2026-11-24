import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-standard-owned-call-rules.js',moduleName:'rust_standard_owned_call_rules',model:'standard Rust owned value calls 1',programAsset:'rust-standard-owned-call-program'});
