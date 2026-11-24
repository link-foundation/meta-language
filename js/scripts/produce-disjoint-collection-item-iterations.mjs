// Future CI recipe, no execution observations authored here.
import {disjointCollectionItemIterationOriginals as originals,disjointCollectionItemIterationRefusals as refusals} from '../tests/fixtures/disjoint-collection-item-iterations.mjs';
import {produceCollectionControlFixture} from './collection-control-producer.mjs';
await produceCollectionControlFixture({model:'closed disjoint collection item iteration controls 1',originals,refusals});
