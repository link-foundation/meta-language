import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/optional-policy-rules.js',moduleName:'optional_policy_rules',model:'JavaScript optional absence-policy widening with exact present domains 1',programAsset:'optional-policy-program'});
