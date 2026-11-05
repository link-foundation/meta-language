import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-type-token-rules.js',moduleName:'rust_type_token_rules',model:'contextual Rust generic closing punctuation 1',programAsset:'rust-type-token-program'});
