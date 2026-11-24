// CI-only future producer. This source contains no execution observations.
import {disjointCollectionIterationOriginals as originals,disjointCollectionIterationRefusals as refusals,disjointCollectionIterationSourceAnchors as sourceAnchors} from '../tests/fixtures/disjoint-collection-iterations.mjs';
import {produceCollectionControlFixture} from './collection-control-producer.mjs';
await produceCollectionControlFixture({model:'closed disjoint collection iteration controls 1',originals,refusals,sourceAnchors});
