import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-formatting-rules.js',moduleName:'rust_formatting_rules',sourceDependencies:true,
 model:'Rust Display source formatting grammar and ordered operand captures 1',programAsset:'rust-formatting-program'});
