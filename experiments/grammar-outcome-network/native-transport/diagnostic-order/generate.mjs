// Source-derived publication author for exact diagnostic field permutations.
import assert from 'node:assert/strict';
import {compileGrammar} from '../../../../js/src/grammar.js';
import {parseGrammarLinks} from '../../../../js/src/grammar-links.js';
import {grammarSelectionFixture} from '../../../../js/tests/support/grammar-selection-cases.js';
import {parseCargoLock,parseCargoManifest} from '../../../../js/scripts/dependency-inventory.mjs';
import {satisfies} from '../../../../js/scripts/semver-range.mjs';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync,mkdirSync,existsSync} from 'node:fs';
import {resolve,dirname,relative,isAbsolute,sep} from 'node:path';
import {fileURLToPath} from 'node:url';
import {parseNative} from '../../../../js/src/native-grammar-parser.js';
import {tokenize} from '../../../../js/src/translation/lexer.js';
import {outcomeWirePolicy} from '../wire/wire-policy.mjs';
import {serializeProjectData,writeProjectData,readProjectData} from '../../../../js/scripts/project-data.mjs';

export const javascriptPath='js/src/grammar-runtime/grammar-selection.js';
export const nativePath='rust/src/grammar/feature_runtime/grammar_selection.rs';
const repository=process.env.META_LANGUAGE_DIAGNOSTIC_ORDER_REPOSITORY??fileURLToPath(new URL('../../../../',import.meta.url));
const digest=source=>createHash('sha256').update(source).digest('hex');
const children=node=>node.children.filter(child=>child.node.named&&!child.node.isExtra).map(child=>child.node);
const field=(node,name)=>{const found=node.children.filter(child=>child.field===name);assert.equal(found.length,1,'one complete actual '+name+' field');return found[0].node;};
const optionalField=(node,name)=>{const found=node.children.filter(child=>child.field===name);assert.ok(found.length<=1);return found[0]?.node;};
const canonical=value=>serializeProjectData(value,'diagnostic-order.lino');
const sameKeys=(left,right)=>assert.deepEqual([...left].sort(),[...right].sort(),'exact key membership is preserved');
// There is exactly one admitted frozen source pair. A private copy avoids
// reparsing it for each inverse/tamper control without trusting caller objects.
let frozenDerivation;
function visit(node,ancestors,operation){operation(node,ancestors);for(const child of node.children)visit(child.node,[...ancestors,node],operation);}

function actualSource(source,language,path) {
  assert.equal(typeof source,'string');
  const bytes=Buffer.from(source);assert.ok(bytes.length>0&&bytes.length<=131072,'bounded complete source owner');
  assert.equal(bytes.toString('utf8'),source,'complete source must be valid Unicode');
  const tree=parseNative(language,source);
  assert.equal(tree.term,language==='native-javascript'?'program':'source_file');
  assert.equal(tree.hasError,false,'actual source CST has no error or missing nodes');
  assert.ok(Number.isSafeInteger(tree.start)&&Number.isSafeInteger(tree.end)
    &&tree.start>=0&&tree.end>=tree.start&&tree.end<=bytes.length,'actual projected root has bounded byte coverage');
  const text=node=>bytes.subarray(node.start,node.end).toString('utf8');
  const proof=node=>({path,kind:node.term,span:{start:node.start,end:node.end},sha256:digest(text(node))});
  // A projected CST may exclude outer trivia. Keep the complete original bytes
  // and prove each excluded gap has no language token; never discard its text.
  const gaps=[{start:0,end:tree.start},{start:tree.end,end:bytes.length}].map(span=>{
    const source=bytes.subarray(span.start,span.end).toString('utf8');
    const syntax=tokenize(source,language==='native-javascript'?'JavaScript':'Rust');
    assert.equal(syntax.tokens.filter(token=>token.kind!=='eof').length,0,'outside-root source gap contains only lexical trivia');
    return {span,bytes:Buffer.byteLength(source),sha256:digest(source),comments:syntax.comments};
  });
  return {bytes,tree,text,proof,coverage:{root:proof(tree),gaps}};
}

function stringLiteral(node,parsed,language) {
  assert.ok(node.term==='string'||node.term==='string_literal','a complete literal string is required');
  const source=parsed.text(node),tokens=tokenize(source,language).tokens.filter(token=>token.kind!=='eof');
  assert.equal(tokens.length,1);assert.equal(tokens[0].kind,'string');
  assert.equal(tokens[0].start,0);assert.equal(tokens[0].end,source.length);
  return tokens[0].value;
}

/** Every JS call keeps its entire actual function and conditional ancestry as
 * byte spans in the complete retained source. No isolated snippet is executed.
 */
export function readJavaScriptDiagnosticOrders(source) {
  const parsed=actualSource(source,'native-javascript',javascriptPath),result=[];
  visit(parsed.tree,[],(call,ancestors)=>{
    if(call.term!=='call_expression')return;
    const operation=field(call,'function');
    if(operation.term!=='identifier'||parsed.text(operation)!=='reportSelectionDiagnostic')return;
    const owner=[...ancestors].reverse().find(node=>node.term==='function_declaration');assert.ok(owner,'actual named function owner');
    const args=children(field(call,'arguments'));assert.equal(args.length,2);
    assert.equal(parsed.text(args[0]),'executor');assert.equal(args[1].term,'object');
    const bindings=[];
    visit(owner,[],(node,parents)=>{
      if(node.term!=='variable_declarator')return;
      const name=field(node,'name');if(name.term!=='identifier')return;
      const scope=[...parents].reverse().find(parent=>parent.term==='statement_block');
      bindings.push({name:parsed.text(name),node,parents,scope});
    });
    const local=name=>{
      const admitted=bindings.filter(binding=>binding.name===name&&binding.node.start<call.start
        &&binding.scope&&ancestors.includes(binding.scope));
      assert.ok(admitted.length,'spread/reason has an actual preceding local owner');
      const nearest=Math.max(...admitted.map(binding=>binding.scope.start));
      const selected=admitted.filter(binding=>binding.scope.start===nearest);
      assert.equal(selected.length,1,'spread/reason local owner is unique');
      const binding=selected[0],declaration=binding.parents.at(-1);
      assert.equal(declaration.term,'lexical_declaration');
      assert.match(parsed.text(declaration),/^const\s/u,'source binding is immutable');
      visit(owner,[],node=>{
        if(!['assignment_expression','augmented_assignment_expression','update_expression'].includes(node.term))return;
        const left=optionalField(node,'left')??optionalField(node,'argument');
        if(!left)return;
        let writes=false;visit(left,[],part=>{if(part.term==='identifier'&&parsed.text(part)===name)writes=true;});
        assert.equal(writes,false,'the retained source binding has no assignment or mutation');
      });
      return {...binding,value:field(binding.node,'value')};
    };
    const dependencies=[];
    const readObject=(object,active=new Set())=>{
      assert.equal(object.term,'object');const members=[];
      for(const item of children(object)) {
        if(item.term==='spread_element') {
          const operands=children(item);assert.equal(operands.length,1);assert.equal(operands[0].term,'identifier');
          const name=parsed.text(operands[0]);assert.ok(!active.has(name),'cyclic source spread');
          const binding=local(name);assert.equal(binding.value.term,'object','only complete actual object-local spread is admitted');
          dependencies.push({role:'spread',binding:parsed.proof(binding.node),scope:parsed.proof(binding.scope)});
          const nested=new Set(active);nested.add(name);members.push(...readObject(binding.value,nested));continue;
        }
        let key,value;
        if(item.term==='pair') {
          const keyNode=field(item,'key');
          key=['property_identifier','identifier'].includes(keyNode.term)?parsed.text(keyNode):stringLiteral(keyNode,parsed,'JavaScript');
          value=field(item,'value');
        } else {
          assert.equal(item.term,'shorthand_property_identifier','unknown/computed/getter/method object property refuses');
          key=parsed.text(item);value=item;
        }
        assert.match(key,/^[A-Za-z_][A-Za-z_0-9]*$/u,'finite ordinary property key');assert.notEqual(key,'__proto__');
        members.push({key,value,source:parsed.proof(item)});
      }
      assert.equal(new Set(members.map(item=>item.key)).size,members.length,'no duplicate/overwritten source keys');return members;
    };
    const members=readObject(args[1]);
    const literalMember=key=>{
      const matches=members.filter(item=>item.key===key);assert.ok(matches.length<=1);
      if(!matches.length)return null;let node=matches[0].value;
      if(['identifier','shorthand_property_identifier'].includes(node.term)) {
        const binding=local(parsed.text(node));dependencies.push({role:key,binding:parsed.proof(binding.node),scope:parsed.proof(binding.scope)});node=binding.value;
      }
      return stringLiteral(node,parsed,'JavaScript');
    };
    result.push({sourceCoverage:parsed.coverage,call:parsed.proof(call),object:parsed.proof(args[1]),
      owner:{...parsed.proof(owner),name:parsed.text(field(owner,'name'))},
      contexts:ancestors.filter(node=>['if_statement','for_statement','for_in_statement','while_statement','ternary_expression'].includes(node.term)).map(parsed.proof),
      dependencies,reason:literalMember('reason'),detail:literalMember('detail'),keys:members.map(item=>item.key),members:members.map(({key,source})=>({key,source}))});
  });
  assert.equal(result.length,9,'complete finite JS diagnostic call inventory');return result;
}

/** Read a CST-owned json! object as comma-separated complete pair slices. The
 * native expressions remain untouched; their values are not reimplemented.
 */
function readNativeObject(node,parsed) {
  const source=parsed.text(node),tokens=tokenize(source,'Rust').tokens.filter(token=>token.kind!=='eof');
  assert.equal(tokens[0]?.kind,'macro');assert.equal(tokens[0]?.value,'json');assert.equal(tokens[1]?.value,'(');assert.equal(tokens[2]?.value,'{');
  const stack=['{'],segments=[];let first=3,start=tokens[2].end,close;
  const add=end=>{
    const selected=tokens.slice(first,end);
    if(!selected.length)return false;
    assert.equal(selected[0].kind,'string','native object key is literal');assert.equal(selected[1]?.value,':');assert.ok(selected.length>=3);
    segments.push({key:selected[0].value,source:source.slice(start,tokens[end].start),value:source.slice(selected[2].start,selected.at(-1).end)});return true;
  };
  for(let index=3;index<tokens.length;index++) {
    const token=tokens[index];if(token.kind!=='punct')continue;
    if(['{','[','('].includes(token.value)){stack.push(token.value);continue;}
    if(['}',']',')'].includes(token.value)) {
      const opening=stack.pop();assert.equal(token.value,{'{':'}','[':']','(':')'}[opening],'balanced complete native expression');
      if(!stack.length){add(index);close=index;break;}continue;
    }
    if(token.value===','&&stack.length===1){assert.ok(add(index),'no empty native object member');first=index+1;start=token.end;}
  }
  assert.ok(close!==undefined);assert.equal(tokens[close+1]?.value,')');assert.equal(close+2,tokens.length,'whole native json! invocation consumed');
  assert.equal(new Set(segments.map(item=>item.key)).size,segments.length,'native keys are unique');
  const trailing=first===close,tail=trailing?source.slice(start,tokens[close].start):'';
  const literal=key=>{
    const found=segments.find(item=>item.key===key);if(!found)return null;
    const value=tokenize(found.value,'Rust').tokens.filter(token=>token.kind!=='eof');
    assert.equal(value.length,1);assert.equal(value[0].kind,'string');return value[0].value;
  };
  const prefix=source.slice(0,tokens[2].end),suffix=source.slice(tokens[close].start);
  const render=keys=>{sameKeys(keys,segments.map(item=>item.key));return prefix+keys.map(key=>segments.find(item=>item.key===key).source).join(',')+(trailing?',':'')+tail+suffix;};
  assert.equal(render(segments.map(item=>item.key)),source,'whole native object is exactly reconstructible');
  return {source,segments,keys:segments.map(item=>item.key),reason:literal('reason'),detail:literal('detail'),render};
}

export function readNativeDiagnosticOrders(source) {
  const parsed=actualSource(source,'native-rust',nativePath),result=[];
  visit(parsed.tree,[],(call,ancestors)=>{
    if(call.term!=='call_expression')return;
    const operation=field(call,'function');if(operation.term!=='field_expression')return;
    if(parsed.text(field(operation,'field'))!=='report_selection_diagnostic')return;
    assert.equal(parsed.text(field(operation,'value')),'self');
    const args=children(field(call,'arguments'));assert.equal(args.length,1);assert.equal(args[0].term,'macro_invocation');
    const owner=[...ancestors].reverse().find(node=>node.term==='function_item');assert.ok(owner,'actual complete native function owner');
    const object=readNativeObject(args[0],parsed);
    result.push({sourceCoverage:parsed.coverage,call:parsed.proof(call),object:parsed.proof(args[0]),
      owner:{...parsed.proof(owner),name:parsed.text(field(owner,'name'))},
      contexts:ancestors.filter(node=>['if_expression','for_expression','while_expression','match_arm','let_declaration'].includes(node.term)).map(parsed.proof),
      keys:object.keys,reason:object.reason,detail:object.detail,
      source:object.source,pairs:object.segments,render:object.render});
  });
  assert.equal(result.length,10,'complete finite native diagnostic inventory includes one native-only guard');return result;
}

const signature=row=>canonical({reason:row.reason,detail:row.detail,keys:[...row.keys].sort()});
const patchBytes=(source,edits,inverse=false)=>{
  let bytes=Buffer.from(source);
  for(const edit of [...edits].sort((left,right)=>right.span.start-left.span.start)) {
    const from=inverse?edit.after:edit.before,to=inverse?edit.before:edit.after;
    assert.equal(Buffer.byteLength(from),edit.span.end-edit.span.start,'a permutation preserves its complete byte extent');
    assert.equal(Buffer.byteLength(to),Buffer.byteLength(from));
    assert.equal(bytes.subarray(edit.span.start,edit.span.end).toString('utf8'),from,'exact qualified source occurrence');
    bytes=Buffer.concat([bytes.subarray(0,edit.span.start),Buffer.from(to),bytes.subarray(edit.span.end)]);
  }
  return bytes.toString('utf8');
};

export function deriveDiagnosticOrderDecorator(javascriptSource,nativeSource) {
  assert.equal(digest(javascriptSource),outcomeWirePolicy.pins[javascriptPath],'complete original JS producer remains authenticated');
  assert.equal(digest(nativeSource),outcomeWirePolicy.pins[nativePath],'complete original native producer remains authenticated');
  if(frozenDerivation)return structuredClone(frozenDerivation);
  const javascript=readJavaScriptDiagnosticOrders(javascriptSource),native=readNativeDiagnosticOrders(nativeSource),used=new Set(),matches=[],edits=[];
  for(const row of javascript) {
    const found=native.filter(candidate=>signature(candidate)===signature(row));assert.equal(found.length,1,'one exact source-qualified diagnostic correspondence');
    const candidate=found[0];assert.ok(!used.has(candidate));used.add(candidate);
    const after=candidate.render(row.keys),changed=after!==candidate.source;
    matches.push({javascript:row,native:{...candidate,render:undefined},changed});
    // undefined is removed intentionally here, never serialized as source data.
    delete matches.at(-1).native.render;
    if(changed)edits.push({span:candidate.object.span,sourceOwner:candidate.owner,
      before:candidate.source,after,javascriptCall:row.call,
      beforeKeys:candidate.keys,afterKeys:row.keys});
  }
  const unmatched=native.filter(row=>!used.has(row)).map(({render,...row})=>row);
  assert.equal(unmatched.length,1,'native-only mapped-cover refusal is explicit');
  assert.equal(unmatched[0].reason,'unsupportedEmbeddingMapping');assert.equal(unmatched[0].detail,null);
  assert.deepEqual(unmatched[0].keys,['reason','start','end','mapping']);
  assert.equal(edits.length,2,'exactly the two spread-built diagnostic orders require a permutation');
  for(const edit of edits){sameKeys(edit.beforeKeys,edit.afterKeys);assert.notDeepEqual(edit.beforeKeys,edit.afterKeys);}
  const corrected=patchBytes(nativeSource,edits);
  assert.equal(patchBytes(corrected,edits,true),nativeSource,'whole original native producer inverse');
  const afterRows=readNativeDiagnosticOrders(corrected);
  for(const match of matches) {
    const selected=afterRows.filter(row=>signature(row)===signature(match.javascript));assert.equal(selected.length,1);
    assert.deepEqual(selected[0].keys,match.javascript.keys,'all nine native diagnostics now retain actual JS key order');
  }
  const inputs=[{path:javascriptPath,source:javascriptSource},{path:nativePath,source:nativeSource}]
    .map(input=>({...input,bytes:Buffer.byteLength(input.source),sha256:digest(input.source)}));
  const decorator={schemaVersion:1,kind:'source-derived grammar selection diagnostic key order',
    baselinePins:structuredClone(outcomeWirePolicy.pins),inputs,matches,unmatchedNative:unmatched,edits,
    result:{path:nativePath,bytes:Buffer.byteLength(corrected),sha256:digest(corrected)},
    boundary:'Two source-pair permutations only; no expression, predicate, fixture, comparator, opaque-JSON policy or frozen baseline pin changes.'};
  frozenDerivation=structuredClone({decorator,corrected});
  return structuredClone(frozenDerivation);
}

export function captureDiagnosticOrderDecorator(root=repository,{archive}={}) {
  const javascript=readFileSync(resolve(root,javascriptPath),'utf8'),native=readFileSync(resolve(root,nativePath),'utf8');
  assert.equal(digest(javascript),outcomeWirePolicy.pins[javascriptPath],'current JS producer remains the unchanged original authority');
  if(digest(native)===outcomeWirePolicy.pins[nativePath])return deriveDiagnosticOrderDecorator(javascript,native);
  // Once production is corrected, regeneration still starts with the retained
  // original source archive, never by silently repinning the corrected file.
  const retained=archive??readProjectData(new URL('./decorator.lino',import.meta.url));
  const originalNative=retained.inputs.find(input=>input.path===nativePath)?.source;
  const derived=deriveDiagnosticOrderDecorator(javascript,originalNative);
  assert.equal(canonical(retained),canonical(derived.decorator),'complete retained shared decorator archive');
  assert.equal(native,derived.corrected,'actual current producer matches the exact generated correction');
  return derived;
}

/** Both production and frozen overlay use these identical authenticated bytes.
 * The receipt separately names current and frozen roles; neither repins history.
 */
export function applyDiagnosticOrderDecorator(source,decorator,{direction='forward',role}={}) {
  assert.ok(direction==='forward'||direction==='inverse');assert.ok(['production','frozen CI overlay'].includes(role));
  const originalJavaScript=decorator.inputs.find(input=>input.path===javascriptPath)?.source;
  const originalNative=decorator.inputs.find(input=>input.path===nativePath)?.source;
  const regenerated=deriveDiagnosticOrderDecorator(originalJavaScript,originalNative);
  assert.equal(canonical(decorator),canonical(regenerated.decorator),'complete actual-source shared Links decorator conservation');
  const expected=direction==='forward'?originalNative:regenerated.corrected;
  assert.equal(source,expected,'whole selected source must match its separately authenticated role');
  const result=patchBytes(source,decorator.edits,direction==='inverse');
  assert.equal(result,direction==='forward'?regenerated.corrected:originalNative);
  return {source:result,receipt:{schemaVersion:1,role,direction,path:nativePath,
    decoratorSha256:digest(canonical(decorator)),input:{bytes:Buffer.byteLength(source),sha256:digest(source)},
    output:{bytes:Buffer.byteLength(result),sha256:digest(result)},
    preservedBaselinePins:structuredClone(outcomeWirePolicy.pins),
    inverseVerified:patchBytes(result,decorator.edits,direction!=='inverse')===source}};
}

export function writeDiagnosticOrderDecorator(result,output) {
  assert.equal(typeof output,'string');
  assert.ok(isAbsolute(output)&&output.endsWith('.lino'),'explicit absolute Links output required');
  const regenerated=deriveDiagnosticOrderDecorator(result.decorator.inputs[0].source,result.decorator.inputs[1].source);
  assert.equal(result.corrected,regenerated.corrected);assert.equal(canonical(result.decorator),canonical(regenerated.decorator));
  writeProjectData(output,result.decorator);
}

export function captureDiagnosticDependencyProfile(root=repository) {
  const lock=readFileSync(root+'/rust/Cargo.lock','utf8'),manifest=readFileSync(root+'/rust/Cargo.toml','utf8');
  assert.ok(Buffer.byteLength(lock)<=131072&&Buffer.byteLength(manifest)<=131072,'bounded complete dependency owners');
  const packages=parseCargoLock(lock),selected=packages.filter(item=>item.name==='serde_json');assert.equal(selected.length,1,'one exact active locked serde_json owner');
  const version=selected[0].version,parts=/^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$/u.exec(version);
  assert.ok(parts,'uncovered prerelease or non-numeric dependency version');
  const patch=Number(parts[3]);assert.ok(Number.isSafeInteger(patch)&&patch<Number.MAX_SAFE_INTEGER);
  const headers=[...lock.matchAll(/^\[\[package\]\]$/gmu)];assert.equal(headers.length,packages.length,'normal parser and complete package boundary inventory agree');
  const index=packages.indexOf(selected[0]),start=headers[index].index,end=headers[index+1]?.index??lock.length;
  const block=lock.slice(start,end),checksums=[...block.matchAll(/^checksum = "([0-9a-f]{64})"$/gmu)];assert.equal(checksums.length,1);
  const declaration=parseCargoManifest(manifest).dependencies.filter(item=>item.name==='serde_json'&&item.kind==='runtime');assert.equal(declaration.length,1);
  assert.ok(satisfies(version,declaration[0].requirement,'cargo'),'locked dependency must satisfy the actual manifest');
  const sections=[...manifest.matchAll(/^\[[^\]\n]+\]\s*$/gmu)],runtime=sections.filter(item=>item[0].trim()==='[dependencies]');assert.equal(runtime.length,1);
  const sectionIndex=sections.indexOf(runtime[0]),sectionStart=runtime[0].index,sectionEnd=sections[sectionIndex+1]?.index??manifest.length;
  const section=manifest.slice(sectionStart,sectionEnd),lines=section.split('\n').filter(line=>/^serde_json\s*=/u.test(line));assert.equal(lines.length,1);
  const line=/^serde_json\s*=\s*\{\s*version\s*=\s*"([^"]+)"\s*,\s*features\s*=\s*(\[[^\]]*\])\s*\}\s*$/u.exec(lines[0]);assert.ok(line,'uncovered dependency declaration/profile form');
  assert.equal(line[1],declaration[0].requirement);
  const tokens=tokenize(line[2],'JavaScript').tokens.filter(token=>token.kind!=='eof'),features=[];let at=0;
  assert.equal(tokens[at++].value,'[');
  while(tokens[at]?.value!==']'){assert.equal(tokens[at]?.kind,'string');features.push(tokens[at++].value);if(tokens[at]?.value!==',')break;at++;}
  assert.equal(tokens[at++]?.value,']');assert.equal(at,tokens.length);assert.equal(new Set(features).size,features.length);assert.ok(features.includes('preserve_order'),'uncovered order feature profile');
  const proof=(path,source,begin,finish)=>({path,bytes:Buffer.byteLength(source),sha256:digest(source),
    selected:{source:source.slice(begin,finish),span:{start:Buffer.byteLength(source.slice(0,begin)),end:Buffer.byteLength(source.slice(0,finish))},sha256:digest(source.slice(begin,finish))}});
  return {schemaVersion:1,name:'serde_json',version,
    coveredRange:{minimumInclusive:version,maximumExclusive:parts[1]+'.'+parts[2]+'.'+(patch+1)},
    packageSource:selected[0].source,packageChecksum:checksums[0][1],
    lock:proof('rust/Cargo.lock',lock,start,end),manifest:proof('rust/Cargo.toml',manifest,sectionStart,sectionEnd),
    declaration:{requirement:declaration[0].requirement,source:lines[0],features},requiredFeatures:['preserve_order'],
    boundary:'Exact locked source/manifest profile and a required runtime order capability; Cargo compiler-artifact effective-feature attestation remains separate. Any changed version, lock or declaration is uncovered until revalidated.'};
}

export function buildDiagnosticOrderFixtures(root=repository,{archive}={}) {
  const {decorator}=captureDiagnosticOrderDecorator(root,{archive}),fixture=grammarSelectionFixture();
  const linked=new Map(fixture.grammars.map(row=>[row.name,parseGrammarLinks(row.listing)]));
  const cases=[fixture.unknown,fixture.escaped].map(row=>{
    const outcome=compileGrammar(parseGrammarLinks(row.listing),{resolveGrammar:name=>linked.get(name)}).parseTree(row.source);
    assert.equal(outcome.ok,false);assert.equal(outcome.rejection.reason,row.reason);
    const matches=decorator.matches.filter(match=>match.changed&&match.javascript.reason===row.reason);
    assert.equal(matches.length,1);const descriptor=matches[0].javascript;
    assert.ok(outcome.rejection.diagnostics.length>0);
    for(const value of outcome.rejection.diagnostics)assert.deepEqual(Object.keys(value),descriptor.keys,'actual original JS field order');
    return {id:row.reason,listing:row.listing,source:row.source,
      grammars:fixture.grammars.map(value=>({name:value.name,listing:value.listing})),
      expectedReason:row.reason,expectedDiagnosticKeys:descriptor.keys,
      expectedDiagnostics:outcome.rejection.diagnostics,sourceCall:descriptor.call};
  });
  const files=['js/tests/support/grammar-selection-cases.js','js/src/grammar-runtime/grammar-selection.js'];
  const sources=files.map(path=>{const bytes=readFileSync(root+'/'+path);return {path,bytes:bytes.length,sha256:digest(bytes)};});
  return {schemaVersion:1,profile:'actual JavaScript selection diagnostic order 1',sources,cases,
    dependencyProfile:captureDiagnosticDependencyProfile(root),
    boundary:'Expected values only; native assertions run in CI. Original fourteen transport cases and independent ordered comparator are unchanged.'};
}

// This fragment is included by the private portable-source author below.
// The normal generator checks/writes only four explicitly generated files.
export const diagnosticFixturePath='parity/fixtures/grammar-selection-diagnostic-order.lino';
const diagnosticDirectory='experiments/grammar-outcome-network/native-transport/diagnostic-order';

export function renderDiagnosticDecoratorModule(decorator) {
  const hash=digest(serializeProjectData(decorator,'decorator.lino'));
  return "// Generated from the complete authenticated actual-source decorator.\n"
    +"import assert from 'node:assert/strict';\n"
    +"import {createHash} from 'node:crypto';\n"
    +"import {readProjectData,serializeProjectData} from '../../../../js/scripts/project-data.mjs';\n"
    +"export const diagnosticOrderDecorator=readProjectData(new URL('./decorator.lino',import.meta.url));\n"
    +"assert.equal(createHash('sha256').update(serializeProjectData(diagnosticOrderDecorator,'decorator.lino')).digest('hex'),'"+hash+"','complete source-derived decorator serialization');\n";
}

export function buildDiagnosticPublication(root=repository,{archive}={}) {
  const result=captureDiagnosticOrderDecorator(root,{archive});
  const fixture=buildDiagnosticOrderFixtures(root,{archive:result.decorator});
  return {...result,fixture,decoratorModule:renderDiagnosticDecoratorModule(result.decorator),
    boundary:'Generated expected data and exact source corrections only; no native execution observation.'};
}

async function diagnosticPublicationMain() {
  const args=process.argv.slice(2);let mode,root=resolve(repository),outputRoot=resolve(repository);
  for(let index=0;index<args.length;index++) {
    const argument=args[index];
    if(argument==='--check'||argument==='--write'){assert.equal(mode,undefined);mode=argument;continue;}
    assert.ok(argument==='--repository'||argument==='--output-root');assert.equal(typeof args[index+1],'string');
    const selected=resolve(args[++index]);if(argument==='--repository')root=selected;else outputRoot=selected;
  }
  assert.ok(mode==='--check'||mode==='--write','explicit normal generation mode required');
  const archiveFile=resolve(outputRoot,diagnosticDirectory,'decorator.lino');
  const archive=existsSync(archiveFile)?readProjectData(archiveFile):undefined;
  const result=buildDiagnosticPublication(root,{archive});
  const generated=[
    [nativePath,result.corrected],
    [diagnosticFixturePath,serializeProjectData(result.fixture,'fixture.lino')],
    [diagnosticDirectory+'/decorator.lino',serializeProjectData(result.decorator,'decorator.lino')],
    [diagnosticDirectory+'/decorator.mjs',result.decoratorModule]
  ];
  for(const [path,source]of generated) {
    const output=resolve(outputRoot,path),destination=relative(outputRoot,output);
    assert.ok(destination&&!isAbsolute(destination)&&destination.split(sep)[0]!=='..');
    if(mode==='--check')assert.equal(readFileSync(output,'utf8'),source,'complete generated diagnostic source/data freshness: '+path);
    else {mkdirSync(dirname(output),{recursive:true});writeFileSync(output,source);}
  }
  console.log('Diagnostic source publication '+mode.slice(2)+': two exact producer permutations, nine source correspondences, two original diagnostic fixtures; native execution remains CI-only.');
}
if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url))await diagnosticPublicationMain();
