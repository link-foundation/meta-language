// Token and source-origin transport. Shared plans decide field order/bindings;
// ordinary constructors and exhaustive matches retain all checked value owners.
import{unsupported,typeError}from'./diagnostics.js';
import{resolveRustTypeAliasPath}from'./rust-type-aliases.js';
import{planRustRecordFields,planRustRecordPatternFields,planRustRecordUpdateFields,acceptRustRecordUpdateOwner,acceptRustIrrefutablePatternNode,planTuplePatternFields,readRustRecordFieldPosition,readRustRecordBindingNames,readRustRecordConstructorName,acceptRustRecordSourceName}from'./rust-record-rules.js';

function span(from,to){return{start:from.start,end:Math.max(from.end,to&&to.kind!=='eof'?to.start:from.end)};}

function recordNames(parser,count){return readRustRecordBindingNames([...parser.reservedNames],count);}
function sourceType(parser,type,path){
 if(!type)return undefined;
 if(type.kind==='named'){
  const owner=parser.origins.lookup(parser.resolve(type.path,path),path,['data']);
  return owner?{...type,path:['crate',...owner.fullName.split('.')]}:type;
 }
 if(type.kind==='array')return{...type,element:sourceType(parser,type.element,path)};
 if(type.kind==='tuple')return{...type,elements:type.elements.map(element=>sourceType(parser,element,path))};
 return type;
}
function nominal(parser,segments,path){
 const found=parser.origins.lookup(parser.resolve(segments,path),path);
 return found?parser.namedRecords.get(found.fullName):null;
}
function braceOwner(parser,segments,path){
 segments=resolveRustTypeAliasPath(parser,segments,path,'braced record');
 const record=nominal(parser,segments,path);
 if(record)return record;
 const found=parser.origins.lookup(parser.resolve(segments,path),path,['ctor']);
 return found?{owner:found.owner,constructor:found.fullName,names:found.fields.map((_,index)=>String(index)),syntax:found.fields.length?'tuple':'unit',update:false}:null;
}

export function readRustRecordItem(parser,path){
 const c=parser.cursor,start=c.next(),token=c.identifier('struct'),name=token.value;
 if(!acceptRustRecordSourceName(name))throw unsupported('Rust record type owner','a builtin-shadowing record needs its complete namespace interpretation',span(token,token));
 if(c.is('<')||c.is('where'))throw unsupported('generic Rust record','record parameters and predicates need their complete ownership interpretation',span(start,c.peek()));
 const fields=[],origins=[],names=[];
 let syntax='unit';
 if(c.eat('{')){
  syntax='named';
  while(!c.is('}')){
   parser.attributes();parser.visibility();const field=c.identifier('record field');c.expect(':','record field');
   names.push(field.value);fields.push({name:field.value,type:parser.type()});origins.push(parser.origins.type);
   if(!c.eat(','))break;
  }
  c.expect('}','struct');
 }else if(c.eat('(')){
  syntax='tuple';
  while(!c.is(')')){
   parser.attributes();parser.visibility();names.push(String(fields.length));fields.push({type:parser.type()});origins.push(parser.origins.type);
   if(!c.eat(','))break;
  }
  c.expect(')','struct');c.expect(';','struct');
 }else c.expect(';','struct');
 const order=planRustRecordFields(names,names);
 if(order[0]<0)throw typeError('a Rust record declares a duplicate field',span(start,c.peek()));
 const fullName=[...path,name].join('.');
 const constructorName=readRustRecordConstructorName(name,parser.inherentMethods.has(fullName+'.'+name),[...parser.reservedNames]);
 const constructor=fullName+'.'+constructorName;
 parser.namedRecords.set(fullName,{owner:fullName,constructor,names,syntax,update:true});
 parser.origins.declaration(name,path,{k:'data',unitFields:fields.length===0});
 parser.origins.declaration(constructorName,[...path,name],{k:'ctor',owner:fullName,fields:origins,
  result:{wrappers:[],type:{kind:'named',path:['crate',...path,name]},path:[]}});
 return{k:'data',name,ctors:[{name:constructorName,fields}],span:span(start,c.peek())};
}

export function prepareRustRecordMethodNamespace(parser,owner,method,where){
 const key=[...owner,method].join('.'),existing=parser.origins.catalog.get(key);
 if(existing?.k!=='ctor')return;
 const record=parser.namedRecords.get(owner.join('.'));
 if(!record||!record.update)throw unsupported('Rust associated item namespace','an enum constructor homograph needs its complete associated-item resolution',where);
 const constructorName=readRustRecordConstructorName(owner.at(-1),true,[...parser.reservedNames]);
 const next=[...owner,constructorName].join('.');
 if(parser.origins.collecting){parser.origins.catalog.delete(key);parser.origins.catalog.set(next,existing);}
 record.constructor=next;
}

export function readRustRecordConstruction(parser,segments,path,start){
 const c=parser.cursor,record=braceOwner(parser,segments,path),entries=[];
 let base=null;
 c.expect('{','record construction');
 while(!c.is('}')){
  if(c.eat('..')){base=parser.expr(path);break;}
  parser.attributes();
  const numeric=c.peek().kind==='number'&&/^(0|[1-9][0-9]*)$/u.test(c.peek().raw);
  const field=numeric?c.next():c.identifier('record field'),name=numeric?field.raw:field.value;
  const colon=Boolean(c.eat(':'));
  if(numeric&&!colon)c.expect(':','tuple record field');
  const value=colon?parser.expr(path):{k:'name',path:[field.value],span:span(field,field)};
  entries.push({name,value});if(!c.eat(','))break;
 }
 c.expect('}','record construction');
 if(!record){
  if(!parser.origins.collecting)throw unsupported('Rust record constructor','the complete source must own its nominal record declaration',span(start,c.peek()));
  return{k:'app',fn:{k:'name',path:[...parser.resolve(segments,path),segments.at(-1)]},args:entries.map(entry=>entry.value),span:span(start,c.peek())};
 }
 if(base)return readRecordUpdate(parser,record,entries,base,path,span(start,c.peek()));
 const plan=planRustRecordFields(record.names,entries.map(entry=>entry.name));
 if(plan[0]<0)throw typeError('a Rust record construction has duplicate, unknown or missing fields',span(start,c.peek()));
 const bindings=recordNames(parser,entries.length),where=span(start,c.peek());
 parser.origins.call(record.constructor.split('.'),plan.map(index=>entries[index].value),[],where);
 const constructor={k:'name',path:['crate',...record.constructor.split('.')],span:where};
 let body=plan.length?{k:'app',fn:constructor,args:plan.map(index=>({k:'name',path:[bindings[index]],span:where})),span:where}:constructor;
 const result=parser.origins.lookup(record.constructor.split('.'),[],['ctor']);
 for(let index=entries.length-1;index>=0;index-=1){
  const origin=result?.fields[plan.indexOf(index)],type=origin?sourceType(parser,origin.type,origin.path):undefined;
  body={k:'let',name:bindings[index],...(type?{type}:{}),value:entries[index].value,body,span:where};
 }
 if(result)parser.origins.mark(body,result.result);
 return body;
}

export function readRustRecordProjection(parser,object,field,path,where){
 const origin=parser.origins.origin(object,path);
 const owner=origin.type?.kind==='named'?parser.origins.lookup(origin.type.path,origin.path,['data']):null;
 const record=owner?parser.namedRecords.get(owner.fullName):null;
 if(!record)return parser.origins.collecting&&parser.hasRecordDeclarations?{k:'field',object,field,span:where}:null;
 const index=readRustRecordFieldPosition(record.names,field);
 if(index<0)throw typeError('a Rust record has no field '+field,where);
 const node=recordProjectionNode(parser,record,object,index,where);
 const constructor=parser.origins.lookup(record.constructor.split('.'),[],['ctor']);
 if(constructor?.fields[index])parser.origins.mark(node,constructor.fields[index]);
 return node;
}

function recordProjectionNode(parser,record,object,index,where){
 const binds=recordNames(parser,record.names.length);
 return{k:'match',scrutinees:[object],rows:[{patterns:[{k:'ctor',path:['crate',...record.constructor.split('.')],args:binds.map(name=>({k:'bind',name}))}],
  body:{k:'name',path:[binds[index]],span:where}}],span:where};
}

function readRecordUpdate(parser,record,entries,base,path,where){
 const plan=planRustRecordUpdateFields(record.names,entries.map(entry=>entry.name));
 if(plan[0]<0)throw typeError('a Rust record update has duplicate or unknown fields',where);
 const origin=parser.origins.origin(base,path);
 const owner=origin.type?.kind==='named'?parser.origins.lookup(origin.type.path,origin.path,['data']):null;
 const admitted=acceptRustRecordUpdateOwner(record.update,Boolean(owner),owner?.fullName??'',record.owner,origin.wrappers.length,Boolean(origin.diverging));
 if(!admitted&&(!parser.origins.collecting||origin.type||origin.proven||!record.update))throw unsupported('Rust record update owner','an update needs the same owned source struct; borrowed and enum bases need another interpretation',where);
 const bindings=recordNames(parser,entries.length+1+record.names.length),baseName=bindings[entries.length],constructor=parser.origins.lookup(record.constructor.split('.'),[],['ctor']);
 const positions=plan.slice(1),baseReference={k:'name',path:[baseName],span:where};
 const args=positions.map((position,index)=>({k:'name',path:[bindings[position<0?entries.length+1+index:position]],span:where}));
 const fn={k:'name',path:['crate',...record.constructor.split('.')],span:where};
 let body=args.length?{k:'app',fn,args,span:where}:fn;
 body={k:'match',scrutinees:[baseReference],rows:[{patterns:[{k:'ctor',path:fn.path,args:positions.map((position,index)=>position<0?{k:'bind',name:bindings[entries.length+1+index]}:{k:'wild'})}],body}],span:where};
 body={k:'let',name:baseName,type:{kind:'named',path:['crate',...record.owner.split('.')]},value:base,body,span:where};
 for(let index=entries.length-1;index>=0;index-=1){
  const field=constructor?.fields[positions.indexOf(index)],type=field?sourceType(parser,field.type,field.path):undefined;
  if(field)parser.origins.argument(parser.origins.origin(entries[index].value,path),field,where);
  body={k:'let',name:bindings[index],...(type?{type}:{}),value:entries[index].value,body,span:where};
 }
 if(constructor)parser.origins.mark(body,constructor.result);
 return body;
}

export function readRustRecordValuePath(parser,segments,path,called){
 segments=resolveRustTypeAliasPath(parser,segments,path,'associated value');
 if(segments.length===1&&parser.origins.scope.has(segments[0]))return null;
 const record=nominal(parser,segments,path);
 return record&&(called?record.syntax==='tuple':record.syntax==='unit')?['crate',...record.constructor.split('.')]:null;
}

export function readRustNamedVariantFields(parser,path,name,variant){
 const c=parser.cursor,fields=[],origins=[],names=[];
 c.expect('{','record variant');
 while(!c.is('}')){
  parser.attributes();const field=c.identifier('record variant field');c.expect(':','record variant field');
  names.push(field.value);fields.push({name:field.value,type:parser.type()});origins.push(parser.origins.type);
  if(!c.eat(','))break;
 }
 c.expect('}','record variant');
 if(planRustRecordFields(names,names)[0]<0)throw typeError('a Rust record declares a duplicate field',span(variant,c.peek()));
 const constructor=[...path,name,variant.value].join('.');
 parser.namedRecords.set(constructor,{owner:[...path,name].join('.'),constructor,names,syntax:'named',update:false});
 return{fields,origins};
}

export function readRustRecordPattern(parser,segments,path,aliases,start){
 const c=parser.cursor,record=braceOwner(parser,segments,path),names=[],patterns=[];
 let rest=null;
 c.expect('{','record pattern');
 while(!c.is('}')){
  if(c.is('..')){rest=c.next();break;}
  parser.attributes();
  const numeric=c.peek().kind==='number'&&/^(0|[1-9][0-9]*)$/u.test(c.peek().raw);
  const field=numeric?c.next():c.identifier('record pattern field');
  let pattern;
  if(c.eat(':')){
   parser.recordPatternDepth+=1;
   try{pattern=parser.pattern(path,aliases);}finally{parser.recordPatternDepth-=1;}
  }else{
   if(numeric)c.expect(':','tuple record pattern field');
   pattern={k:'bind',name:field.value,span:span(field,field)};
  }
  names.push(numeric?field.raw:field.value);patterns.push(pattern);
  if(!c.eat(','))break;
 }
 c.expect('}','record pattern');
 const where=span(start,c.peek());
 if(!record){
  if(!parser.origins.collecting)throw unsupported('Rust record pattern','the complete source must own its nominal constructor',where);
  return{k:'ctor',path:[...parser.resolve(segments,path),segments.at(-1)],args:patterns,span:where};
 }
 const plan=planRustRecordPatternFields(record.names,names,Boolean(rest));
 if(plan[0]<0)throw typeError('a Rust record pattern has duplicate, unknown or missing fields',where);
 return{k:'ctor',path:['crate',...record.constructor.split('.')],
  args:plan.slice(1).map(index=>index<0?{k:'wild',span:{start:rest.start,end:rest.end}}:patterns[index]),span:where};
}

export function readRustNominalBindingPattern(parser){
 const c=parser.cursor;
 if(c.peek().kind!=='identifier')return null;
 const candidate=parser.origins.lookup(parser.resolve([c.peek().value],parser.modulePath),parser.modulePath);
 if(!['::','(','{'].includes(c.peek(1).value)&&candidate?.k!=='ctor'&&candidate?.k!=='data')return null;
 const aliases=[];parser.recordPatternDepth+=1;
 let pattern;
 try{pattern=parser.pattern(parser.modulePath,aliases);}finally{parser.recordPatternDepth-=1;}
 if(aliases.length)throw unsupported('Rust nominal binding alias','whole-value aliases require their complete move and borrow interpretation',pattern.span);
 return{k:'nominal',pattern,span:pattern.span};
}

export function rustNominalBindingNames(parser,pattern,validate=false){
 const names=[];
 const visit=node=>{
  const entry=node.k==='ctor'?parser.origins.lookup(node.path,parser.modulePath,['ctor']):node.k==='bindOrCtor'?parser.origins.lookup([node.name],parser.modulePath,['ctor']):null;
  const kind=entry?'ctor':node.k==='bindOrCtor'?'bind':node.k;
  const variants=entry?[...parser.origins.catalog.values()].filter(item=>item.k==='ctor'&&item.owner===entry.owner).length:0;
  if(validate&&!parser.origins.collecting&&!acceptRustIrrefutablePatternNode(kind,Boolean(entry),variants,node.args?.length??0,entry?.fields.length??0))throw unsupported('refutable product binding','only immutable names, ignored fields and anonymous tuple products are admitted',node.span);
  if(kind==='bind')names.push({name:node.name,span:node.span});
  for(const item of node.args??[])visit(item);
 };
 visit(pattern);return names;
}

export function lowerRustNominalBinding(parser,pattern,value,path,bindings,fallback=null){
 const names=rustNominalBindingNames(parser,pattern,fallback===null),scope=parser.origins.cases([pattern],value,path);
 const members=names.map(item=>({k:'name',path:[item.name],span:item.span}));
 const body=members.length===1?members[0]:members.length?{k:'tuple',items:members,span:pattern.span}:{k:'unit',span:pattern.span};
 const rows=[{patterns:[pattern],body}];
 if(fallback!==null)rows.push({patterns:[{k:'wild'}],body:fallback});
 const matched={k:'match',scrutinees:[value],rows,span:pattern.span};
 const bundle=parser.freshProductBinding();
 bindings.push({name:bundle,value:matched,span:pattern.span});
 for(const[index,item]of names.entries()){
  const object={k:'name',path:[bundle],span:item.span};
  const projection=members.length===1?object:{k:'tupleIndex',object,index,span:item.span};
  const origin=scope.get(item.name);
  if(origin)parser.origins.mark(projection,origin);
  parser.origins.bind(item.name,projection,path);
  bindings.push({name:item.name,value:projection,span:item.span});
 }
}

export function readRustLetElsePattern(parser,path){
 const c=parser.cursor;let depth=0,initializer=false,found=false;
 for(let offset=0;c.peek(offset).kind!=='eof';offset+=1){
  const token=c.peek(offset);
  if(depth===0&&token.kind==='punct'&&token.value===';')break;
  if(depth===0&&token.kind==='punct'&&token.value==='=')initializer=true;
  if(initializer&&depth===0&&token.kind==='identifier'&&c.is('else',offset)&&c.peek(offset-1).value!=='}'){found=true;break;}
  if(token.kind==='punct'&&['(','[','{'].includes(token.value))depth+=1;
  else if(token.kind==='punct'&&[')',']','}'].includes(token.value)){if(depth===0)break;depth-=1;}
 }
 if(!found)return null;
 const aliases=[],pattern=parser.pattern(path,aliases);
 if(aliases.length)throw unsupported('Rust nominal binding alias','whole-value aliases require their complete move and borrow interpretation',pattern.span);
 return{k:'nominal',pattern,span:pattern.span};
}

export function expandRustConstructorRest(parser,resolved,args,rest,path){
 if(!rest)return args;
 const entry=parser.origins.lookup(resolved,path,['ctor']);
 if(!entry){
  if(parser.origins.collecting)return args;
  throw unsupported('Rust constructor rest pattern','the constructor needs its actual source field count',{start:rest.token.start,end:rest.token.end});
 }
 const plan=planTuplePatternFields(entry.fields.length,args.length,rest.index);
 if(plan[0]<0)throw typeError('a Rust constructor rest pattern has too many fields',{start:rest.token.start,end:rest.token.end});
 return plan.slice(1).map(index=>index<0?{k:'wild',span:{start:rest.token.start,end:rest.token.end}}:args[index]);
}
