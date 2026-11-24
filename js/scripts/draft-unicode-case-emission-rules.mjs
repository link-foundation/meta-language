import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/unicode-case-emission-rules.js',moduleName:'unicode_case_emission_rules',model:'Unicode default casing target string transport 1',programAsset:'unicode-case-emission-program'});
