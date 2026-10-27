import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-integer-float-plan-rules.js',moduleName:'rust_integer_float_plan_rules',model:'Rust exact small integer binary64 source term decomposition 1',programAsset:'rust-integer-float-plan-program'});
