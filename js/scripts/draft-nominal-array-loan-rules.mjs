// Source generation only, never projections or future controls.
import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/nominal-array-loan-rules.js',moduleName:'nominal_array_loan_rules',
 model:'checked immutable nominal array field loans 1',programAsset:'nominal-array-loan-program'});
