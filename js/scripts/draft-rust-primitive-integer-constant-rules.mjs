import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-primitive-integer-constant-rules.js',moduleName:'rust_primitive_integer_constant_rules',sourceDependencies:true,model:'Rust exact primitive integer associated source constants 1',programAsset:'rust-primitive-integer-constant-program'});
