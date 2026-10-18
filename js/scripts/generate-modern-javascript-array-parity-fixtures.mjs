// Author expected inputs/provenance; execution observations belong to tests.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync,readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import ts from 'typescript';
import {tokenize} from '../src/translation/lexer.js';
import {checkProjectData,readProjectData,writeProjectData} from './project-data.mjs';

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
const literal=node=>{assert.ok(ts.isStringLiteral(node)||ts.isNoSubstitutionTemplateLiteral(node),'an actual complete literal is required');return node.text;};
function visit(node,callback){callback(node);ts.forEachChild(node,child=>visit(child,callback));}
function originalBody(source){const at=source.indexOf(MODERN_ARRAY_APPEND_MARKER);if(at<0)return source;assert.equal(source.split(MODERN_ARRAY_APPEND_MARKER).length,2);return source.slice(0,at);}
function readOriginalJavaScriptInputs(source){
 const tree=ts.createSourceFile('translation-arrays.test.js',source,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
 assert.equal(tree.parseDiagnostics.length,0);
 const calls=tree.statements.filter(node=>ts.isExpressionStatement(node)&&ts.isCallExpression(node.expression)
  &&ts.isIdentifier(node.expression.expression)&&node.expression.expression.text==='test').map(node=>node.expression);
 assert.equal(calls.length,5,'all five original callback bodies remain');
 let arrays;
 for(const node of tree.statements)if(ts.isVariableStatement(node))for(const declaration of node.declarationList.declarations)
  if(ts.isIdentifier(declaration.name)&&declaration.name.text==='ARRAYS')arrays=literal(declaration.initializer);
 assert.equal(typeof arrays,'string');
 const readLocalSource=title=>{
  const selected=calls.filter(call=>literal(call.arguments[0])===title);assert.equal(selected.length,1);
  const found=[];visit(selected[0].arguments[1],node=>{if(ts.isVariableDeclaration(node)&&ts.isIdentifier(node.name)&&node.name.text==='source')found.push(literal(node.initializer));});
  assert.equal(found.length,1);return found[0];
 };
 const refusalCall=calls.find(call=>literal(call.arguments[0])==='the array forms that are not kept are refused with a reason');assert.ok(refusalCall);
 const refusals=[];visit(refusalCall.arguments[1],node=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==='refusal')refusals.push(literal(node.arguments[0]));});
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
 const bases=currentProfiles.map(([path,sha256])=>{const source=originalBody(readFileSync(new URL(path,root),'utf8'));assert.equal(digest(source),sha256,'every original test body remains byte exact');return {path,source,sha256,bytes:Buffer.byteLength(source)};});
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
 return {schemaVersion:1,model:'retained modern JavaScript array parity 1',contract:{fixtureId:MODERN_ARRAY_CONTRACT_ID,path:MODERN_ARRAY_CONTRACT_FILE,sha256:MODERN_ARRAY_CONTRACT_SHA256},
  historicalNative:{path:'rust/tests/unit/translation_arrays.rs',revision:historicalRevision,source:historicalSource,bytes:5228,sha256:historicalSha256},
  sourceProfiles:bases.map(({path,sha256,bytes})=>({path,sha256,bytes})),
  correction:{revision:'93c65ca1ada6c5001a56acc4430500e55062027a',previous:{inBoundsAssumptionRequired:true,stringLengthDiagnostic:oldStringDiagnostic},current:{inBoundsAssumptionRequired:false,originalStringLengthAdmitted:true},
   basis:'Correction of project expectations, not a dependency-version change. UTF-16 length and absent numeric properties retain stable language semantics.'},
  cases:sources.map((source,index)=>({id:MODERN_ARRAY_CASE_IDS[index],origin:index<9?'original':'added',source,sourceSha256:digest(source),
   targets:expected[index].outcome==='refused'||index===1?['Rust']:['Rust','Lean','Rocq'],expected:expected[index]}))};
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
