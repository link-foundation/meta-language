// Literal complete-original inventory. Importing it executes no original.
import {rustExclusiveSourceAdmissionOriginals as loans} from '../tests/fixtures/rust-exclusive-source-admission.mjs';
import {rustExclusiveSourceCompoundOriginals as compounds} from '../tests/fixtures/rust-exclusive-source-compounds.mjs';
import {rustExclusiveSourceStringOriginals as strings} from '../tests/fixtures/rust-exclusive-source-strings.mjs';
import {rustExclusiveSourceVectorOriginals as vectors} from '../tests/fixtures/rust-exclusive-source-vectors.mjs';
import {rustExclusiveCollectionSourceOriginals as exclusiveCollections} from '../tests/fixtures/rust-exclusive-collection-source.mjs';
import {rustStandardCollectionOriginals as collections} from '../tests/fixtures/rust-standard-collections.mjs';
import {rustStandardCollectionEqualityOriginals as equality} from '../tests/fixtures/rust-standard-collection-equality.mjs';
import {rustStandardOrderedCollectionOriginals as orderedCollections} from '../tests/fixtures/rust-standard-ordered-collections.mjs';
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
];
