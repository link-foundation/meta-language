import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-primitive-function-value-rules.js',moduleName:'rust_primitive_function_value_rules',sourceDependencies:true,model:'Rust standard scalar primitive function values 1',programAsset:'rust-primitive-function-value-program'});
