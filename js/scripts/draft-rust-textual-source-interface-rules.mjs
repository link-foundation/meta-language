import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-textual-source-interface-rules.js',moduleName:'rust_textual_source_interface_rules',model:'retained textual Rust function interface 1',programAsset:'rust-textual-source-interface-program',sourceDependencies:true});
