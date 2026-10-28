import {writeProjectData} from './project-data.mjs';
import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
writeProjectData(new URL('parity/self-translation/source-boolean-match-sharing.lino',root),{schemaVersion:1,model:'literal Boolean constructor match arms 1',preserveScrutinee:true,requireCompleteOwner:true,scope:'Only fully covered closed nominal constructors with literal Boolean bodies; no expression evaluation, guard removal or binder-dependent body substitution.'});
draftSourceLibrary({sourcePath:'js/src/translation/boolean-constructor-match-rules.js',moduleName:'boolean_constructor_match_rules',model:'complete finite Boolean constructor match sharing 1',programAsset:'boolean-constructor-match-program'});
await import('./draft-boolean-constructor-match-fixtures.mjs');
