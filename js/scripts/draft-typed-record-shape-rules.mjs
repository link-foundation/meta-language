import {draftSourceLibrary} from './draft-source-library.mjs';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),profile={schemaVersion:1,model:'ordered typed program record fields 1',rootField:'record layout',profileTerm:'typed program record layout',shapeTerm:'typed program record field shape',recordOccurrences:false};
for(const runtime of ['js','rust'])writeProjectData(new URL(runtime+'/src/data/typed-program-record-shapes.lino',root),profile);
draftSourceLibrary({sourcePath:'js/src/translation/typed-record-shape-rules.js',moduleName:'typed_record_shape_rules',model:profile.model,programAsset:'typed-record-shape-program'});
