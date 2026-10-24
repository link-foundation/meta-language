// Original-term transport only. The shared plan chooses endpoints; the common
// unifier retains variable identity, occurs checks and ordinary type errors.
import { createSurfaceReferenceFacts } from './surface-reference-facts.js';
import { planJavaScriptArrayDefaultInference,planJavaScriptBindingDefaultInference,readJavaScriptBindingDefaultAbsenceName } from './array-binding-rules.js';

export function readJavaScriptArrayDefaultConstraints(resolved, fallback, result) {
  const facts = createSurfaceReferenceFacts({ resolved, fallback, result }, { retainValues: true });
  const child = name => {
    for (let at = facts.starts[facts.root]; at < facts.starts[facts.root] + facts.counts[facts.root]; at += 1) {
      if (facts.labels[at] === name) return facts.targets[at];
    }
    return -1;
  };
  const plan = planJavaScriptArrayDefaultInference(facts.texts, facts.starts, facts.counts, facts.labels, facts.targets, child('resolved'), child('fallback'), child('result'));
  if (!plan.length) return null;
  if (plan[0] < 0) return { refused: true, pairs: [] };
  const pairs = [];
  for (let at = 0; at < plan.length; at += 2) pairs.push([facts.values[plan[at]], facts.values[plan[at + 1]]]);
  return { refused: false, pairs };
}

export function readJavaScriptBindingDefaultConstraints(value,fallback,result,resolve=term=>term){
 // Project actual optional children through the solver, retaining base owners.
 const project=term=>{const node=resolve(term);return node.kind==='option'?{...node,element:project(node.element)}:node;};
 const facts=createSurfaceReferenceFacts({value:project(value),fallback:project(fallback)},{retainValues:true});
 const child=name=>{for(let at=facts.starts[facts.root];at<facts.starts[facts.root]+facts.counts[facts.root];at+=1)if(facts.labels[at]===name)return facts.targets[at];return -1;};
 const plan=planJavaScriptBindingDefaultInference(facts.texts,facts.starts,facts.counts,facts.labels,facts.targets,child('value'),child('fallback'));
 if(!plan.length)return null;
 if(plan[0]<0)return{refused:true,pairs:[]};
 const absence=readJavaScriptBindingDefaultAbsenceName(plan[0]),element=facts.values[plan[1]];
 const pairs=[];
 for(let at=2;at<plan.length;at+=2)pairs.push([facts.values[plan[at]],facts.values[plan[at+1]]]);
 pairs.push([result,absence?{kind:'option',element,absence}:element]);
 return{refused:false,pairs};
}
