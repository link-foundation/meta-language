// Future CI producer recipe, never executed during source authoring.
import {closedClassCollectionDefaultOriginals as originals,closedClassCollectionDefaultRefusals as refusals} from '../tests/fixtures/closed-class-collection-defaults.mjs';
import {produceCollectionControlFixture} from './collection-control-producer.mjs';
await produceCollectionControlFixture({model:'closed class collection defaults controls 1',originals,refusals});
