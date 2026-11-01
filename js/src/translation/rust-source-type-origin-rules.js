// Semantic clearing plan shared by JavaScript and native structural transports.
// Capture/ABI/lifetime/owner/type-validation witnesses deliberately remain.
/** Alternating existing source-origin field and clearing operation.
 * @returns {string[]} */
export function readRustSourceTypeOriginRuntimeFieldPlan(){return [
 'sourceBindingIdentity','delete','sourcePlaceBindingIdentity','delete',
 'sourceLexicalBindingIdentity','delete','sourceRecordStateCollectingSite','delete','sourceValueBorrowBindings','empty list','sourceValueFieldBorrowBindings','delete',
 'sourceLoanBindingIdentities','empty list','sourcePlaceLoanBindingIdentities','empty list',
 'sourceExtendingTemporaryBindings','empty list',
 'sourceBorrowedPlace','false','sourceOwnedResultProof','false',
 'sourceStandardSumPatternMove','false','sourceStandardSumBorrowedField','false',
 'sourceNominalPatternMove','false','sourceNominalBorrowedField','false',
 'freshArrayValue','absent fresh array',
];}
