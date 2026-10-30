import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-constructed-string-interface-rules.js',moduleName:'rust_constructed_string_interface_rules',model:'retained constructed Rust string function interface 1',programAsset:'rust-constructed-string-interface-program',sourceDependencies:true});
