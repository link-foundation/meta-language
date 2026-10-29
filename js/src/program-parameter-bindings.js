// Coordinate transport from the existing concrete tree to shared definitions.
import {acceptJavaScriptSourceParameterIdentifierTerm,acceptJavaScriptSourceBindingRootTerm,planJavaScriptSourceParameterBindings} from './translation/source-parameter-binding-rules.js';
export function readJavaScriptSourceParameterBindings(tokens,syntax,first,last){
 return readBindingPositions(tokens,syntax,first,last,false);
}
export function readJavaScriptSourceVariableBindings(tokens,syntax,index){
 const declaration=syntax.find(node=>(node.term==='lexical_declaration'||node.term==='variable_declaration')&&node.start===tokens[index].start);
 if(!declaration)return null;
 let last=index+1;while(last<tokens.length&&tokens[last].start<declaration.end)last+=1;
 return readBindingPositions(tokens,syntax,index+1,last,true,declaration.end);
}
function readBindingPositions(tokens,syntax,first,last,variables,boundary){
 if(first<0||last<first||last>tokens.length||first===last)return [];
 const selected=tokens.slice(first,last),begin=selected[0].start,end=boundary??tokens[last]?.start??selected.at(-1).end;
 const lower=offset=>{let low=0,high=selected.length;while(low<high){const middle=Math.floor((low+high)/2);if(selected[middle].start<offset)low=middle+1;else high=middle;}return low;};
 const identifiers=new Set(syntax.filter(node=>acceptJavaScriptSourceParameterIdentifierTerm(node.term)).map(node=>node.start+':'+node.end));
 const defaults=[],computed=[];
 for(const node of syntax){
  if(node.start<begin||node.start>=end||node.end>end)continue;
  const interval=[lower(node.start),lower(node.end)];
  if(node.term==='assignment_pattern'||node.term==='object_assignment_pattern')defaults.push(interval);
  if(node.term==='computed_property_name')computed.push(interval);
  if(variables&&node.term==='variable_declarator'){
   const root=syntax.filter(child=>acceptJavaScriptSourceBindingRootTerm(child.term)&&child.start===node.start&&child.end<=node.end).sort((left,right)=>right.end-left.end)[0];
   const equals=root&&selected[lower(root.end)];
   if(equals?.kind==='punctuation'&&equals.text==='=')defaults.push(interval);
  }
 }
 defaults.sort((left,right)=>(left[1]-left[0])-(right[1]-right[0]));
 const plan=planJavaScriptSourceParameterBindings(selected.map(token=>token.kind==='punctuation'?'punct':token.kind),selected.map(token=>token.text),selected.map(token=>identifiers.has(token.start+':'+token.end)),defaults.map(row=>row[0]),defaults.map(row=>row[1]),computed.map(row=>row[0]),computed.map(row=>row[1]));
 // Malformed concrete input retains the existing concrete diagnostics; this
 // pass cannot manufacture admitted bindings from invalid source columns.
 if(plan.length===1&&plan[0]===-1)return [];
 return plan.map(index=>first+index);
}
