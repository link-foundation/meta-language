// Reparse/type actual source; only model I/O changes when full native source is
// byte-identical. Never replay an archived program or run its operations.
import './draft-javascript-primitive-conversion-spellings.mjs';
import {readSourceLibraryClosure} from './read-source-library-closure.mjs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {draftNativeModelCodecs} from './draft-native-model-codecs.mjs';
import {readProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const closure=readSourceLibraryClosure('js/src/translation/javascript-property-key-conversion.js');
parseAuthoredJavaScriptModule(closure.source,'js/src/translation/javascript-property-key-conversion.js');
const program=checkProgram(parseJavaScript(closure.source,{referenceSemantics:false})),emitted=emitRust(program,{arrayConsumption:true});
const recipe=readProjectData(new URL('../../parity/self-translation/javascript-property-key-conversion-native.lino',import.meta.url));
if(recipe.sourceClosure.source!==closure.source||recipe.emitted.text!==emitted.text||recipe.emitted.mappings.length!==emitted.mappings.length
 ||recipe.emitted.mappings.some((row,index)=>row.kind!==emitted.mappings[index].kind||row.source!==emitted.mappings[index].source||row.target!==emitted.mappings[index].target))throw new TypeError('actual normative native source must be regenerated before authoring its model I/O');
const definition=readProjectData(new URL('../../parity/translation/native-model-wire-rules.lino',import.meta.url));
if(definition.schemaVersion!==1||definition.model!=='JavaScript semantic model exact Number wire fields 1')throw new TypeError('actual qualified Links Number rules are required');
draftNativeModelCodecs({program,emitted,roots:['JavaScriptOrdinaryHeap','JavaScriptSemanticValue','JavaScriptPrimitiveConversionState','JavaScriptPrimitiveConversionCompletion','JavaScriptPropertyKeyConversionTransition'],wireRules:definition.rules,
 output:'rust/tests/unit/javascript_property_key_conversion_codecs.rs',receipt:'parity/self-translation/javascript-property-key-conversion-model-codecs.lino'});
