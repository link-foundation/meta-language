// A complete finite constructor inventory proves the Boolean default arm.
// The caller retains the scrutinee and all nondefault cases without change.
/** Empty refuses. Otherwise the first element is the literal Boolean default
 * (zero/one), followed by the original indices of every retained case.
 * @param {string} owner @param {string[]} declared @param {string[]} owners
 * @param {string[]} constructors @param {number[]} literals @returns {number[]} */
export function planBooleanConstructorMatchSharing(owner,declared,owners,constructors,literals){
 if(literals.length>256)return [];
 for(const literal of literals)if(literal!==0&&literal!==1)return [];
 return planClosedConstructorMatchSharing(owner,declared,owners,constructors,literals);
}
/** Classification identities name independently established closed outcomes;
 * minus one means an unmerged case. The host must retain their actual bodies.
 * @param {string} owner @param {string[]} declared @param {string[]} owners
 * @param {string[]} constructors @param {number[]} classes @returns {number[]} */
export function planClosedConstructorMatchSharing(owner,declared,owners,constructors,classes){
 const count=declared.length;
 if(owner===''||count<2||count>256||owners.length!==count||constructors.length!==count||classes.length!==count)return [];
 let covered=[];
 for(let index=0;index<count;index+=1){
  const declaration=declared[index]??'';
  if(declaration===''||declared.indexOf(declaration)!==index)return [];
 }
 for(let index=0;index<count;index+=1){
  const constructor=constructors[index]??'',classification=classes[index]?? -2;
  if(owners[index]!==owner||!declared.includes(constructor)||covered.includes(constructor))return [];
  covered.push(constructor);
  if(!Number.isInteger(classification)||classification< -1||classification>=count)return [];
 }
 let fallback=-1,largest=1;
 for(let candidate=0;candidate<count;candidate+=1){
  let matches=0;
  for(const classification of classes)if(classification===candidate)matches+=1;
  if(matches>largest){fallback=candidate;largest=matches;}
 }
 if(fallback<0)return [];
 let plan=[fallback];
 for(let index=0;index<count;index+=1)if(classes[index]!==fallback)plan.push(index);
 return plan;
}

/** Empty names are already ignored slots. Reference names are conservative:
 * an occurrence under a shadowing binder still keeps the outer slot.
 * @param {string[]} bindings @param {string[]} references @returns {number[]} */
export function planUnusedConstructorBindings(bindings,references){
 if(bindings.length>256||references.length>16384)return [];
 let result=[];
 for(let index=0;index<bindings.length;index+=1){
  const name=bindings[index]??'';
  if(name!==''&&!references.includes(name))result.push(index);
 }
 return result;
}

/** Administrative aliases read an already bound constructor/local value.
 * No property access, invocation, allocation or arithmetic is admitted.
 * @param {string} name @param {string} value @param {string[]} available
 * @param {string[]} references @returns {boolean} */
export function testUnusedConstructorAlias(name,value,available,references){
 return name!==''&&value!==''&&available.length<=4096&&references.length<=16384&&available.includes(value)&&!references.includes(name);
}

/** Hoist only fields already projected from one immutable constructor.
 * @param {number} constructors @param {number[]} counts @returns {number[]} */
export function planSingleConstructorProjections(constructors,counts){
 if(constructors!==1||counts.length>256)return [];
 let total=0,result=[];
 for(let index=0;index<counts.length;index+=1){
  const count=counts[index]?? -1;
  if(!Number.isSafeInteger(count)||count<0||count>100000)return [];
  total+=count;if(count>0)result.push(index);
 }
 return total>=2&&total<=100000?result:[];
}
