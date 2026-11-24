import { ByteRange, Point, SourceSpan } from './primitives.js';

// Binary strings expose one original byte per JS code unit. Link spans still
// use UTF-8 byte offsets for those strings, including their U+0080–U+00FF bytes.
export function binaryStringCoordinates(bytes) {
  const blockSize = 4096;
  const checkpoints = [{ offset: 0, row: 0, rowStart: 0 }];
  let offset = 0;
  let row = 0;
  let rowStart = 0;
  for (let index = 0; index < bytes.length; index += 1) {
    const byte = bytes[index];
    offset += byte < 128 ? 1 : 2;
    if (byte === 10) { row += 1; rowStart = offset; }
    if ((index + 1) % blockSize === 0) checkpoints.push({ offset, row, rowStart });
  }
  function at(byteOffset) {
    const block = Math.floor(byteOffset / blockSize);
    let { offset, row, rowStart } = checkpoints[block];
    for (let index = block * blockSize; index < byteOffset; index += 1) {
      const byte = bytes[index];
      offset += byte < 128 ? 1 : 2;
      if (byte === 10) { row += 1; rowStart = offset; }
    }
    return { offset, point: new Point(row, offset - rowStart) };
  }
  return (start, end) => {
    const from = at(start);
    const to = at(end);
    return new SourceSpan(new ByteRange(from.offset, to.offset), from.point, to.point);
  };
}
