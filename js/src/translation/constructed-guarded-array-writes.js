// Transport actual source occurrences selected by the shared joint ownership plan.
import {createSurfaceReferenceFacts,createSurfaceReferenceColumns} from './surface-reference-facts.js';
import {planConstructedGuardedArrayWrites} from './guarded-private-array-write-rules.js';
export function lowerConstructedGuardedArrayWrites(statements,parameters,parser){
 const facts=createSurfaceReferenceFacts({statements,parameters},{retainValues:true});
 const packed=createSurfaceReferenceColumns(facts),plan=planConstructedGuardedArrayWrites(packed.columns,packed.rows,packed.size,facts.root),proofs=[];
 if(plan.length%4!==0)throw new TypeError('a constructed guarded array plan requires four actual fields');
 for(let at=0;at<plan.length;at+=4){
  const code=plan[at],statement=facts.values[plan[at+1]],operation=facts.values[plan[at+2]],allocation=facts.values[plan[at+3]],owner=allocation.name;
  if(code===1){
   if(statement.expr!==operation||operation.object.path[0]!==owner)throw new TypeError('a constructed append lost its actual source owners');
   const where=operation.span;
   proofs.push({operationSpan:{start:operation.object.span.end,end:operation.args[0].span.start}});
   statement.s='assign';statement.name=owner;statement.value={k:'array',items:[{spread:true,value:operation.object},...operation.args.map(value=>({value}))],span:where};delete statement.expr;
  }else{
   if(code!==0||statement.then[0]!==operation||operation.object.path[0]!==owner)throw new TypeError('a constructed guarded write lost its actual source owners');
   const where=operation.span;
   const temporary=role=>{let name='';do{name='ml_constructed_guarded_'+role+'_'+(++parser.generatedCount);}while(facts.texts.includes(name));return name;};
   const position=temporary('position'),value=temporary('value'),element=temporary('element'),ordinal=temporary('ordinal');
   const local=name=>({k:'name',path:[name],span:where});
   const select={k:'lambda',params:[{name:element,type:null},{name:ordinal,type:null}],ret:null,captures:[],body:{k:'if',cond:{k:'binary',op:'eq',left:local(ordinal),right:local(position),span:where},then:local(value),else:local(element),span:where},span:where};
   statement.then=[{s:'const',name:position,value:operation.index,span:where},{s:'const',name:value,value:operation.value,span:where},{s:'assign',name:owner,value:{k:'method',op:'map',object:operation.object,args:[select],span:where},span:where}];
   proofs.push({operationSpan:{start:operation.object.span.end,end:operation.value.span.start}});
  }
 }
 return proofs;
}
