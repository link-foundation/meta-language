import {unsupported,typeError} from './diagnostics.js';
import {BOOL} from './types.js';
import {readPropositionValueForm,readPropositionValueConditionalBranch,readPropositionValueQuantifierJoin,planPropositionValueQuantifier,readPropositionValueDomainError,readPropositionValueExpansionLimit} from './proposition-value-rules.js';
// Source-state callers normalize before the checker: genuine expression
// leaves stay the very same nodes, preserving their source ownership facts.
export function expandSurfacePropositionValue(proposition){
 let work=0;const visit=(prop,depth)=>{
  if(++work>readPropositionValueExpansionLimit()||depth>readPropositionValueExpansionLimit())throw unsupported('proposition value','the exhaustive proposition expansion exceeds its finite source budget',prop.span);
  const form=readPropositionValueForm(prop.p);if(form==='value')return prop.expr;if(form==='comparison')return {k:'propositionValue',prop,span:prop.span};
  if(form==='negation')return {k:'unary',op:'not',arg:visit(prop.arg,depth+1)};
  if(form==='and'||form==='or'||form==='implies'){const left=visit(prop.left,depth+1),right=visit(prop.right,depth+1),branch=truth=>{const mode=readPropositionValueConditionalBranch(form,truth);return mode==='right'?right:{k:'bool',value:mode==='true'};};return {k:'if',cond:left,then:branch(true),else:branch(false)};}
  if(form==='finite quantification'){const bind=index=>{if(++work>readPropositionValueExpansionLimit())throw unsupported('proposition value','the exhaustive proposition expansion exceeds its finite source budget',prop.span);if(index===prop.binders.length)return visit(prop.body,depth+1);const binder=prop.binders[index],error=readPropositionValueDomainError(binder.type?.kind??'');if(error)throw unsupported('proposition value',error,prop.span);const branches=planPropositionValueQuantifier(binder.type.kind).map(value=>({k:'let',name:binder.name,type:binder.type,value:binder.type.kind==='bool'?{k:'bool',value:value===1}:{k:'unit'},body:bind(index+1)}));return branches.length===1?branches[0]:{k:'binary',op:readPropositionValueQuantifierJoin(),left:branches[0],right:branches[1]};};return bind(0);}
  throw typeError('unknown proposition value '+prop.p,prop.span);
 };return visit(proposition,0);
}
export function checkPropositionValue(checker,node,environment,path){
 const checked=checker.prop(node.prop,environment,path);let expanded=0;
 const literal=value=>({k:'lit',value,type:BOOL});
 const conditional=(operation,left,right)=>{const branch=truth=>{const mode=readPropositionValueConditionalBranch(operation,truth);return mode==='right'?right:literal(mode==='true');};return {k:'if',cond:left,then:branch(true),else:branch(false),type:BOOL};};
 const lower=(prop,depth)=>{
  if(++expanded>readPropositionValueExpansionLimit()||depth>readPropositionValueExpansionLimit())throw unsupported('proposition value','the exhaustive proposition expansion exceeds its finite source budget',node.span);
  const form=readPropositionValueForm(prop.p);
  if(form==='value')return prop.expr;
  if(form==='comparison'){const {p,...comparison}=prop;return {k:'propositionValue',...comparison,op:p,type:BOOL,...(node.span?{span:node.span}:{})};}
  if(form==='negation')return {k:'unary',op:'not',arg:lower(prop.arg,depth+1),type:BOOL};
  if(form==='and'||form==='or'||form==='implies')return conditional(form,lower(prop.left,depth+1),lower(prop.right,depth+1));
  if(form==='finite quantification'){
   const bind=(index)=>{
    if(++expanded>readPropositionValueExpansionLimit())throw unsupported('proposition value','the exhaustive proposition expansion exceeds its finite source budget',node.span);
    // Distinct exhaustive instances have distinct lexical binders. Their
    // checked variable occurrences must not share graph reference identity.
    if(index===prop.binders.length)return lower(structuredClone(prop.body),depth+1);
    const binder=prop.binders[index],error=readPropositionValueDomainError(binder.type.kind);if(error)throw unsupported('proposition value',error,node.span);
    const values=planPropositionValueQuantifier(binder.type.kind),branches=values.map(value=>({k:'let',name:binder.name,value:binder.type.kind==='bool'?{k:'lit',value:value===1,type:binder.type}:{k:'unit',type:binder.type},body:bind(index+1),type:BOOL}));
    return branches.length===1?branches[0]:conditional(readPropositionValueQuantifierJoin(),branches[0],branches[1]);
   };return bind(0);
  }
  throw typeError('unknown checked proposition value '+prop.p,node.span);
 };return lower(checked,0);
}
