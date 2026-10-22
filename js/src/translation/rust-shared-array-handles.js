// Actual parser-owned source facts; all admission decisions use shared rules.
import {unsupported} from './diagnostics.js';
import {array,BOOL,sameType} from './types.js';
import {tokenize} from './lexer.js';
import {acceptRustSourceArrayOwner} from './rust-array-rules.js';
import {acceptRustSharedArrayHandleSourceSize,acceptRustSharedArrayHandleLayer,
  acceptRustSharedArrayHandlePayload,readRustSharedArrayHandleSourceForm,acceptRustSharedArrayHandleValueObservation} from './rust-shared-array-handle-rules.js';
const states=new WeakMap();
const model='standard Rust shared array identity handles 1';
const sourceModel='standard shared array handle';
const fail=(reason,span)=>{throw unsupported('Rust shared array handle source',reason,span);};
const state=parser=>{const value=states.get(parser);if(!value||value.tokens!==parser.cursor.tokens)fail('source facts require this actual parser-owned token stream');return value;};
export function requireRustSharedArrayHandleSourceText(source){
  if(typeof source!=='string'||!acceptRustSharedArrayHandleSourceSize(source.length))fail('the profile requires its complete source within 65536 UTF-16 units');
}
export function verifyRustSharedArrayHandleTokens(tokens,source){
  requireRustSharedArrayHandleSourceText(source);
  const actual=tokenize(source,'Rust').tokens;
  if(tokens.length!==actual.length)fail('the actual complete source token stream differs');
  for(let at=0;at<tokens.length;at++){
    const row=tokens[at],expected=actual[at],keys=Object.keys(expected);
    const descriptors=row&&Object.getOwnPropertyDescriptors(row);
    if(!row||Object.getPrototypeOf(row)!==Object.prototype||Reflect.ownKeys(row).length!==keys.length
      ||keys.some(key=>!Object.hasOwn(descriptors,key)||!Object.hasOwn(descriptors[key],'value')||!descriptors[key].enumerable||descriptors[key].value!==expected[key]))fail('every original source token and span must be authentic');
  }
}
export function createRustSharedArrayHandleState(parser,selected){
  if(!selected)return;
  if(selected!==model)fail('select the standard shared array identity handle source model');
  states.set(parser,{tokens:parser.cursor.tokens,payloadDepth:0,names:new Map(),borrows:new Set(),acceptedNames:new Set(),acceptedBorrows:new Set()});
}
function owner(parser,segments,role,path,span){
  const resolved=parser.resolve(segments,path);
  const shadowed=Boolean(parser.origins.lookup([segments[0]],path)||parser.origins.lookup([resolved[0]],path)||parser.origins.scope.has(segments[0]));
  return acceptRustSharedArrayHandleLayer(role,resolved.join('::'),shadowed);
}
export function isRustSharedArrayHandleOrigin(parser,origin){
  return Boolean(parser.sharedArrayHandleSourceModel&&origin?.proven&&origin.arraySourceModel===sourceModel
    &&origin.type?.kind==='array'&&origin.type.reference===true&&origin.wrappers.length===0);
}
function mark(parser,node,element,path){
  return parser.origins.mark(node,{type:array(element,true),arraySourceModel:sourceModel,path:[...path],wrappers:[],fields:null,textSourceModel:null,proven:true,copyEvidence:false,diverging:false,uncertain:false});
}
export function readRustSharedArrayHandleType(parser,segments,start){
  if(!parser.sharedArrayHandleSourceModel)return null;
  state(parser);const c=parser.cursor,path=parser.modulePath;
  if(segments.join('::')!=='std::rc::Rc')return null;
  if(!owner(parser,segments,'handle',path,start))fail('the outer type requires the actual unshadowed standard Rc owner',start);
  c.expect('<','shared array handle');
  const cellStart=c.peek(),cell=parser.pathSegments('shared array cell type');
  if(!owner(parser,cell,'cell',path,cellStart))fail('the payload requires the actual standard RefCell owner',cellStart);
  c.expect('<','shared array cell');
  const vectorStart=c.peek(),vector=parser.pathSegments('shared array vector type'),resolved=parser.resolve(vector,path);
  const shadowed=Boolean(parser.origins.lookup([vector[0]],path)||parser.origins.lookup([resolved[0]],path));
  if(!acceptRustSourceArrayOwner(resolved.join('::'),shadowed))fail('the cell requires its actual standard Vec payload',vectorStart);
  c.expect('<','shared array vector');
  const element=parser.type(),origin=parser.origins.type;
  if(!acceptRustSharedArrayHandlePayload(element.kind,origin.wrappers.length>0)||element.reference)fail('the vector needs direct admitted scalar elements',vectorStart);
  c.expect('>','shared array vector');c.expect('>','shared array cell');c.expect('>','shared array handle');
  const type=array(element,true);parser.origins.typed(type,path);parser.origins.type.arraySourceModel=sourceModel;
  return type;
}
export function refuseRustSharedArrayBareVector(parser,span){
  if(parser.sharedArrayHandleSourceModel)fail('a bare value Vec or slice requires an independent composition proof',span);
}
export function noteRustSharedArrayHandleName(parser,node,path){
  if(!parser.sharedArrayHandleSourceModel)return node;
  const value=state(parser),origin=parser.origins.origin(node,path);
  if(node.path?.length===1&&parser.origins.scope.has(node.path[0])&&isRustSharedArrayHandleOrigin(parser,origin))value.names.set(node,node.span);
  const declaration=parser.origins.lookup(node.path??[],path);
  if(declaration?.k==='fn'&&!parser.origins.scope.has(node.path?.[0]))fail('a function value needs an actual call, not implicit invocation',node.span);
  return node;
}
export function noteRustSharedArrayHandleBorrow(parser,node,path){
  if(!parser.sharedArrayHandleSourceModel)return node;
  const value=state(parser),origin=parser.origins.origin(node,path);
  if(!parser.origins.collecting&&!(origin.arraySourceModel===sourceModel&&origin.type?.reference&&origin.wrappers.length===1&&origin.wrappers[0]==='reference'))fail('only one immutable handle borrow for actual Rc::ptr_eq is admitted',node.span);
  value.borrows.add(node);return node;
}
function acceptName(parser,node){const value=state(parser);if(value.names.has(node))value.acceptedNames.add(node);}
export function readRustSharedArrayHandleMethod(parser,receiver,method,args,range){
  if(!parser.sharedArrayHandleSourceModel)return null;
  state(parser);
  const origin=parser.origins.origin(receiver,parser.modulePath),actual=isRustSharedArrayHandleOrigin(parser,origin);
  if(method.value!=='clone'){if(!parser.origins.collecting&&origin.arraySourceModel===sourceModel)fail('payload methods are outside the actual shared handle source profile',range);return null;}
  const form=readRustSharedArrayHandleSourceForm('clone',args.length,true,actual,false,origin.wrappers.length>0,false,false);
  if(!form){if(parser.origins.collecting)return null;fail('only the actual owned shared handle can clone as an alias',range);}
  acceptName(parser,receiver);
  // A new occurrence node retains the real method span; it is still the same
  // existing semantic value and has no allocation or payload read operation.
  const result={...receiver,span:{start:receiver.span?.start??range.start,end:parser.cursor.peek(-1).end}};
  return parser.origins.mark(result,origin);
}
export function readRustSharedArrayHandleConstructor(parser,segments,start,path){
  if(!parser.sharedArrayHandleSourceModel||segments.join('::')!=='std::rc::Rc::new')return null;
  const value=state(parser),c=parser.cursor;
  if(!owner(parser,segments.slice(0,-1),'handle',path,start))fail('the constructor needs the actual unshadowed Rc owner',start);
  c.expect('(','shared handle constructor');const cellStart=c.peek(),cell=parser.pathSegments('shared cell constructor');
  if(cell.at(-1)!=='new'||!owner(parser,cell.slice(0,-1),'cell',path,cellStart))fail('one actual RefCell::new layer is required',cellStart);
  c.expect('(','shared cell constructor');
  if(c.peek().kind!=='macro'||c.peek().value!=='vec')fail('the one cell argument must be a fresh nonempty vec literal',c.peek());
  value.payloadDepth++;
  let payload;try{payload=parser.expr(path);}finally{value.payloadDepth--;}
  c.expect(')','shared cell constructor');c.expect(')','shared handle constructor');
  const origin=parser.origins.origin(payload,path),fresh=payload.k==='array'&&payload.items.length>0&&origin.type?.element
    &&payload.items.every(item=>!item.spread)&&acceptRustSharedArrayHandlePayload(origin.type.element.kind,false);
  if(!readRustSharedArrayHandleSourceForm('allocate',1,true,false,false,false,false,Boolean(fresh)))fail('only one complete fresh scalar vec literal constructs a shared array handle',start);
  return mark(parser,{...payload,span:{start:start.start,end:c.peek(-1).end}},origin.type.element,path);
}
export function noteRustSharedArrayLiteral(parser,node,path){
  if(!parser.sharedArrayHandleSourceModel)return node;
  if(state(parser).payloadDepth!==1)fail('value array literals are admitted only as the one owned shared-handle payload',node.span);
  return node;
}
export function readRustSharedArrayHandleCall(parser,segments,args,path,range){
  if(!parser.sharedArrayHandleSourceModel||segments.join('::')!=='std::rc::Rc::ptr_eq')return null;
  const value=state(parser),origins=args.map(arg=>parser.origins.origin(arg,path));
  const handles=origins.map(origin=>Boolean(origin.proven&&origin.arraySourceModel===sourceModel&&origin.type?.reference));
  const borrowed=origins.length===2&&origins.every((origin,index)=>origin.wrappers.length===1&&origin.wrappers[0]==='reference'&&value.borrows.has(args[index]));
  const same=origins.length===2&&origins[0].type&&origins[1].type&&sameType(origins[0].type,origins[1].type);
  const form=readRustSharedArrayHandleSourceForm('pointer equality',args.length,owner(parser,segments.slice(0,-1),'handle',path,range),handles[0]===true,handles[1]===true,Boolean(borrowed),Boolean(same),false);
  if(!form&&!parser.origins.collecting)fail('ptr_eq needs two actual immutable borrowed shared handles in the same complete domain',range);
  if(args.length!==2)fail('ptr_eq needs exactly two arguments',range);
  for(let at=0;at<args.length;at++){
    acceptName(parser,args[at]);value.acceptedBorrows.add(args[at]);
    if(origins[at].wrappers[0]==='reference')parser.origins.mark(args[at],{...origins[at],wrappers:origins[at].wrappers.slice(1)});
  }
  const callRange={start:range.start,end:parser.cursor.peek(-1).end};
  return parser.origins.mark({k:'referenceIdentity',op:'eq',left:args[0],right:args[1],span:callRange},{type:BOOL,path:[...path],wrappers:[],proven:true,copyEvidence:true,fields:null,textSourceModel:null});
}
export function finishRustSharedArrayHandleSource(parser,root){
  if(!parser.sharedArrayHandleSourceModel)return;
  const value=state(parser);
  if(!root.items.length&&!root.main||root.items.some(item=>item.k!=='fn')||root.main?.effects.some(effect=>effect.k==='assert')||parser.aliases.size||parser.crateDeclarations.size)fail('this first profile requires closed flat functions with standard fully qualified owners');
  for(const [node,span]of value.names)if(!value.acceptedNames.has(node))fail('a shared handle source use needs its actual explicit clone or ptr_eq borrow',span);
  for(const node of value.borrows)if(!value.acceptedBorrows.has(node))fail('a handle borrow cannot escape the actual ptr_eq call',node.span);
}

export function refuseRustSharedArrayHandleValueComparison(parser,left,right,path,span){
  if(!parser.sharedArrayHandleSourceModel||parser.origins.collecting)return;
  const a=parser.origins.origin(left,path),b=parser.origins.origin(right,path);
  if(!acceptRustSharedArrayHandleValueObservation(a.arraySourceModel===sourceModel,b.arraySourceModel===sourceModel))fail('ordinary Rust comparison observes payloads; only actual Rc::ptr_eq is admitted',span);
}
export function refuseRustSharedArrayHandleObservation(parser,node,path,span){
  if(!parser.sharedArrayHandleSourceModel||parser.origins.collecting)return;
  if(!acceptRustSharedArrayHandleValueObservation(parser.origins.origin(node,path).arraySourceModel===sourceModel,false))fail('Display of a shared handle needs an independent payload observation model',span);
}
