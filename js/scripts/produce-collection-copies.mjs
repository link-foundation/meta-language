// CI-only producer recipe. It records actual producer output only when later
// invoked; this authored file is not a result or an execution observation.
import {collectionCopySourceAnchors as sourceAnchors,collectionCopyOriginals as originals,collectionCopyRefusals as refusals} from '../tests/fixtures/collection-copies.mjs';
import {produceCollectionControlFixture} from './collection-control-producer.mjs';
await produceCollectionControlFixture({model:'standard same-kind collection copy controls 1',originals,refusals,sourceAnchors});
