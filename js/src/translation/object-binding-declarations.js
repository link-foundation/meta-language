// Actual lexical declaration transport through the shared object-pattern plan.
import {unsupported,TranslationError} from './diagnostics.js';
import {JAVASCRIPT_ARRAY_BINDING_POLICY} from './array-binding-policy.js';
import {requireJavaScriptIdentifierRole} from './javascript-identifier-roles.js';
import {readJavaScriptFormalInitializer} from './formal-initializers.js';
import {planJavaScriptObjectPatternBindings,readJavaScriptObjectBindingStatement,readJavaScriptRecordFormalDefaultOperation,acceptJavaScriptObjectBindingName,readJavaScriptObjectPatternBindingPositions} from './javascript-record-parameter-rules.js';
import {readJavaScriptObjectRestOperation} from './object-rest-rules.js';
export function readJavaScriptObjectBindingNames(parser,start){
 const close=parser.matching(start),tokens=parser.cursor.tokens.slice(start,close+1),plan=planJavaScriptObjectPatternBindings(tokens.map(token=>token.kind),tokens.map(token=>token.value),0,JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings,[],3);
 return plan[0]<0?[]:readJavaScriptObjectPatternBindingPositions(plan).map(position=>tokens[position]);
}
export function readJavaScriptObjectBindingDeclaration(parser,startToken,mode){
 const c=parser.cursor,start=c.index,close=parser.matching(start),tokens=c.tokens.slice(start,close+1);
 const plan=planJavaScriptObjectPatternBindings(tokens.map(token=>token.kind),tokens.map(token=>token.value),0,JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings,[],3);
 if(plan[0]===-4){c.index=start+tokens.length;throw unsupported('empty object binding','object destructibility needs a checked representation',{start:startToken.start,end:c.peek().end});}
 if(plan[0]===-6)throw unsupported('rest property','name every field',tokens[plan[1]]);
 if(plan[0]===-8){const token=tokens[plan[1]];requireJavaScriptIdentifierRole(token,true);if(acceptJavaScriptObjectBindingName(token.kind,token.value,JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings))throw new TranslationError('syntax','duplicate lexical binding '+token.value,{start:token.start,end:token.end});}
 if(plan[0]<0)throw unsupported('object binding syntax','flat actual property names and distinct lexical bindings need their complete source meanings (reason '+(-plan[0])+')',tokens[plan[1]]??c.peek());
 if(plan[0]!==tokens.length||readJavaScriptObjectBindingStatement(mode)==='')throw unsupported('object binding transport','the declaration needs its complete actual object pattern and lexical mode',c.peek());
 const rows=[];for(let index=0;index<plan[1];index+=1){const at=2+index*4;rows.push({property:plan[at],binding:plan[at+1],first:plan[at+2],last:plan[at+3]});}
 const positions=readJavaScriptObjectPatternBindingPositions(plan),bindings=positions.map(position=>tokens[position]);
 for(const token of bindings){requireJavaScriptIdentifierRole(token,true);parser.lexicalDeclaration(token.value,{start:token.start,end:token.end});parser.scope.tdz.add(token.value);}
 c.index=start+plan[0];c.expect('=','destructuring');const value=parser.expr(),where={start:startToken.start,end:c.peek().end};
 const name=parser.generatedPrefix+'temporary_binding'+(++parser.generatedCount),object={k:'name',path:[name],span:where};
 const statements=[{s:'const',name,value,span:where}],excluded=[];
 for(const row of rows){
  const property=tokens[row.property],binding=tokens[row.binding],span={start:property.start,end:binding.end};
  if(row.first===-2){
   const read={k:'math',op:readJavaScriptObjectRestOperation(),name:'',args:[{value:object,spread:false},{value:{k:'array',items:excluded.map(token=>({value:{k:'str',value:token.value,span:{start:token.start,end:token.end}},spread:false})),span},spread:false}],span};
   statements.push({s:readJavaScriptObjectBindingStatement(mode),name:binding.value,value:read,span:where});continue;
  }
  excluded.push(property);
  const fallback=row.first>=0?readJavaScriptFormalInitializer(parser,{tokens,start,first:row.first,last:row.last,binding:row.binding,positions,context:'ordinary object binding declaration',profile:'object binding initializer'}):null;
  const read=fallback?{k:'math',op:readJavaScriptRecordFormalDefaultOperation(),name:'',args:[{value:object,spread:false},{value:{k:'str',value:property.value,span:{start:property.start,end:property.end}},spread:false},{value:fallback,spread:false}],span:{start:property.start,end:fallback.span?.end??binding.end}}:{k:'field',object,field:property.value,span};
  statements.push({s:readJavaScriptObjectBindingStatement(mode),name:binding.value,value:read,span:where});
 }
 for(const token of bindings){parser.declareLocal(token.value,mode==='let');parser.lexicalBinding(token.value,c.peek().start);}
 return {statements,bindings,span:where};
}
