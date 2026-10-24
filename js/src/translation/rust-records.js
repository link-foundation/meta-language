// Token and source-origin transport. Shared plans decide field order/bindings;
// ordinary constructors and exhaustive matches retain all checked value owners.
import{unsupported,typeError}from'./diagnostics.js';
import{planRustRecordFields,readRustRecordFieldPosition,readRustRecordBindingNames,acceptRustRecordSourceName}from'./rust-record-rules.js';

function span(from,to){return{start:from.start,end:Math.max(from.end,to&&to.kind!=='eof'?to.start:from.end)};}

function recordNames(parser,count){return readRustRecordBindingNames([...parser.reservedNames],count);}
function nominal(parser,segments,path){
 const found=parser.origins.lookup(parser.resolve(segments,path),path);
 return found?parser.namedRecords.get(found.fullName):null;
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
 const fullName=[...path,name].join('.'),constructor=fullName+'.'+name;
 parser.namedRecords.set(fullName,{owner:fullName,constructor,names,syntax});
 parser.origins.declaration(name,path,{k:'data',unitFields:fields.length===0});
 parser.origins.declaration(name,[...path,name],{k:'ctor',owner:fullName,fields:origins,
  result:{wrappers:[],type:{kind:'named',path:['crate',...path,name]},path:[]}});
 return{k:'data',name,ctors:[{name,fields}],span:span(start,c.peek())};
}

export function readRustRecordConstruction(parser,segments,path,start){
 const c=parser.cursor,record=nominal(parser,segments,path),entries=[];
 c.expect('{','record construction');
 while(!c.is('}')){
  if(c.is('..'))throw unsupported('Rust record update','update syntax needs complete move and field provenance',span(start,c.peek()));
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
 const plan=planRustRecordFields(record.names,entries.map(entry=>entry.name));
 if(plan[0]<0)throw typeError('a Rust record construction has duplicate, unknown or missing fields',span(start,c.peek()));
 const bindings=recordNames(parser,entries.length),where=span(start,c.peek());
 parser.origins.call(record.constructor.split('.'),plan.map(index=>entries[index].value),[],where);
 const constructor={k:'name',path:['crate',...record.constructor.split('.')],span:where};
 let body=plan.length?{k:'app',fn:constructor,args:plan.map(index=>({k:'name',path:[bindings[index]],span:where})),span:where}:constructor;
 for(let index=entries.length-1;index>=0;index-=1)body={k:'let',name:bindings[index],value:entries[index].value,body,span:where};
 const result=parser.origins.lookup(record.constructor.split('.'),[],['ctor']);
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
 const binds=recordNames(parser,record.names.length);
 const node={k:'match',scrutinees:[object],rows:[{patterns:[{k:'ctor',path:['crate',...record.constructor.split('.')],args:binds.map(name=>({k:'bind',name}))}],
  body:{k:'name',path:[binds[index]],span:where}}],span:where};
 const constructor=parser.origins.lookup(record.constructor.split('.'),[],['ctor']);
 if(constructor?.fields[index])parser.origins.mark(node,constructor.fields[index]);
 return node;
}

export function readRustRecordValuePath(parser,segments,path,called){
 const record=nominal(parser,segments,path);
 return record&&(called?record.syntax==='tuple':record.syntax==='unit')?['crate',...record.constructor.split('.')]:null;
}
