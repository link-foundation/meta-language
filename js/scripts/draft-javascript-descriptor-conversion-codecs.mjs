// Fast I/O source author: reparse/type actual normative definitions and emit
// actual target mappings, without repartitioning unchanged native libraries.
// No archived program, fixture or represented conversion is executed.
import './draft-javascript-descriptor-field-rules.mjs';
import {readSourceLibraryClosure} from './read-source-library-closure.mjs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {draftNativeModelCodecs} from './draft-native-model-codecs.mjs';
import {readProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const closure=readSourceLibraryClosure('js/src/translation/javascript-descriptor-conversion.js');
parseAuthoredJavaScriptModule(closure.source,'js/src/translation/javascript-descriptor-conversion.js');
const program=checkProgram(parseJavaScript(closure.source,{referenceSemantics:false})),emitted=emitRust(program,{arrayConsumption:true});
const recipe=readProjectData(new URL('../../parity/self-translation/javascript-descriptor-conversion-native.lino',import.meta.url));
if(recipe.sourceClosure.source!==closure.source||recipe.emitted.text!==emitted.text||recipe.emitted.mappings.length!==emitted.mappings.length
 ||recipe.emitted.mappings.some((row,index)=>row.kind!==emitted.mappings[index].kind||row.source!==emitted.mappings[index].source||row.target!==emitted.mappings[index].target))throw new TypeError('regenerate the actual native source before authoring its model transport');
const definition=readProjectData(new URL('../../parity/translation/native-model-wire-rules.lino',import.meta.url));
if(definition.schemaVersion!==1||definition.model!=='JavaScript semantic model exact Number wire fields 1')throw new TypeError('actual qualified Links wire rules are required');
draftNativeModelCodecs({program,emitted,roots:['JavaScriptOrdinaryHeap','JavaScriptSemanticValue','JavaScriptDescriptorConversionState','JavaScriptSemanticCompletion','JavaScriptDescriptorConversionTransition'],wireRules:definition.rules,
 output:'rust/tests/unit/javascript_descriptor_conversion_codecs.rs',receipt:'parity/self-translation/javascript-descriptor-conversion-model-codecs.lino'});
