import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/ordered-record-layout-rules.js',moduleName:'ordered_record_layout_rules',sourceDependencies:true,model:'separate ordered record payload and own-field order target layouts 1'});
