// Recursive results are reusable only within their exact seed revisions.
// Dependency cells never reference a rule, tree, result or active frame.
export function readRecursiveMemoSeed(entry,depth) {
  return entry.recursiveMemoSeed??= {depth,revision:0,keys:new Set()};
}

export function rememberRecursiveMemoDependency(entry,seed) {
  if(entry.recursiveMemoConflict)return;
  const dependencies=entry.recursiveMemoDependencies??= [];
  const known=dependencies.find(row=>row.seed===seed);
  if(known){
    if(known.revision!==seed.revision)entry.recursiveMemoConflict=true;
  }else if(dependencies.length<8){
    dependencies.push({seed,revision:seed.revision});
  }else{
    entry.recursiveMemoConflict=true;
  }
}

export function acceptRecursiveMemoEntry(entry,stack) {
  const dependencies=entry.recursiveMemoDependencies;
  return !entry.recursiveMemoConflict&&dependencies?.length>0
    &&dependencies.every(({seed,revision})=>revision===seed.revision&&stack[seed.depth]?.recursiveMemoSeed===seed);
}

export function reuseRecursiveMemoEntry(entry,stack) {
  if(!acceptRecursiveMemoEntry(entry,stack))return false;
  for(const {seed} of entry.recursiveMemoDependencies){
    for(let index=seed.depth+1;index<stack.length;index++){
      stack[index].involved=true;
      rememberRecursiveMemoDependency(stack[index],seed);
    }
  }
  return true;
}

export function advanceRecursiveMemoSeed(entry,memo) {
  const seed=entry.recursiveMemoSeed;
  if(!seed)return;
  for(const key of seed.keys){
    const known=memo.get(key);
    if(!known?.evaluating&&known?.recursiveMemoDependencies?.some(row=>row.seed===seed))memo.delete(key);
  }
  seed.keys.clear();
  seed.revision++;
}

export function finishRecursiveMemoSeed(entry,memo,stack) {
  const seed=entry.recursiveMemoSeed;
  if(!seed)return;
  // Substitute the remaining outer dependencies only when the complete
  // answer is the exact final seed. Different trees at one offset refuse.
  // An independent identical seed turns conditional entries into ordinary ones.
  const dependencies=entry.involved?entry.recursiveMemoDependencies:[];
  if((!entry.involved||acceptRecursiveMemoEntry(entry,stack))
    &&dependencies.every(row=>row.seed!==seed&&row.seed.depth<seed.depth)
    &&entry.results&&entry.results.length===entry.seed.length
    &&entry.results.every((result,index)=>result===entry.seed[index])){
    for(const key of seed.keys){
      const known=memo.get(key);
      if(!known||known.evaluating||!known.recursiveMemoDependencies?.some(row=>row.seed===seed))continue;
      const merged=[];
      let valid=true;
      for(const row of [...known.recursiveMemoDependencies.filter(row=>row.seed!==seed),...dependencies]){
        const prior=merged.find(value=>value.seed===row.seed);
        if(prior&&prior.revision!==row.revision){valid=false;break;}
        if(!prior)merged.push(row);
      }
      if(!valid||merged.length>8||!acceptRecursiveMemoEntry(known,stack)){memo.delete(key);continue;}
      if(merged.length===0)memo.set(key,{results:known.results,keywordContext:known.keywordContext});
      else{
        known.recursiveMemoDependencies=merged;
        for(const row of merged)row.seed.keys.add(key);
      }
    }
    seed.keys.clear();
    seed.revision++;
  }else advanceRecursiveMemoSeed(entry,memo);
}

export function storeRecursiveMemoEntry(entry,key,memo) {
  memo.set(key,{results:entry.results,keywordContext:entry.keywordContext,
    recursiveMemoDependencies:entry.recursiveMemoDependencies});
  for(const {seed} of entry.recursiveMemoDependencies)seed.keys.add(key);
}
