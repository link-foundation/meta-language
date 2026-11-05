import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-source-owner-retention-rules.js',moduleName:'rust_source_owner_retention_rules',model:'retained Rust source signature owners 2',programAsset:'rust-source-owner-retention-program'});
