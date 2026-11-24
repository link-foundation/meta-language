// CI-only future controls. Never execute while requirements are still drafted.
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {assertVersions,heapFixture,observeHeapCase,observeInvalidHeap} from './heap-controls.mjs';
assertVersions();
process.stdout.write(structuredDataToLino({cases:heapFixture.cases.map(observeHeapCase),invalid:heapFixture.invalid.map(observeInvalidHeap)}));
