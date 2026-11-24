import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-iterator-pipeline-rules.js',moduleName:'rust_iterator_pipeline_rules',sourceDependencies:true,model:'lazy ordinary Rust scalar iterator pipelines 1',programAsset:'rust-iterator-pipeline-program'});
