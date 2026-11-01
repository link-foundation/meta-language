// Deferred CI recipe. It produces observations only when later invoked.
import {closedClassPrototypeOriginals as originals,closedClassPrototypeRefusals as refusals,closedClassPrototypeSourceAnchors as sourceAnchors} from '../tests/fixtures/closed-class-prototype-methods.mjs';
import {produceCollectionControlFixture} from './collection-control-producer.mjs';
await produceCollectionControlFixture({model:'closed class prototype methods controls 1',originals,refusals,sourceAnchors});
