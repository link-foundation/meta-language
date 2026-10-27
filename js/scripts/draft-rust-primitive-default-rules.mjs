import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-primitive-default-rules.js',moduleName:'rust_primitive_default_rules',sourceDependencies:true,model:'Rust exact standard primitive Default source values 1',programAsset:'rust-primitive-default-program'});
