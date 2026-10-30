import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-primitive-from-rules.js',moduleName:'rust_primitive_from_rules',sourceDependencies:true,model:'Rust exact primitive From source conversions 1',programAsset:'rust-primitive-from-program'});
