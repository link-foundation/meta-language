// Future automatic source authoring only. This script never executes a sorter
// fixture or writes replacement native semantics. Do not run while drafting.
import {readSourceLibraryClosure} from './read-source-library-closure.mjs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {writeProjectData} from './project-data.mjs';
import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),sourcePath='js/src/translation/stable-vector-sort-template.js',sourceClosure=readSourceLibraryClosure(sourcePath),sourceSyntax=parseAuthoredJavaScriptModule(sourceClosure.source,sourcePath),surface=parseJavaScript(sourceClosure.source,{referenceSemantics:false}),owners=surface.items.filter(item=>item.k==='data'&&item.name==='OpaqueValue'),owner=owners[0];
if(owners.length!==1||owner.ctors.length!==1||owner.ctors[0].name!=='opaqueValue'||owner.ctors[0].fields.length!==1||owner.ctors[0].fields[0].name!=='value'||owner.ctors[0].fields[0].type.kind!=='float')throw new TypeError('stable vector source authoring needs its actual one-field opaque payload owner');
const template={version:1,model:'complete stable opaque UTF16 keyed vector insertion sort 1',sourceClosure,sourceSyntax,surface,parameter:{data:owner.name,constructor:owner.ctors[0].name,field:owner.ctors[0].fields[0].name},keyed:{data:'KeyedValue',constructor:'keyedValue'},entry:'sortWrappedStableVectorValues',comparison:'compareStableVectorKeys',scope:'Only the opaque payload field type specializes; all complete source bodies, UTF16 comparison and stable insertion remain ordinary source. Equal keys retain their original entry order. The payload wrapper is never removed inside sorting.'};
for(const directory of ['js','rust'])writeProjectData(new URL(directory+'/src/data/stable-vector-sort-template.lino',root),template);
draftSourceLibrary({sourcePath,moduleName:'stable_vector_sort',sourceDependencies:true,model:template.model,programAsset:'stable-vector-sort-program'});
