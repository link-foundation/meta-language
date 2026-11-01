// Source authoring only. Never imports/evaluates fixtures or policy functions.
// The caller owns atomic publication; this module returns reviewable products.
import {readFileSync,realpathSync,statSync} from 'node:fs';
import {resolve,relative,sep} from 'node:path';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {tokenize} from '../src/translation/lexer.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {structuredDataToLino} from '../src/structured-data-links.js';
import {readSourceLibraryClosure,readSourceLibraryDeclarationOwnership} from './read-source-library-closure.mjs';
import {readPortableLibraryRelocation} from './record-rule-library.mjs';

const project=realpathSync(fileURLToPath(new URL('../../',import.meta.url)));
const digest=text=>createHash('sha256').update(text).digest('hex');
const identifier=value=>typeof value==='string'&&/^[A-Za-z_$][A-Za-z_0-9$]*$/u.test(value);
const limit=(value,bound,message)=>{if(value>bound)throw new RangeError(message);};
function projectFile(path){
 if(typeof path!=='string'||path.length>1024||path.includes('\\')||path.includes('\0')||path.startsWith('/')||path.split('/').some(part=>!part||part==='.'||part==='..'))throw new TypeError('policy control inputs need normalized project-relative paths');
 const absolute=realpathSync(resolve(project,path)),actual=relative(project,absolute).split(sep).join('/');
 if(actual==='..'||actual.startsWith('../')||!statSync(absolute).isFile())throw new TypeError('policy control inputs must be actual project files');
 return {absolute,path:actual};
}
function unwrap(value){while(value.kind==='reference')value=value.target;return value;}
function requirePlainData(value){let visits=0;const active=new Set();function walk(item,depth){limit(++visits,131072,'native assertion input metadata exceeds its budget');limit(depth,64,'native assertion input metadata exceeds its depth');if(item===null||['string','boolean'].includes(typeof item))return;if(typeof item==='number'&&Number.isFinite(item))return;if(typeof item!=='object'||active.has(item)||![Object.prototype,Array.prototype,null].includes(Object.getPrototypeOf(item)))throw new TypeError('native assertion metadata must be acyclic plain serializable data');active.add(item);for(const key of Reflect.ownKeys(item)){if(typeof key!=='string')throw new TypeError('native assertion metadata cannot contain symbols');const descriptor=Object.getOwnPropertyDescriptor(item,key);if(!Object.hasOwn(descriptor,'value'))throw new TypeError('native assertion metadata cannot contain accessors');if(Array.isArray(item)&&key==='length')continue;walk(descriptor.value,depth+1);}active.delete(item);}walk(value,0);}
function boundedReceipt(value){let bytes=0,visits=0;const pending=[value];while(pending.length){limit(++visits,262144,'native assertion receipt exceeds its node budget');const item=pending.pop();if(typeof item==='string')bytes+=Buffer.byteLength(item);else if(item&&typeof item==='object')for(const [key,child]of Object.entries(item)){bytes+=Buffer.byteLength(key);pending.push(child);}limit(bytes,16*1024*1024,'native assertion receipt source metadata exceeds 16 MiB');}const text=structuredDataToLino(value);limit(Buffer.byteLength(text),32*1024*1024,'native assertion Links receipt exceeds 32 MiB');return text;}

/** Read a selected literal export, primitive/array/record const references and
 * direct literal collection .push appends. Other expressions never execute.
 * Original lexical text remains distinct from substituted assertion text. */
export function readNativePolicyLiteralControls(source,{path,exportName}){
 if(typeof source!=='string'||!identifier(exportName)||typeof path!=='string')throw new TypeError('a complete literal fixture export is required');
 limit(Buffer.byteLength(source),4*1024*1024,'literal policy fixture exceeds 4 MiB');
 const tokens=tokenize(source,'JavaScript').tokens;limit(tokens.length,262144,'literal policy fixture exceeds its token budget');
 const declarations=new Map(),mutations=[];let depth=0;
 const actual=(at,value)=>tokens[at]?.value===value&&tokens[at]?.raw===value;
 function endStatement(start){let nesting=0;for(let at=start;at<tokens.length;at+=1){const token=tokens[at];if(nesting===0&&actual(at,';'))return at;if(token.kind==='punct'&&['(','[','{'].includes(token.value))nesting+=1;else if(token.kind==='punct'&&[')',']','}'].includes(token.value))nesting-=1;if(nesting<0)throw new TypeError('literal fixture needs complete top-level statements');}return tokens.length-1;}
 for(let at=0;at<tokens.length;at+=1){
  const token=tokens[at];
  if(depth===0&&actual(at,'const')){
   if(tokens[at+1]?.kind!=='identifier'||!actual(at+2,'='))throw new TypeError('literal fixture const declarations require one actual initializer');
   const name=tokens[at+1].value,end=endStatement(at+3);if(declarations.has(name))throw new TypeError('literal fixture has duplicate const '+name);
   declarations.set(name,{name,start:at+3,end,exported:actual(at-1,'export')});limit(declarations.size,4096,'literal fixture exceeds 4096 const declarations');at=end;continue;
  }
  if(depth===0&&token.kind==='identifier'&&(actual(at+1,'.')||actual(at+1,'=')||actual(at+1,'['))){const end=endStatement(at);mutations.push({name:token.value,start:at,end,standalone:at===0||[';','}'].includes(tokens[at-1]?.value)});at=end;continue;}
  if(token.kind==='punct'&&['(','[','{'].includes(token.value))depth+=1;else if(token.kind==='punct'&&[')',']','}'].includes(token.value))depth-=1;
 }
 const selected=declarations.get(exportName);if(!selected?.exported)throw new TypeError('literal fixture has no actual exported const '+exportName);
 const cache=new Map(),active=new Set();let nodes=0;
 function parse(start,end,level=0){
  let at=start;
  const take=value=>actual(at,value)?(at+=1,true):false;
  const need=value=>{if(!take(value))throw new TypeError('literal fixture requires '+value+' at '+(tokens[at]?.start??source.length));};
  function value(nesting){
   limit(++nodes,131072,'literal fixture exceeds its value budget');limit(nesting,64,'literal fixture exceeds its value depth');
   const first=at,token=tokens[at];if(!token||at>=end)throw new TypeError('literal fixture requires an actual complete value');
   let result;
   if(take('[')){const items=[];while(!take(']')){if(actual(at,','))throw new TypeError('sparse literal policy arrays are unsupported');items.push(value(nesting+1));if(take(']'))break;need(',');}result={kind:'array',items};}
   else if(take('{')){const entries=[];while(!take('}')){const key=tokens[at++];if(!key||!['identifier','string'].includes(key.kind)||key.value==='__proto__'||entries.some(entry=>entry.key===key.value))throw new TypeError('literal policy records need unique ordinary data keys');need(':');entries.push({key:key.value,value:value(nesting+1)});if(take('}'))break;need(',');}result={kind:'record',entries};}
   else if(token.kind==='string'&&["'",'"'].includes(token.raw[0])){at+=1;result={kind:'string',text:token.raw,value:token.value};}
   else if(token.kind==='number'||actual(at,'-')){let text='';if(take('-'))text='-';const number=tokens[at++];if(!number||number.kind!=='number'&&!['Infinity'].includes(number.raw)||number.raw.endsWith('n'))throw new TypeError('literal policy controls need actual Number literals');text+=number.raw;if(number.kind==='number'&&!Number.isFinite(Number(number.raw.replaceAll('_',''))))throw new TypeError('overflowed Number literals need an explicit Infinity declaration');result={kind:'number',text};}
   else if(token.kind==='identifier'&&declarations.has(token.value)){at+=1;const declaration=declarations.get(token.value);if(tokens[declaration.start].start>=token.start)throw new TypeError('literal const references need an earlier initialized declaration');result={kind:'reference',name:token.value,target:binding(token.value,nesting+1),definition:{start:tokens[declaration.start].start,end:tokens[declaration.end-1].end,coordinateUnit:'UTF-16 code units'}};}
   else if(['true','false','null','NaN','Infinity'].some(word=>actual(at,word))){at+=1;result={kind:['true','false'].includes(token.raw)?'boolean':token.raw==='null'?'null':'number',text:token.raw};}
   else throw new TypeError('dynamic or unsupported literal policy value at '+token.start);
   result.originalSource=source.slice(tokens[first].start,tokens[at-1].end);return result;
  }
  const result=value(level);return {value:result,next:at};
 }
 function binding(name,level=0){if(cache.has(name))return cache.get(name);if(active.has(name))throw new TypeError('literal const references must be acyclic');active.add(name);try{const declaration=declarations.get(name),parsed=parse(declaration.start,declaration.end,level);if(parsed.next!==declaration.end)throw new TypeError('literal const initializer contains an unsupported expression');cache.set(name,parsed.value);return parsed.value;}finally{active.delete(name);}}
 const collection=unwrap(binding(exportName));if(collection.kind!=='array')throw new TypeError('policy decision export must be an actual literal array');
 const appends=new Map();let discovering=true,discoverySteps=0;
 while(discovering){discovering=false;for(const mutation of mutations){
  limit(++discoverySteps,131072,'literal append dependency discovery exceeds its budget');
  if(!cache.has(mutation.name)||appends.has(mutation))continue;
  if(!mutation.standalone)throw new TypeError('literal policy appends require unconditional top-level statements');
  if(tokens[declarations.get(mutation.name).start].start>=tokens[mutation.start].start)throw new TypeError('literal appends need an earlier initialized collection');
  let at=mutation.start+1;
  if(!actual(at++,'.')||!actual(at++,'push')||!actual(at++,'(')||!actual(mutation.end-1,')'))throw new TypeError('literal policy collections support only direct literal push appends');
  const target=unwrap(cache.get(mutation.name));if(target.kind!=='array')throw new TypeError('literal push needs an actual array const');const values=[];
  while(at<mutation.end-1){const spread=actual(at,'...');if(spread)at+=1;const parsed=parse(at,mutation.end-1);at=parsed.next;if(spread){const items=unwrap(parsed.value);if(items.kind!=='array')throw new TypeError('literal push spread needs an actual literal array');values.push({spread:true,value:items});}else values.push({spread:false,value:parsed.value});if(at<mutation.end-1){if(!actual(at,','))throw new TypeError('literal push arguments must remain separate complete literals');at+=1;}}
  appends.set(mutation,{target,values});discovering=true;
 }}
 const allowed=new Uint8Array(tokens.length),pushStarts=new Set();for(const name of cache.keys()){const declaration=declarations.get(name);allowed.fill(1,declaration.start,declaration.end);allowed[declaration.start-2]=1;}for(const mutation of appends.keys()){pushStarts.add(mutation.start);allowed.fill(1,mutation.start,mutation.end+1);}
 for(let at=0;at<tokens.length;at+=1){const token=tokens[at];if(token.kind!=='identifier'||!cache.has(token.value))continue;if(actual(at+1,'.')&&actual(at+2,'push')&&!pushStarts.has(at))throw new TypeError('nested or dynamic literal policy push must not be omitted');if(['array','record'].includes(unwrap(cache.get(token.value)).kind)&&allowed[at]!==1)throw new TypeError('literal policy collection escapes its captured declarations or literal appends');}
 for(const mutation of mutations){const append=appends.get(mutation);if(!append)continue;for(const item of append.values){const values=item.spread?item.value.items:[item.value];if(item.value===append.target)throw new TypeError('literal append must not create self-referential control data');limit(append.target.items.length+values.length,4096,'literal policy collection exceeds 4096 controls');append.target.items.push(...values);}}
 let renderedNodes=0,units=0;
 function render(value,level=0){limit(++renderedNodes,131072,'resolved literal policy values exceed their expansion budget');limit(level,64,'resolved literal policy values exceed their depth');let text;if(value.kind==='reference')text=render(value.target,level+1);else if(value.kind==='array')text='['+value.items.map(item=>render(item,level+1)).join(',')+']';else if(value.kind==='record')text='{'+value.entries.map(entry=>JSON.stringify(entry.key)+':'+render(entry.value,level+1)).join(',')+'}';else text=value.text;units+=text.length;limit(units,8*1024*1024,'resolved literal policy values exceed 8 MiB');return text;}
 const checked=new Set(),checking=new Set();let graphSteps=0;
 function checkGraph(value,level=0){limit(++graphSteps,131072,'literal reference graph exceeds its budget');limit(level,64,'literal reference graph exceeds its depth');if(checking.has(value))throw new TypeError('literal appends must preserve an acyclic value graph');if(checked.has(value))return;checking.add(value);if(value.kind==='reference')checkGraph(value.target,level+1);else if(value.kind==='array')value.items.forEach(item=>checkGraph(item,level+1));else if(value.kind==='record')value.entries.forEach(entry=>checkGraph(entry.value,level+1));checking.delete(value);checked.add(value);}
 checkGraph(collection);
 function receipt(value){const refs=[],seen=new Set(),objects=new Set();function walk(item){if(objects.has(item))return;objects.add(item);if(item.kind==='reference'){if(!seen.has(item.name)){seen.add(item.name);refs.push({name:item.name,definition:item.definition,source:item.target.originalSource});}walk(item.target);}else if(item.kind==='array')item.items.forEach(walk);else if(item.kind==='record')item.entries.forEach(entry=>walk(entry.value));}walk(value);return {kind:unwrap(value).kind,source:render(value),originalSource:value.originalSource,constantReferences:refs};}
 limit(collection.items.length,4096,'literal policy collection exceeds 4096 controls');
 const controls=collection.items.map((item,index)=>{
  const row=unwrap(item);if(row.kind!=='record')throw new TypeError('policy decision rows need actual literal records');const fields=new Map(row.entries.map(entry=>[entry.key,entry.value]));
  const operation=unwrap(fields.get('operation')??{kind:''});if(operation.kind!=='string'||!identifier(operation.value))throw new TypeError('policy decision needs its actual exported operation');
  if(fields.has('args')&&fields.has('arguments'))throw new TypeError('policy decision cannot declare two argument collections');
  const args=unwrap(fields.get('arguments')??fields.get('args')??{kind:''});if(args.kind!=='array'||!fields.has('expected'))throw new TypeError('policy decision needs literal arguments and expected');
  return {fixture:path,exportName,index,operation:operation.value,argumentField:fields.has('arguments')?'arguments':'args',args:args.items.map(receipt),expected:receipt(fields.get('expected')),authoredRow:receipt(item)};
 });
 return {path,exportName,source,bytes:Buffer.byteLength(source),sourceSha256:digest(source),controls};
}

function compose({sourcePath,closure,fixtures,operationOwners=[],assignment=null,moduleName=''}){
 if(!Array.isArray(fixtures)||fixtures.length===0||fixtures.length>128||!Array.isArray(operationOwners)||operationOwners.length>4096)throw new TypeError('native policy controls need bounded actual fixture selections');
 requirePlainData(fixtures);requirePlainData(operationOwners);
 for(const fixture of fixtures){limit(Buffer.byteLength(fixture.source),4*1024*1024,'literal policy fixture exceeds 4 MiB');if(fixture.sourceSha256!==digest(fixture.source))throw new TypeError('literal fixture source hash is not exact');const actual=readNativePolicyLiteralControls(fixture.source,{path:fixture.path,exportName:fixture.exportName});if(JSON.stringify(actual.controls)!==JSON.stringify(fixture.controls))throw new TypeError('policy control rows must match their complete original literal source');}
 const root=closure.inputs.find(input=>input.path===sourcePath);if(!root)throw new TypeError('native controls need their actual complete source root');
 const overrides=new Map();for(const owner of operationOwners){if(!identifier(owner.operation)||overrides.has(owner.operation))throw new TypeError('policy operations need unique explicit owners');overrides.set(owner.operation,owner.sourcePath);}
 const controls=fixtures.flatMap(fixture=>fixture.controls).filter(row=>!assignment||assignment.get(row.fixture+'\0'+row.exportName+'\0'+row.index)?.moduleName===moduleName).map(row=>({...row}));if(controls.length===0)throw new TypeError('native policy assertion programs require at least one control');limit(controls.length,4096,'native policy assertion program exceeds 4096 controls');
 const selected=new Set(),assertions=[];let assertionBytes=Buffer.byteLength(closure.source)+64;limit(assertionBytes,8*1024*1024,'complete native assertion source exceeds 8 MiB');
 for(const row of controls){
  const identity=row.fixture+'\0'+row.exportName+'\0'+row.index;if(selected.has(identity))throw new TypeError('duplicate native policy fixture cell');selected.add(identity);
  const owner=closure.inputs.find(input=>input.path===(overrides.get(row.operation)??sourcePath));if(!owner?.exports.includes(row.operation))throw new TypeError('unknown policy operation or missing actual exported owner: '+row.operation);
  row.owner={path:owner.path,sourceSha256:owner.sourceSha256};row.compiledOperation=owner.declarationAliases?.find(alias=>alias.original===row.operation)?.compiled??row.operation;
  const assertion='assert.deepEqual('+row.compiledOperation+'('+row.args.map(value=>value.source).join(',')+'),'+row.expected.source+');';assertionBytes+=Buffer.byteLength(assertion)+1;limit(assertionBytes,8*1024*1024,'complete native assertion source exceeds 8 MiB');assertions.push(assertion);
 }
 if(closure.inputs.some(input=>(input.compiledDeclarations??input.declarations).some(name=>['assert','main','NaN','Infinity'].includes(name))))throw new TypeError('complete policy closure collides with its assertion module, generated entry or literal Number globals');
 const source=closure.source+"\n\nimport assert from 'node:assert/strict';\n"+assertions.join('\n')+'\n';limit(Buffer.byteLength(source),8*1024*1024,'complete native assertion source exceeds 8 MiB');
 return {source,controls,bytes:Buffer.byteLength(source),sourceSha256:digest(source)};
}
export function composeNativePolicyAssertionProgram(options){if('assignment' in options)throw new TypeError('filtered native controls require exhaustive batch reconciliation');return compose(options);}

/** Translate once; no test/rule/represented program is executed and no file is
 * written. Root authoring owns atomic registration/publication of the result. */
function translate({sourcePath,closure,fixtures,moduleName,testName,operationOwners=[],assignment=null}){
 const program=compose({sourcePath,closure,fixtures,operationOwners,assignment,moduleName});
 const checked=checkProgram(parseJavaScript(program.source)),sourceDeclarationOwners=readSourceLibraryDeclarationOwnership(closure,{...checked,main:null});
 const emitted=emitRust(checked,{arrayConsumption:true});limit(Buffer.byteLength(emitted.text),8*1024*1024,'native assertion output exceeds 8 MiB');
 const relocation=readPortableLibraryRelocation(emitted.text),tokens=tokenize(relocation.source,'Rust').tokens;if(tokens.some((token,index)=>token.value==='fn'&&tokens[index+1]?.value===testName))throw new TypeError('native assertion wrapper collides with an actual generated function');
 const rust=relocation.source+'\n#[test]\nfn '+testName+'() { main(); }\n',output='rust/tests/generated/'+moduleName+'.rs';
 limit(Buffer.byteLength(rust),8*1024*1024,'native assertion output with test entry exceeds 8 MiB');
 const excludedControls=assignment?fixtures.flatMap(fixture=>fixture.controls).filter(row=>assignment.get(row.fixture+'\0'+row.exportName+'\0'+row.index).moduleName!==moduleName).map(row=>({...row,assignedTo:assignment.get(row.fixture+'\0'+row.exportName+'\0'+row.index)})):[];
 const receipt={schemaVersion:1,model:'complete independently authored native policy assertions 1',sourceRoot:sourcePath,sourceClosure:closure,sourceDeclarationOwners,fixtures,controls:program.controls,excludedControls,assertionProgram:{source:program.source,bytes:program.bytes,sha256:program.sourceSha256},translation:{language:'Rust',source:emitted.text,bytes:Buffer.byteLength(emitted.text),sha256:digest(emitted.text)},relocation,output:{path:output,source:rust,bytes:Buffer.byteLength(rust),sha256:digest(rust)},testName};
 return {source:program.source,rust,output,receipt,receiptLino:boundedReceipt(receipt),registration:'#[path = "../generated/'+moduleName+'.rs"]\nmod '+moduleName+';\n'};
}
function readFixture(selection){const input=projectFile(selection.path);limit(statSync(input.absolute).size,4*1024*1024,'literal policy fixture exceeds 4 MiB');return readNativePolicyLiteralControls(readFileSync(input.absolute,'utf8'),{path:input.path,exportName:selection.exportName});}
function moduleRequest({moduleName,testName='native_policy_literal_assertions',aliases=false}){if(!/^[a-z][a-z_0-9]*$/u.test(moduleName)||!/^[a-z][a-z_0-9]*$/u.test(testName)||typeof aliases!=='boolean')throw new TypeError('invalid native policy assertion module');return {moduleName,testName,aliases};}
export function draftNativePolicyAssertionControls(options){
 const request=moduleRequest(options),{fixtureSelections,operationOwners=[]}=options;if(!Array.isArray(fixtureSelections)||fixtureSelections.length===0||fixtureSelections.length>128||fixtureSelections.some(selection=>'operations' in selection))throw new TypeError('single-root controls require complete bounded unfiltered fixture exports');
 const root=projectFile(options.sourcePath),closure=readSourceLibraryClosure(root.path,{aliases:request.aliases}),fixtures=fixtureSelections.map(readFixture);limit(fixtures.reduce((bytes,fixture)=>bytes+fixture.bytes,0),16*1024*1024,'native policy fixture selection exceeds 16 MiB');
 return translate({...request,sourcePath:root.path,closure,fixtures,operationOwners});
}

/** Explicit multi-owner assignment: all original coordinates occur exactly
 * once. Filtering is available only after complete batch reconciliation. */
export function draftNativePolicyAssertionBatch({groups}){
 if(!Array.isArray(groups)||groups.length===0||groups.length>32)throw new TypeError('native assertion batches require 1..32 actual root groups');
 const fixtures=new Map(),assignment=new Map(),modules=new Set(),prepared=[];let closureBytes=0,fixtureBytes=0;
 for(const group of groups){
  const request=moduleRequest(group);if(modules.has(request.moduleName))throw new TypeError('native assertion modules must have unique registration owners');modules.add(request.moduleName);
  const root=projectFile(group.sourcePath),closure=readSourceLibraryClosure(root.path,{aliases:request.aliases});closureBytes+=closure.bytes;limit(closureBytes,16*1024*1024,'native assertion batch source closures exceed 16 MiB');
  if(!Array.isArray(group.fixtureSelections)||group.fixtureSelections.length===0||group.fixtureSelections.length>128)throw new TypeError('native assertion group needs bounded explicit fixture selections');
  const captures=[];
  for(const selection of group.fixtureSelections){
   if(!Array.isArray(selection.operations)||selection.operations.length===0||selection.operations.length>4096||selection.operations.some(operation=>!identifier(operation))||new Set(selection.operations).size!==selection.operations.length)throw new TypeError('batch policy selections need unique explicit operation names');
   const input=projectFile(selection.path),key=input.path+'\0'+selection.exportName;let fixture=fixtures.get(key);
   if(!fixture){fixture=readFixture(selection);fixtures.set(key,fixture);fixtureBytes+=fixture.bytes;limit(fixtureBytes,16*1024*1024,'native assertion batch fixtures exceed 16 MiB');}
   captures.push(fixture);
   for(const operation of selection.operations)if(!fixture.controls.some(row=>row.operation===operation))throw new TypeError('batch operation selection has no original fixture coordinates: '+operation);
   const owner=closure.inputs.find(input=>input.path===root.path);
   for(const row of fixture.controls)if(selection.operations.includes(row.operation)){
    if(!owner.exports.includes(row.operation))throw new TypeError('batch operation must have its actual selected root export: '+row.operation);
    const coordinate=row.fixture+'\0'+row.exportName+'\0'+row.index;if(assignment.has(coordinate))throw new TypeError('duplicate native policy assertion coordinate');assignment.set(coordinate,{moduleName:request.moduleName,sourcePath:root.path,sourceSha256:owner.sourceSha256});limit(assignment.size,16384,'native assertion batch exceeds 16384 coordinates');
   }
  }
  prepared.push({...request,sourcePath:root.path,closure,fixtures:captures});
 }
 for(const fixture of fixtures.values())for(const row of fixture.controls)if(!assignment.has(row.fixture+'\0'+row.exportName+'\0'+row.index))throw new TypeError('unassigned original native policy fixture coordinate: '+row.fixture+'/'+row.exportName+'/'+row.index);
 const products=[];let outputBytes=0,receiptBytes=0;for(const group of prepared){const product=translate({...group,assignment});outputBytes+=Buffer.byteLength(product.rust);receiptBytes+=Buffer.byteLength(product.receiptLino);limit(outputBytes,16*1024*1024,'native assertion batch outputs exceed 16 MiB');limit(receiptBytes,32*1024*1024,'native assertion batch module receipts exceed 32 MiB');products.push(product);}
 const receipt={schemaVersion:1,model:'exhaustive original native policy assertion assignments 1',fixtures:[...fixtures.values()],assignments:[...assignment].map(([coordinate,owner])=>({coordinate,owner})),modules:products.map(product=>({path:product.output,sha256:digest(product.rust),receiptSha256:digest(product.receiptLino)}))};
 return {modules:products,registration:products.map(product=>product.registration).join('\n'),receipt,receiptLino:boundedReceipt(receipt)};
}
