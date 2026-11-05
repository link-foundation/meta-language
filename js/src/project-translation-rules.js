// Shared project worklist decisions. Native hosts supply actual source and
// compiler results; no source stub or behavioral observation is manufactured.

/** @param {string} status @param {boolean} sameFamily @returns {boolean} */
export function acceptProjectTranslationItem(status, sameFamily) {
  return status === 'translated' || status === 'comment' || status === 'provenance'
    || sameFamily && status === 'kept';
}

/** Names are kept in source-inventory order, with one next visit per dependent.
 * @param {string[]} identifiers @param {string[]} changed
 * @param {string[]} importers @param {string[]} providers @returns {string[]}
 */
export function readProjectTranslationFrontier(identifiers, changed, importers, providers) {
  const active = new Set(changed), selected = [];
  for (let edge = 0; edge < importers.length; edge += 1) {
    const provider = providers[edge], importer = importers[edge];
    if (provider !== undefined && importer !== undefined && active.has(provider ?? '')) selected.push(importer ?? '');
  }
  const affected = new Set(selected);
  const next = [];
  for (const identifier of identifiers) if (affected.has(identifier)) next.push(identifier);
  return next;
}

/** @param {string} source @param {string} target @returns {boolean} */
export function acceptProjectTranslationSourceFamily(source, target) {
  return source === target || source === 'JavaScript' && target === 'TypeScript';
}

/** Source spelling order, including UTF-16 surrogate ordering, is retained by
 * the ordinary shared string-order operation in the native translation.
 * @param {string} first @param {string} second @returns {number}
 */
export function compareProjectTranslationIdentifiers(first, second) {
  return first < second ? -1 : first > second ? 1 : 0;
}

/** Deterministic reverse dependency witnesses. Triples are [module,next,root];
 * [-1] exhausts explicit edge and directory-search work. Append-only columns
 * preserve complete first-discovery witnesses without shared indexed writes.
 * @param {number} count @param {number[]} failed @param {number[]} importers
 * @param {number[]} providers @param {number} limit @returns {number[]} */
export function planProjectTranslationDependencyGaps(count,failed,importers,providers,limit){
 const parents=[],roots=[],queue=[],result=[];let work=0;
 for(const index of failed){work+=queue.length;if(work>limit)return [-1];if(!queue.includes(index)){parents.push(index);roots.push(index);queue.push(index);}}
 for(let at=0;at<queue.length;at+=1){const current=queue[at]??-1;
  for(let edge=0;edge<providers.length;edge+=1){
   work+=1;if(work>limit)return [-1];
   if(providers[edge]!==current)continue;
   work+=queue.length;if(work>limit)return [-1];
   const importer=importers[edge]??-1;
   if(!queue.includes(importer)){parents.push(current);roots.push(roots[at]??-1);queue.push(importer);}
  }
 }
 for(let index=0;index<count;index+=1){
  work+=queue.length;if(work>limit)return [-1];const position=queue.indexOf(index);
  if(position>=0&&parents[position]!==index){result.push(index);result.push(parents[position]??-1);result.push(roots[position]??-1);}
 }
 return result;
}
