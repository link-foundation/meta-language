// Private source-only draft. No inverse, native execution or v1 wire changes.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readProjectData,serializeProjectData} from '../../../../js/scripts/project-data.mjs';
import {wellFormedString} from '../../../../js/src/structured-data-model.js';
import {readOutcomeInventory,checkOutcomeInventory} from '../wire/generate.mjs';
import {readRustSourceItems} from '../../../../js/scripts/generate-private-output-test-model.mjs';
import {tokenize} from '../../../../js/src/translation/lexer.js';

const digest=source=>createHash('sha256').update(source).digest('hex');
const copy=value=>structuredClone(value);
const declarationIdentity=(file,name)=>'declaration:'+file+'#'+name;
const modulePath=file=>'crate::'+file.replace(/^rust\/src\//u,'').replace(/(?:\/mod)?\.rs$/u,'').replaceAll('/','::');
const typeTokens=source=>tokenize(source,'Rust').tokens.filter(token=>token.kind!=='eof');
const typePolicy=readProjectData(new URL('./type-policy.lino',import.meta.url));
assert.equal(digest(serializeProjectData(typePolicy,'type-policy.lino')),'29e78eb3915ff47788b70f935de31384cd29b519e3a8a578e3c58b70df942a65','source-derived shared type policy');
const builtinOwners=typePolicy.builtinOwners;
const ownerIdentity=qualified=>'owner:'+qualified;
const identifier=value=>typeof value==='string'&&/^[A-Za-z_][A-Za-z_0-9]*$/u.test(value);

function qualifyPath(segments,file) {
  assert.ok(segments.length&&segments.every(identifier),'unsupported qualified type path');
  if(segments[0]==='crate') return segments.join('::');
  if(['self','super'].includes(segments[0])) {
    const result=modulePath(file).split('::');let index=0;
    if(segments[0]==='self') index=1;
    else while(segments[index]==='super'){assert.ok(result.length>1,'type path escapes crate');result.pop();index++;}
    return [...result,...segments.slice(index)].join('::');
  }
  return segments.join('::');
}

/** Decode complete actual use declarations; never resolve by equal display names. */
function readImportBindings(input,owners) {
  const bindings=[];
  for(const declaration of input.imports) {
    const tokens=typeTokens(declaration.source),use=tokens.findIndex(token=>token.value==='use');
    assert.ok(use>=0,'an actual use declaration is required');let index=use+1;
    const value=()=>tokens[index]?.value;
    const take=expected=>{assert.equal(value(),expected,'complete import syntax');index++;};
    const branch=prefix=>{
      const path=[...prefix];
      while(identifier(value())) {path.push(value());index++;if(value()!=='::')break;index++;if(value()==='{')break;}
      assert.ok(path.length,'import path required');
      if(value()==='{') {take('{');while(value()!=='}'){branch(path);if(value()!==',')break;take(',');}take('}');return;}
      assert.notEqual(value(),'*','glob import has no unique type owner');
      let name=path.at(-1);
      if(value()==='as'){take('as');assert.equal(value(),'_','unsupported import alias');name='_';index++;}
      if(name!=='_') {
        const qualified=qualifyPath(path,input.file),local=modulePath(input.file)+'::'+path.join('::');
        const admitted=owners.filter(owner=>owner.qualified===qualified||owner.qualified===local);
        assert.ok(admitted.length<=1,'ambiguous import owner');
        bindings.push({name,qualified:admitted[0]?.qualified??qualified,
          source:declaration.source,span:copy(declaration.span),sha256:digest(declaration.source)});
      }
    };
    branch([]);take(';');assert.equal(index,tokens.length,'complete import consumption');
  }
  return bindings;
}

function resolveTypeOwner(path,scope) {
  if(path.length===1&&path[0]==='Self') {
    const owner=scope.owners.find(candidate=>candidate.identity===scope.owner);
    assert.ok(owner?.kind==='declaration','Self requires its actual declaration owner');
    return {...owner,binding:{name:'Self',identity:owner.identity,role:'Self'}};
  }
  const found=path.length===1?scope.bindings.filter(binding=>binding.name===path[0]):
    scope.owners.filter(owner=>owner.qualified===qualifyPath(path,scope.file));
  assert.equal(found.length,1,'unknown, shadowed or ambiguous type owner: '+path.join('::'));
  const resolved=scope.owners.find(owner=>owner.identity===found[0].identity);
  assert.ok(resolved,'a type binding must resolve to an admitted source-qualified owner');
  return {...resolved,binding:copy(found[0])};
}

/** Pure bounded type helper for actual CST text and negative controls.
 * Every type is a finite syntax graph; named recursive owners remain references.
 */
export function readPreErasureType(source,scope,identity='type:control',absoluteStart=0) {
  assert.equal(typeof source,'string');assert.ok(wellFormedString(source),'unpaired UTF-16 type source');assert.ok(source.length>0&&source.length<=8192,'bounded complete type syntax');
  assert.ok(Number.isSafeInteger(absoluteStart)&&absoluteStart>=0,'exact source byte offset required');
  assert.ok(Array.isArray(scope.bindings)&&Array.isArray(scope.owners));
  const tokens=typeTokens(source);assert.ok(tokens.length>0&&tokens.length<=512,'bounded type token inventory');
  const nodes=[];let index=0;
  const value=()=>tokens[index]?.value;
  const take=expected=>{assert.equal(value(),expected,'unsupported type syntax');index++;};
  const byte=offset=>absoluteStart+Buffer.byteLength(source.slice(0,offset));
  const parse=()=>{
    const first=index,id=identity+':'+first;let kind,references=[],properties={};
    if(value()==='&') {
      take('&');take("'");assert.equal(value(),typePolicy.referenceProperties.lifetime,'unsupported borrowed lifetime');take(typePolicy.referenceProperties.lifetime);
      assert.notEqual(value(),'mut','unsupported borrowed mutability');
      const item=parse();assert.equal(item.kind,typePolicy.referenceTarget,'only the source-owned static str borrow is admitted');
      kind='reference';references=[item.identity];properties={...copy(typePolicy.referenceProperties),
        lifetimeTokens:tokens.slice(first+1,first+3).map(token=>copy(token)),mutabilityTokens:[],
        obligations:copy(typePolicy.referenceObligations)};
    } else {
      assert.ok(identifier(value()),'unsupported type form');const path=[value()];index++;
      while(value()==='::'){take('::');assert.ok(identifier(value()),'unsupported qualified type segment');path.push(value());index++;}
      const owner=resolveTypeOwner(path,scope);references=[owner.identity];
      assert.ok(typePolicy.admittedForms.includes(owner.form),'unsupported source-qualified type owner');
      if(value()==='<') {
        const admitted=typePolicy.genericArities;kind=owner.form;
        assert.ok(Object.hasOwn(admitted,kind),'unsupported generic type owner');take('<');
        for(let argument=0;argument<admitted[kind];argument++){if(argument)take(',');references.push(parse().identity);}
        take('>');properties={owner:owner.identity,arguments:references.slice(1),binding:owner.binding};
      } else {
        assert.ok(!typePolicy.requiredArguments.includes(owner.form),'generic type arguments are required');
        kind=owner.form;properties={owner:owner.identity,binding:owner.binding};
        Object.assign(properties,copy(typePolicy.scalarProperties[kind]??{}));
      }
    }
    const selected=tokens.slice(first,index),start=selected[0].start,end=selected.at(-1).end;
    const node={identity:id,kind,references,...properties,source:source.slice(start,end),
      normalized:selected.map(token=>token.value).join(''),span:{start:byte(start),end:byte(end)},
      tokens:selected.map(token=>({...copy(token),byteSpan:{start:byte(token.start),end:byte(token.end)}}))};
    node.sha256=digest(node.source);nodes.push(node);return node;
  };
  const root=parse();assert.equal(index,tokens.length,'unsupported trailing or ambiguous type syntax');
  return {root:root.identity,nodes};
}

function sourceSlice(input,span) {
  assert.ok(Number.isSafeInteger(span?.start)&&Number.isSafeInteger(span?.end)&&span.start>=0&&span.end>=span.start&&span.end<=input.bytes,'source span must belong to authenticated complete input');
  return Buffer.from(input.source).subarray(span.start,span.end).toString('utf8');
}

function collectSyntaxNodes(node,output=[]) {
  output.push(node);for(const child of node.children??[])collectSyntaxNodes(child.node,output);return output;
}

function buildTypeGraph(inventory) {
  checkOutcomeInventory(inventory);
  const declarations=inventory.declarations,owners=declarations.map(declaration=>({
    identity:declarationIdentity(declaration.file,declaration.name),qualified:modulePath(declaration.file)+'::'+declaration.name,
    kind:'declaration',form:'named',name:declaration.name}));
  for(const [name,qualified]of Object.entries(builtinOwners))owners.push({identity:ownerIdentity(qualified),qualified,kind:'builtin',form:name,name});
  for(const external of typePolicy.externalOwners)owners.push({identity:ownerIdentity(external.qualified),...copy(external)});
  const nodes=owners.filter(owner=>owner.kind!=='declaration').map(owner=>({...copy(owner),references:[]}));
  const scopes=[],inputProofs=[];
  for(const input of inventory.inputs) {
    assert.equal(Buffer.byteLength(input.source),input.bytes);assert.equal(digest(input.source),input.sha256);
    const parsed=readRustSourceItems(Buffer.from(input.source)),imports=readImportBindings(input,owners);
    const bindings=declarations.filter(declaration=>declaration.file===input.file).map(declaration=>({
      name:declaration.name,identity:declarationIdentity(input.file,declaration.name),source:declaration.source,
      span:copy(declaration.span),sha256:declaration.sha256,role:'local'}));
    for(const binding of imports)bindings.push({...binding,identity:owners.find(owner=>owner.qualified===binding.qualified)?.identity??'unadmitted:'+binding.qualified,role:'import'});
    for(const [name,qualified]of Object.entries(builtinOwners)) {
      assert.ok(!parsed.items.some(item=>item.name===name),'shadowed prelude type owner');
      assert.ok(!bindings.some(binding=>binding.name===name),'shadowed prelude import owner');
      bindings.push({name,identity:ownerIdentity(qualified),role:'prelude'});
    }
    for(const binding of bindings)assert.equal(bindings.filter(candidate=>candidate.name===binding.name).length,1,'ambiguous type scope binding');
    for(const declaration of declarations.filter(item=>item.file===input.file)) {
      assert.equal(sourceSlice(input,declaration.span),declaration.source,'whole declaration source slice');
      assert.equal(digest(declaration.source),declaration.sha256,'whole declaration source hash');
      const actual=parsed.items.filter(item=>item.name===declaration.name&&item.kind===declaration.kind);
      assert.equal(actual.length,1,'one actual complete declaration owner');
      assert.deepEqual({start:actual[0].start,end:actual[0].end},declaration.span,'whole declaration actual CST span');
      const syntax=collectSyntaxNodes(actual[0].node),scope={file:input.file,owner:declarationIdentity(input.file,declaration.name),
        qualifiedOwner:modulePath(input.file)+'::'+declaration.name,owners,bindings};
      scopes.push(copy(scope));
      const typed=(record,typeNode)=>{
        assert.ok(typeNode,'complete actual type CST is required');
        if(record.span)assert.equal(sourceSlice(input,record.span),record.source,'complete field or tuple source slice');
        const source=parsed.text(typeNode),span={start:typeNode.start,end:typeNode.end};
        assert.equal(sourceSlice(input,span),source,'exact type CST bytes');
        assert.equal(typeTokens(source).map(token=>token.value).join(''),record.type??record.alias,'source type normalization agrees with the complete inventory');
        const result=readPreErasureType(source,scope,'type:'+input.file+'@'+span.start,span.start);nodes.push(...result.nodes);
        return {...copy(record),sourceSha256:digest(record.source),typeReference:result.root,
          sourceType:{source,span,sha256:digest(source),cst:copy(typeNode)}};
      };
      const field=record=>{
        const found=syntax.filter(node=>node.term==='field_declaration'&&node.start===record.span.start&&node.end===record.span.end);
        assert.equal(found.length,1,'field belongs to actual complete CST');return typed(record,parsed.field(found[0],'type'));
      };
      const projected={...copy(declaration),identity:scope.owner,references:[]};
      if(declaration.fields)projected.fields=declaration.fields.map(field);
      if(declaration.variants)projected.variants=declaration.variants.map(variant=>{
        assert.equal(sourceSlice(input,variant.span),variant.source,'complete variant source slice');
        const result={...copy(variant),sourceSha256:digest(variant.source)};
        if(variant.fields)result.fields=variant.fields.map(field);
        if(variant.tuple) {
          const actualVariant=syntax.filter(node=>node.term==='enum_variant'&&node.start===variant.span.start&&node.end===variant.span.end);
          assert.equal(actualVariant.length,1,'tuple variant belongs to actual complete CST');
          const body=parsed.field(actualVariant[0],'body');assert.equal(body?.term,'ordered_field_declaration_list');
          const tupleTypes=body.children.filter(({node})=>node.named&&!['attribute_item','visibility_modifier'].includes(node.term)).map(({node})=>node);
          assert.equal(tupleTypes.length,variant.tuple.length,'complete actual tuple type inventory');
          result.tuple=variant.tuple.map((record,index)=>{
            assert.deepEqual({start:tupleTypes[index].start,end:tupleTypes[index].end},record.span,'actual tuple type span');return typed(record,tupleTypes[index]);
          });
        }
        return result;
      });
      if(declaration.alias){const alias=typed({...declaration,type:declaration.alias},parsed.field(actual[0].node,'type'));projected.aliasTypeReference=alias.typeReference;projected.sourceType=alias.sourceType;}
      projected.references=[...(projected.fields??[]).map(record=>record.typeReference),
        ...(projected.variants??[]).flatMap(variant=>[...(variant.fields??[]),...(variant.tuple??[])].map(record=>record.typeReference)),
        ...(projected.aliasTypeReference?[projected.aliasTypeReference]:[])];
      nodes.push(projected);
    }
    inputProofs.push({file:input.file,bytes:input.bytes,sha256:input.sha256,importsSha256:input.importsSha256});
  }
  assert.equal(new Set(nodes.map(node=>node.identity)).size,nodes.length,'every graph node has one source identity');
  const identities=new Set(nodes.map(node=>node.identity));
  for(const node of nodes)for(const reference of node.references)assert.ok(identities.has(reference),'closed complete type reference graph');
  const roots=declarations.map(declaration=>declarationIdentity(declaration.file,declaration.name));
  const reached=new Set(),pending=[...roots];while(pending.length){const id=pending.pop();if(reached.has(id))continue;reached.add(id);pending.push(...nodes.find(node=>node.identity===id).references);}
  assert.ok(nodes.filter(node=>node.kind==='named'||node.kind==='declaration'||node.name&&declarations.some(item=>item.name===node.name)).every(node=>reached.has(node.identity)),'all declaration and named references are reachable');
  return {schemaVersion:1,boundary:'Source-derived type inventory only; no DTO inverse, native representation capture, target-width selection or execution evidence.',
    sourceInventory:copy(inventory),inputProofs,rootReexport:copy(inventory.rootReexport),roots,nodes,scopes,
    obligations:['native target pointer width unresolved','serde_json representation/version/effective feature profile unresolved',
      'static reference reconstruction unresolved','physical source ownership and original DTO reconstruction not implemented']};
}

/** Public capture reads the actual authenticated repository; no caller can
 * authenticate changed projections by retaining only the original input text.
 * One capture is an explicit immutable source snapshot, not a filesystem watcher.
 */
export function captureNativeOutcomeTypeInventory(root) {
  const authenticated=readOutcomeInventory(root);checkOutcomeInventory(authenticated);
  const prepared=buildTypeGraph(authenticated);
  const sourceInventory=copy(authenticated);
  const derive=(candidate=sourceInventory)=>{
    assert.deepEqual(candidate,authenticated,'complete authenticated declaration, field, variant, binding and source inventory conservation');
    return copy(prepared);
  };
  return {sourceInventory,scopes:copy(prepared.scopes),derive};
}
