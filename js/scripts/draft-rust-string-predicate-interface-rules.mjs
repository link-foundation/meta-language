import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-string-predicate-interface-rules.js',moduleName:'rust_string_predicate_interface_rules',model:'retained string predicate Rust function interface 1',programAsset:'rust-string-predicate-interface-program',sourceDependencies:true});
