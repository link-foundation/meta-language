// Source occurrence transport for the complete shared ownership planner.
// A selection identifies actual nodes; this adapter supplies no admission rule.
import {planGuardedPrivateArrayWrites} from './guarded-private-array-write-rules.js';
import {createSurfaceReferenceFacts,createSurfaceReferenceColumns} from './surface-reference-facts.js';

export function lowerGuardedPrivateArrayWrites(statements,parameters,parser){
  const facts=createSurfaceReferenceFacts({statements,parameters},{retainValues:true});
  const {columns,rows,size}=createSurfaceReferenceColumns(facts);
  const plan=planGuardedPrivateArrayWrites(columns,rows,size,facts.root),proofs=[];
  if(plan.length%3!==0)throw new TypeError('a guarded array write selection requires three actual identities');
  for(let at=0;at<plan.length;at+=3){
    const statement=facts.values[plan[at]],write=facts.values[plan[at+1]],allocation=facts.values[plan[at+2]];
    if(statement.then[0]!==write||write.object.path[0]!==allocation.name)throw new TypeError('a guarded array write selection lost its actual source owners');
    const owner=allocation.name,where=write.span;
    const temporary=role=>{
      let name='';
      do{name='ml_guarded_write_'+role+'_'+(++parser.generatedCount);}while(facts.texts.includes(name));
      return name;
    };
    const position=temporary('position'),value=temporary('value'),element=temporary('element'),ordinal=temporary('ordinal');
    const local=name=>({k:'name',path:[name],span:where});
    const select={k:'lambda',params:[{name:element,type:null},{name:ordinal,type:null}],ret:null,captures:[],
      body:{k:'if',cond:{k:'binary',op:'eq',left:local(ordinal),right:local(position),span:where},then:local(value),else:local(element),span:where},span:where};
    statement.then=[{s:'const',name:position,value:write.index,span:where},{s:'const',name:value,value:write.value,span:where},
      {s:'assign',name:owner,value:{k:'method',op:'map',object:write.object,args:[select],span:where},span:where}];
    proofs.push({operationSpan:{start:write.object.span.end,end:write.value.span.start}});
  }
  return proofs;
}
