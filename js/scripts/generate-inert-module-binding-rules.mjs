// Complete JavaScript rules and both checked codecs own native rule generation.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {writeRecordRuleLibrary} from './record-rule-library.mjs';
import {checkProjectData,readProjectData,writeProjectData} from './project-data.mjs';
const root=new URL('../../',import.meta.url),sourcePath='js/src/translation/inert-module-binding-rules.js';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
export async function generateInertModuleBindingRules({check=false,native=false}={}){
 const imports="import {readRequiredNumberFact,readRequiredTextFact,readRequiredBooleanFact} from './fact-column-rules.js';\n";
 const additionalPath='js/src/translation/empty-module-collection-rules.js',additionalImport="import {readRequiredNumberFact,readRequiredTextFact} from './fact-column-rules.js';\n";
 const sources=['js/src/translation/fact-column-rules.js',sourcePath,additionalPath].map(path=>{const source=readFileSync(new URL(path,root),'utf8');return{path,source,...pin(source)};});
 const bodies=[imports,additionalImport].map((removed,index)=>{const own=sources[index+1].source;assert.equal(own.split(removed).length,2);const body=own.replace(removed,'');assert.doesNotMatch(body,/^\s*import\b/mu);return body;});assert.doesNotMatch(sources[0].source,/^\s*import\b/mu);
 const emptyCollections={schemaVersion:1,model:'empty standard collection module bindings 1',sourcePath:additionalPath,cases:[
  {source:'const cache=new Map();',name:'cache',owner:1},
  {source:'const cache=new Map;',name:'cache',owner:1},
  {source:'const cache=new Set();',name:'cache',owner:2},
  {source:'const cache=new Set;',name:'cache',owner:2},
  {source:'/*λ*/ export const cache = new Map(/* empty */);\r\n',name:'cache',owner:1},
  {source:'export const cache = new Set /* empty */;',name:'cache',owner:2},
 ],assertionVersions:{Node:'>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0'},boundary:'Declared actual source cases. Every syntax decision requires the complete parser-owned CST and a separate unshadowed standard owner proof; these declarations are not execution observations.'};
 const fixture=new URL('parity/fixtures/empty-module-collections.lino',root);if(check)checkProjectData(fixture,emptyCollections);else writeProjectData(fixture,emptyCollections);
 const source=[sources[0].source,...bodies].join('\n'),policy={schemaVersion:1,model:'complete inert literal module binding scope 1',sourcePath,sourceClosure:{source,...pin(source),sources,removedImports:[imports,additionalImport]},decisions:['readInertModuleBinding','acceptInertModuleBindingScope','readEmptyStandardCollectionModuleBinding'],additionalSourceInterpretations:[{model:emptyCollections.model,sourcePath:additionalPath,fixture:'parity/fixtures/empty-module-collections.lino'}],assertionVersions:emptyCollections.assertionVersions,boundary:'Scope classification only, from complete actual native CST facts within65536 source bytes/4096 nodes. Literal containers and exact root standard Set/Map literal constructors or Object.freeze on literal containers retain their real lexical owner. An additive interpretation admits empty standard Map/Set allocation, with or without argument parentheses. Actual module binding names must exclude each required standard owner. This supplies no initializer translation, execution evidence, complete module behavior or class support.'};
 for(const path of ['js/src/translation/data/inert-module-bindings.lino','rust/src/data/inert-module-bindings.lino']){if(check)checkProjectData(new URL(path,root),policy);else writeProjectData(new URL(path,root),policy);}
 const featurePath=new URL('parity/language-features.lino',root),catalog=readProjectData(featurePath);
 const feature={id:'inert-literal-module-binding-scope',required:true,
  javascript:{status:'implemented',evidence:[sourcePath,'js/src/translation/inert-module-bindings.js',additionalPath,'js/src/translation/empty-module-collections.js','js/src/self-translation.js','js/scripts/generate-inert-module-binding-rules.mjs','js/src/translation/data/inert-module-bindings.lino','js/src/data/inert-module-binding-program.lino','js/tests/translation-inert-module-bindings.test.js','js/tests/translation-empty-module-collections.test.js','parity/fixtures/empty-module-collections.lino']},
  rust:{status:'implemented',evidence:['rust/src/self_translation/binding.rs','rust/src/self_translation/binding/deferred_scope.rs','rust/src/self_translation/binding/recursive_groups.rs','rust/src/translation/inert_module_binding_rules.rs','rust/src/translation/inert_module_binding_rules/part_1.rs','rust/src/translation/inert_module_binding_rules/part_2.rs','rust/src/translation/inert_module_binding_rules/part_3.rs','rust/src/translation/inert_module_binding_rules/part_4.rs','rust/src/data/inert-module-bindings.lino','rust/src/data/inert-module-binding-program.lino','rust/tests/unit/self_translation_inert_module_bindings.rs','parity/self-translation/inert-module-binding-rules-native.lino','parity/fixtures/empty-module-collections.lino']}};
 const registered=catalog.features.filter(row=>row.id===feature.id);assert.ok(registered.length<=1);
 if(check)assert.deepEqual(registered,[feature]);else{if(registered.length)Object.assign(registered[0],feature);else catalog.features.push(feature);writeProjectData(featurePath,catalog);}
 if(!native)return{policy};
 const {parseJavaScript}=await import('../src/translation/javascript.js'),{checkProgram}=await import('../src/translation/check.js'),{emitRust}=await import('../src/translation/emit-rust.js'),{encodeTypedProgramLinks,decodeTypedProgramLinks}=await import('../src/typed-program-links.js'),{recheckLinkedTypedProgram}=await import('../src/typed-program-checked-projection.js'),{LinkNetwork}=await import('../src/network.js'),{encodeLinkNetworkBinary,decodeLinkNetworkBinary}=await import('../src/link-network-binary.js');
 const program=checkProgram(parseJavaScript(source,{referenceSemantics:false})),emitted=emitRust(program,{arrayConsumption:true}),packet=encodeTypedProgramLinks(program),text=packet.network.toLino();
 const replays=[];for(const[format,network]of[['text',LinkNetwork.fromLino(text)],['binary',decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]]){
  const checked=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true}),replay=emitRust(checked,{arrayConsumption:true});
  assert.deepEqual([...checked.declarations.keys()],[...program.declarations.keys()]);for(const field of['text','definitions','preludes','mappings','assumptions','encodings'])assert.deepEqual(replay[field],emitted[field]);replays.push({format,...pin(replay.text),owners:[...checked.declarations.keys()]});
 }
 for(const path of['js/src/data/inert-module-binding-program.lino','rust/src/data/inert-module-binding-program.lino']){if(check)assert.equal(readFileSync(new URL(path,root),'utf8'),text);else writeFileSync(new URL(path,root),text);}
 const files=writeRecordRuleLibrary(emitted,'inert_module_binding_rules',{check});
 for(const file of files)assert.ok(feature.rust.evidence.includes('rust/src/translation/'+(file.path??'inert_module_binding_rules.rs')),'every generated native source part belongs to the feature inventory');
 const artifact={schemaVersion:1,model:policy.model,sourceClosure:policy.sourceClosure,owners:[...program.declarations.keys()],replays,archive:{root:packet.root.asU64(),...pin(text)},emitted:{text:emitted.text,definitions:emitted.definitions,preludes:emitted.preludes,mappings:emitted.mappings,assumptions:emitted.assumptions,encodings:emitted.encodings},files:files.map(file=>({path:file.path??'inert_module_binding_rules.rs',source:file.source,...pin(file.source)})),boundary:'Automatic complete JavaScript rule generation and both source-free archives only. Native source adapters, actual scope integration and native compilation/execution remain separate work.'};
 const path=new URL('parity/self-translation/inert-module-binding-rules-native.lino',root);if(check)checkProjectData(path,artifact);else writeProjectData(path,artifact);return{policy,artifact};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){assert.ok(process.argv.slice(2).every(value=>['--check','--native'].includes(value)));await generateInertModuleBindingRules({check:process.argv.includes('--check'),native:process.argv.includes('--native')});}
