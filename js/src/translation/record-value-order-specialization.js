// Specialize the one declared opaque payload slot, then use the ordinary
// checker. The wrapper is retained: nullish payloads cannot become missing slots.
import {readFileSync} from 'node:fs';
import {readStructuredDataLino} from '../structured-data-links.js';
import {checkProgram} from './check.js';
import {FLOAT,STRING,array,data,tuple,sameType} from './types.js';
import {typeError} from './diagnostics.js';

export function specializeRecordValueOrder(element,namespace){
 const template=readStructuredDataLino(readFileSync(new URL('../data/record-value-order-template.lino',import.meta.url),'utf8'));
 if(template.version!==1||template.model!=='ordinary record value ordering with one opaque payload type parameter 1'||template.surface.language!=='JavaScript'||template.surface.externals?.length||template.surface.main?.effects?.length)throw typeError('record value ordering needs its complete closed source template');
 const surface=structuredClone(template.surface),parameter=template.parameter;
 const owners=surface.items.filter(item=>item.k==='data'&&item.name===parameter.data);
 const owner=owners[0];
 if(owners.length!==1||owner.ctors.length!==1||owner.ctors[0].name!==parameter.constructor||owner.ctors[0].fields.length!==1||owner.ctors[0].fields[0].name!==parameter.field)throw typeError('record value ordering needs its one actual opaque payload owner');
 owner.ctors[0].fields[0].type=element;
 const library=checkProgram({...surface,items:[{k:'module',name:namespace,items:surface.items}],main:null});
 const type=data(namespace+'.'+parameter.data),entry=namespace+'.'+template.entry;
 const declaration=library.declarations.get(entry),shape=library.declarations.get(type.name);
 if(shape?.k!=='data'||shape.ctors.length!==1||shape.ctors[0].name!==parameter.constructor||shape.ctors[0].fields.length!==1||!sameType(shape.ctors[0].fields[0].type,element)||declaration?.k!=='fn'||declaration.params.length!==2||!sameType(declaration.params[0].type,array(type))||!sameType(declaration.params[1].type,array(FLOAT))||!sameType(declaration.ret,array(type)))throw typeError('record value specialization changed its owned source signature');
 const pair=template.pair,pairType=data(namespace+'.'+pair?.data),pairEntry=namespace+'.'+pair?.entry;
 const pairOwner=library.declarations.get(pairType.name),pairFunction=library.declarations.get(pairEntry);
 if(pairOwner?.k!=='data'||pairOwner.ctors.length!==1||pairOwner.ctors[0].name!==pair.constructor||pairOwner.ctors[0].fields.length!==2||pairOwner.ctors[0].fields[0].name!=='key'||!sameType(pairOwner.ctors[0].fields[0].type,STRING)||pairOwner.ctors[0].fields[1].name!=='value'||!sameType(pairOwner.ctors[0].fields[1].type,type)||pairFunction?.k!=='fn'||pairFunction.params.length!==2||!sameType(pairFunction.params[0].type,array(STRING))||!sameType(pairFunction.params[1].type,array(type))||!sameType(pairFunction.ret,array(pairType)))throw typeError('own entry pairing changed its owned source signature');
 return {library,type,entry,constructor:parameter.constructor,pairType,pairEntry,pairConstructor:pair.constructor};
}

// Adapt the declared wrapper/entry constructors to the existing tuple IR.
// Complete source above owns length admission and actual pairing iteration.
export function lowerObjectEntryValues(node,args,specialization,fresh){
 const valuesType=args[1].type,element=valuesType.element,keysType=array(STRING),wrapped=array(specialization.type),pairType=tuple([STRING,element]);
 const captureName=fresh('entryOperands'),captureType={kind:'record',fields:[{name:'keys',type:keysType},{name:'values',type:valuesType}]};
 const capture={k:'var',name:captureName,type:captureType};
 const read=(field,index,type)=>({k:'recordField',object:capture,field,index,type});
 const parameterName=fresh('entryValue'),parameter={k:'var',name:parameterName,type:element};
 const wrapper={k:'ctor',data:specialization.type.name,ctor:specialization.constructor,args:[parameter],type:specialization.type};
 const wrap={k:'lambda',params:[{name:parameterName,type:element}],body:wrapper,type:{kind:'function',params:[element],ret:specialization.type}};
 const values={k:'method',op:'map',object:read('values',1,valuesType),args:[wrap],type:wrapped};
 const pairedType=array(specialization.pairType),paired={k:'call',fn:specialization.pairEntry,args:[read('keys',0,keysType),values],type:pairedType};
 const entryName=fresh('ownEntry'),keyName=fresh('entryKey'),wrapperName=fresh('entryWrapper'),payloadName=fresh('entryPayload');
 const key={k:'var',name:keyName,type:STRING},payload={k:'var',name:payloadName,type:element};
 const project={k:'match',scrutinee:{k:'var',name:wrapperName,type:specialization.type},cases:[{pattern:{k:'ctor',data:specialization.type.name,ctor:specialization.constructor,binds:[payloadName]},body:{k:'tuple',items:[key,payload],type:pairType}}],type:pairType};
 const body={k:'match',scrutinee:{k:'var',name:entryName,type:specialization.pairType},cases:[{pattern:{k:'ctor',data:specialization.pairType.name,ctor:specialization.pairConstructor,binds:[keyName,wrapperName]},body:project}],type:pairType};
 const unwrap={k:'lambda',params:[{name:entryName,type:specialization.pairType}],body,type:{kind:'function',params:[specialization.pairType],ret:pairType}};
 return {k:'let',name:captureName,value:{k:'record',entries:[{name:'keys',value:args[0]},{name:'values',value:args[1]}],type:captureType},body:{k:'method',op:'map',object:paired,args:[unwrap],type:node.type},type:node.type,...(node.span?{span:node.span}:{})};
}

export function lowerRecordValueOrder(node,args,specialization,fresh){
 const valuesType=args[0].type,element=valuesType.element,positions=array(FLOAT),wrapped=array(specialization.type);
 const captureName=fresh('recordValueOperands');
 const captureType={kind:'record',fields:[{name:'fields',type:valuesType},{name:'order',type:positions}]};
 const capture={k:'var',name:captureName,type:captureType};
 const read=(name,index,type)=>({k:'recordField',object:capture,field:name,index,type});
 const parameterName=fresh('recordValue'),parameter={k:'var',name:parameterName,type:element};
 const wrapper={k:'ctor',data:specialization.type.name,ctor:specialization.constructor,args:[parameter],type:specialization.type};
 const wrap={k:'lambda',params:[{name:parameterName,type:element}],body:wrapper,type:{kind:'function',params:[element],ret:specialization.type}};
 const values={k:'method',op:'map',object:read('fields',0,valuesType),args:[wrap],type:wrapped};
 const ordered={k:'call',fn:specialization.entry,args:[values,read('order',1,positions)],type:wrapped};
 const wrapperName=fresh('recordValueWrapper'),payloadName=fresh('recordValuePayload');
 const payload={k:'var',name:payloadName,type:element};
 const body={k:'match',scrutinee:{k:'var',name:wrapperName,type:specialization.type},cases:[{pattern:{k:'ctor',data:specialization.type.name,ctor:specialization.constructor,binds:[payloadName]},body:payload}],type:element};
 const unwrap={k:'lambda',params:[{name:wrapperName,type:specialization.type}],body,type:{kind:'function',params:[specialization.type],ret:element}};
 const result={k:'method',op:'map',object:ordered,args:[unwrap],type:valuesType};
 return {k:'let',name:captureName,value:{k:'record',entries:[{name:'fields',value:args[0]},{name:'order',value:args[1]}],type:captureType},body:result,type:node.type,...(node.span?{span:node.span}:{})};
}
