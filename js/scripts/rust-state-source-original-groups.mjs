// Literal complete-original inventory. Importing it executes no original.
import {rustExclusiveSourceAdmissionOriginals as loans} from '../tests/fixtures/rust-exclusive-source-admission.mjs';
import {rustExclusiveSourceCompoundOriginals as compounds} from '../tests/fixtures/rust-exclusive-source-compounds.mjs';
import {rustExclusiveSourceStringOriginals as strings} from '../tests/fixtures/rust-exclusive-source-strings.mjs';
import {rustExclusiveSourceVectorOriginals as vectors} from '../tests/fixtures/rust-exclusive-source-vectors.mjs';
import {rustExclusiveCollectionSourceOriginals as exclusiveCollections} from '../tests/fixtures/rust-exclusive-collection-source.mjs';
import {rustStandardCollectionOriginals as collections} from '../tests/fixtures/rust-standard-collections.mjs';
import {rustStandardCollectionEqualityOriginals as equality} from '../tests/fixtures/rust-standard-collection-equality.mjs';
import {rustStandardOrderedCollectionOriginals as orderedCollections} from '../tests/fixtures/rust-standard-ordered-collections.mjs';
export const rustStateSourceOriginalGroups=[
 {id:'exclusive-loans',rows:loans,collections:false},
 {id:'primitive-compounds',rows:compounds,collections:false},
 {id:'owned-strings',rows:strings,collections:false},
 {id:'owned-vectors',rows:vectors,collections:false},
 {id:'exclusive-collections',rows:exclusiveCollections,collections:true},
 {id:'standard-collections',rows:collections,collections:true},
 {id:'collection-equality',rows:equality,collections:true},
 {id:'ordered-collections',rows:orderedCollections,collections:true},
];
