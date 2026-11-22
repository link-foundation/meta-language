/** @typedef {{ $: 'sample', size: number }} Sample */
/** @typedef {Sample | { $: 'empty' }} Observation */
/** @typedef {{ $: 'directory', observations: Observation[] }} ObservationDirectory */

/** @param {Observation} observation @returns {number} */
function measureObservation(observation) {
  switch (observation.$) {
    case 'sample': return observation.size;
    default: return 0;
  }
}

/** @param {ObservationDirectory} directory @param {string} tag @returns {number} */
function countObservationTags(directory, tag) {
  let count = 0;
  for (const observation of directory.observations) {
    if (observation.$ === 'sample' && observation.$ === tag) count += 1;
  }
  return count;
}

/** @param {ObservationDirectory} directory @returns {number} */
function measureFirstObservation(directory) {
  const first = directory.observations[0];
  if (first === undefined) return 0;
  if (first.$ !== 'sample') return 0;
  return first.size;
}

/** @returns {ObservationDirectory} */
function constructObservationDirectory() {
  return { $: 'directory', observations: [{ $: 'sample', size: 7 }, { $: 'empty' }, { $: 'sample', size: 5 }] };
}

const directory = constructObservationDirectory();
console.log(measureObservation({ $: 'sample', size: 9 }));
console.log(measureFirstObservation(directory));
console.log(countObservationTags(directory, 'sample'));
console.log(countObservationTags(directory, 'empty'));
