import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-displayed-string-interface-rules.js',moduleName:'rust_displayed_string_interface_rules',model:'retained displayed Rust string function interface 1',programAsset:'rust-displayed-string-interface-program',sourceDependencies:true});
