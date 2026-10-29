// Source construction only; never import or execute represented controls.
import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/source-parameter-binding-rules.js',moduleName:'source_parameter_binding_rules',model:'actual JavaScript formal binding positions 1',programAsset:'source-parameter-binding-program'});
