// Literal complete-original inventory. Importing it executes no original.
import {rustExclusiveSourceAdmissionOriginals as loans} from '../tests/fixtures/rust-exclusive-source-admission.mjs';
import {rustExclusiveSourceCompoundOriginals as compounds} from '../tests/fixtures/rust-exclusive-source-compounds.mjs';
import {rustExclusiveSourceStringOriginals as strings} from '../tests/fixtures/rust-exclusive-source-strings.mjs';
import {rustExclusiveSourceVectorOriginals as vectors} from '../tests/fixtures/rust-exclusive-source-vectors.mjs';
import {rustExclusiveCollectionSourceOriginals as exclusiveCollections} from '../tests/fixtures/rust-exclusive-collection-source.mjs';
import {rustStandardCollectionOriginals as collections} from '../tests/fixtures/rust-standard-collections.mjs';
import {rustStandardCollectionEqualityOriginals as equality} from '../tests/fixtures/rust-standard-collection-equality.mjs';
import {rustStandardOrderedCollectionOriginals as orderedCollections} from '../tests/fixtures/rust-standard-ordered-collections.mjs';
import {rustStandardCollectionLoopOriginals as borrowedLoops} from '../tests/fixtures/rust-standard-collection-source-loops.mjs';
import {rustSourceVectorSortOriginals as stableFieldSort} from '../tests/fixtures/rust-source-vector-sort.mjs';
import {rustSourceStringCharacterPushOriginals as characterPush} from '../tests/fixtures/rust-source-string-character-push.mjs';
import {rustOwnedVectorLoopOriginals as ownedVectorLoops} from '../tests/fixtures/rust-owned-vector-source-loops.mjs';
import {rustFnMutSourceOriginals as mutableFunctions} from '../tests/fixtures/rust-fnmut-source.mjs';
import {rustSourcePrimitiveVectorSortOriginals as primitiveVectorSort} from '../tests/fixtures/rust-source-vector-primitive-sort.mjs';
const sourceVersionRange='>=1.90.0 <1.100.0';
export const rustStateSourceOriginalGroups=[
 {id:'exclusive-loans',rows:loans,collections:false,sourceVersionRange},
 {id:'primitive-compounds',rows:compounds,collections:false,sourceVersionRange},
 {id:'owned-strings',rows:strings,collections:false,sourceVersionRange},
 {id:'owned-vectors',rows:vectors,collections:false,sourceVersionRange},
 {id:'exclusive-collections',rows:exclusiveCollections,collections:true,sourceVersionRange},
 {id:'standard-collections',rows:collections,collections:true,sourceVersionRange},
 {id:'collection-equality',rows:equality,collections:true,sourceVersionRange},
 {id:'ordered-collections',rows:orderedCollections,collections:true,sourceVersionRange},
 {id:'borrowed-ordered-collection-loops',rows:borrowedLoops,collections:true,sourceVersionRange},
 {id:'stable-utf16-nominal-field-sort',rows:stableFieldSort,collections:false,sourceVersionRange},
 {id:'literal-character-string-push',rows:characterPush,collections:false,sourceVersionRange},
 {id:'owned-vector-into-loops',rows:ownedVectorLoops,collections:false,sourceVersionRange},
 {id:'concrete-source-mutable-functions',rows:mutableFunctions,fnMut:true,collections:false,sourceVersionRange},
 {id:'primitive-vector-sort',rows:primitiveVectorSort,collections:false,sourceVersionRange},
];
