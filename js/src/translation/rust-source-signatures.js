// Structured, owned facts from actual parsed headers. No raw source or hash.
import {readRuntimeProjectData} from '../project-data-loader.js';
import {unsupported,typeError} from './diagnostics.js';
import {typeKey} from './types.js';
import {acceptProgramIntegralMetadata} from '../program-numeric-metadata-rules.js';
import {acceptRustSourceLifetimeBinding} from './rust-indirection-rules.js';
import {acceptRustSourceSignatureModel,acceptRustSourceSignaturePrimitive,acceptRustSourceSignatureWrapper,acceptRustSourceSignatureOwnerKind} from './rust-source-signature-rules.js';
import {acceptRustRetainedSourceSignatureModel,acceptRustRetainedFunctionOwner} from './rust-source-owner-retention-rules.js';
import {readRustStandardSumSignatureArgumentConstructors} from './rust-standard-sum-signature-rules.js';
import {checkRustStandardCollectionCarrier} from './rust-standard-collection-signatures.js';
import {acceptRustOwnedBoxTypeNode,acceptRustOwnedStandardSumBoxPayload} from './rust-owned-recursive-datatype-rules.js';
import {createRustBorrowedVectorElementPosition} from './rust-borrowed-vectors.js';
import {acceptRustBorrowedVectorSignatureModel,acceptRustBorrowedVectorFunctionOwner,readRustBorrowedVectorPositionError,readRustBorrowedVectorElementPositionError,readRustBorrowedVectorPositionBudgetError} from './rust-borrowed-vector-rules.js';
export const RUST_SOURCE_SIGNATURE_POLICY=readRuntimeProjectData(new URL('./data/rust-source-signatures.lino',import.meta.url));

export function readRustSourceTypeOwner(parser,head,children,type) {
  const owner=(kind,path,arguments_)=>({kind,path,arguments:arguments_});
  if(head[0]==='fn'&&type.kind==='function')return owner('function',[],children.length===type.params.length?[...children,owner('unit',[],[])]:children);
  if(head[0]==='&') {
    if(head.includes('['))return owner('slice',[],children);
    if(children.length)return children[0];
    const declared=parser.origins.lookup(['str'],parser.modulePath,RUST_SOURCE_SIGNATURE_POLICY.typeOwnerKinds)??parser.origins.lookup(parser.resolve(['str'],parser.modulePath),parser.modulePath,RUST_SOURCE_SIGNATURE_POLICY.typeOwnerKinds);
    return declared?owner('declaration',declared.fullName.split('.'),[]):owner('primitive',['str'],[]);
  }
  if(head[0]==='(') {
    if(type.kind==='unit')return owner('unit',[],[]);
    if(type.kind==='tuple')return owner('tuple',[],children);
    return children[0];
  }
  const resolved=parser.resolve(head,parser.modulePath);
  const declared=parser.origins.lookup(head,parser.modulePath,RUST_SOURCE_SIGNATURE_POLICY.typeOwnerKinds)??parser.origins.lookup(resolved,parser.modulePath,RUST_SOURCE_SIGNATURE_POLICY.typeOwnerKinds);
  if(declared)return owner('declaration',declared.fullName.split('.'),children);
  if(type.kind==='named'){
    const actual=parser.origins.lookup(type.path,parser.origins.type.path,RUST_SOURCE_SIGNATURE_POLICY.typeOwnerKinds);
    if(actual?.k==='data')return owner('declaration',actual.fullName.split('.'),children);
  }
  const library=RUST_SOURCE_SIGNATURE_POLICY.libraries[head.join('::')]??Object.values(RUST_SOURCE_SIGNATURE_POLICY.libraries).find(path=>path.join('::')===resolved.join('::'));
  if(library)return owner('library',[...library],children);
  if(head.length===1&&Object.hasOwn(RUST_SOURCE_SIGNATURE_POLICY.primitives,head[0]))return owner('primitive',[...head],children);
  return owner('declaration',resolved,children);
}

export function readRustSourceTypeHead(cursor) {
  const first=cursor.peek().value;
  if(['&','('].includes(first)) {
    const head=[first];
    if(first==='&') {
      let at=1;if(cursor.is("'",at))at+=2;
      if(cursor.is('[',at))head.push('[');
    }
    return head;
  }
  const path=[first];let at=1;
  while(cursor.is('::',at)&&cursor.peek(at+1).kind==='identifier'){path.push(cursor.peek(at+1).value);at+=2;}
  return path;
}

export function createRustSourceSignature(model,parameters,result,lifetimeBindings) {
  if(!(acceptRustSourceSignatureModel(model)||acceptRustRetainedSourceSignatureModel(model)||acceptRustBorrowedVectorSignatureModel(model))||!model)throw unsupported('Rust source signature model','select a supported owned source signature model');
  const position=origin=>{const error=readRustBorrowedVectorPositionError(model,Boolean(origin.arrayElementPosition));if(error)throw typeError(error);return createRustBorrowedVectorElementPosition(origin);};
  return {model,language:'Rust',parameters:parameters.map(position),result:position(result),lifetimeBindings:[...lifetimeBindings]};
}

export function checkRustSourceSignature(signature,parameters,result,language,resolveType=value=>value,resolveData=()=>null) {
  const policy=RUST_SOURCE_SIGNATURE_POLICY;
  const borrowed=acceptRustBorrowedVectorSignatureModel(signature?.model??'');
  const retained=acceptRustRetainedSourceSignatureModel(signature?.model??'')||borrowed;
  let visits=0;
  const exact=(value,keys)=>{if(!value||typeof value!=='object'||Array.isArray(value)||Object.keys(value).length!==keys.length||keys.some(key=>!Object.hasOwn(value,key)))throw typeError('an owned Rust source signature has unrecognized or missing fields');};
  exact(signature,policy.fields.signature);
  if(signature.model!==policy.model&&!retained||signature.language!==policy.language||language!==policy.language)throw typeError('an owned Rust source signature has an unsupported model or language');
  if(!Array.isArray(signature.parameters)||signature.parameters.length!==parameters.length||!Array.isArray(signature.lifetimeBindings)||signature.lifetimeBindings.some(value=>typeof value!=='string'||!acceptRustSourceLifetimeBinding(value,false,false))||new Set(signature.lifetimeBindings).size!==signature.lifetimeBindings.length)throw typeError('an owned Rust source signature has inconsistent parameter or lifetime bindings');
  function valueType(value,allowNamed=false,depth=0) {
    if(++visits>100000||depth>256)throw typeError('an owned Rust source signature exceeds its resource limit');
    if(allowNamed&&value?.kind==='named') {
      const keys=Object.keys(value);
      if(keys.some(key=>!['kind','path','span'].includes(key))||!Array.isArray(value.path)||value.path.some(part=>typeof part!=='string'))throw typeError('an owned Rust source type has unrecognized value-type fields');
      return;
    }
    const fields=retained&&value?.kind==='function'?['kind','params','ret']:policy.valueTypeFields[value?.kind];
    if(!fields)throw typeError('an owned Rust source type has an unsupported value type');
    exact(value,fields);
    if(value.kind==='fixed'&&(!acceptProgramIntegralMetadata(value.kind,'bits',value.bits,Object.is(value.bits,-0))||typeof value.signed!=='boolean'))throw typeError('an owned Rust source type has invalid fixed-width metadata');
    if(value.kind==='array')valueType(value.element,allowNamed,depth+1);
    if(value.kind==='tuple') {if(!Array.isArray(value.elements))throw typeError('an owned Rust source tuple has invalid fields');value.elements.forEach(child=>valueType(child,allowNamed,depth+1));}
    if(value.kind==='function'){if(!Array.isArray(value.params))throw typeError('an owned Rust source function has invalid parameters');value.params.forEach(child=>valueType(child,allowNamed,depth+1));valueType(value.ret,allowNamed,depth+1);}
  }
  function owner(value,type,depth=0) {
    if(++visits>100000||depth>256)throw typeError('an owned Rust source signature exceeds its resource limit');
    exact(value,policy.fields.owner);
    if(!(acceptRustSourceSignatureOwnerKind(value.kind)||retained&&value.kind==='function')||!Array.isArray(value.path)||value.path.some(part=>typeof part!=='string'||!part)||!Array.isArray(value.arguments))throw typeError('an owned Rust source type has an invalid owner');
    const path=value.path.join('::');
    if(value.kind==='function'){
      if(!(acceptRustRetainedFunctionOwner(signature.model,value.kind,type.kind,value.path.length,value.arguments.length,type.params?.length??-1,type.reference===true)||acceptRustBorrowedVectorFunctionOwner(signature.model,value.kind,type.kind,value.path.length,value.arguments.length,type.params?.length??-1,type.reference===true)))throw typeError('an owned Rust source function differs from its value type');
      value.arguments.forEach((child,index)=>owner(child,index<type.params.length?type.params[index]:type.ret,depth+1));
    } else if(value.kind==='primitive') {
      if(value.path.length!==1||value.arguments.length||!acceptRustSourceSignaturePrimitive(type.kind,path,String(type.bits??''),Boolean(type.signed)))throw typeError('an owned Rust source primitive differs from its value type');
    } else if(value.kind==='declaration') {
      if(!value.path.length||value.arguments.length||type.kind!=='data'||value.path.join('.')!==type.name)throw typeError('an owned Rust source declaration differs from its value type');
    } else if(value.kind==='unit') {
      if(value.path.length||value.arguments.length||type.kind!=='unit')throw typeError('an owned Rust source unit differs from its value type');
    } else if(value.kind==='tuple') {
      if(value.path.length||type.kind!=='tuple'||value.arguments.length!==type.elements.length)throw typeError('an owned Rust source tuple differs from its value type');
      value.arguments.forEach((child,index)=>owner(child,type.elements[index],depth+1));
    } else if(value.kind==='slice'||path==='std::vec::Vec') {
      if(value.kind==='slice'&&value.path.length||type.kind!=='array'||value.arguments.length!==1)throw typeError('an owned Rust source array differs from its value type');
      owner(value.arguments[0],type.element,depth+1);
    } else if(path==='std::boxed::Box'||path==='alloc::boxed::Box') {
      if(value.arguments.length!==1)throw typeError('an owned Rust source box has invalid arguments');
      owner(value.arguments[0],type,depth+1);
    } else if(path==='std::string::String') {
      if(value.arguments.length||type.kind!=='string')throw typeError('an owned Rust source string differs from its value type');
    } else if(['std::collections::HashMap','std::collections::HashSet','std::collections::BTreeMap','std::collections::BTreeSet'].includes(path)) {
      if(value.kind!=='library'||type.kind!=='data')throw typeError('an owned standard collection needs its actual distinct nominal carrier');
      const declaration=resolveData(type),carrier=declaration&&checkRustStandardCollectionCarrier(declaration,language,resolveType,name=>resolveData({kind:'data',name}));
      if(!carrier)throw typeError('an owned standard collection needs its actual checked carrier specialization');
      const pairs=[[value,carrier.origin.sourceOwner]];
      for(let at=0;at<pairs.length;at++){
        if(++visits>100000)throw typeError('an owned Rust source signature exceeds its resource limit');
        const [left,right]=pairs[at];exact(left,policy.fields.owner);
        if(!right||left.kind!==right.kind||!Array.isArray(left.path)||!Array.isArray(left.arguments)||left.path.length!==right.path.length||left.path.some((part,index)=>part!==right.path[index])||left.arguments.length!==right.arguments.length)throw typeError('a collection source position changes its actual complete standard owner or domains');
        for(let child=0;child<left.arguments.length;child++)pairs.push([left.arguments[child],right.arguments[child]]);
      }
    } else if(path==='std::option::Option'||path==='std::result::Result') {
      if(value.kind!=='library'||type.kind!=='data')throw typeError('an owned Rust standard sum needs its actual datatype encoding');
      const declaration=resolveData(type),fields=declaration?.sourceFieldSignatures;
      if(!declaration||declaration.k!=='data'||declaration.fullName!==type.name||!Array.isArray(declaration.ctors)||!fields)throw typeError('an owned Rust standard sum needs its complete source field signatures');
      const selected=readRustStandardSumSignatureArgumentConstructors(path,declaration.ctors.map(ctor=>ctor.name),declaration.ctors.map(ctor=>ctor.fields?.length??-1));
      if(selected.length!==value.arguments.length||selected.length===0)throw typeError('an owned Rust standard sum differs from its complete constructor inventory');
      exact(fields,['model','language','dataTypeName','ctors']);
      if(fields.model!=='Rust data type field signatures 1'||fields.language!==language||fields.dataTypeName!==type.name||!Array.isArray(fields.ctors)||fields.ctors.length!==declaration.ctors.length)throw typeError('an owned Rust standard sum has inconsistent source field ownership');
      for(let index=0;index<declaration.ctors.length;index+=1){
        const actual=declaration.ctors[index],retained=fields.ctors[index];exact(retained,['name','fields']);
        if(retained.name!==actual.name||!Array.isArray(retained.fields)||retained.fields.length!==actual.fields.length)throw typeError('an owned Rust standard sum has inconsistent retained constructors');
      }
      for(let index=0;index<selected.length;index+=1){
        const constructor=selected[index],actual=declaration.ctors[constructor].fields[0].type,field=fields.ctors[constructor].fields[0];
        const checked=position(field,actual,depth+1);
        if(checked.wrappers.length||checked.lifetimes.length){let child=checked.owner,boxes=0;while(boxes<256&&acceptRustOwnedBoxTypeNode(['box'],child.kind,child.path.join('::'),child.arguments.length)){boxes+=1;child=child.arguments[0];}if(!acceptRustOwnedStandardSumBoxPayload(checked.wrappers,checked.lifetimes.length,checked.owner.kind,checked.owner.path.join('::'),checked.owner.arguments.length,boxes))throw typeError('an owned Rust standard sum payload requires a direct source origin or an actual closed standard Box chain');}
        owner(value.arguments[index],actual,depth+1);
        const pairs=[[value.arguments[index],checked.owner]];
        for(let at=0;at<pairs.length;at+=1){
          if(++visits>100000)throw typeError('an owned Rust source signature exceeds its resource limit');
          const [left,right]=pairs[at];
          if(left.kind!==right.kind||left.path.length!==right.path.length||left.path.some((part,slot)=>part!==right.path[slot])||left.arguments.length!==right.arguments.length)throw typeError('an owned Rust standard sum changes a retained payload owner');
          for(let child=0;child<left.arguments.length;child+=1)pairs.push([left.arguments[child],right.arguments[child]]);
        }
      }
    } else throw typeError('an owned Rust source type has an unsupported owner');
  }
  const position=(value,expected,depth=0)=>{
    if(++visits>100000||depth>256)throw typeError('an owned Rust source signature exceeds its resource limit');
    const child=Object.hasOwn(value??{},'arrayElementPosition'),profileError=readRustBorrowedVectorPositionError(signature.model,child);if(profileError)throw typeError(profileError);
    if(child){const budget=readRustBorrowedVectorPositionBudgetError(visits,depth);if(budget)throw typeError(budget);}
    exact(value,child?[...policy.fields.position,'arrayElementPosition']:policy.fields.position);
    valueType(value.type,true);
    const type=resolveType(value.type);
    valueType(type);
    if(typeKey(type)!==typeKey(expected))throw typeError('an owned Rust source signature differs from its function type');
    if(!Array.isArray(value.wrappers)||value.wrappers.some(wrapper=>!acceptRustSourceSignatureWrapper(wrapper))||!Array.isArray(value.lifetimes)||value.lifetimes.length!==value.wrappers.filter(wrapper=>wrapper==='reference').length||value.lifetimes.some(lifetime=>lifetime!==null&&(typeof lifetime!=='string'||!signature.lifetimeBindings.includes(lifetime)&&!policy.specialLifetimes.includes(lifetime))))throw typeError('an owned Rust source type has inconsistent wrapper or lifetime facts');
    owner(value.owner,type,depth+1);
    let arrayElementPosition;
    if(child){
      if(type.kind!=='array')throw typeError('a retained array element position requires an array value type');
      arrayElementPosition=position(value.arrayElementPosition,type.element,depth+1);
      const childOwner=value.owner.arguments[0],sameOwner=JSON.stringify(childOwner)===JSON.stringify(arrayElementPosition.owner);
      const error=readRustBorrowedVectorElementPositionError(type.kind,value.owner.kind,value.owner.path.join('::'),value.owner.arguments.length,typeKey(arrayElementPosition.type)===typeKey(type.element),sameOwner);if(error)throw typeError(error);
    }
    let boxed = 0, leaf = value.owner;
    while (leaf.kind === 'library' && ['std::boxed::Box','alloc::boxed::Box'].includes(leaf.path.join('::'))) { boxed += 1; leaf = leaf.arguments[0]; }
    if (boxed !== value.wrappers.filter(wrapper=>wrapper==='box').length ||
      ((leaf.kind==='slice'||leaf.kind==='primitive'&&leaf.path[0]==='str')&&!value.wrappers.includes('reference'))) throw typeError('an owned Rust source type has inconsistent owner and wrapper facts');
    return {...value,type,...(child?{arrayElementPosition}:{})};
  };
  return {...signature,parameters:signature.parameters.map((value,index)=>position(value,parameters[index])),result:position(signature.result,result)};
}
