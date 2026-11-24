import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-data-type-field-signature-rules.js',moduleName:'rust_data_type_field_signature_rules',model:'Rust data type field signatures 1',programAsset:'rust-data-type-field-signature-program'});
