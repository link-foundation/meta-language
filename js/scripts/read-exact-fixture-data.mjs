// Explicit successor authoring profile. This is a bounded syntax interpreter
// for data construction, NOT a JavaScript evaluator. It imports no fixture,
// calls no represented function and computes no policy result or oracle.
import {createHash} from 'node:crypto';
import {tokenize} from '../src/translation/lexer.js';
const profileName='exact pure fixture data 2';
const bound=(value,maximum,message)=>{if(value>maximum)throw new RangeError(message);};
const identifier=value=>typeof value==='string'&&/^[A-Za-z_$][A-Za-z_0-9$]*$/u.test(value);
const bindingIdentifier=value=>identifier(value)&&!['true','false','null','this','super','new','return','function','class','const','let','var','import','export','default','extends','await','yield','if','else','throw','try','catch','delete','typeof','void','in','instanceof','switch','case','break','continue','do','while','for','with','debugger','finally'].includes(value);
const tokenText=token=>{if(!token.stringUnits)return token.value;let value='';for(const unit of token.stringUnits)value+=String.fromCharCode(unit);return value;};
const indexKey=key=>/^(?:0|[1-9][0-9]*)$/u.test(key)&&Number(key)<4294967295?Number(key):-1;

export function assertExactFixtureAssertionTree(values){
 const seen=new Set();let visits=0;
 function walk(value,depth){bound(++visits,131072,'fixture alias inspection exceeds its node budget');bound(depth,64,'fixture alias inspection exceeds its depth');if(value.$!=='fixtureArray'&&value.$!=='fixtureRecord')return;if(seen.has(value))throw new TypeError('native exact fixture assertions require distinct aggregate identities; shared input aliases need explicit const-DAG lowering');seen.add(value);if(value.$==='fixtureArray')for(const child of value.items)walk(child,depth+1);else for(const entry of value.entries)walk(entry.value,depth+1);}
 for(const value of values)walk(value,0);
}

export function renderExactFixtureData(value,{profile='distinct aggregate assertion tree 2'}={}){
 if(profile==='distinct aggregate assertion tree 2')assertExactFixtureAssertionTree([value]);
 else if(profile!=='structural descriptor display 1')throw new TypeError('unknown exact fixture rendering profile');
 let visits=0,units=0;
 function render(node,depth){bound(++visits,131072,'fixture expansion exceeds its node budget');bound(depth,64,'fixture expansion exceeds its depth');let source;
  switch(node.$){
   case 'fixtureUndefined':source='undefined';break;
   case 'fixtureNull':source='null';break;
   case 'fixtureBoolean':source=node.value?'true':'false';break;
   case 'fixtureNumber':source=node.source;break;
   case 'fixtureInteger':source=node.digits+'n';break;
   case 'fixtureText':source=JSON.stringify(node.value);break;
   case 'fixtureArray':source='['+node.items.map(item=>render(item,depth+1)).join(',')+']';break;
   case 'fixtureRecord':source='{'+node.entries.map(entry=>JSON.stringify(entry.key)+':'+render(entry.value,depth+1)).join(',')+'}';break;
   default:throw new TypeError('only exact data descriptors can leave the fixture interpreter');
  }
  units+=source.length;bound(units,8*1024*1024,'fixture expansion exceeds 8 MiB');return source;
 }
 return render(value,0);
}

/** The descriptor contains no raw BigInt/undefined, so ordinary Links data
 * preserves both without JSON null coercion. Complete source and dependency
 * ranges retain every author input separately from the expanded assertion. */
export function readExactFixtureExport(source,{path,exportName,profile}={}){
 if(profile!==profileName||typeof source!=='string'||typeof path!=='string'||!identifier(exportName))throw new TypeError('exact fixture data requires its explicit profile and actual named export');
 bound(Buffer.byteLength(source),4*1024*1024,'exact fixture source exceeds 4 MiB');
 const tokens=tokenize(source,'JavaScript').tokens;bound(tokens.length,262144,'exact fixture token budget exceeded');
 const actual=(at,value)=>tokens[at]?.value===value&&tokens[at]?.raw===value;
 const declarations=new Map(),cache=new Map(),active=new Set(),dependencies=new Set(),derivations=[],origins=new WeakMap();let nodes=0,operations=0;
 function groupEnd(first,end){const opening=tokens[first]?.value,closing={'(':')','[':']','{':'}'}[opening];if(!closing)throw new TypeError('a complete fixture syntax group is required');const stack=[closing];for(let at=first+1;at<end;at++){if(tokens[at].kind!=='punct')continue;const value=tokens[at].value;if(['(','[','{'].includes(value))stack.push({'(':')','[':']','{':'}'}[value]);else if([')',']','}'].includes(value)){if(stack.pop()!==value)throw new TypeError('fixture syntax groups must match');if(!stack.length)return at;}}throw new TypeError('fixture syntax groups must close');}
 for(let at=0;at<tokens.length&&tokens[at].kind!=='eof';){
  const first=at,exported=actual(at,'export');if(exported)at++;
  if(!actual(at,'const')||tokens[at+1]?.kind!=='identifier'||!actual(at+2,'='))throw new TypeError('exact fixture modules admit only actual const data/constructor declarations');
  const name=tokens[at+1].value,start=at+3;if(!bindingIdentifier(name))throw new TypeError('fixture const names require actual ordinary binding identifiers');let end=start;
  for(;end<tokens.length&&tokens[end].kind!=='eof'&&!actual(end,';');end++)if(tokens[end].kind==='punct'&&['(','[','{'].includes(tokens[end].value))end=groupEnd(end,tokens.length);
  if(!actual(end,';')||declarations.has(name))throw new TypeError('exact fixture declarations need unique names and complete semicolon boundaries');
  declarations.set(name,{name,start,end,first,exported});bound(declarations.size,4096,'exact fixture declaration budget exceeded');at=end+1;
 }
 const selected=declarations.get(exportName);if(!selected?.exported)throw new TypeError('the exact fixture export must be an actual exported const declaration');
 const scalar=node=>['fixtureUndefined','fixtureNull','fixtureBoolean','fixtureNumber','fixtureInteger','fixtureText'].includes(node.$);
 function primitive(node){switch(node.$){case 'fixtureUndefined':return undefined;case 'fixtureNull':return null;case 'fixtureBoolean':return node.value;case 'fixtureText':return node.value;case 'fixtureInteger':return BigInt(node.digits);case 'fixtureNumber':return node.source==='NaN'?NaN:node.source==='Infinity'?Infinity:node.source==='-Infinity'?-Infinity:Number(node.source.replaceAll('_',''));default:throw new TypeError('fixture comparisons require actual scalar data');}}
 function parse(first,end,environment=new Map(),mode='data',depth=0){
  let at=first;const take=value=>actual(at,value)?(at++,true):false,need=value=>{if(!take(value))throw new TypeError('exact fixture requires '+value+' at '+(tokens[at]?.start??source.length));};
  function evaluate(nesting){
   bound(++nodes,131072,'exact fixture value budget exceeded');bound(nesting,64,'exact fixture value depth exceeded');
   const start=at,token=tokens[at];if(!token||at>=end)throw new TypeError('an exact fixture value is required');let result;
   if(take('[')){const items=[];while(!take(']')){if(actual(at,','))throw new TypeError('sparse fixture arrays require a separate exact descriptor');if(take('...')){const spread=evaluate(nesting+1);if(spread.$!=='fixtureArray')throw new TypeError('fixture array spread requires actual array data');items.push(...spread.items);}else items.push(evaluate(nesting+1));bound(items.length,4096,'fixture array length budget exceeded');if(take(']'))break;need(',');}result={$:'fixtureArray',items};}
   else if(take('{')){const entries=[];function put(key,value){if(key==='__proto__')throw new TypeError('fixture prototype setter keys are not data constructors');const prior=entries.findIndex(entry=>entry.key===key);if(prior<0)entries.push({$:'fixtureEntry',key,value});else entries[prior]={$:'fixtureEntry',key,value};bound(entries.length,4096,'fixture record field budget exceeded');}
    while(!take('}')){if(take('...')){const spread=evaluate(nesting+1);if(spread.$!=='fixtureRecord')throw new TypeError('fixture record spread requires actual plain record data');for(const entry of spread.entries)put(entry.key,entry.value);}else{const key=tokens[at++];if(!key||!['identifier','string'].includes(key.kind))throw new TypeError('fixture records require fixed ordinary data keys');let value;if(take(':'))value=evaluate(nesting+1);else{if(key.kind!=='identifier')throw new TypeError('only actual identifiers can be fixture shorthand fields');value=reference(key.value,key.start,nesting+1);}put(tokenText(key),value);}if(take('}'))break;need(',');}result={$:'fixtureRecord',entries:entries.toSorted((left,right)=>{const a=indexKey(left.key),b=indexKey(right.key);return a>=0&&b>=0?a-b:a>=0?-1:b>=0?1:0;})};}
   else if(take('(')){const close=groupEnd(start,end);if(actual(close+1,'=>')){result=arrow(start,close,close+2,nesting);at=end;}else{result=evaluate(nesting+1);need(')');}}
   else if(token.kind==='identifier'&&actual(at+1,'=>')){result=arrow(at-1,at,at+2,nesting);at=end;}
   else if(token.kind==='string'){at++;let value=token.value;if(token.stringUnits){value='';for(const unit of token.stringUnits)value+=String.fromCharCode(unit);}result={$:'fixtureText',value};}
   else if(token.kind==='template'){at++;if(!Array.isArray(token.parts)||token.parts.some(part=>part.expression))throw new TypeError('fixture templates must contain literal text only');let value='';for(const part of token.parts){if(part.stringUnits)for(const unit of part.stringUnits)value+=String.fromCharCode(unit);else value+=part.text;}result={$:'fixtureText',value};}
   else if(token.kind==='number'||actual(at,'-')){let sign='';if(take('-'))sign='-';const number=tokens[at++];if(!number||number.kind!=='number'&&number.raw!=='Infinity')throw new TypeError('a signed fixture literal needs its actual numeric token');const raw=sign+number.raw.replaceAll('_','');if(raw.endsWith('n')){bound(raw.length,4096,'fixture BigInt literal exceeds its digit budget');const magnitude=BigInt(number.raw.replaceAll('_','').slice(0,-1));result={$:'fixtureInteger',digits:(sign==='-'?-magnitude:magnitude).toString()};}else{if(number.kind==='number'&&!Number.isFinite(Number(raw)))throw new TypeError('overflowed Number fixture literals require explicit Infinity');result={$:'fixtureNumber',source:raw};}}
   else if(['true','false','null','undefined','NaN','Infinity'].some(word=>actual(at,word))&&!environment.has(token.value)&&!declarations.has(token.value)){at++;result=token.value==='undefined'?{$:'fixtureUndefined'}:token.value==='null'?{$:'fixtureNull'}:['true','false'].includes(token.value)?{$:'fixtureBoolean',value:token.value==='true'}:{$:'fixtureNumber',source:token.value};}
   else if(token.kind==='identifier'){at++;result=reference(token.value,token.start,nesting+1);}
   else throw new TypeError('dynamic fixture syntax is outside the pure data profile at '+token.start);
   while(at<end){
    if(take('.')){const key=tokens[at++];if(key?.kind!=='identifier')throw new TypeError('fixture projection requires its actual fixed field');
     if(actual(at,'(')){
      if(result.$!=='fixtureArray'||!['slice','filter'].includes(key.value))throw new TypeError('only bounded actual array slice/filter build fixture data');const open=at,close=groupEnd(open,end);at++;
      if(key.value==='slice'){
       const arguments_=[];while(at<close){arguments_.push(evaluate(nesting+1));if(at===close)break;need(',');}if(arguments_.length>2)throw new TypeError('fixture slice uses only its actual begin/end');const indexes=arguments_.map(value=>{if(value.$!=='fixtureNumber'||!Number.isSafeInteger(primitive(value)))throw new TypeError('fixture slice indexes must be actual safe integer data');return primitive(value);});result={$:'fixtureArray',items:result.items.slice(...indexes)};
      }else{
       const callback=parse(at,close,environment,'predicate',nesting+1).value;if(callback.$!=='fixtureConstructor'||callback.parameters.length!==1)throw new TypeError('fixture filter requires its single pure local selector');const items=[];for(const item of result.items){const selected=invoke(callback,[item],nesting+1,'predicate');if(selected.$!=='fixtureBoolean')throw new TypeError('fixture selectors must return exact Boolean comparisons');if(selected.value)items.push(item);}result={$:'fixtureArray',items};at=close;
      }
      need(')');operation(key.value,start,at);
     }else{if(result.$!=='fixtureRecord')throw new TypeError('fixture projection requires actual record data');result=result.entries.find(entry=>entry.key===key.value)?.value??{$:'fixtureUndefined'};}
    }else if(take('(')){
     if(result.$!=='fixtureConstructor'||mode==='predicate')throw new TypeError('only local data-constructor syntax may be called by a fixture');const arguments_=[];while(!take(')')){arguments_.push(evaluate(nesting+1));if(take(')'))break;need(',');}bound(arguments_.length,64,'fixture constructor argument budget exceeded');result=invoke(result,arguments_,nesting+1,'data');if(!['fixtureArray','fixtureRecord'].includes(result.$))throw new TypeError('fixture constructors must return data aggregates, never computed scalar oracles');operation('constructor',start,at);
    }else break;
   }
   if(mode==='predicate'&&['===','!=='].some(value=>actual(at,value))){const negate=take('!==');if(!negate)need('===');const parsedRight=parse(at,end,environment,'data',nesting+1),right=parsedRight.value;at=parsedRight.next;if(!scalar(result)||!scalar(right))throw new TypeError('fixture selectors compare only actual scalar data');result={$:'fixtureBoolean',value:negate?primitive(result)!==primitive(right):primitive(result)===primitive(right)};}
   if(result.$!=='fixtureConstructor')origins.set(result,{start:tokens[start].start,end:tokens[at-1].end,coordinateUnit:'UTF-16 code units'});return result;
  }
  function reference(name,position,nesting){if(environment.has(name)){const value=environment.get(name);if(value.$==='fixtureUninitialized')throw new TypeError('fixture defaults cannot read uninitialized parameter owners');return value;}const declaration=declarations.get(name);if(!declaration||tokens[declaration.start].start>=position)throw new TypeError('fixture references require an actual earlier initialized local data owner');return binding(name,nesting);}
  function arrow(open,close,body,nesting){
   const parameters=[];let index=open+1;
   // The single unparenthesized parameter uses open=parameter-1.
   const single=open>=-1&&tokens[close]?.kind==='identifier'&&actual(close+1,'=>');if(single)index=close;
   while(index<(single?close+1:close)){
    const name=tokens[index++];if(name?.kind!=='identifier'||!bindingIdentifier(name.value)||parameters.some(parameter=>parameter.name===name.value))throw new TypeError('fixture constructors need distinct fixed parameter names');let defaultFirst=-1,defaultEnd=-1;
    if(actual(index,'=')){defaultFirst=++index;for(;index<close&&!actual(index,',');index++)if(tokens[index].kind==='punct'&&['(','[','{'].includes(tokens[index].value))index=groupEnd(index,close);defaultEnd=index;}
    parameters.push({name:name.value,defaultFirst,defaultEnd});if(!single&&index<close){if(!actual(index,','))throw new TypeError('fixture parameters need exact separators');index++;}
   }
   if(tokens[body]?.value==='{')throw new TypeError('fixture arrows require expression bodies; statements and policy execution are forbidden');
   return {$:'fixtureConstructor',parameters,first:body,end,environment:new Map(environment),mode};
  }
  const value=evaluate(depth);return {value,next:at};
 }
 function operation(kind,first,end){bound(++operations,131072,'pure fixture construction operation budget exceeded');if(derivations.length<4096)derivations.push({kind,start:tokens[first].start,end:tokens[end-1].end,coordinateUnit:'UTF-16 code units'});else throw new RangeError('fixture derivation receipt budget exceeded');}
 function invoke(constructor,args,depth,mode){
  const environment=new Map(constructor.environment);for(const parameter of constructor.parameters)environment.set(parameter.name,{$:'fixtureUninitialized'});for(let at=0;at<constructor.parameters.length;at++){const parameter=constructor.parameters[at];let value=args[at]??{$:'fixtureUndefined'};if(value.$==='fixtureUndefined'&&parameter.defaultFirst>=0){const result=parse(parameter.defaultFirst,parameter.defaultEnd,environment,'data',depth+1);if(result.next!==parameter.defaultEnd)throw new TypeError('fixture defaults must be complete literal data');value=result.value;}environment.set(parameter.name,value);}
  const result=parse(constructor.first,constructor.end,environment,mode,depth+1);if(result.next!==constructor.end)throw new TypeError('fixture constructors/selectors must contain one complete allowed expression');return result.value;
 }
 function binding(name,depth){
  if(cache.has(name))return cache.get(name);if(active.has(name))throw new TypeError('fixture const construction must be acyclic');active.add(name);dependencies.add(name);
  try{const declaration=declarations.get(name),result=parse(declaration.start,declaration.end,new Map(),'data',depth);if(result.next!==declaration.end)throw new TypeError('unconsumed fixture initializer syntax is not data');cache.set(name,result.value);return result.value;}finally{active.delete(name);}
 }
 const descriptor=binding(exportName,0);
 // Other top-level const initializers also execute in the original module.
 // Reject an unselected dynamic initializer rather than silently overlooking
 // its potential mutation of selected data. Functions themselves stay syntax.
 for(const name of declarations.keys())binding(name,0);
 const renderedSource=renderExactFixtureData(descriptor,{profile:'structural descriptor display 1'});
 const references=[...dependencies].map(name=>{const row=declarations.get(name);return {name,start:tokens[row.start].start,end:tokens[row.end-1].end,source:source.slice(tokens[row.start].start,tokens[row.end-1].end),coordinateUnit:'UTF-16 code units'};});
 return {schemaVersion:2,model:profileName,profile,path,exportName,source,bytes:Buffer.byteLength(source),sourceSha256:createHash('sha256').update(source).digest('hex'),descriptor,renderedSource,renderedSourceProfile:'structural descriptor display 1',exportRange:{start:tokens[selected.start].start,end:tokens[selected.end-1].end,coordinateUnit:'UTF-16 code units'},constantReferences:references,derivations};
}

export function readExactFixturePolicyControls(source,options){
 const capture=readExactFixtureExport(source,options);if(capture.descriptor.$!=='fixtureArray'||capture.descriptor.items.length>4096)throw new TypeError('exact policy controls require a bounded actual array export');
 const receipt=value=>({kind:value.$.slice(7).toLowerCase(),source:renderExactFixtureData(value),originalSource:capture.source.slice(capture.exportRange.start,capture.exportRange.end),constantReferences:capture.constantReferences});
 const controls=capture.descriptor.items.map((row,index)=>{
  if(row.$!=='fixtureRecord')throw new TypeError('exact policy rows require actual records');const fields=new Map(row.entries.map(entry=>[entry.key,entry.value])),operation=fields.get('operation'),args=fields.get('arguments')??fields.get('args');
  if(operation?.$!=='fixtureText'||!identifier(operation.value)||args?.$!=='fixtureArray'||!fields.has('expected')||fields.has('arguments')&&fields.has('args'))throw new TypeError('exact policy rows require one operation, one argument array and an independent expected value');
  assertExactFixtureAssertionTree([args,fields.get('expected')]);
  return {fixture:capture.path,exportName:capture.exportName,index,operation:operation.value,argumentField:fields.has('arguments')?'arguments':'args',args:args.items.map(receipt),expected:receipt(fields.get('expected')),authoredRow:receipt(row)};
 });
 return {path:capture.path,exportName:capture.exportName,source:capture.source,bytes:capture.bytes,sourceSha256:capture.sourceSha256,profile:capture.profile,exportCapture:capture,controls};
}
