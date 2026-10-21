// Author expected inputs/provenance; execution observations belong to tests.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync,readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {parseNative} from '../src/native-grammar-parser.js';
import {tokenize} from '../src/translation/lexer.js';
import {checkProjectData,readProjectData,writeProjectData,serializeProjectData} from './project-data.mjs';

const root=new URL('../../',import.meta.url);
export const MODERN_ARRAY_FIXTURE_FILE='parity/fixtures/modern-javascript-array-parity.lino';
export const MODERN_ARRAY_FIXTURE_ID='modern-javascript-array-parity-programs';
export const MODERN_ARRAY_CONTRACT_FILE='parity/sources/modern-javascript-parity-expectations-2026-10-16.txt';
export const MODERN_ARRAY_CONTRACT_ID='conversation-contract:modernJavaScriptParityRequest';
export const MODERN_ARRAY_CONTRACT_SHA256='e2c4a14e4cff20b8bd4e77c1110151f5add84292836591c5eac2c1dc40c141af';
export const MODERN_ARRAY_APPEND_MARKER='\n// BEGIN modern JavaScript array parity observations\n';
const historicalRevision='e7a716520d5a3af4376845437a469bf96c0c9d93';
const historicalSha256='1eec6a923a4d78d79b85fb5da684670f7b2383727539e19b26a363ff8a0bbee2';
const currentProfiles=[
 ['js/tests/translation-arrays.test.js','9ff1d4811e7482088a90167c5f2f048c41fe97d444405f265bf144f00e24d8af'],
 ['rust/tests/unit/translation_arrays.rs','04e317f3c6fa1ed36a91fb59a3ce480fa95e52f1e6bf60043491089bb302e7a9'],
];
export const MODERN_ARRAY_CASE_IDS=Object.freeze(['original-array-corpus','original-empty-array-inference','original-data-array',
 'original-array-console-refusal','original-array-mutation-refusal','original-array-identity-refusal','original-string-length',
 'original-array-hole-refusal','original-for-in-refusal','supplementary-string-length','observable-array-absence']);
export const MODERN_ARRAY_IN_BOUNDS='the translation agrees with the source on executions whose array reads are in bounds; JavaScript reads undefined at an index outside an array, where the target aborts';
const digest=value=>createHash('sha256').update(value).digest('hex');
const field=(node,name)=>{const found=node.children.filter(child=>child.field===name);assert.equal(found.length,1,`one ${name} field is required`);return found[0].node;};
const namedChildren=node=>node.children.filter(child=>child.node.named&&!child.node.isExtra).map(child=>child.node);
function literal(node,bytes){
 assert.ok(node.term==='string'||node.term==='template_string','an actual complete literal is required');
 const raw=bytes.subarray(node.start,node.end).toString('utf8'),tokens=tokenize(raw,'JavaScript').tokens;
 assert.equal(tokens.length,2);assert.equal(tokens[1].kind,'eof');assert.equal(tokens[0].start,0);assert.equal(tokens[0].end,raw.length);
 const token=tokens[0];if(token.kind==='string')return token.value;
 assert.equal(token.kind,'template');assert.equal(token.parts.length,1);assert.equal(Object.hasOwn(token.parts[0],'expression'),false,'template substitution is not a literal');
 return token.parts[0].text;
}
function visit(node,callback){callback(node);for(const child of node.children)visit(child.node,callback);}
function originalBody(source){const at=source.indexOf(MODERN_ARRAY_APPEND_MARKER);if(at<0)return source;assert.equal(source.split(MODERN_ARRAY_APPEND_MARKER).length,2);return source.slice(0,at);}
// These finite assertion-source inverses preserve the independently pinned
// original test bodies. They never change any extracted program input.
function originalIdentitySourceBody(source,path){
 let body=originalBody(source);
 const recipes=[{"path": "js/tests/translation-arrays.test.js", "before": "  assert.equal(refusal('const xs = [1, 2];\\nconsole.log(xs === xs);\\n'), 'comparison of arrays: === of two arrays compares which array each is, which a value translation does not keep; compare their elements at 31..40');", "after": "  assertModernArrayIdentityTranslation('const xs = [1, 2];\\nconsole.log(xs === xs);\\n', 'comparison of arrays: === of two arrays compares which array each is, which a value translation does not keep; compare their elements at 31..40');"}, {"path": "rust/tests/unit/translation_arrays.rs", "before": "        assert_eq!(refusal(source), message, \"{source}\");", "after": "        if source == \"const xs = [1, 2];\\nconsole.log(xs === xs);\\n\" {\n            assert_modern_array_identity_translation(source, message);\n        } else {\n            assert_eq!(refusal(source), message, \"{source}\");\n        }"}];
 const recipe=recipes.find(row=>row.path===path);
 if(recipe&&body.includes(recipe.after)){assert.equal(body.split(recipe.after).length,2);body=body.replace(recipe.after,()=>recipe.before);}
 return body;
}
const oldIdentityFixturePin={"bytes": 43302, "sha256": "4ac886fa12c5c748f022911a041d96badcf053a16992d7f33e007d1c0ef38de4"};
const identityImplementation={model:'same-domain structural reference identity equality 1',source:'js/src/translation/reference-identity-rules.js',sha256:'31c61031a2b2bc4c856f85687f8195bb60f51edc823344711962966df39aae1e'};
function readOriginalJavaScriptInputs(source){
 const bytes=Buffer.from(source),text=node=>bytes.subarray(node.start,node.end).toString('utf8');
 const tree=parseNative('native-javascript',source);assert.equal(tree.term,'program');assert.equal(tree.hasError,false);assert.equal(tree.end,bytes.length);
 const calls=namedChildren(tree).filter(node=>node.term==='expression_statement').flatMap(namedChildren)
  .filter(node=>node.term==='call_expression'&&field(node,'function').term==='identifier'&&text(field(node,'function'))==='test');
 assert.equal(calls.length,5,'all five original callback bodies remain');
 let arrays;
 for(const node of namedChildren(tree))if(node.term==='lexical_declaration')for(const declaration of namedChildren(node))
  if(declaration.term==='variable_declarator'&&field(declaration,'name').term==='identifier'&&text(field(declaration,'name'))==='ARRAYS')arrays=literal(field(declaration,'value'),bytes);
 assert.equal(typeof arrays,'string');
 const argumentsOf=call=>namedChildren(field(call,'arguments'));
 const readLocalSource=title=>{
  const selected=calls.filter(call=>literal(argumentsOf(call)[0],bytes)===title);assert.equal(selected.length,1);
  const found=[];visit(argumentsOf(selected[0])[1],node=>{if(node.term==='variable_declarator'&&field(node,'name').term==='identifier'&&text(field(node,'name'))==='source')found.push(literal(field(node,'value'),bytes));});
  assert.equal(found.length,1);return found[0];
 };
 const refusalCall=calls.find(call=>literal(argumentsOf(call)[0],bytes)==='the array forms that are not kept are refused with a reason');assert.ok(refusalCall);
 const refusals=[];visit(argumentsOf(refusalCall)[1],node=>{if(node.term==='call_expression'&&field(node,'function').term==='identifier'&&text(field(node,'function'))==='refusal')refusals.push(literal(argumentsOf(node)[0],bytes));});
 const distinct=[...new Set(refusals)];assert.equal(distinct.length,6);
 return [arrays,readLocalSource('inference fixes the element type of an empty array from its later use'),
  readLocalSource('a @typedef field may hold an array, of data too'),...distinct];
}
function readNativeSourceLiteral(source,name){
 const tokens=tokenize(source,'Rust').tokens;
 const positions=tokens.flatMap((token,index)=>token.value==='fn'&&tokens[index+1]?.value===name?[index]:[]);assert.equal(positions.length,1);
 let open=positions[0];while(tokens[open]?.value!=='{')open+=1;
 let depth=1,end=open+1;for(;end<tokens.length&&depth;end++){if(tokens[end].kind!=='punct')continue;if(tokens[end].value==='{')depth++;if(tokens[end].value==='}')depth--;}
 assert.equal(depth,0);const body=tokens.slice(open+1,end-1),found=[];
 for(let at=0;at<body.length;at++)if(body[at].value==='let'&&body[at+1]?.value==='source'&&body[at+2]?.value==='='){
  assert.equal(body[at+3]?.kind,'string');found.push(body[at+3].value);
 }
 assert.equal(found.length,1);return found[0];
}
export function buildModernArrayParityFixture({historicalSource}={}){
 const target=new URL(MODERN_ARRAY_FIXTURE_FILE,root);
 if(historicalSource===undefined){assert.ok(existsSync(target),'first authoring requires explicit Root-supplied historical source');historicalSource=readProjectData(target).historicalNative.source;}
 assert.equal(digest(historicalSource),historicalSha256,'the complete Root-authenticated historical source stays immutable');
 assert.equal(Buffer.byteLength(historicalSource),5228);
 const contract=readFileSync(new URL(MODERN_ARRAY_CONTRACT_FILE,root));assert.equal(digest(contract),MODERN_ARRAY_CONTRACT_SHA256);
 const bases=currentProfiles.map(([path,sha256])=>{const source=originalIdentitySourceBody(readFileSync(new URL(path,root),'utf8'),path);assert.equal(digest(source),sha256,'every original test body remains byte exact');return {path,source,sha256,bytes:Buffer.byteLength(source)};});
 const originals=readOriginalJavaScriptInputs(bases[0].source);assert.equal(originals.length,9);assert.equal(new Set(originals).size,9);
 const oldLiterals=tokenize(historicalSource,'Rust').tokens.filter(token=>token.kind==='string').map(token=>token.value);
 for(const source of originals)assert.ok(oldLiterals.includes(source),'the complete original input also occurs in the historical native test');
 assert.ok(historicalSource.includes('assert!(contract.assumptions.contains(&IN_BOUNDS)'));
 const oldStringDiagnostic='length of a string: String.prototype.length counts UTF-16 code units, which the portable string types do not keep at 28..36';
 assert.ok(oldLiterals.includes(oldStringDiagnostic));
 const sources=[...originals,readNativeSourceLiteral(bases[1].source,'utf16_string_length_retains_supplementary_characters_in_each_target'),
  readNativeSourceLiteral(bases[1].source,'out_of_bounds_array_reads_retain_absence_instead_of_an_in_bounds_assumption')];
 const expected=[
  {outcome:'admitted',stdout:'4\n3n\n'},{outcome:'admitted',stdout:'3\n'},{outcome:'admitted',stdout:'2\n'},
  {outcome:'refused',match:'exact',diagnostic:'text of an array: console.log lays an array out with util.inspect and String joins its elements with commas, which the translation does not reproduce yet; print the elements one by one at 31..33'},
  {outcome:'refused',match:'exact',diagnostic:'reference mutation: this mutation requires its shared reference and slot operations to be represented at 19..28'},
  {outcome:'refused',match:'exact',diagnostic:'comparison of arrays: === of two arrays compares which array each is, which a value translation does not keep; compare their elements at 31..40'},
  {outcome:'admitted',stdout:'2\n'},
  {outcome:'refused',match:'prefix',diagnostic:'array hole'},
  {outcome:'refused',match:'prefix',diagnostic:'for…in loop'},
  {outcome:'admitted',stdout:'4\n'},{outcome:'admitted',stdout:'9\n'},
 ];
 const fixture={schemaVersion:1,model:'retained modern JavaScript array parity 1',contract:{fixtureId:MODERN_ARRAY_CONTRACT_ID,path:MODERN_ARRAY_CONTRACT_FILE,sha256:MODERN_ARRAY_CONTRACT_SHA256},
  historicalNative:{path:'rust/tests/unit/translation_arrays.rs',revision:historicalRevision,source:historicalSource,bytes:5228,sha256:historicalSha256},
  sourceProfiles:bases.map(({path,sha256,bytes})=>({path,sha256,bytes})),
  correction:{revision:'93c65ca1ada6c5001a56acc4430500e55062027a',previous:{inBoundsAssumptionRequired:true,stringLengthDiagnostic:oldStringDiagnostic},current:{inBoundsAssumptionRequired:false,originalStringLengthAdmitted:true},
   basis:'Correction of project expectations, not a dependency-version change. UTF-16 length and absent numeric properties retain stable language semantics.'},
  cases:sources.map((source,index)=>({id:MODERN_ARRAY_CASE_IDS[index],origin:index<9?'original':'added',source,sourceSha256:digest(source),
   targets:expected[index].outcome==='refused'||index===1?['Rust']:['Rust','Lean','Rocq'],expected:expected[index]}))};
 const before=serializeProjectData(fixture,MODERN_ARRAY_FIXTURE_FILE);
 assert.deepEqual({bytes:Buffer.byteLength(before),sha256:digest(before)},oldIdentityFixturePin,'the complete original fixture including every old row/profile/alias remains exact');
 const previousRow=fixture.cases[5];assert.equal(previousRow.id,'original-array-identity-refusal');assert.deepEqual(previousRow.targets,['Rust']);
 fixture.cases[5]={...previousRow,expected:{outcome:'admitted',stdout:'true\n'}};
 fixture.identityCorrection={model:'retained array identity expectation correction 1',implementation:identityImplementation,
  versions:{npm:'[1.0.0,1.0.1)',crate:'[1.0.0,1.0.1)',Node:'>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0'},
  previous:{fixture:oldIdentityFixturePin,row:previousRow,admitted:6,refused:5},current:{admitted:7,refused:4},
  sourceEpochs:currentProfiles.map(([path])=>{const source=readFileSync(new URL(path,root));return{path,bytes:source.length,sha256:digest(source)};}),
  basis:'The identical original array owner compares true in JavaScript. The source-qualified shared reference policy preserves that identity in Rust; explicit referenceSemantics:false still retains the original value-mode refusal. Case IDs, order, target sets, historicalNative and original sourceProfiles remain unchanged.'};
 const inverse={...fixture,cases:fixture.cases.map((row,index)=>index===5?fixture.identityCorrection.previous.row:row)};delete inverse.identityCorrection;
 assert.equal(serializeProjectData(inverse,MODERN_ARRAY_FIXTURE_FILE),before,'the entire old fixture reconstructs byte exactly');
 return fixture;
}
export function generateModernArrayParityFixtures({check=false,historicalSourcePath}={}){
 const fixture=buildModernArrayParityFixture({historicalSource:historicalSourcePath===undefined?undefined:readFileSync(historicalSourcePath,'utf8')});
 if(check)checkProjectData(new URL(MODERN_ARRAY_FIXTURE_FILE,root),fixture);else writeProjectData(new URL(MODERN_ARRAY_FIXTURE_FILE,root),fixture);
 return fixture;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const args=process.argv.slice(2);let check=false,historicalSourcePath;
 for(let index=0;index<args.length;index++){
  if(args[index]==='--check'){assert.equal(check,false);check=true;}
  else if(args[index]==='--historical-source'){assert.equal(historicalSourcePath,undefined);historicalSourcePath=args[++index];assert.equal(typeof historicalSourcePath,'string');}
  else throw new Error('only --check and --historical-source PATH are accepted');
 }
 generateModernArrayParityFixtures({check,historicalSourcePath});
}
