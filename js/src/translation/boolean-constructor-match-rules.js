// A complete finite constructor inventory proves the Boolean default arm.
// The caller retains the scrutinee and all nondefault cases without change.
/** Empty refuses. Otherwise the first element is the literal Boolean default
 * (zero/one), followed by the original indices of every retained case.
 * @param {string} owner @param {string[]} declared @param {string[]} owners
 * @param {string[]} constructors @param {number[]} literals @returns {number[]} */
export function planBooleanConstructorMatchSharing(owner,declared,owners,constructors,literals){
 const count=declared.length;
 if(owner===''||count<2||count>256||owners.length!==count||constructors.length!==count||literals.length!==count)return [];
 let covered=[],zero=0,one=0;
 for(let index=0;index<count;index+=1){
  const declaration=declared[index]??'';
  if(declaration===''||declared.indexOf(declaration)!==index)return [];
 }
 for(let index=0;index<count;index+=1){
  const constructor=constructors[index]??'',literal=literals[index]?? -1;
  if(owners[index]!==owner||!declared.includes(constructor)||covered.includes(constructor))return [];
  covered.push(constructor);
  if(literal===0)zero+=1;else if(literal===1)one+=1;else return [];
 }
 const fallback=zero>=one?0:1;
 if(Math.max(zero,one)<2)return [];
 let plan=[fallback];
 for(let index=0;index<count;index+=1)if(literals[index]!==fallback)plan.push(index);
 return plan;
}
