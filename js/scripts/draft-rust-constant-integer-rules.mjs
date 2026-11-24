import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-constant-integer-rules.js',moduleName:'rust_constant_integer_rules',sourceDependencies:true,model:'exact bounded Rust constant integer expressions 1',programAsset:'rust-constant-integer-program'});
