// Regenerate this finite source-derived type layer from its retained Links archive.
// Importing this module writes nothing and does not read current generated code.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {tokenize} from '../../../../js/src/translation/lexer.js';
import {serializeProjectData,readProjectData,writeProjectData,checkProjectData} from '../../../../js/scripts/project-data.mjs';
import {explicitOutputDirectory} from '../wire/output.mjs';

const digest=source=>createHash('sha256').update(source).digest('hex');
const originalModuleSha256='48aa4d8bbde33913d67868c73a73cf25d0c27ef1d61a934c03cf295ae2ace8dc';
const originalTestSha256='8a10917d6d612b733072d4e8f909b8c008807a06d4ba58b0657c73a06f7ee4d2';
const archivePath=new URL('./type-publication.lino',import.meta.url);
const publicationDirectory=new URL('./',import.meta.url);

function unique(source,needle) {
  const index=source.indexOf(needle);assert.ok(index>=0&&source.indexOf(needle,index+1)===-1,'one complete source occurrence: '+needle);return index;
}

/** Restricted literal data reader, not eval/JSON or a replacement JS parser. */
function literalReader(source) {
  const tokens=tokenize(source,'JavaScript').tokens.filter(token=>token.kind!=='eof'),witnesses=[];
  const extract=(selector,offset=selector.length)=>{
    const selected=unique(source,selector)+offset;let index=tokens.findIndex(token=>token.start>=selected);
    assert.ok(index>=0&&source.slice(selected,tokens[index].start).trim()==='','literal starts at its exact selected source token');
    const first=index,value=()=>tokens[index]?.value;
    const take=expected=>{assert.equal(value(),expected,'source literal syntax');index++;};
    const read=()=>{
      const token=tokens[index];assert.ok(token,'complete source literal');
      if(token.kind==='string'){index++;return token.value;}
      if(value()==='true'||value()==='false'){index++;return token.value==='true';}
      if(value()==='null'){index++;return null;}
      if(/^(?:0|[1-9][0-9]*)$/u.test(String(value()))){index++;return Number(token.value);}
      if(value()==='['){take('[');const result=[];while(value()!==']'){result.push(read());if(value()!==',')break;take(',');}take(']');return result;}
      if(value()==='{'){
        take('{');const result={};while(value()!=='}'){
          const key=value();assert.ok(typeof key==='string'&&/^[A-Za-z_][A-Za-z_0-9]*$/u.test(key),'literal data key');index++;take(':');
          assert.ok(!Object.hasOwn(result,key),'duplicate source literal key');Object.defineProperty(result,key,{value:read(),enumerable:true});
          if(value()!==',')break;take(',');
        }take('}');return result;
      }
      throw new TypeError('unsupported source data expression');
    };
    const data=read(),start=tokens[first].start,end=tokens[index-1].end,text=source.slice(start,end);
    witnesses.push({selector,offset,source:text,span:{start:Buffer.byteLength(source.slice(0,start)),end:Buffer.byteLength(source.slice(0,end))},sha256:digest(text),data});
    return data;
  };
  return {extract,witnesses};
}

export function reverseTypePublication(source,edits) {
  for(const {from,to}of [...edits].reverse()){unique(source,to);source=source.replace(to,from);}return source;
}

export function buildTypePublication({portable=true,archive=readProjectData(archivePath)}={}) {
  assert.equal(portable,true,'repository regeneration emits portable sources');
  assert.equal(archive.schemaVersion,1,'retained normal Links authoring archive');
  assert.equal(typeof archive.portable,'boolean','historical publication mode is explicit');
  const {original,originalTests}=archive;
  assert.equal(digest(original),originalModuleSha256,'complete r1-passed authoring module');
  assert.equal(digest(originalTests),originalTestSha256,'all original r1-passed assertions and inputs');
  assert.equal(archive.originalModuleSha256,originalModuleSha256);assert.equal(archive.originalTestSha256,originalTestSha256);
  const importRewrites=structuredClone(archive.importRewrites);
  assert.ok(Array.isArray(importRewrites)&&importRewrites.length===5,'complete historical import rewrite inventory');
  const targets=['../wire/generate.mjs','../../../../js/scripts/generate-private-output-test-model.mjs',
    '../../../../js/src/translation/lexer.js','../../../../js/scripts/project-data.mjs','../../../../js/src/structured-data-model.js'];
  assert.deepEqual(importRewrites.map(pair=>pair[1]),targets,'only the declared portable source-owner imports');
  for(const pair of importRewrites)assert.ok(Array.isArray(pair)&&pair.length===2&&typeof pair[0]==='string'&&pair[0].length>0);
  assert.equal(new Set(importRewrites.map(pair=>pair[0])).size,5,'unique historical import owners');
  const historicalImport=target=>{const found=importRewrites.filter(pair=>pair[1]===target);assert.equal(found.length,1);return found[0][0];};
  const {extract,witnesses}=literalReader(original);
  const scalar=(selector)=>extract(selector,selector.indexOf(':')+1);
  const formsSelector="assert.ok(['named','String'";
  const builtinOwners=extract('const builtinOwners=');
  const admittedForms=extract(formsSelector,formsSelector.indexOf('['));
  const genericSelector='const admitted={Option:1';
  const genericArities=extract(genericSelector,'const admitted='.length);
  const requiredSelector="assert.ok(!['Option','Box','Vec','IndexMap']";
  const requiredArguments=extract(requiredSelector,requiredSelector.indexOf('['));
  assert.deepEqual(requiredArguments,Object.keys(genericArities),'generic arity and presence share exactly one source-derived domain');
  const referenceProperties={ownership:extract('properties={ownership:'),lifetime:scalar("lifetime:'static'"),mutable:scalar('mutable:false')};
  const referenceObligations=extract('        obligations:');
  const referenceTarget=extract('assert.equal(item.kind,');
  const scalarProperties={String:{ownership:extract("if(kind==='String')properties.ownership=")},
    i64:{integer:extract("if(kind==='i64')properties.integer=")},
    usize:{integer:extract("if(kind==='usize'){properties.integer="),
      obligations:extract("if(kind==='usize'){properties.integer={signed:false,bits:null};properties.obligations=")},
    json:{obligations:extract("if(kind==='json')properties.obligations=")}};
  const external=(qualified,form,name)=>({qualified:scalar("qualified:'"+qualified+"'"),kind:scalar("kind:'external',form:'"+form+"'"),form:scalar("form:'"+form+"'"),name:scalar("name:'"+name+"'")});
  // Read kind from its literal at the colon; the full selector qualifies duplicates.
  const externalOwners=[external('indexmap::IndexMap','IndexMap','IndexMap'),external('serde_json::Value','json','Value')];
  const policy={schemaVersion:1,originalModuleSha256,builtinOwners,admittedForms,genericArities,requiredArguments,referenceProperties,referenceObligations,referenceTarget,scalarProperties,externalOwners};
  const policySource=serializeProjectData(policy,'type-policy.lino'),policySha256=digest(policySource);
  let module=original;const edits=[];
  const edit=(from,to)=>{unique(module,from);module=module.replace(from,to);edits.push({from,to,count:1});};
  const extraImports="import {readProjectData,serializeProjectData} from '"+historicalImport('../../../../js/scripts/project-data.mjs')+"';\n"
    +"import {wellFormedString} from '"+historicalImport('../../../../js/src/structured-data-model.js')+"';\n";
  edit("import {createHash} from 'node:crypto';\n","import {createHash} from 'node:crypto';\n"+extraImports);
  const builtinStart=unique(original,'const builtinOwners='),builtinEnd=original.indexOf(';\n',builtinStart)+2;
  assert.ok(builtinEnd>builtinStart);
  edit(original.slice(builtinStart,builtinEnd),"const typePolicy=readProjectData(new URL('./type-policy.lino',import.meta.url));\n"
    +"assert.equal(digest(serializeProjectData(typePolicy,'type-policy.lino')),'"+policySha256+"','source-derived shared type policy');\n"
    +'const builtinOwners=typePolicy.builtinOwners;\n');
  edit("assert.equal(typeof source,'string');","assert.equal(typeof source,'string');assert.ok(wellFormedString(source),'unpaired UTF-16 type source');");
  edit("assert.equal(value(),'static','unsupported borrowed lifetime');take('static');","assert.equal(value(),typePolicy.referenceProperties.lifetime,'unsupported borrowed lifetime');take(typePolicy.referenceProperties.lifetime);");
  edit("assert.equal(item.kind,'str','only the source-owned static str borrow is admitted');","assert.equal(item.kind,typePolicy.referenceTarget,'only the source-owned static str borrow is admitted');");
  edit("properties={ownership:'borrowed',lifetime:'static',mutable:false,","properties={...copy(typePolicy.referenceProperties),");
  edit("obligations:['caller supplied existing static reference or separate owned reconstructed type']","obligations:copy(typePolicy.referenceObligations)");
  edit("['named','String','str','bool','usize','i64','json','Option','Box','Vec','IndexMap'].includes(owner.form)",'typePolicy.admittedForms.includes(owner.form)');
  edit('const admitted={Option:1,Box:1,Vec:1,IndexMap:2};','const admitted=typePolicy.genericArities;');
  edit("['Option','Box','Vec','IndexMap'].includes(owner.form)",'typePolicy.requiredArguments.includes(owner.form)');
  const propertiesStart=unique(original,"        if(kind==='String')properties.ownership="),propertiesEnd=original.indexOf('\n      }',propertiesStart);
  assert.ok(propertiesEnd>propertiesStart);
  edit(original.slice(propertiesStart,propertiesEnd),'        Object.assign(properties,copy(typePolicy.scalarProperties[kind]??{}));');
  const externalStart=unique(original,"  owners.push({identity:ownerIdentity('indexmap::IndexMap')"),externalEnd=original.indexOf(';\n',externalStart)+1;
  assert.ok(externalEnd>externalStart);
  edit(original.slice(externalStart,externalEnd),'  for(const external of typePolicy.externalOwners)owners.push({identity:ownerIdentity(external.qualified),...copy(external)});');
  for(const [from,to]of importRewrites)edit("from '"+from+"'","from '"+to+"'");
  assert.equal(reverseTypePublication(module,edits),original,'exact complete original module inverse, including every untouched source byte');
  const tests=originalTests+unicodeControls;
  assert.equal(tests.slice(0,originalTests.length),originalTests,'every original test declaration/input/assertion remains an exact prefix');
  assert.deepEqual(archive.witnesses,witnesses,'all historical source-literal witnesses remain exact');
  const historicalEdits=archive.portable?edits:edits.slice(0,-importRewrites.length);
  assert.deepEqual(archive.edits,historicalEdits,'retained historical publication edits are exact');
  let historical=original;
  for(const {from,to}of historicalEdits){unique(historical,from);historical=historical.replace(from,to);}
  assert.equal(reverseTypePublication(historical,historicalEdits),original,'historical complete source inverse');
  assert.deepEqual(archive.publishedModule,{bytes:Buffer.byteLength(historical),sha256:digest(historical)},'historical generated module source hash');
  assert.deepEqual(archive.publishedTests,{bytes:Buffer.byteLength(tests),sha256:digest(tests)},'all historical and appended source controls');
  assert.deepEqual(archive.policy,{path:'type-policy.lino',bytes:Buffer.byteLength(policySource),sha256:policySha256},'historical canonical artifact bytes and hash');
  const receipt={schemaVersion:1,portable,original,originalTests,
    originalModuleSha256,originalTestSha256,edits,witnesses,importRewrites,
    publishedModule:{bytes:Buffer.byteLength(module),sha256:digest(module)},publishedTests:{bytes:Buffer.byteLength(tests),sha256:digest(tests)},
    policy:{path:'type-policy.lino',bytes:Buffer.byteLength(policySource),sha256:policySha256},
    boundary:'Source-derived pre-erasure type layer only; no native behavior, inverse implementation or completion claim.'};
  assert.deepEqual(Object.keys(archive),Object.keys(receipt),'complete archival field inventory is retained');
  assert.equal(archive.boundary,receipt.boundary,'historical finite semantic boundary is retained');
  return {module,tests,policy,policySource,receipt};
}

/** Freshness compares regenerated bytes; the runtime guard separately hashes
 * normal serialization of the decoded policy value, not raw input formatting.
 */
export function checkTypePublication(directory=publicationDirectory) {
  const result=buildTypePublication();
  assert.equal(readFileSync(new URL('inventory.mjs',directory),'utf8'),result.module,'complete generated type module freshness');
  assert.equal(readFileSync(new URL('inventory.test.mjs',directory),'utf8'),result.tests,'complete retained and appended test freshness');
  checkProjectData(new URL('type-policy.lino',directory),result.policy);
  checkProjectData(new URL('type-publication.lino',directory),result.receipt);
  return result;
}

const unicodeControls=`

test('Unicode type comments retain exact UTF-8 source spans token spans and hashes',()=>{
  const scope=current().scopes.find(value=>value.qualifiedOwner.endsWith('::SyntaxTree'));
  const source="Vec<& /* 漢字 😀 é */ 'static str>",offset=37;
  const result=readPreErasureType(source,scope,'control:unicode',offset);
  const root=result.nodes.find(value=>value.identity===result.root),bytes=Buffer.from(source);
  assert.equal(root.source,source);assert.deepEqual(root.span,{start:offset,end:offset+bytes.length});assert.equal(root.sha256,digest(source));
  assert.ok(bytes.length>source.length,'non-ASCII fixture actually distinguishes UTF-8 bytes from UTF-16 offsets');
  for(const node of result.nodes){
    assert.equal(bytes.subarray(node.span.start-offset,node.span.end-offset).toString(),node.source);
    assert.equal(node.sha256,digest(node.source));
    for(const token of node.tokens)assert.equal(bytes.subarray(token.byteSpan.start-offset,token.byteSpan.end-offset).toString(),source.slice(token.start,token.end));
  }
  const text=result.nodes.find(value=>value.kind==='str'),at=source.lastIndexOf('str');
  assert.deepEqual(text.span,{start:offset+Buffer.byteLength(source.slice(0,at)),end:offset+bytes.length-1});
});

test('unpaired type source refuses before token interpretation UTF-8 conversion and hashing',()=>{
  const scope=current().scopes.find(value=>value.qualifiedOwner.endsWith('::SyntaxTree'));
  for(const unit of [0xd800,0xdfff])for(const source of ['String /* '+String.fromCharCode(unit)+' */',String.fromCharCode(unit)+'String'])
    assert.throws(()=>readPreErasureType(source,scope),/unpaired UTF-16 type source/u);
});
`;

if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2);
  if(args.length===1&&args[0]==='--check'){checkTypePublication();process.exitCode=0;}
  else {
  const output=explicitOutputDirectory(args);
  const result=buildTypePublication();
  writeProjectData(resolve(output,'type-policy.lino'),result.policy);
  assert.deepEqual(readProjectData(resolve(output,'type-policy.lino')),result.policy,'normal automated policy conversion');
  writeProjectData(resolve(output,'type-publication.lino'),result.receipt);
  writeFileSync(resolve(output,'inventory.mjs'),result.module);
  writeFileSync(resolve(output,'inventory.test.mjs'),result.tests);
  }
}
