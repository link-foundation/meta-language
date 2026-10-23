import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {createHash} from 'node:crypto';
import {readProjectData} from '../scripts/project-data.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';
import {readNativeCompilerSourceStyles} from '../scripts/native-compiler-source-styles.mjs';
import {readSourceTextRustBlock} from '../scripts/source-text-byte-library-style.mjs';
import {tokenize} from '../src/translation/lexer.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');
const fixture=readProjectData(new URL('../../parity/fixtures/ci-39890308543-native-source-styles.lino',import.meta.url));
const sourceOf=family=>fixture.sources.find(row=>row.family===family).files.map(row=>row.source).join('\n');
const values=source=>tokenize(source,'Rust').tokens.map(token=>[token.kind,token.value]);

test('every complete native feedback source and diagnostic retain their original independent identity',()=>{
 assert.equal(fixture.sources.length,8);assert.equal(fixture.diagnostic.bytes,243316);
 assert.equal(Buffer.byteLength(fixture.diagnostic.source),fixture.diagnostic.bytes);assert.equal(createHash('sha256').update(fixture.diagnostic.source).digest('hex'),fixture.diagnostic.sha256);
 assert.match(fixture.diagnostic.kind,/reviewed separately from machine-applicable/u);
 for(const row of fixture.sources)for(const item of row.files){
  const bytes=Buffer.from(item.source);assert.equal(bytes.length,item.bytes);assert.equal(createHash('sha256').update(bytes).digest('hex'),item.sha256);
  verifyGitArtifactCertificate(item.certificate,bytes,{revision:fixture.revision,path:item.file});
 }
});

test('all eight reviewed families replay every complete source edit without hidden replacement semantics',()=>{
 for(const {family}of fixture.sources){
  const original=sourceOf(family),styled=readNativeCompilerSourceStyles(original,family);assert.ok(styled.actions.length>0,family);
  let replay=original;for(const action of styled.actions){assert.equal(action.op,'replace');assert.equal(action.field,'source');assert.equal(replay.split(action.from).length,2);replay=replay.replace(action.from,()=>action.to);}
  assert.equal(replay,styled.source);assert.deepEqual(tokenize(styled.source,'Rust').comments.map(row=>row.text),tokenize(original,'Rust').comments.map(row=>row.text),'all original comments remain');
  assert.doesNotMatch(styled.source,/#\[allow\(clippy/u,'reviewed corrections do not add blanket lint suppression');
 }
});

test('branch selection renaming preserves all operations strings comments and lexical references',()=>{
 const name='read_reference_identity_comparison_error';
 const source='pub fn '+name+'(a:bool,b:bool,c:bool,d:bool)->bool{a&&b&&c&&d}\n'+
  'pub fn probe()->bool{let ml_s2=true;let ml_s12=ml_s2;let text="ml_s2";/* ml_s12 */ml_s12}';
 const styled=readNativeCompilerSourceStyles(source,'reference_identity_rules').source;
 const original=readSourceTextRustBlock(source,'fn','probe'),changed=readSourceTextRustBlock(styled,'fn','probe');
 assert.deepEqual(values(changed),values(original).map(([kind,value])=>[kind,kind==='identifier'&&/^ml_s\d+$/u.test(value)?'ml_s'+[...value.slice(4)].map(digit=>digit+digit).join(''):value]));
 assert.ok(changed.includes('"ml_s2"'));assert.ok(changed.includes('/* ml_s12 */'));
 assert.throws(()=>readNativeCompilerSourceStyles(source.replace('"ml_s2"','"{ml_s2}"'),'reference_identity_rules'));
 assert.deepEqual(readNativeCompilerSourceStyles(source,'unregistered family'),{source,actions:[]});
});

test('the rewritten publication guard preserves ordered Boolean meaning including NaN and signed zero',()=>{
 const original=sourceOf('declaration_dependency_rules'),styled=readNativeCompilerSourceStyles(original,'declaration_dependency_rules').source;
 const execute=source=>{
  const block=readSourceTextRustBlock(source,'fn','accept_recursive_declaration_owned_publication');
  const match=/\bif\s+([\s\S]*?)\s*\{\s*false\b/u.exec(block);assert.ok(match);
  const condition=match[1].replace(/\(proofs\.len\(\)\s+as\s+f64\)/gu,'length');
  const rust='fn guard(members:f64,declarations:f64,signatures:f64,completed:f64,length:f64)->bool{'+condition+'}fn main(){}';
  const context={};vm.runInNewContext(emitJavaScript(checkProgram(parseRust(rust))).text,context,{timeout:1000});return context.guard;
 };
 const before=execute(original),after=execute(styled),base=[2,3,2,2,7];
 for(let axis=0;axis<5;axis++)for(const value of[-Infinity,-1,-0,0,1,1.5,2,3,7,Number.MAX_SAFE_INTEGER,Infinity,NaN]){
  const args=base.slice();args[axis]=value;assert.equal(after(...args),before(...args),String(args));
 }
});
