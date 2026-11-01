import {readRequiredNumberFact,readRequiredTextFact,readRequiredBooleanFact} from './fact-column-rules.js';
/** @param {number} depth @param {number} visits @returns {string} */
export function readCallableDataTraversalError(depth,visits){return !Number.isInteger(depth)||!Number.isInteger(visits)||depth<0||visits<0||depth>256||visits>65536?'callable data type analysis exceeds its finite source profile':'';}
/** @param {string} kind @param {boolean[]} children @returns {boolean} */
export function acceptCallableDataNode(kind,children){return kind==='function'||children.some(value=>value);}
/** A conservative constructive profile; every referenced datatype derives a
 * real default independently, including function-domain dependencies.
 * @param {string} kind @param {boolean[]} children @returns {boolean} */
export function acceptCallableDataConstructiveNode(kind,children){
 if(kind==='function')return children.length>0&&children.every(value=>value);
 if(kind==='data')return children.length>0&&children.every(value=>value);
 if(kind==='constructor'||kind==='tuple'||kind==='record')return children.every(value=>value);
 return ['nat','int','fixed','float','bool','string','unit','output','option','array','set','map'].includes(kind)&&children.every(value=>value);
}
/** @param {boolean} callable @param {boolean} acyclic @param {boolean} constructive @returns {string} */
export function readCallableDataProfileError(callable,acyclic,constructive){return !callable?'':!acyclic?'recursive callable data needs an independent positivity and constructive default profile':!constructive?'callable data needs actual constructive defaults for every referenced datatype and callable result':'';}
/** @param {string} target @param {boolean} callable @param {boolean} float @param {boolean} output @param {boolean} undecidable @returns {string} */
export function readCallableDataDerivation(target,callable,float,output,undecidable){
 if(target==='Rust')return callable?'Clone':float?'Clone, Debug, PartialEq':'Clone, Debug, PartialEq, Eq';
 if(target==='Lean')return callable||output?'Inhabited':undecidable?'Repr, Inhabited':'Repr, DecidableEq, Inhabited';
 return '';
}
/** @param {string} operation @param {boolean} callable @returns {string} */
export function readCallableDataObservationError(operation,callable){return callable&&['display','comparison','proof'].includes(operation)?'callable values and aggregates retain no portable identity, structural equality, debug display or proof observation':'';}
/** Flat postorder actual type graph; references index declaration roots.
 * Decisions, finite witnesses and strict-positive self recursion stay shared.
 * @param {string[]} kinds @param {number[]} ends @param {number[]} edges
 * @param {number[]} references @param {number[]} roots @param {number} root
 * @returns {string[]} */
export function readCallableDataRecursivePlan(kinds,ends,edges,references,roots,root){
 return readDatatypeConstructivePlan(kinds,ends,edges,references,roots,root,false);
}
/** Independent mutual-inductive profile; old callable admission is unchanged.
 * @param {string[]} kinds @param {number[]} ends @param {number[]} edges
 * @param {number[]} references @param {number[]} roots @param {number} root
 * @returns {string[]} */
export function readFormalDatatypeConstructivePlan(kinds,ends,edges,references,roots,root){
 return readDatatypeConstructivePlan(kinds,ends,edges,references,roots,root,true);
}
/** @param {string[]} kinds @param {number[]} ends @param {number[]} edges
 * @param {number[]} references @param {number[]} roots @param {number} root
 * @param {boolean} mutual @returns {string[]} */
function readDatatypeConstructivePlan(kinds,ends,edges,references,roots,root,mutual){
 const count=kinds.length;
 if(count===0||count>65536||edges.length>65536||roots.length>count||ends.length!==count||references.length!==count||!Number.isInteger(root)||root<0||root>=count)return ['error','incomplete callable data graph'];
 let previous=0;
 for(let at=0;at<count;at+=1){
  const end=readRequiredNumberFact(ends,at),reference=readRequiredNumberFact(references,at);
  if(!Number.isInteger(end)||end<previous||end>edges.length||!Number.isInteger(reference)||reference< -1||reference>=roots.length)return ['error','incomplete callable data graph'];
  for(let child=previous;child<end;child+=1){const index=readRequiredNumberFact(edges,child);if(!Number.isInteger(index)||index<0||index>=at)return ['error','callable data children need complete postorder identities'];}
  if(reference>=0&&(readRequiredTextFact(kinds,at)!=='reference'||end!==previous)||reference<0&&readRequiredTextFact(kinds,at)==='reference')return ['error','callable data reference needs its actual leaf identity'];
  previous=end;
 }
 if(previous!==edges.length||roots.some(value=>!Number.isInteger(value)||value<0||value>=count||readRequiredTextFact(kinds,value)!=='data'))return ['error','incomplete callable data declaration roots'];
 const callable=kinds.includes('function');if(!callable&&!mutual)return ['profile','false','false','true','-1'];
 let work=0,recursive=false;
 for(let declaration=0;declaration<roots.length;declaration+=1){
  work+=count*4;if(work>1048576)return ['error','callable data graph reasoning exceeds its finite source profile'];
  const pending=[readRequiredNumberFact(roots,declaration)],domains=[false],crossings=[false],seen=[];for(let at=0;at<count*4;at+=1)seen.push(false);
  while(pending.length>0){
   if(++work>1048576)return ['error','callable data graph reasoning exceeds its finite source profile'];
   const at=readRequiredNumberFact(pending,pending.length-1),domain=readRequiredBooleanFact(domains,domains.length-1),crossed=readRequiredBooleanFact(crossings,crossings.length-1),key=at*4+(domain?2:0)+(crossed?1:0);pending.pop();domains.pop();crossings.pop();
   if(readRequiredBooleanFact(seen,key))continue;seen[key]=true;
   const reference=readRequiredNumberFact(references,at);
   if(reference>=0){
    if(reference===declaration){if(domain)return ['error','recursive callable data in a function domain lacks strict positivity'];if(crossed&&!mutual)return ['error','mutually recursive callable data needs a mutual-inductive backend profile'];recursive=true;continue;}
    pending.push(readRequiredNumberFact(roots,reference));domains.push(domain);crossings.push(true);continue;
   }
   const start=at===0?0:readRequiredNumberFact(ends,at-1),end=readRequiredNumberFact(ends,at);
   for(let child=start;child<end;child+=1){pending.push(readRequiredNumberFact(edges,child));domains.push(domain||readRequiredTextFact(kinds,at)==='function'&&child<end-1);crossings.push(crossed);}
  }
 }
 const known=roots.map(()=>false),witnesses=roots.map(()=>-1),ranks=roots.map(()=>-1);let changed=true,rank=0;
 while(changed){
  changed=false;rank+=1;const values=[];
  for(let at=0;at<count;at+=1){
   if(++work>1048576)return ['error','callable data graph reasoning exceeds its finite source profile'];
   const start=at===0?0:readRequiredNumberFact(ends,at-1),children=[];for(let edge=start;edge<readRequiredNumberFact(ends,at);edge+=1)children.push(readRequiredBooleanFact(values,readRequiredNumberFact(edges,edge)));
   const reference=readRequiredNumberFact(references,at),kind=readRequiredTextFact(kinds,at);
   values.push(reference>=0?readRequiredBooleanFact(known,reference):kind==='data'?children.some(value=>value):kind==='function'?children.length>0&&readRequiredBooleanFact(children,children.length-1):['array','option','set','map'].includes(kind)?true:acceptCallableDataConstructiveNode(kind,children));
  }
  for(let declaration=0;declaration<roots.length;declaration+=1)if(!readRequiredBooleanFact(known,declaration)){
   const at=readRequiredNumberFact(roots,declaration),start=at===0?0:readRequiredNumberFact(ends,at-1);
   for(let edge=start;edge<readRequiredNumberFact(ends,at);edge+=1)if(readRequiredBooleanFact(values,readRequiredNumberFact(edges,edge))){known[declaration]=true;witnesses[declaration]=edge-start;ranks[declaration]=rank;changed=true;break;}
  }
 }
 if(!known.every(value=>value))return ['error','callable data has no finite constructive witness for every referenced datatype'];
 if(mutual){const result=['witnesses'];for(let at=0;at<roots.length;at+=1){result.push(String(readRequiredNumberFact(witnesses,at)));result.push(String(readRequiredNumberFact(ranks,at)));}return result;}
 const declaration=readRequiredNumberFact(references,root);
 return ['profile','true',recursive?'true':'false','true',String(declaration<0?-1:readRequiredNumberFact(witnesses,declaration))];
}
/** @param {string} name @param {string} constructor @param {number} fields @returns {string} */
export function renderCallableDataLeanDefault(name,constructor,fields){let value=constructor;for(let at=0;at<fields;at+=1)value+=' default';return 'instance : Inhabited '+name+' := ⟨'+value+'⟩';}
/** Mutual blocks provide finite default instances afterward in witness order.
 * @param {boolean} callable @param {boolean} output @param {boolean} undecidable
 * @returns {string} */
export function readFormalDatatypeLeanBlockDerivation(callable,output,undecidable){return callable||output?'':undecidable?'Repr':'Repr, DecidableEq';}
/** The attribute belongs to the whole mutual command, including nested list
 * fields appearing only in later members.
 * @param {string[]} definitions @returns {string} */
export function renderFormalDatatypeRocqMutualBlock(definitions){return '#[warnings="-register-all"]\n'+definitions.join('\n');}
