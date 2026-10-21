// The normal checked translator emits every native policy algorithm.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {writeRecordRuleLibrary} from './record-rule-library.mjs';
import {tokenize} from '../src/translation/lexer.js';
const root=new URL('../../',import.meta.url),sourcePath='js/src/translation/array-classification-rules.js';
const decisions=['readJavaScriptArrayClassification','readJavaScriptArrayClassificationPlan'];
const digest=value=>createHash('sha256').update(value).digest('hex');
// Normative independently authored inputs/expectations. These are declarations,
// never observations; JavaScript and native CI execute every retained input.
const cases=[
  {name:'closed represented arrays tuples and non-array values',source:`function probe(){return [Array.isArray([1]),Array.isArray({left:1}),Array.isArray('A😀'),Array.isArray(3),Array.isArray(null),Array.isArray(undefined),Array.isArray(true),Array.isArray(Object.freeze([1,2]))].join('|');}console.log(probe());`,
    expected:'true|false|false|false|false|false|false|true\n'},
  {name:'single evaluation and both optional array absence policies',source:`function arrayValue(){console.log('array');return [1];}function scalarValue(){console.log('scalar');return 7;}/** @param {boolean} flag */function nullable(flag){console.log('nullable');return flag?[1]:null;}/** @param {boolean} flag */function optional(flag){console.log('optional');return flag?[2]:undefined;}/** @param {boolean} flag */function optionalScalar(flag){console.log('optional scalar');return flag?7:null;}console.log(Array.isArray(arrayValue()));console.log(Array.isArray(scalarValue()));console.log(Array.isArray(nullable(true)));console.log(Array.isArray(nullable(false)));console.log(Array.isArray(optional(true)));console.log(Array.isArray(optional(false)));console.log(Array.isArray(optionalScalar(true)));console.log(Array.isArray(optionalScalar(false)));`,
    expected:'array\ntrue\nscalar\nfalse\nnullable\ntrue\nnullable\nfalse\noptional\ntrue\noptional\nfalse\noptional scalar\nfalse\noptional scalar\nfalse\n'},
  {name:'flat optional immutable tuple presence',source:`/** @param {boolean} flag */function probe(flag){const value=flag?Object.freeze([1,2]):null;return Array.isArray(value);}console.log(probe(true));console.log(probe(false));`,expected:'true\nfalse\n'},
  {name:'typed empty arrays stay true independently of their length',source:`/** @returns {number[]} */function empty(){return [];}/** @param {boolean} flag @returns {number[]|null} */function optionalEmpty(flag){return flag?empty():null;}console.log(Array.isArray(empty()));console.log(Array.isArray(optionalEmpty(true)));console.log(Array.isArray(optionalEmpty(false)));`,expected:'true\ntrue\nfalse\n'},
];
cases.push(
  {name:'both nullish tags and a present empty array share one actual checked optional owner',source:`/** @param {number} flag @returns {number[]|null|undefined} */function choose(flag){return flag===0?[]:flag===1?null:undefined;}console.log(Array.isArray(choose(0)));console.log(Array.isArray(choose(1)));console.log(Array.isArray(choose(2)));`,expected:'true\nfalse\nfalse\n'},
  {name:'classification keeps actual shared array identity through a real array parameter',source:`/** @param {number[]} value */function classify(value){return Array.isArray(value);}function probe(){const array=[1];const alias=array;console.log(classify(array));console.log(array===alias);return classify([]);}console.log(probe());`,expected:'true\ntrue\ntrue\n'},
  {name:'array argument effects remain ordered before and after an empty result',source:`/** @returns {number[]} */function operand(){console.log('operand');return [];}console.log('before');console.log(Array.isArray(operand()));console.log('after');`,expected:'before\noperand\ntrue\nafter\n'},
);
const throwing=[{name:'argument failure cannot disappear behind static classification',source:`/** @returns {number[]} */function fail(){throw new Error('kept array predicate failure');}function probe(){return Array.isArray(fail());}console.log(probe());`,errorName:'Error',message:'kept array predicate failure'}];
const refusals=[
  {name:'zero arguments remain outside the selected exact-one profile',source:'console.log(Array.isArray());',expected:'false\n'},
  {name:'extra arguments retain an independent observable oracle',source:"function second(){console.log('second');return 2;}console.log(Array.isArray([1],second()));",expected:'second\ntrue\n'},
  {name:'an unresolved array and number union keeps both actual results',source:'/** @param {boolean} flag */function value(flag){return flag?[1]:3;}function probe(flag){return Array.isArray(value(flag));}console.log(probe(true));console.log(probe(false));',expected:'true\nfalse\n'},
  {name:'a source proxy cannot be classified using its target type',source:'const value=new Proxy([1],{});console.log(Array.isArray(value));',expected:'true\n'},
  {name:'actual parameter shadows the intrinsic Array owner',source:'function probe(Array){return Array.isArray([1]);}console.log(probe(0));',errorName:'TypeError'},
  {name:'actual module shadows the intrinsic Array owner',source:'const Array=0;function probe(){return Array.isArray([1]);}console.log(probe());',errorName:'TypeError'},
];
const decisionVectors=[
  ...['array','tuple'].flatMap(kind=>[{kind,element:'',expected:[true,false]},{kind:'option',element:kind,expected:[true,true]}]),
  ...['unit','absence','bool','string','float','int','nat','fixed','literal','record','data','set','map','cell','function'].flatMap(kind=>[
    {kind,element:'',expected:[false,false]},{kind:'option',element:kind,expected:[false,false]}]),
  ...['','unknown','var','named','union','proxy','hypothesis','output','option'].flatMap(kind=>[
    {kind,element:'',expected:[]},{kind:'option',element:kind,expected:[]}]),
];
export function arrayClassificationRulesPolicy(){
  const source=readFileSync(new URL(sourcePath,root),'utf8');
  assert.doesNotMatch(source,/^\s*import\b/mu,'the complete policy is a closed source module');
  assert.deepEqual([...source.matchAll(/^export function (\w+)\(/gmu)].map(match=>match[1]),decisions);
  return{schemaVersion:1,model:'actual JavaScript array classification with optional presence 1',sourcePath,source,
    bytes:Buffer.byteLength(source),sourceSha256:digest(source),decisions,cases,throwing,refusals,decisionVectors,
    sourceClosure:{source,bytes:Buffer.byteLength(source),sourceSha256:digest(source),inputs:[{path:sourcePath,source,bytes:Buffer.byteLength(source),sourceSha256:digest(source)}]},
    assertionVersions:{Node:'>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0'},
    admission:'The actual unshadowed Array owner and exactly one source argument are required. Array and represented tuple values classify true. A flat optional array or tuple uses the existing checked Truthy operation on a single bound value, so present empty arrays are true and both nullish absence cases are false. Closed represented non-array types classify false. Unknown, unresolved named, union, proxy and nested-option domains refuse.',
    evaluation:'The existing checked Let evaluates the actual argument exactly once even when classification is statically false or true. The optional presence branch reads only that bound variable. Structural walkers retain the original operand for inference, binding, effect and provenance traversal.',
    representation:'Reference-qualified arrays retain the array kind and are classified without cloning their payload. Cell values classify false because the existing JavaScript cell renderer produces an object with a value field; normal source binding reads are unwrapped before classification. Optional presence uses the actual null/undefined policy carried by the checked value.',
    boundary:'This is a closed checked-value profile. It admits neither arbitrary external Proxies nor revocation/inherited-property tricks. Actual source owner capture, normal shared generation and byte-identical archive replays establish no native execution until CI runs.'};
}
export async function generateArrayClassificationRules({check=false,native=false}={}){
  const policy=arrayClassificationRulesPolicy();
  for(const path of ['js/src/translation/data/array-classification-rules.lino','rust/src/data/array-classification-rules.lino']){
    if(check)checkProjectData(new URL(path,root),policy);else writeProjectData(new URL(path,root),policy);
  }
  let artifact=null;
  if(native){
    const {parseJavaScript}=await import('../src/translation/javascript.js');
    const {checkProgram}=await import('../src/translation/check.js');
    const {emitRust}=await import('../src/translation/emit-rust.js');
    const {encodeTypedProgramLinks,decodeTypedProgramLinks}=await import('../src/typed-program-links.js');
    const {recheckLinkedTypedProgram}=await import('../src/typed-program-checked-projection.js');
    const {LinkNetwork}=await import('../src/network.js');
    const {encodeLinkNetworkBinary,decodeLinkNetworkBinary}=await import('../src/link-network-binary.js');
    const surface=parseJavaScript(policy.source,{referenceSemantics:false});
    const inventory=surface.items.map((item,ordinal)=>({ordinal,name:item.name,kind:item.k,generated:item.generated===true}));
    assert.deepEqual(inventory.filter(row=>!row.generated).map(row=>row.name),decisions);
    const owners=program=>inventory.map(row=>{const declaration=program.declarations.get(row.name);assert.ok(declaration&&declaration.k===row.kind);return{...row,fullName:declaration.fullName};});
    const checked=checkProgram(surface),checkedNamedSourceInventory=owners(checked);assert.equal(checkedNamedSourceInventory.length,checked.declarations.size);
    const emitted=emitRust(checked,{arrayConsumption:true}),packet=encodeTypedProgramLinks(checked),replays=[];
    const programText=packet.network.toLino(),programPaths=['js/src/data/array-classification-program.lino','rust/src/data/array-classification-program.lino'];
    for(const path of programPaths){if(check)assert.equal(readFileSync(new URL(path,root),'utf8'),programText);else writeFileSync(new URL(path,root),programText);}
    for(const [format,network]of [['text',LinkNetwork.fromLino(packet.network.toLino())],['binary',decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]]){
      const restored=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});assert.deepEqual(owners(restored),checkedNamedSourceInventory);
      const replay=emitRust(restored,{arrayConsumption:true});for(const field of ['text','definitions','preludes','mappings','assumptions','encodings'])assert.deepEqual(replay[field],emitted[field]);
      replays.push({format,checkedNamedSourceInventory:owners(restored),emittedTextSha256:digest(replay.text)});
    }
    for(const owner of checkedNamedSourceInventory)assert.equal(emitted.mappings.filter(row=>row.source===owner.fullName&&row.kind==='function').length,1);
    const files=writeRecordRuleLibrary(emitted,'array_classification_rules',{check}),libraryInterface=[];
    for(const file of files){const tokens=tokenize(file.source,'Rust').tokens;
      for(let at=0;at<tokens.length;at+=1){if(tokens[at].value!=='fn'||tokens[at+1]?.kind!=='identifier')continue;
        const target=tokens[at+1].value,mapping=emitted.mappings.find(row=>row.target===target&&row.kind==='function');if(!mapping)continue;
        let body=at+2;while(body<tokens.length&&tokens[body].value!=='{')body+=1;assert.ok(body<tokens.length);
        libraryInterface.push({source:mapping.source,target,path:file.path??'array_classification_rules.rs',signature:file.source.slice(tokens[at].start,tokens[body].start).trim()});
      }}
    for(const name of decisions)assert.equal(libraryInterface.filter(row=>row.source===name).length,1);
    artifact={schemaVersion:1,model:policy.model,sourceClosure:policy.sourceClosure,inventory,checkedNamedSourceInventory,replays,libraryInterface,
      archive:{root:packet.root.asU64(),paths:programPaths,bytes:Buffer.byteLength(programText),sha256:digest(programText)},
      emitted:{text:emitted.text,definitions:emitted.definitions,preludes:emitted.preludes,mappings:emitted.mappings,assumptions:emitted.assumptions,encodings:emitted.encodings},
      files:files.map(file=>({path:file.path??'array_classification_rules.rs',source:file.source,bytes:Buffer.byteLength(file.source),sourceSha256:digest(file.source)})),
      boundary:'Generated complete policy closure and actual ABI inventory only; native host assembly must consume this exact ABI, and CI supplies native compilation/execution.'};
    const output=new URL('parity/self-translation/array-classification-rules-native.lino',root);if(check)checkProjectData(output,artifact);else writeProjectData(output,artifact);
  }
  return{policy,artifact};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2);assert.ok(new Set(args).size===args.length&&args.every(value=>['--check','--native'].includes(value)));
  await generateArrayClassificationRules({check:args.includes('--check'),native:args.includes('--native')});
}
