import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-primitive-expression-rules.js',moduleName:'rust_primitive_expression_rules',sourceDependencies:true,model:'Rust exact primitive expression source owner closure 1',programAsset:'rust-primitive-expression-program'});
