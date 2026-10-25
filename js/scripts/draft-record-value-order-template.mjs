// Capture the complete source and actual parameter owner; no template executes.
import {readSourceLibraryClosure} from './read-source-library-closure.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {writeProjectData} from './project-data.mjs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
const sourceClosure=readSourceLibraryClosure('js/src/translation/record-value-order-template.js');
const sourceSyntax=parseAuthoredJavaScriptModule(sourceClosure.source,'record-value-order-template');
const surface=parseJavaScript(sourceClosure.source,{referenceSemantics:false});
const owner=surface.items.find(item=>item.k==='data'&&item.name==='OpaqueValue');
if(!owner||owner.ctors.length!==1||owner.ctors[0].fields.length!==1||owner.ctors[0].fields[0].name!=='value')throw new TypeError('source specialization needs its one actual opaque payload owner');
const value={version:1,model:'ordinary record value ordering with one opaque payload type parameter 1',sourceClosure,sourceSyntax,surface,
 parameter:{data:owner.name,constructor:owner.ctors[0].name,field:owner.ctors[0].fields[0].name},entry:'reorderWrappedRecordPropertyValues',
 pair:{data:'WrappedEntry',constructor:'wrappedEntry',entry:'pairWrappedOwnPropertyValues'},
 scope:'Only the actual opaque payload field type is specialized. Its wrapper remains present, so null/undefined payloads remain distinct from absent array entries. Every source declaration and body remains intact.',
};
for(const directory of ['js','rust'])writeProjectData(new URL(directory+'/src/data/record-value-order-template.lino',root),value);
