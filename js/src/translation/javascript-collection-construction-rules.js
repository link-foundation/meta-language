// The actual protected constructor kind selects internal storage independently
// of the selected newTarget prototype. Source names never establish the kind.
/** @param {string} kind @returns {boolean} */
export function testJavaScriptConstructedCollectionKind(kind){return ['Map','Set','WeakMap','WeakSet'].includes(kind);}
/** @param {string} kind @returns {string} */
export function readJavaScriptConstructedCollectionAllocationKind(kind){return ['Map','Set'].includes(kind)?'collection':['WeakMap','WeakSet'].includes(kind)?'weak collection':'';}
