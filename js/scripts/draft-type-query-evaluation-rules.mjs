// Actual source translation only; no fixtures or represented functions execute.
import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/type-query-evaluation-rules.js',moduleName:'type_query_evaluation_rules',
 model:'evaluated JavaScript known type queries 1',programAsset:'type-query-evaluation-program'});
