// The original reviewed policy remains inspectable Links data with its source.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readProjectData} from '../../../../js/scripts/project-data.mjs';

export const runnerPublication=readProjectData(new URL('./publication.lino',import.meta.url));
const hash=value=>createHash('sha256').update(value).digest('hex');
assert.equal(hash(runnerPublication.originalSources.find(source=>source.file==='policy.mjs').source),
  'cab0eda69d1fe2be2cbd4dde4b4997eaf70dd420eb2fe749a13b64ed5803b238');
assert.equal(hash(JSON.stringify(runnerPublication.originalPolicy)),
  '8a12deb1cd59946232fe2d9b65fdb75aa9ab214e355d935ef05539bf3e023791');
export const overlayPolicy=runnerPublication.originalPolicy;
