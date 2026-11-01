// Future CI-only producer; no execution or observation performed while drafting.
import {disjointCollectionSpreadOriginals as originals,disjointCollectionSpreadRefusals as refusals,disjointCollectionSpreadSourceAnchors as sourceAnchors} from '../tests/fixtures/disjoint-collection-spreads.mjs';
import {produceCollectionControlFixture} from './collection-control-producer.mjs';
await produceCollectionControlFixture({model:'closed eager collection spread controls 1',originals,refusals,sourceAnchors});
