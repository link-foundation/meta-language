import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-standard-string-view-rules.js',moduleName:'rust_standard_string_view_rules',model:'immediate standard Rust string views 1',programAsset:'rust-standard-string-view-program'});
