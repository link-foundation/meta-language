import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';

const digest = artifact => createHash('sha256').update(structuredDataToLino(artifact)).digest('hex');
const historicalDigest = 'c54e33d7bd0c4b22301bd6cefdd719494ec87260f8c35771c874858b2d872eb4';
const currentDigest = '468566b3bd80bccc46a078d89dd8eef801f2189053927e7674b1d95cb6faac55';

/** Invert the reviewed archive encoding change after both current archive replays. */
export function restoreRawConsumingArchiveReceipt(artifact) {
  assert.equal(digest(artifact), currentDigest, 'the complete replayed raw archive successor is exact');
  const restored = {
    ...artifact,
    archive: {
      ...artifact.archive,
      textBytes: 8864557,
      textSha256: '1b6ab57437e999959ead8d158f69e94ca1060bd74c46604dee57a49fd7aa5303',
      binarySha256: '3bcca11784d5029c4e0e432ab2698395e397afe621102ae19e4f3ec6da191bf2',
    },
  };
  assert.equal(digest(restored), historicalDigest, 'the three receipt fields restore every original artifact byte');
  return restored;
}
