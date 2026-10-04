// Times the JavaScript runtime observation and one parity comparison.
import { mismatchedSections } from '../js/scripts/issue-195-parity-evidence.mjs';
import { runtimeObservation } from '../js/scripts/issue-195-runtime-observation.mjs';

let start = performance.now();
const observation = runtimeObservation();
console.log('observation ms', Math.round(performance.now() - start));
const copy = structuredClone(observation);
start = performance.now();
mismatchedSections(observation, copy);
console.log('comparison ms', Math.round(performance.now() - start));
