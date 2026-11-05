import {readRequiredNumberFact} from './fact-column-rules.js';
/** Source-order component representatives for the actual datatype dependency
 * graph. Iterative two-pass traversal visits each vertex/edge a bounded number
 * of times. Grouping grants no positivity, ownership or default capability.
 * @param {number} count @param {number[]} ends @param {number[]} edges
 * @returns {number[]} */
export function planDatatypeDependencyGroups(count,ends,edges){
 if(!Number.isInteger(count)||Object.is(count,-0)||count<0||count>8192||ends.length!==count||edges.length>131072)return [];
 const starts=[],heads=[],next=[],sources=[],seen=[],labels=[],order=[];
 for(let node=0;node<count;node+=1){heads.push(-1);seen.push(0);labels.push(-1);}
 let first=0;
 for(let node=0;node<count;node+=1){
  const last=readRequiredNumberFact(ends,node);
  if(!Number.isInteger(last)||Object.is(last,-0)||last<first||last>edges.length)return [];
  starts.push(first);
  for(let edge=first;edge<last;edge+=1){
   const target=readRequiredNumberFact(edges,edge);
   if(!Number.isInteger(target)||Object.is(target,-0)||target<0||target>=count)return [];
   next.push(readRequiredNumberFact(heads,target));sources.push(node);heads[target]=edge;
  }
  first=last;
 }
 if(first!==edges.length)return [];
 for(let root=0;root<count;root+=1){
  if(readRequiredNumberFact(seen,root)!==0)continue;
  const stack=[root],positions=[readRequiredNumberFact(starts,root)];seen[root]=1;
  while(stack.length){
   const top=stack.length-1,node=readRequiredNumberFact(stack,top),position=readRequiredNumberFact(positions,top);
   if(position<readRequiredNumberFact(ends,node)){
    positions[top]=position+1;const target=readRequiredNumberFact(edges,position);
    if(readRequiredNumberFact(seen,target)===0){seen[target]=1;stack.push(target);positions.push(readRequiredNumberFact(starts,target));}
   }else{stack.pop();positions.pop();order.push(node);}
  }
 }
 for(let at=order.length-1;at>=0;at-=1){
  const root=readRequiredNumberFact(order,at);if(readRequiredNumberFact(labels,root)>=0)continue;
  const stack=[root],members=[];labels[root]=root;let representative=root;
  while(stack.length){
   const node=readRequiredNumberFact(stack,stack.length-1);stack.pop();members.push(node);representative=Math.min(representative,node);
   let edge=readRequiredNumberFact(heads,node);
   while(edge>=0){const source=readRequiredNumberFact(sources,edge);if(readRequiredNumberFact(labels,source)<0){labels[source]=root;stack.push(source);}edge=readRequiredNumberFact(next,edge);}
  }
  for(let index=0;index<members.length;index+=1)labels[readRequiredNumberFact(members,index)]=representative;
 }
 return labels;
}
