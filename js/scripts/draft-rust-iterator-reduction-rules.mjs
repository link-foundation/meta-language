import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-iterator-reduction-rules.js',moduleName:'rust_iterator_reduction_rules',sourceDependencies:true,model:'ordinary Rust immediate copied scalar iterator reductions 1',programAsset:'rust-iterator-reduction-program'});
