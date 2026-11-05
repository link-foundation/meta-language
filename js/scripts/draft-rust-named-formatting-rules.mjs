import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-named-formatting-rules.js',moduleName:'rust_named_formatting_rules',sourceDependencies:true,model:'Rust explicit named formatting source operands 1',programAsset:'rust-named-formatting-program'});
