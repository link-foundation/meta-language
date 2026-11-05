import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-signed-integer-property-rules.js',moduleName:'rust_signed_integer_property_rules',sourceDependencies:true,model:'Rust exact signed integer sign source properties 1',programAsset:'rust-signed-integer-property-program'});
